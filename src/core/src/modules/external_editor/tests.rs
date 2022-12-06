use std::{fs, fs::File};

use input::KeyCode;
use view::assert_rendered_output;

use super::*;
use crate::{assert_results, events::Event, module::ExitStatus, process::Artifact, testutil::module_test};

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

fn create_external_editor(editor: &str, todo_file: TodoFile) -> ExternalEditor {
	ExternalEditor::new(editor, Arc::new(Mutex::new(todo_file)))
}

#[test]
fn activate() {
	module_test(&["pick aaa comment1", "drop bbb comment2"], &[], |mut test_context| {
		let todo_file = test_context.take_todo_file();
		let todo_path = String::from(todo_file.get_filepath().to_str().unwrap());

		let mut module = create_external_editor("editor", todo_file);
		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::ExternalCommand((String::from("editor"), vec![String::from(todo_path.as_str())]))
		);
		assert_eq!(module.todo_file.lock().get_lines_owned(), vec![
			Line::new("pick aaa comment1").unwrap(),
			Line::new("drop bbb comment2").unwrap()
		]);
		assert!(!module.lines.is_empty());
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		assert_eq!(module.external_command, (String::from("editor"), vec![todo_path]));
	});
}

#[test]
fn activate_write_file_fail() {
	module_test(&["pick aaa comment"], &[], |mut test_context| {
		let todo_file = test_context.take_todo_file();
		let path = todo_file.get_filepath();
		let todo_file_path = String::from(path.to_str().unwrap());
		let file = File::open(path).unwrap();
		let mut permissions = file.metadata().unwrap().permissions();
		permissions.set_readonly(true);
		file.set_permissions(permissions).unwrap();

		let mut module = create_external_editor("editor", todo_file);

		assert_results!(
			test_context.activate(&mut module, State::List),
			Artifact::Error(anyhow!("Unable to read file `{}`", todo_file_path), Some(State::List))
		);
	});
}

#[test]
fn activate_file_placement_marker() {
	module_test(&[], &[], |mut test_context| {
		let todo_file = test_context.take_todo_file();
		let todo_path = String::from(todo_file.get_filepath().to_str().unwrap());

		let mut module = create_external_editor("editor a % b", todo_file);
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
	module_test(&["pick aaa comment", "drop bbb comment2"], &[], |mut test_context| {
		let mut module = create_external_editor("editor", test_context.take_todo_file());
		let _ = test_context.deactivate(&mut module);
		assert_eq!(module.lines, vec![]);
	});
}

#[test]
fn edit_success() {
	module_test(
		&["pick aaa comment"],
		&[Event::from(MetaEvent::ExternalCommandSuccess)],
		|mut test_context| {
			let mut module = create_external_editor("editor", test_context.take_todo_file());
			let _ = test_context.activate(&mut module, State::List);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(view_data, "{TITLE}", "{LEADING}", "{Normal}Editing...");
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ExternalCommandSuccess)),
				Artifact::ChangeState(State::List)
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		},
	);
}

#[test]
fn empty_edit_error() {
	module_test(
		&["pick aaa comment"],
		&[Event::from('1'), Event::from(MetaEvent::ExternalCommandSuccess)],
		|mut test_context| {
			let mut module = create_external_editor("editor", test_context.take_todo_file());
			let _ = test_context.activate(&mut module, State::List);
			let mut todo_file = module.todo_file.lock();
			todo_file.set_lines(vec![]);
			todo_file.write_file().unwrap();
			drop(todo_file);
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ExternalCommandSuccess))
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
		},
	);
}

#[test]
fn empty_edit_abort_rebase() {
	module_test(&["pick aaa comment"], &[Event::from('1')], |mut test_context| {
		let mut module = create_external_editor("editor", test_context.take_todo_file());
		let _ = test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Empty;
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from('1')),
			Artifact::ExitStatus(ExitStatus::Good)
		);
	});
}

#[test]
fn empty_edit_re_edit_rebase_file() {
	module_test(&["pick aaa comment"], &[Event::from('2')], |mut test_context| {
		let todo_file = test_context.take_todo_file();
		let todo_path = String::from(todo_file.get_filepath().to_str().unwrap());

		let mut module = create_external_editor("editor", todo_file);
		let _ = test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Empty;
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from('2')),
			Artifact::ExternalCommand((String::from("editor"), vec![todo_path]))
		);
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
	});
}

