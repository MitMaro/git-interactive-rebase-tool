use input::{Event, KeyCode};

use super::*;
use crate::{assert_process_result, assert_rendered_output, process::testutil::process_module_test};

fn assert_external_editor_state_eq(actual: &ExternalEditorState, expected: &ExternalEditorState) {
	let actual_state = match *actual {
		ExternalEditorState::Active => String::from("Active"),
		ExternalEditorState::Empty => String::from("Empty"),
		ExternalEditorState::Error(ref err) => format!("Error({:#})", err),
	};

	let expected_state = match *expected {
		ExternalEditorState::Active => String::from("Active"),
		ExternalEditorState::Empty => String::from("Empty"),
		ExternalEditorState::Error(ref err) => format!("Error({:#})", err),
	};

	if actual_state != expected_state {
		panic!(
			"{}",
			vec![
				"\n",
				"ExternalEditorState does not match",
				"==========",
				"Expected:",
				expected_state.as_str(),
				"Actual:",
				actual_state.as_str(),
				"==========\n"
			]
			.join("\n")
		);
	}
}

#[macro_export]
macro_rules! assert_external_editor_state_eq {
	($actual:expr, $expected:expr) => {
		assert_external_editor_state_eq(&$actual, &$expected);
	};
}

#[test]
fn activate() {
	process_module_test(&["pick aaa comment1", "drop bbb comment2"], &[], |test_context| {
		let mut module = ExternalEditor::new("editor");
		assert_process_result!(
			test_context.activate(&mut module, State::List),
			external_command = (String::from("editor"), vec![String::from(
				test_context.rebase_todo_file.get_filepath()
			)])
		);
		assert_eq!(test_context.rebase_todo_file.get_lines_owned(), vec![
			Line::new("pick aaa comment1").unwrap(),
			Line::new("drop bbb comment2").unwrap()
		]);
		assert!(!module.lines.is_empty());
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		assert_eq!(
			module.external_command,
			(String::from("editor"), vec![String::from(
				test_context.rebase_todo_file.get_filepath()
			)])
		);
	});
}

#[test]
fn activate_write_file_fail() {
	process_module_test(&["pick aaa comment"], &[], |test_context| {
		let todo_path = test_context.get_todo_file_path();
		let mut module = ExternalEditor::new("editor");
		test_context.set_todo_file_readonly();
		assert_process_result!(
			test_context.activate(&mut module, State::List),
			state = State::List,
			error = anyhow!("Error opening file: {}: Permission denied (os error 13)", todo_path)
		);
	});
}

#[test]
fn activate_file_placement_marker() {
	process_module_test(&[], &[], |test_context| {
		let mut module = ExternalEditor::new("editor a % b");
		assert_process_result!(
			test_context.activate(&mut module, State::List),
			external_command = (String::from("editor"), vec![
				String::from("a"),
				String::from(test_context.rebase_todo_file.get_filepath()),
				String::from("b")
			])
		);
	});
}

#[test]
fn deactivate() {
	process_module_test(&["pick aaa comment", "drop bbb comment2"], &[], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context.deactivate(&mut module);
		assert_eq!(module.lines, vec![]);
	});
}

#[test]
fn edit_success() {
	process_module_test(&["pick aaa comment"], &[], |mut test_context| {
		test_context
			.event_handler_context
			.event_handler
			.push_event(Event::from(MetaEvent::ExternalCommandSuccess));
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(view_data, "{TITLE}", "{LEADING}", "{Normal}Editing...");
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(MetaEvent::ExternalCommandSuccess),
			state = State::List
		);
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
	});
}

#[test]
fn empty_edit_error() {
	process_module_test(&["pick aaa comment"], &[Event::from('1')], |mut test_context| {
		test_context
			.event_handler_context
			.event_handler
			.push_event(Event::from(MetaEvent::ExternalCommandSuccess));
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		test_context.rebase_todo_file.set_lines(vec![]);
		test_context.rebase_todo_file.write_file().unwrap();
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(MetaEvent::ExternalCommandSuccess)
		);
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"{Normal}The rebase file is empty.",
			"",
			"{BODY}",
			"{Normal}1) Abort rebase",
			"{Normal}2) Edit rebase file",
			"{Normal}3) Undo modifications and edit rebase file",
			"",
			"{IndicatorColor}Please choose an option."
		);
	});
}

#[test]
fn empty_edit_abort_rebase() {
	process_module_test(&["pick aaa comment"], &[Event::from('1')], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Empty;
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from('1'),
			exit_status = ExitStatus::Good
		);
	});
}

#[test]
fn empty_edit_re_edit_rebase_file() {
	process_module_test(&["pick aaa comment"], &[Event::from('2')], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Empty;
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from('2'),
			external_command = (String::from("editor"), vec![String::from(
				test_context.rebase_todo_file.get_filepath()
			)])
		);
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
	});
}

