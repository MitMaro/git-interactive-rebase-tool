use super::*;
use crate::{action_line, assert_rendered_output, test_helpers::testers};

#[test]
fn normal_mode_remove_line_first() {
	testers::module(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[Event::from(StandardEvent::Delete)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "bbb", "c2"),
				action_line!(Pick "ccc", "c3"),
				action_line!(Pick "ddd", "c4"),
				action_line!(Pick "eee", "c5")
			);
		},
	);
}

#[test]
fn normal_mode_remove_line_end() {
	testers::module(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "bbb", "c2"),
				action_line!(Pick "ccc", "c3"),
				action_line!(Selected Pick "ddd", "c4")
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_start_index_first() {
	testers::module(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "ddd", "c4"),
				action_line!(Pick "eee", "c5")
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				module.todo_file.lock().get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_end_index_first() {
	testers::module(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "ddd", "c4"),
				action_line!(Pick "eee", "c5")
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				module.todo_file.lock().get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_start_index_last() {
	testers::module(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Pick "bbb", "c2")
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				module.todo_file.lock().get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_end_index_last() {
	testers::module(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Pick "bbb", "c2")
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				module.todo_file.lock().get_selected_line_index()
			);
		},
	);
}
