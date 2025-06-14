use super::*;
use crate::{assert_results, process::Artifact};

#[test]
fn normal_mode_open_external_editor() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::OpenInEditor)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::OpenInEditor)),
				Artifact::SearchCancel,
				Artifact::ChangeState(State::ExternalEditor)
			);
		},
	);
}

#[test]
fn visual_mode_open_external_editor() {
	testers::module(
		&["pick aaa c1"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::OpenInEditor),
		],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::OpenInEditor)),
				Artifact::SearchCancel,
				Artifact::ChangeState(State::ExternalEditor)
			);
		},
	);
}
