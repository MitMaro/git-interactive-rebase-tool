use super::*;
use crate::{assert_results, process::Artifact};

#[test]
fn insert_line() {
	testers::module(
		&[],
		&[Event::from(StandardEvent::InsertLine)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::InsertLine)),
				Artifact::ChangeState(State::Insert)
			);
		},
	);
}
