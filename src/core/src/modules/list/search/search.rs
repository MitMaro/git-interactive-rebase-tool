use std::{
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc,
	},
	time::Duration,
};

use parking_lot::{Mutex, RwLock};
use todo_file::{Action, TodoFile};

use super::{LineMatch, State};
use crate::search::{Interrupter, SearchResult, SearchState, Searchable};

const LOCK_DURATION: Duration = Duration::from_millis(100);

#[derive(Clone, Debug)]
pub(crate) struct Search {
	cursor: Arc<AtomicUsize>,
	state: Arc<RwLock<State>>,
	todo_file: Arc<Mutex<TodoFile>>,
}

impl Searchable for Search {
	fn reset(&mut self) {
		self.state.write().reset();
	}

	fn search(&mut self, interrupter: Interrupter, term: &str) -> SearchResult {
		let Some(todo_file) = self.todo_file.try_lock_for(LOCK_DURATION)
		else {
			return SearchResult::None;
		};
		let Some(mut state) = self.state.try_write_for(LOCK_DURATION)
		else {
			return SearchResult::None;
		};
		if state.try_invalidate_search(todo_file.version(), term) {
			self.cursor.store(0, Ordering::Release);
		}
		let mut has_matches = false;
		let mut complete = false;

		state.set_search_state(SearchState::Active);
		let mut cursor = self.cursor.load(Ordering::Acquire);
		while interrupter.should_continue() {
			let Some(line) = todo_file.get_line(cursor)
			else {
				complete = true;
				break;
			};

			let action = *line.get_action();

			let has_hash_match = match action {
				Action::Break | Action::Noop | Action::Label | Action::Reset | Action::Merge | Action::Exec => false,
				Action::Drop
				| Action::Edit
				| Action::Fixup
				| Action::Pick
				| Action::Reword
				| Action::Squash
				| Action::UpdateRef => line.get_hash().starts_with(term),
			};
			let has_content_match = match action {
				Action::Break | Action::Noop => false,
				Action::Drop
				| Action::Edit
				| Action::Fixup
				| Action::Pick
				| Action::Reword
				| Action::Squash
				| Action::UpdateRef
				| Action::Label
				| Action::Reset
				| Action::Merge
				| Action::Exec => line.get_content().contains(term),
			};

			has_matches = state.push_match(LineMatch::new(cursor, has_hash_match, has_content_match)) || has_matches;

			cursor += 1;
		}

		self.cursor.store(cursor, Ordering::Release);

		if has_matches {
			SearchResult::Updated
		}
		else if complete {
			state.set_search_state(SearchState::Complete);
			SearchResult::Complete
		}
		else {
			SearchResult::None
		}
	}
}

impl Search {
	/// Create a new instance
	#[inline]
	#[must_use]
	pub(crate) fn new(todo_file: Arc<Mutex<TodoFile>>) -> Self {
		Self {
			cursor: Arc::new(AtomicUsize::new(0)),
			state: Arc::new(RwLock::new(State::new())),
			todo_file,
		}
	}

	/// Select the next search result
	#[inline]
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn next(&mut self) -> Option<usize> {
		let mut state = self.state.write();

		if state.matches().is_empty() {
			return None;
		}

		let new_selected = if let Some(mut current) = state.selected() {
			current += 1;
			if current >= state.number_matches() { 0 } else { current }
		}
		else {
			// select the line after the hint that matches
			let mut index_match = 0;
			for (i, v) in state.matches().iter().copied().enumerate() {
				if v.index() >= state.match_start_hint() {
					index_match = i;
					break;
				}
			}
			index_match
		};
		state.set_selected(new_selected);

		let new_match_hint = state.match_value(new_selected).map_or(0, |s| s.index());
		state.set_match_start_hint(new_match_hint);
		Some(new_match_hint)
	}

	/// Select the previous search result
	#[inline]
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn previous(&mut self) -> Option<usize> {
		let mut state = self.state.write();
		if state.matches().is_empty() {
			return None;
		}

		let new_selected = if let Some(current) = state.selected() {
			if current == 0 {
				state.number_matches().saturating_sub(1)
			}
			else {
				current.saturating_sub(1)
			}
		}
		else {
			// select the line previous to hint that matches
			let mut index_match = state.number_matches().saturating_sub(1);
			for (i, v) in state.matches().iter().copied().enumerate().rev() {
				if v.index() <= state.match_start_hint() {
					index_match = i;
					break;
				}
			}
			index_match
		};
		state.set_selected(new_selected);

		let new_match_hint = state.match_value(new_selected).map_or(0, |s| s.index());
		state.set_match_start_hint(new_match_hint);
		Some(new_match_hint)
	}

