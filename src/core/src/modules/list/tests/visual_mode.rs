use ::input::KeyCode;
use view::{assert_rendered_output, render_line};

use super::*;
use crate::{action_line, assert_results, process::Artifact, testutil::module_test};

#[test]
fn start() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::ToggleVisualMode)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				render_line!(All render_line!(Not Contains "Dimmed"), action_line!(Selected Pick "aaa", "c1")),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn start_cursor_down_one() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Pick "aaa", "c1")),
				render_line!(All render_line!(Not Contains "Dimmed"), action_line!(Selected Pick "aaa", "c2")),
				action_line!(Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn start_cursor_page_down() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorPageDown),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			module.height = 4;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Pick "aaa", "c1")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Pick "aaa", "c2")),
				render_line!(All render_line!(Not Contains "Dimmed"), action_line!(Selected Pick "aaa", "c3")),
				action_line!(Pick "aaa", "c4"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn start_cursor_from_bottom_move_up() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				render_line!(All render_line!(Not Contains "Dimmed"), action_line!(Selected Pick "aaa", "c4")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Pick "aaa", "c5"))
			);
		},
	);
}

#[test]
fn start_cursor_from_bottom_to_top() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				render_line!(All render_line!(Not Contains "Dimmed"), action_line!(Selected Pick "aaa", "c1")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Pick "aaa", "c2")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Pick "aaa", "c3")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Pick "aaa", "c4")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Pick "aaa", "c5"))
			);
		},
	);
}

#[test]
fn action_change_top_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionReword),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Reword "aaa", "c1")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Reword "aaa", "c2")),
				render_line!(All render_line!(Not Contains "Dimmed"), action_line!(Selected Reword "aaa", "c3"))
			);
		},
	);
}

#[test]
fn action_change_bottom_top() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::ActionReword),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				render_line!(All render_line!(Not Contains "Dimmed"), action_line!(Selected Reword "aaa", "c1")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Reword "aaa", "c2")),
				render_line!(All render_line!(Contains "Dimmed"), action_line!(Selected Reword "aaa", "c3"))
			);
		},
	);
}

#[test]
fn toggle_visual_mode() {
	module_test(
		&["pick aaa c1"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::ToggleVisualMode),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ToggleVisualMode))
			);
			assert_eq!(module.visual_index_start, None);
			assert_eq!(module.state, ListState::Normal);
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
