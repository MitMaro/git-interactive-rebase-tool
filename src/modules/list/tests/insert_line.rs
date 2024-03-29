use super::*;
use crate::{assert_results, process::Artifact};

#[test]
fn insert_line() {
	testers::module(&[], &[Event::from(StandardEvent::InsertLine)], |mut test_context| {
		let mut module = create_list(&create_config(), test_context.take_todo_file());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from(StandardEvent::InsertLine)),
			Artifact::ChangeState(State::Insert)
		);
	});
}
