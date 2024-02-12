use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn insert_line() {
	module_test(&[], &[Event::from(StandardEvent::InsertLine)], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from(StandardEvent::InsertLine)),
			Artifact::ChangeState(State::Insert)
		);
	});
}
