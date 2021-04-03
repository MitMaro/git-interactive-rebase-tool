use std::path::Path;

use super::*;
use crate::{
	assert_process_result,
	assert_rendered_output,
	input::{Event, KeyCode},
	process::testutil::{process_module_test, TestContext, ViewState},
};

fn get_external_editor(content: &str, exit_code: &str) -> String {
	format!(
		"{} \"{}\" % \"{}\"",
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("write-content.sh")
			.to_str()
			.unwrap(),
		content,
		exit_code
	)
}

#[macro_export]
macro_rules! assert_external_editor_state_eq {
	($actual:expr, $expected:expr) => {
		assert_external_editor_state_eq(&$actual, &$expected);
	};
}

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

#[test]
fn activate() {
	process_module_test(
		&["pick aaa comment1", "drop bbb comment2"],
		ViewState::default(),
		&[Event::from(KeyCode::Up)],
		|test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("pick aaa comment", "0").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_eq!(test_context.rebase_todo_file.get_lines_owned(), vec![
				Line::new("pick aaa comment1").unwrap(),
				Line::new("drop bbb comment2").unwrap()
			]);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		},
	);
}

#[test]
fn activate_write_file_fail() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(KeyCode::Up)],
		|test_context: TestContext<'_>| {
			let todo_path = test_context.get_todo_file_path();
			let mut module = ExternalEditor::new(get_external_editor("pick aaa comment", "0").as_str());
			test_context.set_todo_file_readonly();
			assert_process_result!(
				test_context.activate(&mut module, State::List),
				state = State::List,
				error = anyhow!("Error opening file: {}: Permission denied (os error 13)", todo_path)
			);
		},
	);
}

#[test]
fn deactivate() {
	process_module_test(
		&["pick aaa comment", "drop bbb comment2"],
		ViewState::default(),
		&[Event::from(KeyCode::Up)],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("pick aaa comment", "0").as_str());
			test_context.deactivate(&mut module);
			assert_eq!(module.lines, vec![]);
		},
	);
}

#[test]
fn edit_success() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(KeyCode::Up)],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("pick aaa comment", "0").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(view_data, "{TITLE}", "{LEADING}", "{Normal}Editing...");
			assert_process_result!(test_context.handle_event(&mut module), state = State::List);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		},
	);
}

#[test]
fn empty_edit_error() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from('1')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("", "0").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
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
		},
	);
}

#[test]
fn empty_edit_abort_rebase() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from('1')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("", "0").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
			test_context.build_view_data(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from('1'),
				exit_status = ExitStatus::Good
			);
		},
	);
}

#[test]
fn empty_edit_re_edit_rebase_file() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from('2')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("", "0").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
			test_context.build_view_data(&mut module);
			assert_process_result!(test_context.handle_event(&mut module), event = Event::from('2'));
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		},
	);
}

#[test]
fn empty_edit_undo_and_edit() {
	process_module_test(
		&["pick aaa comment", "drop bbb comment"],
		ViewState::default(),
		&[Event::from('3')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("", "0").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
			test_context.build_view_data(&mut module);
			assert_process_result!(test_context.handle_event(&mut module), event = Event::from('3'));
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
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from('1')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("noop", "0").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
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
		},
	);
}

#[test]
fn no_editor_set() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(KeyCode::Up)],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new("");
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			assert_external_editor_state_eq!(
				module.state,
				ExternalEditorState::Error(
					anyhow!("Please see the git \"core.editor\" configuration for details")
						.context(anyhow!("No editor configured"))
				)
			);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{Normal}No editor configured",
				"{Normal}Please see the git \"core.editor\" configuration for details",
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
fn invalid_editor_set() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(KeyCode::Up)],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(
				Path::new(env!("CARGO_MANIFEST_DIR"))
					.join("test")
					.join("not-executable.sh")
					.to_str()
					.unwrap(),
			);
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			assert_external_editor_state_eq!(
				module.state,
				ExternalEditorState::Error(
					anyhow!("Permission denied (os error 13)").context(anyhow!("Unable to run editor"))
				)
			);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{Normal}Unable to run editor",
				"{Normal}Permission denied (os error 13)",
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
fn editor_non_zero_exit() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(KeyCode::Up)],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("pick aaa comment", "1").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
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
		},
	);
}

#[test]
fn editor_reload_error() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(KeyCode::Up)],
		|mut test_context: TestContext<'_>| {
			let todo_path = test_context.get_todo_file_path();
			let mut module = ExternalEditor::new("true");
			assert_process_result!(test_context.activate(&mut module, State::List));
			test_context.delete_todo_file();
			assert_process_result!(test_context.handle_event(&mut module));
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
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from('1')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("pick aaa comment", "1").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			test_context.build_view_data(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from('1'),
				exit_status = ExitStatus::Good
			);
			assert!(test_context.rebase_todo_file.is_empty());
		},
	);
}

#[test]
fn error_edit_rebase() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from('2')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("pick aaa comment", "1").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			test_context.build_view_data(&mut module);
			assert_process_result!(test_context.handle_event(&mut module), event = Event::from('2'));
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		},
	);
}

#[test]
fn error_restore_and_abort() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from('3')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("drop aaa comment", "1").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			test_context.build_view_data(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from('3'),
				state = State::List
			);
			assert_eq!(test_context.rebase_todo_file.get_lines_owned(), vec![Line::new(
				"pick aaa comment"
			)
			.unwrap()]);
		},
	);
}

#[test]
fn error_undo_modifications_and_reedit() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from('4')],
		|mut test_context: TestContext<'_>| {
			let mut module = ExternalEditor::new(get_external_editor("rdop aaa comment", "1").as_str());
			assert_process_result!(test_context.activate(&mut module, State::List));
			assert_process_result!(test_context.handle_event(&mut module));
			test_context.build_view_data(&mut module);
			assert_process_result!(test_context.handle_event(&mut module), event = Event::from('4'));
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			assert_eq!(test_context.rebase_todo_file.get_lines_owned(), vec![Line::new(
				"pick aaa comment"
			)
			.unwrap()]);
		},
	);
}
