use claim::{assert_none, assert_some, assert_some_eq};

use super::*;
use crate::testutil::module_test;

#[test]
fn on_fixup_keep_message() {
	module_test(
		&["fixup aaa c1"],
		&[Event::from(MetaEvent::FixupKeepMessage)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.activate(&mut module, State::List);
			_ = test_context.handle_all_events(&mut module);
			let todo_file = module.todo_file.lock();
			let line = todo_file.get_line(0).unwrap();
			assert_some_eq!(line.option(), "-C");
		},
	);
}

#[test]
fn on_fixup_keep_message_with_editor() {
	module_test(
		&["fixup aaa c1"],
		&[Event::from(MetaEvent::FixupKeepMessageWithEditor)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.activate(&mut module, State::List);
			_ = test_context.handle_all_events(&mut module);
			let todo_file = module.todo_file.lock();
			let line = todo_file.get_line(0).unwrap();
			assert_some_eq!(line.option(), "-c");
		},
	);
}

#[test]
fn after_select_line() {
	module_test(
		&["fixup aaa c1", "fixup aaa c2", "fixup aaa c3"],
		&[Event::from(MetaEvent::MoveCursorDown), Event::from('u')],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.activate(&mut module, State::List);
			_ = test_context.handle_all_events(&mut module);
			assert_none!(module.todo_file.lock().get_line(0).unwrap().option());
			assert_some!(module.todo_file.lock().get_line(1).unwrap().option());
			assert_none!(module.todo_file.lock().get_line(2).unwrap().option());
		},
	);
}