#[test]
fn empty_edit_undo_and_edit() {
	process_module_test(
		&["pick aaa comment", "drop bbb comment"],
		&[Event::from('3')],
		|mut test_context| {
			let mut module = ExternalEditor::new("editor");
			test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Empty;
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from('3'),
				external_command = (String::from("editor"), vec![String::from(
					test_context.rebase_todo_file.get_filepath()
				)])
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			assert_eq!(test_context.rebase_todo_file.get_lines_owned(), vec![
				Line::new("pick aaa comment").unwrap(),
				Line::new("drop bbb comment").unwrap()
			]);
		},
	);
}

#[test]
fn empty_edit_noop() {
	process_module_test(&["pick aaa comment"], &[], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Empty;
		test_context.rebase_todo_file.set_lines(vec![]);
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"{Normal}The rebase file is empty.",
			"",
			"{BODY}",
			"{Normal}1) Abort rebase",
			"{Normal}2) Edit rebase file",
			"{Normal}3) Undo modifications and edit rebase file",
			"",
			"{IndicatorColor}Please choose an option."
		);
	});
}

#[test]
fn no_editor_set() {
	process_module_test(&["pick aaa comment"], &[], |test_context| {
		let mut module = ExternalEditor::new("");
		assert_process_result!(
			test_context.activate(&mut module, State::List),
			state = State::List,
			error = anyhow!("No editor configured: Please see the git \"core.editor\" configuration for details")
		);
	});
}

#[test]
fn editor_non_zero_exit() {
	process_module_test(&["pick aaa comment"], &[], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context
			.event_handler_context
			.event_handler
			.push_event(Event::from(MetaEvent::ExternalCommandError));
		test_context.activate(&mut module, State::List);
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(MetaEvent::ExternalCommandError)
		);
		assert_external_editor_state_eq!(
			module.state,
			ExternalEditorState::Error(anyhow!("Editor returned a non-zero exit status"))
		);
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"{Normal}Editor returned a non-zero exit status",
			"",
			"{BODY}",
			"{Normal}1) Abort rebase",
			"{Normal}2) Edit rebase file",
			"{Normal}3) Restore rebase file and abort edit",
			"{Normal}4) Undo modifications and edit rebase file",
			"",
			"{IndicatorColor}Please choose an option."
		);
	});
}

#[test]
fn editor_reload_error() {
	process_module_test(
		&["pick aaa comment"],
		&[Event::from(KeyCode::Up)],
		|mut test_context| {
			let todo_path = test_context.get_todo_file_path();
			let mut module = ExternalEditor::new("editor");
			test_context
				.event_handler_context
				.event_handler
				.push_event(Event::from(MetaEvent::ExternalCommandSuccess));
			test_context.activate(&mut module, State::List);
			test_context.delete_todo_file();
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ExternalCommandSuccess)
			);
			assert_external_editor_state_eq!(
				module.state,
				ExternalEditorState::Error(
					anyhow!("Error reading file: {}", todo_path).context("No such file or directory (os error 2)")
				)
			);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{Normal}No such file or directory (os error 2)",
				format!("{{Normal}}Error reading file: {}", todo_path),
				"",
				"{BODY}",
				"{Normal}1) Abort rebase",
				"{Normal}2) Edit rebase file",
				"{Normal}3) Restore rebase file and abort edit",
				"{Normal}4) Undo modifications and edit rebase file",
				"",
				"{IndicatorColor}Please choose an option."
			);
		},
	);
}

#[test]
fn error_abort_rebase() {
	process_module_test(&["pick aaa comment"], &[Event::from('1')], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Error(anyhow!("Error!"));
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from('1'),
			exit_status = ExitStatus::Good
		);
		assert!(test_context.rebase_todo_file.is_empty());
	});
}

#[test]
fn error_edit_rebase() {
	process_module_test(&["pick aaa comment"], &[Event::from('2')], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Error(anyhow!("Error!"));
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from('2'),
			external_command = (String::from("editor"), vec![String::from(
				test_context.rebase_todo_file.get_filepath()
			)])
		);
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
	});
}

#[test]
fn error_restore_and_abort() {
	process_module_test(&["pick aaa comment"], &[Event::from('3')], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Error(anyhow!("Error!"));
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from('3'),
			state = State::List
		);
		assert_eq!(test_context.rebase_todo_file.get_lines_owned(), vec![Line::new(
			"pick aaa comment"
		)
		.unwrap()]);
	});
}

#[test]
fn error_undo_modifications_and_reedit() {
	process_module_test(&["pick aaa comment"], &[Event::from('4')], |mut test_context| {
		let mut module = ExternalEditor::new("editor");
		test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Error(anyhow!("Error!"));
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from('4'),
			external_command = (String::from("editor"), vec![String::from(
				test_context.rebase_todo_file.get_filepath()
			)])
		);
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		assert_eq!(test_context.rebase_todo_file.get_lines_owned(), vec![Line::new(
			"pick aaa comment"
		)
		.unwrap()]);
	});
}