	/// Set a hint for which result to select first during search
	#[inline]
	pub(crate) fn set_search_start_hint(&mut self, hint: usize) {
		self.state.write().set_match_start_hint(hint);
	}

	/// Get the index of the current selected result, if there is one
	#[inline]
	#[must_use]
	pub(crate) fn current_match(&self) -> Option<LineMatch> {
		let state = self.state.read();
		let selected = state.selected()?;
		state.match_value(selected)
	}

	/// Get the index of the current selected result, if there is one
	#[inline]
	#[must_use]
	pub(crate) fn match_at_index(&self, index: usize) -> Option<LineMatch> {
		self.state.read().match_value_for_line(index)
	}

	/// Get the selected result number, if there is one
	#[inline]
	#[must_use]
	pub(crate) fn current_result_selected(&self) -> Option<usize> {
		self.state.read().selected()
	}

	/// Get the total number of results
	#[inline]
	#[must_use]
	pub(crate) fn total_results(&self) -> usize {
		self.state.read().number_matches()
	}

	/// Is search active
	#[inline]
	#[must_use]
	pub(crate) fn is_active(&self) -> bool {
		self.state.read().search_state() == SearchState::Active
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_none, assert_some_eq};
	use rstest::rstest;
	use todo_file::testutil::with_todo_file;

	use super::*;
	use crate::{modules::list::search::search, search::testutil::SearchableRunner};

	pub(crate) fn create_search(todo_file: TodoFile) -> Search {
		Search::new(Arc::new(Mutex::new(todo_file)))
	}

	pub(crate) fn create_and_run_search(todo_file: TodoFile, term: &str, result: SearchResult) -> Search {
		let search = Search::new(Arc::new(Mutex::new(todo_file)));
		assert_eq!(SearchableRunner::new(&search).run_search(term), result);
		search
	}

