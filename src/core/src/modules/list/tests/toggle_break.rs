use todo_file::{errors::ParseError, Action::Pick};
use view::{assert_rendered_output, testutil::LinePattern};

use super::*;
use crate::{action_line, testutil::module_test};

#[test]
fn change_toggle_break_add() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionBreak)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Pick "aaa", "c1"),
				action_line!(Selected Break)
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
fn change_toggle_break_above_existing() {
	module_test(
		&["pick aaa c1", "break"],
		&[Event::from(MetaEvent::ActionBreak)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Body view_data,
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Break)
			);
		},
	);
}
