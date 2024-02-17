use claims::{assert_none, assert_some_eq};

use super::*;
use crate::{
	assert_results,
	process::Artifact,
	search::{Interrupter, SearchResult, Searchable},
	test_helpers::{create_config, testers},
};

#[derive(Clone)]
struct MockedSearchable;

impl Searchable for MockedSearchable {
	fn reset(&mut self) {}

	fn search(&mut self, _: Interrupter, term: &str) -> SearchResult {
		SearchResult::None
	}
}

#[test]
fn sets_selected_line_action() {
	testers::module(&["pick aaa c1"], &[], |mut test_context| {
		let mut module = create_list(&create_config(), test_context.take_todo_file());
		_ = test_context.activate(&mut module, State::List);
		assert_some_eq!(module.selected_line_action, Action::Pick);
	});
}

#[test]
fn sets_selected_line_action_none_selected() {
	testers::module(&["pick aaa c1", "pick bbb c2"], &[], |mut test_context| {
		let mut todo_file = test_context.take_todo_file();
		todo_file.set_lines(vec![]);

		let mut module = create_list(&create_config(), todo_file);
		_ = test_context.activate(&mut module, State::List);
		assert_none!(module.selected_line_action);
	});
}

#[test]
fn result() {
	testers::module(&["pick aaa c1", "pick bbb c2"], &[], |mut test_context| {
		let mut module = create_list(&create_config(), test_context.take_todo_file());
		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::Searchable(Box::new(MockedSearchable {}))
		);
	});
}

#[test]
fn result_with_serach_term() {
	testers::module(&["pick aaa c1", "pick bbb c2"], &[], |mut test_context| {
		let mut module = create_list(&create_config(), test_context.take_todo_file());
		module.search_bar.start_search(Some("foo"));
		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::Searchable(Box::new(MockedSearchable {})),
			Artifact::SearchTerm(String::from("foo"))
		);
	});
}
