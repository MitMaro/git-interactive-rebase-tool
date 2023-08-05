use view::assert_rendered_output;

use super::*;
use crate::{action_line, testutil::module_test};

#[test]
fn normal_mode_action_change_to_drop() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionDrop)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(Body view_data, action_line!(Selected Drop "aaa", "c1"));
		},
	);
}

#[test]
fn visual_mode_action_change_to_drop() {
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
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionDrop),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Drop "aaa", "c2"),
				action_line!(Drop "aaa", "c3"),
				action_line!(Drop "aaa", "c4"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_edit() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionEdit)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionEdit),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionFixup)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionFixup),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
	module_test(
		&["drop aaa c1"],
		&[Event::from(MetaEvent::ActionPick)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
	module_test(
		&[
			"drop aaa c1",
			"drop aaa c2",
			"drop aaa c3",
			"drop aaa c4",
			"drop aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionPick),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionReword)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionSquash)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data, action_line!(Squash "aaa", "c1")
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_squash() {
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
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionSquash),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Squash "aaa", "c2"),
				action_line!(Squash "aaa", "c3"),
				action_line!(Squash "aaa", "c4"),
				action_line!(Pick "aaa", "c5")
			);
		},
	);
}