	#[test]
	fn reset() {
		with_todo_file(&[], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			search.state.write().set_search_state(SearchState::Active);
			search.reset();
			assert!(!search.is_active());
		});
	}

	#[test]
	fn search_empty_rebase_file() {
		with_todo_file(&[], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_and_run_search(todo_file, "foo", SearchResult::Complete);
			assert_eq!(search.total_results(), 0);
		});
	}

	#[test]
	fn search_with_one_line_no_match() {
		with_todo_file(&["pick abcdef bar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_and_run_search(todo_file, "foo", SearchResult::Complete);
			assert_eq!(search.total_results(), 0);
		});
	}

	#[test]
	fn search_with_incomplete() {
		with_todo_file(&["pick abcdef bar"; 10], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(
				SearchableRunner::new(&search).run_search_with_time("foo", 0),
				SearchResult::None
			);
			assert_eq!(search.total_results(), 0);
		});
	}

	#[test]
	fn search_with_one_line_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_and_run_search(todo_file, "foo", SearchResult::Updated);
			assert_eq!(search.total_results(), 1);
			assert_some_eq!(search.match_at_index(0), LineMatch::new(0, false, true));
		});
	}

	#[test]
	fn search_standard_action_hash() {
		with_todo_file(
			&[
				"pick aaaaa no match",
				"drop abcdef foo",
				"edit abcdef foo",
				"fixup abcdef foo",
				"pick abcdef foo",
				"reword abcdef foo",
				"squash abcdef foo",
			],
			|context| {
				let (_todo_file_path, todo_file) = context.to_owned();
				let mut search = create_and_run_search(todo_file, "abcd", SearchResult::Updated);
				assert_eq!(search.total_results(), 6);
				assert_none!(search.match_at_index(0));
				assert_some_eq!(search.match_at_index(1), LineMatch::new(1, true, false));
				assert_some_eq!(search.match_at_index(2), LineMatch::new(2, true, false));
				assert_some_eq!(search.match_at_index(3), LineMatch::new(3, true, false));
				assert_some_eq!(search.match_at_index(4), LineMatch::new(4, true, false));
				assert_some_eq!(search.match_at_index(5), LineMatch::new(5, true, false));
			},
		);
	}

	#[test]
	fn search_content() {
		with_todo_file(
			&[
				"pick abcdef no match",
				"drop abcdef foobar",
				"edit abcdef foobar",
				"fixup abcdef foobar",
				"pick abcdef foobar",
				"reword abcdef foobar",
				"squash abcdef foobar",
				"label foobar",
				"reset foobar",
				"merge foobar",
				"exec foobar",
				"update-ref foobar",
			],
			|context| {
				let (_todo_file_path, todo_file) = context.to_owned();
				let mut search = create_and_run_search(todo_file, "ooba", SearchResult::Updated);
				assert_eq!(search.total_results(), 11);
				assert_none!(search.match_at_index(0));
				assert_some_eq!(search.match_at_index(1), LineMatch::new(1, false, true));
				assert_some_eq!(search.match_at_index(2), LineMatch::new(2, false, true));
				assert_some_eq!(search.match_at_index(3), LineMatch::new(3, false, true));
				assert_some_eq!(search.match_at_index(4), LineMatch::new(4, false, true));
				assert_some_eq!(search.match_at_index(5), LineMatch::new(5, false, true));
				assert_some_eq!(search.match_at_index(6), LineMatch::new(6, false, true));
				assert_some_eq!(search.match_at_index(7), LineMatch::new(7, false, true));
				assert_some_eq!(search.match_at_index(8), LineMatch::new(8, false, true));
				assert_some_eq!(search.match_at_index(9), LineMatch::new(9, false, true));
				assert_some_eq!(search.match_at_index(10), LineMatch::new(10, false, true));
			},
		);
	}

	#[test]
	fn search_standard_action_hash_starts_only() {
		with_todo_file(&["pick abcdef foobar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_and_run_search(todo_file, "def", SearchResult::Complete);
			assert_eq!(search.total_results(), 0);
		});
	}

	#[rstest]
	#[case::pick("noop")]
	#[case::pick("break")]
	#[case::pick("pick")]
	#[case::drop("drop")]
	#[case::edit("edit")]
	#[case::fixup("fixup")]
	#[case::reword("reword")]
	#[case::squash("squash")]
	#[case::label("label")]
	#[case::reset("reset")]
	#[case::merge("merge")]
	#[case::exec("exec")]
	#[case::update_ref("update-ref")]
	fn search_ignore_action(#[case] action: &str) {
		let line = format!("{action} abcdef foo");
		with_todo_file(&[line.as_str()], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_and_run_search(todo_file, action, SearchResult::Complete);
			assert_eq!(search.total_results(), 0);
		});
	}

	#[test]
	fn next_no_match() {
		with_todo_file(&["pick aaa foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_and_run_search(todo_file, "miss", SearchResult::Complete);
			assert_none!(search.next());
			assert_none!(search.current_match());
		});
	}

	#[test]
	fn next_first_match() {
		with_todo_file(&["pick aaa foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_and_run_search(todo_file, "foo", SearchResult::Updated);
			assert_some_eq!(search.next(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
		});
	}

	#[test]
	fn next_first_match_with_hint_in_range() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			search.set_search_start_hint(1);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.next(), 1);
			assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
		});
	}

	#[test]
	fn next_first_match_with_hint_in_range_but_behind() {
		with_todo_file(&["pick aaa foo", "pick bbb miss", "pick bbb foobar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			search.set_search_start_hint(1);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.next(), 2);
			assert_some_eq!(search.current_match(), LineMatch::new(2, false, true));
		});
	}

	#[test]
	fn next_first_match_with_hint_in_range_wrap() {
		with_todo_file(
			&["pick bbb miss", "pick aaa foo", "pick aaa foo", "pick bbb miss"],
			|context| {
				let (_todo_file_path, todo_file) = context.to_owned();
				let mut search = create_search(todo_file);
				search.set_search_start_hint(3);
				assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
				assert_some_eq!(search.next(), 1);
				assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
			},
		);
	}

	#[test]
	fn next_first_match_with_hint_out_of_range() {
		with_todo_file(
			&["pick bbb miss", "pick aaa foo", "pick aaa foo", "pick bbb miss"],
			|context| {
				let (_todo_file_path, todo_file) = context.to_owned();
				let mut search = create_search(todo_file);
				search.set_search_start_hint(99);
				assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
				assert_some_eq!(search.next(), 1);
				assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
			},
		);
	}

	#[test]
	fn next_continued_match() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.next(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
			assert_some_eq!(search.next(), 1);
			assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
		});
	}

	#[test]
	fn next_continued_match_wrap_single_match() {
		with_todo_file(&["pick aaa foo", "pick bbb miss"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.next(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
			assert_some_eq!(search.next(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
		});
	}

	#[test]
	fn next_continued_match_wrap() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.next(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
			assert_some_eq!(search.next(), 1);
			assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
			assert_some_eq!(search.next(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
		});
	}

	#[test]
	fn next_updates_match_start_hint() {
		with_todo_file(&["pick bbb miss", "pick aaa foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			search.set_search_start_hint(99);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.next(), 1);
			assert_eq!(search.state.read().match_start_hint(), 1);
		});
	}

	#[test]
	fn previous_no_match() {
		with_todo_file(&["pick aaa foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(
				SearchableRunner::new(&search).run_search("miss"),
				SearchResult::Complete
			);
			assert_none!(search.previous());
			assert_none!(search.current_match());
		});
	}

	#[test]
	fn previous_first_match() {
		with_todo_file(&["pick aaa foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.previous(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
		});
	}

	#[test]
	fn previous_first_match_with_hint_in_range() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			search.set_search_start_hint(1);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.previous(), 1);
			assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
		});
	}

	#[test]
	fn previous_first_match_with_hint_in_range_but_ahead() {
		with_todo_file(
			&["pick bbb miss", "pick aaa foo", "pick bbb miss", "pick bbb foobar"],
			|context| {
				let (_todo_file_path, todo_file) = context.to_owned();
				let mut search = create_search(todo_file);
				search.set_search_start_hint(2);
				assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
				assert_some_eq!(search.previous(), 1);
				assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
			},
		);
	}
	#[test]
	fn previous_first_match_with_hint_in_range_wrap() {
		with_todo_file(
			&["pick bbb miss", "pick bbb miss", "pick aaa foo", "pick aaa foo"],
			|context| {
				let (_todo_file_path, todo_file) = context.to_owned();
				let mut search = create_search(todo_file);
				search.set_search_start_hint(1);
				assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
				assert_some_eq!(search.previous(), 3);
				assert_some_eq!(search.current_match(), LineMatch::new(3, false, true));
			},
		);
	}

	#[test]
	fn previous_first_match_with_hint_out_of_range() {
		with_todo_file(
			&["pick bbb miss", "pick aaa foo", "pick aaa foo", "pick bbb miss"],
			|context| {
				let (_todo_file_path, todo_file) = context.to_owned();
				let mut search = create_search(todo_file);
				search.set_search_start_hint(99);
				assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
				assert_some_eq!(search.previous(), 2);
				assert_some_eq!(search.current_match(), LineMatch::new(2, false, true));
			},
		);
	}

	#[test]
	fn previous_continued_match() {
		with_todo_file(&["pick aaa foo", "pick aaa foo", "pick bbb foobar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			search.set_search_start_hint(2);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.previous(), 2);
			assert_some_eq!(search.current_match(), LineMatch::new(2, false, true));
			assert_some_eq!(search.previous(), 1);
			assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
		});
	}

	#[test]
	fn previous_continued_match_wrap_single_match() {
		with_todo_file(&["pick aaa foo", "pick bbb miss"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.previous(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
			assert_some_eq!(search.previous(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
		});
	}

	#[test]
	fn previous_continued_match_wrap() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.previous(), 0);
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
			assert_some_eq!(search.previous(), 1);
			assert_some_eq!(search.current_match(), LineMatch::new(1, false, true));
		});
	}

	#[test]
	fn previous_updates_match_start_hint() {
		with_todo_file(&["pick bbb miss", "pick aaa foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			assert_some_eq!(search.previous(), 1);
			assert_eq!(search.state.read().match_start_hint(), 1);
		});
	}

	#[test]
	fn set_search_start_hint() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			search.set_search_start_hint(42);
			assert_eq!(search.state.read().match_start_hint(), 42);
		});
	}

	#[test]
	fn current_match_without_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(
				SearchableRunner::new(&search).run_search("miss"),
				SearchResult::Complete
			);
			_ = search.next();
			assert_none!(search.current_match());
		});
	}
	#[test]
	fn current_match_with_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			_ = search.next();
			assert_some_eq!(search.current_match(), LineMatch::new(0, false, true));
		});
	}

	#[test]
	fn match_at_index_without_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(
				SearchableRunner::new(&search).run_search("miss"),
				SearchResult::Complete
			);
			_ = search.next();
			assert_none!(search.match_at_index(0));
		});
	}
	#[test]
	fn match_at_index_with_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			_ = search.next();
			assert_some_eq!(search.match_at_index(0), LineMatch::new(0, false, true));
		});
	}

	#[test]
	fn current_result_selected_without_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(
				SearchableRunner::new(&search).run_search("miss"),
				SearchResult::Complete
			);
			_ = search.next();
			assert_none!(search.current_result_selected());
		});
	}

	#[test]
	fn current_result_selected_with_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			_ = search.next();
			assert_some_eq!(search.current_result_selected(), 0);
		});
	}

	#[test]
	fn total_results() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			assert_eq!(SearchableRunner::new(&search).run_search("foo"), SearchResult::Updated);
			_ = search.next();
			assert_eq!(search.total_results(), 1);
		});
	}

	#[test]
	fn is_active() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let (_todo_file_path, todo_file) = context.to_owned();
			let mut search = create_search(todo_file);
			search.state.write().set_search_state(SearchState::Active);
			assert!(search.is_active());
		});
	}
}
