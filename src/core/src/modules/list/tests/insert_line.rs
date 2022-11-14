use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn insert_line() {
	module_test(&[], &[Event::from(MetaEvent::InsertLine)], |mut test_context| {
		let mut module = List::new(&Config::new());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from(MetaEvent::InsertLine)),
			Artifact::ChangeState(State::Insert)
		);
	});
}
