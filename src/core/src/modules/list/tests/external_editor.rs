use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn normal_mode_open_external_editor() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::OpenInEditor)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::OpenInEditor)),
				Artifact::ChangeState(State::ExternalEditor)
			);
		},
	);
}

#[test]
fn visual_mode_open_external_editor() {
	module_test(
		&["pick aaa c1"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::OpenInEditor),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::OpenInEditor)),
				Artifact::ChangeState(State::ExternalEditor)
			);
		},
	);
}
