use std::collections::HashMap;

use version_track::Version;

use super::line_match::LineMatch;
use crate::search::Status;

/// Input thread state.
#[derive(Clone, Debug)]
pub(crate) struct State {
	match_indexes: HashMap<usize, usize>,
	match_start_hint: usize,
	matches: Vec<LineMatch>,
	search_term: String,
	selected: Option<usize>,
	status: Status,
	todo_file_version: Version,
}

impl State {
	pub(crate) fn new() -> Self {
		Self {
			match_indexes: HashMap::new(),
			match_start_hint: 0,
			matches: vec![],
			search_term: String::new(),
			selected: None,
			status: Status::Inactive,
			todo_file_version: Version::sentinel(),
		}
	}

	pub(crate) fn reset(&mut self) {
		self.match_indexes.clear();
		self.matches.clear();
		self.search_term.clear();
		self.selected = None;
		self.status = Status::Inactive;
		self.todo_file_version = Version::sentinel();
	}

	pub(crate) fn try_invalidate_search(&mut self, version: &Version, search_term: &str) -> bool {
		if &self.todo_file_version != version || self.search_term != search_term {
			self.search_term = String::from(search_term);
			self.matches.clear();
			self.match_indexes.clear();
			self.todo_file_version = *version;
			true
		}
		else {
			false
		}
	}

	pub(crate) const fn status(&self) -> Status {
		self.status
	}

	pub(crate) fn set_status(&mut self, status: Status) {
		self.status = status;
	}

	pub(crate) fn push_match(&mut self, line_match: LineMatch) -> bool {
		if line_match.hash() || line_match.content() {
			_ = self.match_indexes.insert(line_match.index(), self.matches.len());
			self.matches.push(line_match);
			true
		}
		else {
			false
		}
	}

	pub(crate) const fn matches(&self) -> &Vec<LineMatch> {
		&self.matches
	}

	pub(crate) fn number_matches(&self) -> usize {
		self.matches.len()
	}

	/// Returns the match value for a line index
	pub(crate) fn match_value_for_line(&self, index: usize) -> Option<LineMatch> {
		let search_index = *self.match_indexes.get(&index)?;
		self.match_value(search_index)
	}

	/// Returns the match value for a search match index
	pub(crate) fn match_value(&self, search_index: usize) -> Option<LineMatch> {
		self.matches.get(search_index).copied()
	}

	pub(crate) fn set_selected(&mut self, selected: usize) {
		self.selected = Some(selected);
	}

	pub(crate) const fn selected(&self) -> Option<usize> {
		self.selected
	}

	pub(crate) fn set_match_start_hint(&mut self, hint: usize) {
		self.match_start_hint = hint;
	}

	pub(crate) const fn match_start_hint(&self) -> usize {
		self.match_start_hint
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_none, assert_some_eq};

	use super::*;

	#[test]
	fn try_invalidate_search_with_no_change() {
		let mut state = State::new();
		assert!(!state.try_invalidate_search(&Version::sentinel(), ""));
	}

	#[test]
	fn try_invalidate_search_with_change_in_term() {
		let mut state = State::new();
		assert!(state.try_invalidate_search(&Version::sentinel(), "foo"));
	}

	#[test]
	fn try_invalidate_search_with_change_in_version() {
		let mut state = State::new();
		assert!(state.try_invalidate_search(&Version::new(), ""));
	}

	#[test]
	fn try_invalidate_search_resets_state() {
		let mut state = State::new();
		state.matches.push(LineMatch::new(1, false, false));
		_ = state.match_indexes.insert(1, 1);
		let version = Version::new();
		assert!(state.try_invalidate_search(&version, "foo"));
		assert_eq!(state.search_term, "foo");
		assert!(state.matches().is_empty());
		assert!(state.match_indexes.is_empty());
		assert_eq!(state.todo_file_version, version);
	}

	#[test]
	fn search_status() {
		let mut state = State::new();
		state.set_status(Status::Active);
		assert_eq!(state.status(), Status::Active);
	}

	#[test]
	fn push_match_with_hash_match() {
		let mut state = State::new();
		assert!(state.push_match(LineMatch::new(1, true, false)));
		assert!(!state.matches().is_empty());
		assert_eq!(state.number_matches(), 1);
	}

	#[test]
	fn push_match_with_content_match() {
		let mut state = State::new();
		assert!(state.push_match(LineMatch::new(1, false, true)));
		assert!(!state.matches().is_empty());
		assert_eq!(state.number_matches(), 1);
	}

	#[test]
	fn push_match_with_hash_and_content_match() {
		let mut state = State::new();
		assert!(state.push_match(LineMatch::new(1, true, true)));
		assert!(!state.matches().is_empty());
		assert_eq!(state.number_matches(), 1);
	}

	#[test]
	fn push_match_with_no_hash_and_no_content_match() {
		let mut state = State::new();
		assert!(!state.push_match(LineMatch::new(1, false, false)));
		assert!(state.matches().is_empty());
		assert_eq!(state.number_matches(), 0);
	}

	#[test]
	fn match_value_for_line_index_miss() {
		let mut state = State::new();
		assert!(state.push_match(LineMatch::new(1, false, true)));
		assert_none!(state.match_value_for_line(99));
	}

	#[test]
	fn match_value_for_line_index_hit() {
		let mut state = State::new();
		let line_match = LineMatch::new(1, false, true);
		assert!(state.push_match(line_match));
		assert_some_eq!(state.match_value_for_line(1), line_match);
	}

	#[test]
	fn match_value_miss() {
		let mut state = State::new();
		assert!(state.push_match(LineMatch::new(1, false, true)));
		assert_none!(state.match_value(99));
	}

	#[test]
	fn match_value_hit() {
		let mut state = State::new();
		let line_match = LineMatch::new(1, false, true);
		assert!(state.push_match(line_match));
		assert_some_eq!(state.match_value(0), line_match);
	}

	#[test]
	fn selected_set() {
		let mut state = State::new();
		state.set_selected(42);
		assert_some_eq!(state.selected(), 42);
	}

	#[test]
	fn selected_not_set() {
		let state = State::new();
		assert_none!(state.selected());
	}

	#[test]
	fn match_start_hint() {
		let mut state = State::new();
		state.set_match_start_hint(42);
		assert_eq!(state.match_start_hint(), 42);
	}
}
