use std::{fs, fs::File, path::Path};

use super::*;
use crate::{
	assert_rendered_output,
	assert_results,
	config::Config,
	input::KeyCode,
	process::Artifact,
	test_helpers::{testers, testers::ModuleTestContext},
};

fn assert_external_editor_state_eq(actual: &ExternalEditorState, expected: &ExternalEditorState) {
	let actual_state = match *actual {
		ExternalEditorState::Active => String::from("Active"),
		ExternalEditorState::Empty => String::from("Empty"),
		ExternalEditorState::Error(ref err) => format!("Error({err:#})"),
	};

	let expected_state = match *expected {
		ExternalEditorState::Active => String::from("Active"),
		ExternalEditorState::Empty => String::from("Empty"),
		ExternalEditorState::Error(ref err) => format!("Error({err:#})"),
	};

	assert_eq!(
		actual_state,
		expected_state,
		"{}",
		[
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

#[macro_export]
macro_rules! assert_external_editor_state_eq {
	($actual:expr, $expected:expr) => {
		assert_external_editor_state_eq(&$actual, &$expected);
	};
}

fn todo_file_path(test_context: &ModuleTestContext) -> String {
	let todo_file = test_context.app_data().todo_file();
	let todo_file_lock = todo_file.lock();
	String::from(todo_file_lock.get_filepath().to_str().unwrap())
}

#[test]
fn activate() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment1", "drop bbb comment2"],
		&[],
		config,
		|test_context| {
			let todo_path = todo_file_path(&test_context);

			let mut module = ExternalEditor::new(&test_context.app_data());
			assert_results!(
				test_context.activate(&mut module, State::List),
				Artifact::ExternalCommand((String::from("editor"), vec![String::from(todo_path.as_str())]))
			);
			assert_eq!(module.todo_file.lock().get_lines_owned(), vec![
				Line::parse("pick aaa comment1").unwrap(),
				Line::parse("drop bbb comment2").unwrap()
			]);
			assert!(!module.lines.is_empty());
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			assert_eq!(module.external_command, (String::from("editor"), vec![todo_path]));
		},
	);
}

#[test]
fn activate_write_file_fail() {
	testers::module(&["pick aaa comment"], &[], |test_context| {
		let todo_path = todo_file_path(&test_context);
		let file = File::open(todo_path.as_str()).unwrap();
		let mut permissions = file.metadata().unwrap().permissions();
		permissions.set_readonly(true);
		file.set_permissions(permissions).unwrap();

		let mut module = ExternalEditor::new(&test_context.app_data());
		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::Error(anyhow!("Unable to read file `{}`", todo_path), Some(State::List))
		);
	});
}

#[test]
fn activate_file_placement_marker() {
	let mut config = Config::default();
	config.git.editor = String::from("editor a % b");
	testers::module_with_config(&[], &[], config, |test_context| {
		let todo_path = todo_file_path(&test_context);
		let mut module = ExternalEditor::new(&test_context.app_data());
		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::ExternalCommand((String::from("editor"), vec![
				String::from("a"),
				todo_path,
				String::from("b")
			]))
		);
	});
}

#[test]
fn deactivate() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment", "drop bbb comment2"],
		&[],
		config,
		|mut test_context| {
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.deactivate(&mut module);
			assert_eq!(module.lines, vec![]);
		},
	);
}

#[test]
fn edit_success() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from(StandardEvent::ExternalCommandSuccess)],
		config,
		|mut test_context| {
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(view_data, "{TITLE}", "{LEADING}", "Editing...");
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ExternalCommandSuccess)),
				Artifact::ChangeState(State::List)
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		},
	);
}

#[test]
fn empty_edit_error() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from('1'), Event::from(StandardEvent::ExternalCommandSuccess)],
		config,
		|mut test_context| {
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			let mut todo_file = module.todo_file.lock();
			todo_file.set_lines(vec![]);
			todo_file.write_file().unwrap();
			drop(todo_file);
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ExternalCommandSuccess))
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"The rebase file is empty.",
				"",
				"{BODY}",
				"1) Abort rebase",
				"2) Edit rebase file",
				"3) Undo modifications and edit rebase file",
				"",
				"Please choose an option."
			);
		},
	);
}

#[test]
fn empty_edit_abort_rebase() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from('1')],
		config,
		|mut test_context| {
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Empty;
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('1')),
				Artifact::ExitStatus(ExitStatus::Good)
			);
		},
	);
}

#[test]
fn empty_edit_re_edit_rebase_file() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");

	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from('2')],
		config,
		|mut test_context| {
			let todo_path = todo_file_path(&test_context);
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Empty;
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('2')),
				Artifact::ExternalCommand((String::from("editor"), vec![todo_path]))
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		},
	);
}

