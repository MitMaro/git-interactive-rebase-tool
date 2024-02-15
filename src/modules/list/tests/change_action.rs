use super::*;
use crate::{action_line, assert_rendered_output, test_helpers::testers};

#[test]
fn normal_mode_action_change_to_drop() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ActionDrop)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(Body view_data, action_line!(Selected Drop "aaa", "c1"));
		},
	);
}

#[test]
fn visual_mode_action_change_to_drop() {
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
			Event::from(StandardEvent::ActionDrop),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Drop "aaa", "c2"),
				action_line!(Selected Drop "aaa", "c3"),
				action_line!(Selected Drop "aaa", "c4"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_edit() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ActionEdit)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Edit "aaa", "c1")
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_edit() {
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
			Event::from(StandardEvent::ActionEdit),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Edit "aaa", "c2"),
				action_line!(Selected Edit "aaa", "c3"),
				action_line!(Selected Edit "aaa", "c4"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_fixup() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ActionFixup)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Fixup "aaa", "c1")
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_fixup() {
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
			Event::from(StandardEvent::ActionFixup),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Fixup "aaa", "c2"),
				action_line!(Selected Fixup "aaa", "c3"),
				action_line!(Selected Fixup "aaa", "c4"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_pick() {
	testers::module(
		&["drop aaa c1"],
		&[Event::from(StandardEvent::ActionPick)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c1")
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_pick() {
	testers::module(
		&[
			"drop aaa c1",
			"drop aaa c2",
			"drop aaa c3",
			"drop aaa c4",
			"drop aaa c5",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ActionPick),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Drop "aaa", "c1"),
				action_line!(Selected Pick "aaa", "c2"),
				action_line!(Selected Pick "aaa", "c3"),
				action_line!(Selected Pick "aaa", "c4"),
				action_line!(Drop "aaa", "c5")
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_reword() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ActionReword)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Reword "aaa", "c1")
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_reword() {
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
			Event::from(StandardEvent::ActionReword),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Reword "aaa", "c2"),
				action_line!(Selected Reword "aaa", "c3"),
				action_line!(Selected Reword "aaa", "c4"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_squash() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ActionSquash)],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data, action_line!(Selected Squash "aaa", "c1")
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_squash() {
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
			Event::from(StandardEvent::ActionSquash),
		],
		|mut test_context| {
			let mut module = create_list(&create_config(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Squash "aaa", "c2"),
				action_line!(Selected Squash "aaa", "c3"),
				action_line!(Selected Squash "aaa", "c4"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}
