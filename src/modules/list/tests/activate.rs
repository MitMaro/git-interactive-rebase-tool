use claims::{assert_none, assert_some_eq};

use super::*;
use crate::{
	assert_results,
	process::Artifact,
	search::{Interrupter, SearchResult},
};

#[derive(Clone)]
struct MockedSearchable;

impl Searchable for MockedSearchable {
	fn reset(&mut self) {}

	fn search(&mut self, _: Interrupter, _: &str) -> SearchResult {
		SearchResult::None
	}
}

#[test]
fn sets_selected_line_action() {
	testers::module(&["pick aaa c1"], &[], None, |test_context| {
		let mut module = List::new(&test_context.app_data());
		_ = test_context.activate(&mut module, State::List);
		assert_some_eq!(module.selected_line_action, Action::Pick);
	});
}

#[test]
fn sets_selected_line_action_none_selected() {
	testers::module(&["pick aaa c1", "pick bbb c2"], &[], None, |test_context| {
		let app_data = test_context.app_data();

		let todo_file = app_data.todo_file();
		todo_file.lock().set_lines(vec![]);

		let mut module = List::new(&app_data);
		_ = test_context.activate(&mut module, State::List);
		assert_none!(module.selected_line_action);
	});
}

#[test]
fn result() {
	testers::module(&["pick aaa c1", "pick bbb c2"], &[], None, |test_context| {
		let mut module = List::new(&test_context.app_data());
		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::Searchable(Box::new(MockedSearchable {}))
		);
	});
}

#[test]
fn result_with_serach_term() {
	testers::module(&["pick aaa c1", "pick bbb c2"], &[], None, |test_context| {
		let mut module = List::new(&test_context.app_data());
		module.search_bar.start_search(Some("foo"));
		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::Searchable(Box::new(MockedSearchable {})),
			Artifact::SearchTerm(String::from("foo"))
		);
	});
}