#[test]
fn empty_edit_undo_and_edit() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment", "drop bbb comment"],
		&[Event::from('3')],
		config,
		|mut test_context| {
			let todo_path = todo_file_path(&test_context);
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Empty;
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('3')),
				Artifact::ExternalCommand((String::from("editor"), vec![todo_path]))
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			assert_eq!(module.todo_file.lock().get_lines_owned(), vec![
				Line::parse("pick aaa comment").unwrap(),
				Line::parse("drop bbb comment").unwrap()
			]);
		},
	);
}

#[test]
fn empty_edit_noop() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(&["pick aaa comment"], &[], config, |test_context| {
		let mut module = ExternalEditor::new(&test_context.app_data());
		_ = test_context.activate(&mut module, State::List);
		module.todo_file.lock().set_lines(vec![]);
		module.state = ExternalEditorState::Empty;
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"The rebase file is empty.",
			"",
			"{BODY}",
			"1) Abort rebase",
			"2) Edit rebase file",
			"3) Undo modifications and edit rebase file",
			"",
			"Please choose an option."
		);
	});
}

#[test]
fn no_editor_set() {
	let mut config = Config::default();
	config.git.editor = String::new();
	testers::module_with_config(&["pick aaa comment"], &[], config, |test_context| {
		let mut module = ExternalEditor::new(&test_context.app_data());
		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::Error(
				anyhow!("No editor configured: Please see the git \"core.editor\" configuration for details"),
				Some(State::List)
			)
		);
	});
}

#[test]
fn editor_non_zero_exit() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from(StandardEvent::ExternalCommandError)],
		config,
		|mut test_context| {
			let mut module = ExternalEditor::new(&test_context.app_data());

			_ = test_context.activate(&mut module, State::List);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ExternalCommandError))
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
				"Editor returned a non-zero exit status",
				"",
				"{BODY}",
				"1) Abort rebase",
				"2) Edit rebase file",
				"3) Restore rebase file and abort edit",
				"4) Undo modifications and edit rebase file",
				"",
				"Please choose an option."
			);
		},
	);
}

#[test]
fn editor_reload_error() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment"],
		&[
			Event::from(KeyCode::Up),
			Event::from(StandardEvent::ExternalCommandSuccess),
		],
		config,
		|mut test_context| {
			let todo_path = todo_file_path(&test_context);
			let path = Path::new(todo_path.as_str());

			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			fs::remove_file(path).unwrap();
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ExternalCommandSuccess))
			);
			assert_external_editor_state_eq!(
				module.state,
				ExternalEditorState::Error(anyhow!("Unable to read file `{}`", todo_path))
			);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				format!("Unable to read file `{todo_path}`"),
				"",
				"{BODY}",
				"1) Abort rebase",
				"2) Edit rebase file",
				"3) Restore rebase file and abort edit",
				"4) Undo modifications and edit rebase file",
				"",
				"Please choose an option."
			);
		},
	);
}

#[test]
fn error_abort_rebase() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");

	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from('1')],
		config,
		|mut test_context| {
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Error(anyhow!("Error!"));
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('1')),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(module.todo_file.lock().is_empty());
		},
	);
}

#[test]
fn error_edit_rebase() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from('2')],
		config,
		|mut test_context| {
			let todo_path = todo_file_path(&test_context);
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Error(anyhow!("Error!"));
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('2')),
				Artifact::ExternalCommand((String::from("editor"), vec![todo_path]))
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		},
	);
}

#[test]
fn error_restore_and_abort() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from('3')],
		config,
		|mut test_context| {
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Error(anyhow!("Error!"));
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('3')),
				Artifact::ChangeState(State::List)
			);
			assert_eq!(module.todo_file.lock().get_lines_owned(), vec![
				Line::parse("pick aaa comment").unwrap()
			]);
		},
	);
}

#[test]
fn error_undo_modifications_and_reedit() {
	let mut config = Config::default();
	config.git.editor = String::from("editor");
	testers::module_with_config(
		&["pick aaa comment"],
		&[Event::from('4')],
		config,
		|mut test_context| {
			let todo_path = todo_file_path(&test_context);
			let mut module = ExternalEditor::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Error(anyhow!("Error!"));
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('4')),
				Artifact::ExternalCommand((String::from("editor"), vec![todo_path]))
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			assert_eq!(module.todo_file.lock().get_lines_owned(), vec![
				Line::parse("pick aaa comment").unwrap()
			]);
		},
	);
}
