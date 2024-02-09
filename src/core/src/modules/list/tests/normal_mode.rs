use ::input::KeyCode;

use super::*;
use crate::{action_line, assert_rendered_output, assert_results, process::Artifact, testutil::module_test};

#[test]
fn change_auto_select_next_with_next_line() {
	module_test(
		&["pick aaa c1", "pick aaa c2"],
		&[Event::from(MetaEvent::ActionSquash)],
		|mut test_context| {
			let mut config = Config::new();
			config.auto_select_next = true;
			let mut module = create_list(&config, test_context.take_todo_file());
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
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ToggleVisualMode)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ToggleVisualMode))
			);
			assert_eq!(module.visual_index_start, Some(0));
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn other_event() {
	module_test(&["pick aaa c1"], &[Event::from(KeyCode::Null)], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from(KeyCode::Null))
		);
	});
}
