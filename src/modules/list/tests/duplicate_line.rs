use super::*;
use crate::{action_line, assert_rendered_output, assert_results, process::Artifact};

#[test]
fn duplicate_line_duplicatable() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::DuplicateLine)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::DuplicateLine))
			);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c1")
			);
		},
	);
}

#[test]
fn duplicate_line_not_duplicatable() {
	testers::module(
		&["break"],
		&[Event::from(StandardEvent::DuplicateLine)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::DuplicateLine))
			);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Break)
			);
		},
	);
}
