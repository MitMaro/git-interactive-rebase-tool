use super::*;
use crate::{action_line, assert_rendered_output};

#[test]
fn normal_mode_change_swap_down() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(StandardEvent::SwapSelectedDown)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_from_top_to_bottom_selection() {
	testers::module(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c5"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_from_bottom_to_top_selection() {
	testers::module(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c5"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_to_limit_from_bottom_to_top_selection() {
	testers::module(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::SwapSelectedDown),
			Event::from(StandardEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c5"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_to_limit_from_top_to_bottom_selection() {
	testers::module(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::SwapSelectedDown),
			Event::from(StandardEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c5"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn normal_mode_change_swap_up() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Pick "aaa", "c2")
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_from_top_to_bottom_selection() {
	testers::module(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4"),
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_from_bottom_to_top_selection() {
	testers::module(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4"),
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_to_limit_from_top_to_bottom_selection() {
	testers::module(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::SwapSelectedUp),
			Event::from(StandardEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4"),
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_to_limit_from_bottom_to_top_selection() {
	testers::module(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::SwapSelectedUp),
			Event::from(StandardEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4"),
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}
