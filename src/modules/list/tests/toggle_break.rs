use super::*;
use crate::{
	action_line,
	assert_rendered_output,
	test_helpers::testers,
	todo_file::{Action::Pick, ParseError},
};

#[test]
fn change_toggle_break_add() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ActionBreak)],
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
	testers::module(
		&["pick aaa c1", "break"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ActionBreak),
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
	testers::module(
		&["pick aaa c1", "break"],
		&[Event::from(StandardEvent::ActionBreak)],
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
