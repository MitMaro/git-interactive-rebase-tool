use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn change_toggle_break_add() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionBreak)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick  aaa      c1",
				"{Selected} > break {Pad( )}"
			);
		},
	);
}

#[test]
fn change_toggle_break_remove() {
	module_test(
		&["pick aaa c1", "break"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionBreak),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let _ = test_context.handle_all_events(&mut module);
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
fn change_toggle_break_above_existing() {
	module_test(
		&["pick aaa c1", "break"],
		&[Event::from(MetaEvent::ActionBreak)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > pick  aaa      c1{Pad( )}",
				"   break"
			);
		},
	);
}
