use super::*;
use crate::{action_line, assert_rendered_output, assert_results, input::KeyCode, process::Artifact};

#[test]
fn change_auto_select_next_with_next_line() {
	let mut config = create_config();
	config.auto_select_next = true;
	testers::module(
		&["pick aaa c1", "pick aaa c2"],
		&[Event::from(StandardEvent::ActionSquash)],
		Some(config),
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Squash "aaa", "c1"),
				action_line!(Selected Pick "aaa", "c2")
			);
		},
	);
}

#[test]
fn toggle_visual_mode() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ToggleVisualMode)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ToggleVisualMode))
			);
			assert_eq!(module.visual_index_start, Some(0));
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn other_event() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(KeyCode::Null)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(KeyCode::Null))
			);
		},
	);
}
