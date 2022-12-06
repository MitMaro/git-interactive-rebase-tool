use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn normal_mode_open_external_editor() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::OpenInEditor)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::OpenInEditor)),
				Artifact::ChangeState(State::ExternalEditor)
			);
		},
	);
}

#[test]
fn cancels_search() {
	module_test(
		&["pick aaa c1"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(MetaEvent::OpenInEditor),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let _ = test_context.handle_all_events(&mut module);
			assert!(!module.search_bar.is_searching());
		},
	);
}
