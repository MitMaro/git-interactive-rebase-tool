use version_track::Version;

use crate::todo_file::{Action, TodoFile};

/// Search handler for the todofile
#[derive(Debug)]
#[allow(clippy::struct_field_names)]
pub(crate) struct Search {
	match_start_hint: usize,
	matches: Vec<usize>,
	rebase_todo_version: Version,
	search_term: String,
	selected: Option<usize>,
}

impl Search {
	/// Create a new instance
	#[must_use]
	pub(crate) const fn new() -> Self {
		Self {
			match_start_hint: 0,
			matches: vec![],
			rebase_todo_version: Version::sentinel(),
			search_term: String::new(),
			selected: None,
		}
	}

	/// Generate search results
	pub(crate) fn search(&mut self, rebase_todo: &TodoFile, term: &str) -> bool {
		if &self.rebase_todo_version != rebase_todo.version() || self.search_term != term || self.matches.is_empty() {
			self.matches.clear();
			self.selected = None;
			self.search_term = String::from(term);
			self.rebase_todo_version = *rebase_todo.version();
			for (i, line) in rebase_todo.lines_iter().enumerate() {
				match *line.get_action() {
					Action::Break | Action::Noop => continue,
					Action::Drop
					| Action::Edit
					| Action::Fixup
					| Action::Pick
					| Action::Reword
					| Action::Squash
					| Action::UpdateRef => {
						if line.get_hash().starts_with(term) || line.get_content().contains(term) {
							self.matches.push(i);
						}
					},
					Action::Label | Action::Reset | Action::Merge | Action::Exec => {
						if line.get_content().contains(term) {
							self.matches.push(i);
						}
					},
				}
			}
		}
		!self.matches.is_empty()
	}

	/// Select the next search result
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn next(&mut self, rebase_todo: &TodoFile, term: &str) {
		if !self.search(rebase_todo, term) {
			return;
		}

		if let Some(mut current) = self.selected {
			current += 1;
			let new_value = if current >= self.matches.len() { 0 } else { current };
			self.selected = Some(new_value);
		}
		else {
			// select the line after the hint that matches
			let mut index_match = 0;
			for (i, v) in self.matches.iter().enumerate() {
				if *v >= self.match_start_hint {
					index_match = i;
					break;
				}
			}
			self.selected = Some(index_match);
		};

		self.match_start_hint = self.matches[self.selected.unwrap()];
	}

	/// Select the previous search result
	#[allow(clippy::missing_panics_doc)]
	pub(crate) fn previous(&mut self, rebase_todo: &TodoFile, term: &str) {
		if !self.search(rebase_todo, term) {
			return;
		}

		if let Some(current) = self.selected {
			let new_value = if current == 0 {
				self.matches.len().saturating_sub(1)
			}
			else {
				current.saturating_sub(1)
			};
			self.selected = Some(new_value);
		}
		else {
			// select the line previous to hint that matches
			let mut index_match = self.matches.len().saturating_sub(1);
			for (i, v) in self.matches.iter().enumerate().rev() {
				if *v <= self.match_start_hint {
					index_match = i;
					break;
				}
			}
			self.selected = Some(index_match);
		}

		self.match_start_hint = self.matches[self.selected.unwrap()];
	}

	/// Set a hint for which result to select first during search
	pub(crate) fn set_search_start_hint(&mut self, hint: usize) {
		if self.match_start_hint != hint {
			self.match_start_hint = hint;
		}
	}

	/// Invalidate current search results
	pub(crate) fn invalidate(&mut self) {
		self.matches.clear();
	}

	/// Cancel search, clearing results, selected result and search term
	pub(crate) fn cancel(&mut self) {
		self.selected = None;
		self.search_term.clear();
		self.matches.clear();
	}

	/// Get the index of the current selected result, if there is one
	#[must_use]
	pub(crate) fn current_match(&self) -> Option<usize> {
		let selected = self.selected?;
		self.matches.get(selected).copied()
	}

	/// Get the selected result number, if there is one
	#[must_use]
	pub(crate) const fn current_result_selected(&self) -> Option<usize> {
		self.selected
	}

	/// Get the total number of results
	#[must_use]
	pub(crate) fn total_results(&self) -> usize {
		self.matches.len()
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_none, assert_some_eq};

	use super::*;
	use crate::todo_file::testutil::with_todo_file;

