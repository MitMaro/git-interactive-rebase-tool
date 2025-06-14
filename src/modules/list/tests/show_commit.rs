use super::*;
use crate::{assert_results, process::Artifact};

#[test]
fn when_hash_available() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ShowCommit)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
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
	testers::module(
		&[],
		&[Event::from(StandardEvent::ShowCommit)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ShowCommit))
			);
		},
	);
}

#[test]
fn do_not_when_hash_not_available() {
	testers::module(
		&["exec echo foo"],
		&[Event::from(StandardEvent::ShowCommit)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ShowCommit))
			);
		},
	);
}