#[test]
fn empty_edit_undo_and_edit() {
	module_test(
		&["pick aaa comment", "drop bbb comment"],
		&[Event::from('3')],
		|mut test_context| {
			let todo_file = test_context.take_todo_file();
			let todo_path = String::from(todo_file.get_filepath().to_str().unwrap());

			let mut module = create_external_editor("editor", todo_file);
			let _ = test_context.activate(&mut module, State::List);
			module.state = ExternalEditorState::Empty;
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('3')),
				Artifact::ExternalCommand((String::from("editor"), vec![todo_path]))
			);
			assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			assert_eq!(module.todo_file.lock().get_lines_owned(), vec![
				Line::new("pick aaa comment").unwrap(),
				Line::new("drop bbb comment").unwrap()
			]);
		},
	);
}

#[test]
fn empty_edit_noop() {
	module_test(&["pick aaa comment"], &[], |mut test_context| {
		let mut module = create_external_editor("editor", test_context.take_todo_file());
		let _ = test_context.activate(&mut module, State::List);
		module.todo_file.lock().set_lines(vec![]);
		module.state = ExternalEditorState::Empty;
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
	module_test(&["pick aaa comment"], &[], |mut test_context| {
		let mut module = create_external_editor("", test_context.take_todo_file());
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
	module_test(
		&["pick aaa comment"],
		&[Event::from(MetaEvent::ExternalCommandError)],
		|mut test_context| {
			let mut module = create_external_editor("editor", test_context.take_todo_file());
			let _ = test_context.activate(&mut module, State::List);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ExternalCommandError))
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
		},
	);
}

#[test]
fn editor_reload_error() {
	module_test(
		&["pick aaa comment"],
		&[Event::from(KeyCode::Up), Event::from(MetaEvent::ExternalCommandSuccess)],
		|mut test_context| {
			let todo_file = test_context.take_todo_file();
			let path = todo_file.get_filepath().to_path_buf();
			let todo_path = String::from(path.to_str().unwrap());
			let mut module = create_external_editor("editor", todo_file);
			let _ = test_context.activate(&mut module, State::List);
			fs::remove_file(path).unwrap();
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ExternalCommandSuccess))
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
				format!("{{Normal}}Unable to read file `{todo_path}`"),
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
	module_test(&["pick aaa comment"], &[Event::from('1')], |mut test_context| {
		let mut module = create_external_editor("editor", test_context.take_todo_file());
		let _ = test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Error(anyhow!("Error!"));
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from('1')),
			Artifact::ExitStatus(ExitStatus::Good)
		);
		assert!(module.todo_file.lock().is_empty());
	});
}

#[test]
fn error_edit_rebase() {
	module_test(&["pick aaa comment"], &[Event::from('2')], |mut test_context| {
		let todo_file = test_context.take_todo_file();
		let todo_path = String::from(todo_file.get_filepath().to_str().unwrap());
		let mut module = create_external_editor("editor", todo_file);
		let _ = test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Error(anyhow!("Error!"));
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from('2')),
			Artifact::ExternalCommand((String::from("editor"), vec![todo_path]))
		);
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
	});
}

#[test]
fn error_restore_and_abort() {
	module_test(&["pick aaa comment"], &[Event::from('3')], |mut test_context| {
		let mut module = create_external_editor("editor", test_context.take_todo_file());
		let _ = test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Error(anyhow!("Error!"));
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from('3')),
			Artifact::ChangeState(State::List)
		);
		assert_eq!(module.todo_file.lock().get_lines_owned(), vec![
			Line::new("pick aaa comment").unwrap()
		]);
	});
}

#[test]
fn error_undo_modifications_and_reedit() {
	module_test(&["pick aaa comment"], &[Event::from('4')], |mut test_context| {
		let todo_file = test_context.take_todo_file();
		let todo_path = String::from(todo_file.get_filepath().to_str().unwrap());
		let mut module = create_external_editor("editor", todo_file);
		let _ = test_context.activate(&mut module, State::List);
		module.state = ExternalEditorState::Error(anyhow!("Error!"));
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from('4')),
			Artifact::ExternalCommand((String::from("editor"), vec![todo_path]))
		);
		assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
		assert_eq!(module.todo_file.lock().get_lines_owned(), vec![
			Line::new("pick aaa comment").unwrap()
		]);
	});
}
