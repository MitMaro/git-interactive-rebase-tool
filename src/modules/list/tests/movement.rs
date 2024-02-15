use super::*;
use crate::{
	action_line,
	assert_rendered_output,
	input::{KeyModifiers, MouseEvent, MouseEventKind},
	test_helpers::testers,
};

#[test]
fn move_down_1() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(StandardEvent::MoveCursorDown)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn move_down_view_end() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(StandardEvent::MoveCursorDown); 2],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn move_down_past_end() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(StandardEvent::MoveCursorDown); 3],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn move_down_scroll_bottom_move_up_one() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn move_down_scroll_bottom_move_up_top() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn move_up_attempt_above_top() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(StandardEvent::MoveCursorUp),
			Event::from(StandardEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_down_attempt_below_bottom() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(StandardEvent::MoveCursorDown); 4],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_page_up_from_top() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(StandardEvent::MoveCursorPageUp)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			module.height = 4;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_page_up_from_one_page_down() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			module.height = 4;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_page_up_from_one_page_down_minus_1() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			module.height = 4;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_page_up_from_bottom() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			module.height = 4;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_home() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorHome),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_end() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(StandardEvent::MoveCursorEnd)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_page_down_past_bottom() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(StandardEvent::MoveCursorPageDown); 3],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			module.height = 4;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_page_down_one_from_bottom() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorPageDown),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn move_page_down_one_page_from_bottom() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorPageDown),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			module.height = 4;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4")
			);
		},
	);
}

#[test]
fn mouse_scroll() {
	testers::module(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollDown,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollDown,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollUp,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Pick "aaa", "c3")
			);
		},
	);
}

#[test]
fn scroll_right() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::MoveCursorRight)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			test_context.view_context.assert_render_action(&["ScrollRight"]);
		},
	);
}

#[test]
fn scroll_left() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::MoveCursorLeft)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			test_context.view_context.assert_render_action(&["ScrollLeft"]);
		},
	);
}
