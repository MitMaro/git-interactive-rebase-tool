use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn pinned_segments() {
	module_test(
		&[
			"break",
			"drop aaa c1",
			"edit aaa c1",
			"fixup aaa c1",
			"pick aaa c1",
			"reword aaa c1",
			"squash aaa c1",
			"exec command",
			"label reference",
			"reset reference",
			"merge command",
		],
		&[Event::from(MetaEvent::ActionDrop)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::INCLUDE_PINNED | AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Pin(3)}{Selected} > break  {Pad( )}",
				"{Pin(2)}   drop   aaa      c1",
				"{Pin(2)}   edit   aaa      c1",
				"{Pin(2)}   fixup  aaa      c1",
				"{Pin(2)}   pick   aaa      c1",
				"{Pin(2)}   reword aaa      c1",
				"{Pin(2)}   squash aaa      c1",
				"{Pin(3)}   exec   command",
				"{Pin(3)}   label  reference",
				"{Pin(3)}   reset  reference",
				"{Pin(3)}   merge  command"
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_drop() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionDrop)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > drop aaa      c1{Pad( )}"
			);
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick aaa      c1",
				"{Selected} > drop aaa      c2{Pad( )}",
				"{Selected} > drop aaa      c3{Pad( )}",
				"{Selected} > drop aaa      c4{Pad( )}",
				"   pick aaa      c5"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > edit aaa      c1{Pad( )}"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick aaa      c1",
				"{Selected} > edit aaa      c2{Pad( )}",
				"{Selected} > edit aaa      c3{Pad( )}",
				"{Selected} > edit aaa      c4{Pad( )}",
				"   pick aaa      c5"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > fixup aaa      c1{Pad( )}"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick  aaa      c1",
				"{Selected} > fixup aaa      c2{Pad( )}",
				"{Selected} > fixup aaa      c3{Pad( )}",
				"{Selected} > fixup aaa      c4{Pad( )}",
				"   pick  aaa      c5"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > pick aaa      c1{Pad( )}"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"   drop aaa      c1",
				"{Selected} > pick aaa      c2{Pad( )}",
				"{Selected} > pick aaa      c3{Pad( )}",
				"{Selected} > pick aaa      c4{Pad( )}",
				"   drop aaa      c5"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > reword aaa      c1{Pad( )}"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick   aaa      c1",
				"{Selected} > reword aaa      c2{Pad( )}",
				"{Selected} > reword aaa      c3{Pad( )}",
				"{Selected} > reword aaa      c4{Pad( )}",
				"   pick   aaa      c5"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > squash aaa      c1{Pad( )}"
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
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick   aaa      c1",
				"{Selected} > squash aaa      c2{Pad( )}",
				"{Selected} > squash aaa      c3{Pad( )}",
				"{Selected} > squash aaa      c4{Pad( )}",
				"   pick   aaa      c5"
			);
		},
	);
}
