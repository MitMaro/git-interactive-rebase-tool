use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn when_hash_available() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ShowCommit)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ShowCommit)),
				Artifact::ChangeState(State::ShowCommit)
			);
		},
	);
}

#[test]
fn when_no_selected_line() {
	module_test(&[], &[Event::from(MetaEvent::ShowCommit)], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from(MetaEvent::ShowCommit))
		);
	});
}

#[test]
fn do_not_when_hash_not_available() {
	module_test(
		&["exec echo foo"],
		&[Event::from(MetaEvent::ShowCommit)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ShowCommit))
			);
		},
	);
}