	#[test]
	fn search_empty_rebase_file() {
		with_todo_file(&[], |context| {
			let mut search = Search::new();
			assert!(!search.search(context.todo_file(), "foo"));
		});
	}

	#[test]
	fn search_with_one_line_no_match() {
		with_todo_file(&["pick abcdef bar"], |context| {
			let mut search = Search::new();
			assert!(!search.search(context.todo_file(), "foo"));
		});
	}

	#[test]
	fn search_with_one_line_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			assert!(search.search(context.todo_file(), "foo"));
		});
	}

	#[test]
	fn search_ignore_break() {
		with_todo_file(&["break"], |context| {
			let mut search = Search::new();
			assert!(!search.search(context.todo_file(), "break"));
		});
	}

	#[test]
	fn search_ignore_noop() {
		with_todo_file(&["noop"], |context| {
			let mut search = Search::new();
			assert!(!search.search(context.todo_file(), "noop"));
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
				let mut search = Search::new();
				assert!(search.search(context.todo_file(), "abcd"));
				assert_eq!(search.total_results(), 6);
			},
		);
	}

	#[test]
	fn search_standard_action_content() {
		with_todo_file(
			&[
				"pick abcdef no match",
				"drop abcdef foobar",
				"edit abcdef foobar",
				"fixup abcdef foobar",
				"pick abcdef foobar",
				"reword abcdef foobar",
				"squash abcdef foobar",
			],
			|context| {
				let mut search = Search::new();
				assert!(search.search(context.todo_file(), "ooba"));
				assert_eq!(search.total_results(), 6);
			},
		);
	}

	#[test]
	fn search_standard_action_hash_starts_only() {
		with_todo_file(&["pick abcdef foobar"], |context| {
			let mut search = Search::new();
			assert!(!search.search(context.todo_file(), "def"));
		});
	}

	#[test]
	fn search_standard_ignore_action() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			assert!(!search.search(context.todo_file(), "pick"));
		});
	}

	#[test]
	fn search_editable_content() {
		with_todo_file(
			&[
				"label no match",
				"label foobar",
				"reset foobar",
				"merge foobar",
				"exec foobar",
				"update-ref foobar",
			],
			|context| {
				let mut search = Search::new();
				assert!(search.search(context.todo_file(), "ooba"));
				assert_eq!(search.total_results(), 5);
			},
		);
	}

	#[test]
	fn search_editable_ignore_action() {
		with_todo_file(&["label no match"], |context| {
			let mut search = Search::new();
			assert!(!search.search(context.todo_file(), "label"));
		});
	}

	#[test]
	fn next_no_match() {
		with_todo_file(&["pick aaa foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "miss");
			assert_none!(search.current_match());
		});
	}

	#[test]
	fn next_first_match() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 0);
		});
	}

	#[test]
	fn next_first_match_with_hint_in_range() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let mut search = Search::new();
			search.set_search_start_hint(1);
			search.next(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 1);
		});
	}

	#[test]
	fn next_first_match_with_hint_in_range_but_behind() {
		with_todo_file(&["pick aaa foo", "pick bbb miss", "pick bbb foobar"], |context| {
			let mut search = Search::new();
			search.set_search_start_hint(1);
			search.next(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 2);
		});
	}

	#[test]
	fn next_first_match_with_hint_in_range_wrap() {
		with_todo_file(
			&["pick bbb miss", "pick aaa foo", "pick aaa foo", "pick bbb miss"],
			|context| {
				let mut search = Search::new();
				search.set_search_start_hint(3);
				search.next(context.todo_file(), "foo");
				assert_some_eq!(search.current_match(), 1);
			},
		);
	}

	#[test]
	fn next_first_match_with_hint_out_of_range() {
		with_todo_file(
			&["pick bbb miss", "pick aaa foo", "pick aaa foo", "pick bbb miss"],
			|context| {
				let mut search = Search::new();
				search.set_search_start_hint(99);
				search.next(context.todo_file(), "foo");
				assert_some_eq!(search.current_match(), 1);
			},
		);
	}

	#[test]
	fn next_continued_match() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			search.next(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 1);
		});
	}

	#[test]
	fn next_continued_match_wrap_single_match() {
		with_todo_file(&["pick aaa foo", "pick bbb miss"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			search.next(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 0);
		});
	}

	#[test]
	fn next_continued_match_wrap() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			search.next(context.todo_file(), "foo");
			search.next(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 0);
		});
	}

	#[test]
	fn next_updates_match_start_hint() {
		with_todo_file(&["pick bbb miss", "pick aaa foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			assert_eq!(search.match_start_hint, 1);
		});
	}

	#[test]
	fn previous_no_match() {
		with_todo_file(&["pick aaa foo"], |context| {
			let mut search = Search::new();
			search.previous(context.todo_file(), "miss");
			assert_none!(search.current_match());
		});
	}

	#[test]
	fn previous_first_match() {
		with_todo_file(&["pick aaa foo"], |context| {
			let mut search = Search::new();
			search.previous(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 0);
		});
	}

	#[test]
	fn previous_first_match_with_hint_in_range() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let mut search = Search::new();
			search.set_search_start_hint(1);
			search.previous(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 1);
		});
	}

	#[test]
	fn previous_first_match_with_hint_in_range_but_ahead() {
		with_todo_file(
			&["pick bbb miss", "pick aaa foo", "pick bbb miss", "pick bbb foobar"],
			|context| {
				let mut search = Search::new();
				search.set_search_start_hint(2);
				search.previous(context.todo_file(), "foo");
				assert_some_eq!(search.current_match(), 1);
			},
		);
	}

	#[test]
	fn previous_first_match_with_hint_in_range_wrap() {
		with_todo_file(
			&["pick bbb miss", "pick bbb miss", "pick aaa foo", "pick aaa foo"],
			|context| {
				let mut search = Search::new();
				search.set_search_start_hint(1);
				search.previous(context.todo_file(), "foo");
				assert_some_eq!(search.current_match(), 3);
			},
		);
	}

	#[test]
	fn previous_first_match_with_hint_out_of_range() {
		with_todo_file(
			&["pick bbb miss", "pick aaa foo", "pick aaa foo", "pick bbb miss"],
			|context| {
				let mut search = Search::new();
				search.set_search_start_hint(99);
				search.previous(context.todo_file(), "foo");
				assert_some_eq!(search.current_match(), 2);
			},
		);
	}

	#[test]
	fn previous_continued_match() {
		with_todo_file(&["pick aaa foo", "pick aaa foo", "pick bbb foobar"], |context| {
			let mut search = Search::new();
			search.set_search_start_hint(2);
			search.previous(context.todo_file(), "foo");
			search.previous(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 1);
		});
	}

	#[test]
	fn previous_continued_match_wrap_single_match() {
		with_todo_file(&["pick aaa foo", "pick bbb miss"], |context| {
			let mut search = Search::new();
			search.previous(context.todo_file(), "foo");
			search.previous(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 0);
		});
	}

	#[test]
	fn previous_continued_match_wrap() {
		with_todo_file(&["pick aaa foo", "pick bbb foobar"], |context| {
			let mut search = Search::new();
			search.previous(context.todo_file(), "foo");
			search.previous(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 1);
		});
	}

	#[test]
	fn previous_updates_match_start_hint() {
		with_todo_file(&["pick bbb miss", "pick aaa foo"], |context| {
			let mut search = Search::new();
			search.previous(context.todo_file(), "foo");
			assert_eq!(search.match_start_hint, 1);
		});
	}

	#[test]
	fn invalidate() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			search.invalidate();
			assert_eq!(search.total_results(), 0);
		});
	}

	#[test]
	fn cancel() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			search.cancel();
			assert_eq!(search.total_results(), 0);
			assert_none!(search.current_match());
			assert!(search.search_term.is_empty());
		});
	}

	#[test]
	fn current_match_with_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			assert_some_eq!(search.current_match(), 0);
		});
	}

	#[test]
	fn current_match_with_no_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "miss");
			assert_none!(search.current_match());
		});
	}

	#[test]
	fn current_result_selected_with_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			assert_some_eq!(search.current_result_selected(), 0);
		});
	}

	#[test]
	fn current_result_selected_with_no_match() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "miss");
			assert_none!(search.current_result_selected());
		});
	}

	#[test]
	fn total_results() {
		with_todo_file(&["pick abcdef foo"], |context| {
			let mut search = Search::new();
			search.next(context.todo_file(), "foo");
			assert_eq!(search.total_results(), 1);
		});
	}
}
