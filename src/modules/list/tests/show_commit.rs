use super::*;
use crate::{assert_results, process::Artifact, test_helpers::testers};

#[test]
fn when_hash_available() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ShowCommit)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ShowCommit)),
				Artifact::ChangeState(State::ShowCommit)
			);
		},
	);
}

#[test]
fn when_no_selected_line() {
	testers::module(&[], &[Event::from(StandardEvent::ShowCommit)], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from(StandardEvent::ShowCommit))
		);
	});
}

#[test]
fn do_not_when_hash_not_available() {
	testers::module(
		&["exec echo foo"],
		&[Event::from(StandardEvent::ShowCommit)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ShowCommit))
			);
		},
	);
}
