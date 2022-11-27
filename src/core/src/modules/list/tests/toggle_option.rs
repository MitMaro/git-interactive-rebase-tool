use claim::{assert_none, assert_some, assert_some_eq};

use super::*;
use crate::testutil::module_test;

#[test]
fn on_fixup_keep_message() {
	module_test(
		&["fixup aaa c1"],
		&[Event::from(MetaEvent::FixupKeepMessage)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.activate(&mut module, State::List);
			let _ = test_context.handle_all_events(&mut module);
			let line = test_context.todo_file_context.todo_file().get_line(0).unwrap();
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
			let mut module = List::new(&Config::new());
			let _ = test_context.activate(&mut module, State::List);
			let _ = test_context.handle_all_events(&mut module);
			let line = test_context.todo_file_context.todo_file().get_line(0).unwrap();
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
			let mut module = List::new(&Config::new());
			let _ = test_context.activate(&mut module, State::List);
			let _ = test_context.handle_all_events(&mut module);
			assert_none!(test_context.todo_file_context.todo_file().get_line(0).unwrap().option());
			assert_some!(test_context.todo_file_context.todo_file().get_line(1).unwrap().option());
			assert_none!(test_context.todo_file_context.todo_file().get_line(2).unwrap().option());
		},
	);
}
