use super::*;
// use crate::assert_process_result;
// use crate::assert_rendered_output;
use super::testutil::{process_module_test, TestContext, ViewState};
use crate::todo_file::line::Line;
use anyhow::anyhow;

#[test]
#[serial_test::serial]
fn window_too_small() {
	process_module_test(
		&["pick aaa comment"],
		ViewState {
			size: (1, 1),
			..ViewState::default()
		},
		&[Input::Exit],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let modules = Modules::new(test_context.display, test_context.config);
			assert_eq!(process.run(modules).unwrap().unwrap(), ExitStatus::Abort);
		},
	);
}

#[test]
#[serial_test::serial]
fn force_abort() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Input::ForceAbort],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let modules = Modules::new(test_context.display, test_context.config);
			assert_eq!(process.run(modules).unwrap().unwrap(), ExitStatus::Good);
			process.rebase_todo.load_file().unwrap();
			assert_eq!(process.rebase_todo.get_lines(), &vec![]);
		},
	);
}

#[test]
#[serial_test::serial]
fn force_rebase() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Input::ForceRebase],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let modules = Modules::new(test_context.display, test_context.config);
			assert_eq!(process.run(modules).unwrap().unwrap(), ExitStatus::Good);
			process.rebase_todo.load_file().unwrap();
			assert_eq!(process.rebase_todo.get_lines(), &vec![
				Line::new("pick aaa comment").unwrap()
			]);
		},
	);
}

#[test]
#[serial_test::serial]
fn error_write_todo() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Input::ForceRebase],
		|test_context: TestContext<'_>| {
			let todo_path = test_context.get_todo_file_path();
			test_context.set_todo_file_readonly();
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let modules = Modules::new(test_context.display, test_context.config);
			assert_eq!(
				process.run(modules).unwrap_err().to_string(),
				format!("Error opening file: {}", todo_path)
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn resize_window_size_okay() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Input::Resize, Input::Exit],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let modules = Modules::new(test_context.display, test_context.config);
			assert_eq!(process.run(modules).unwrap().unwrap(), ExitStatus::Abort);
		},
	);
}

#[test]
#[serial_test::serial]
fn resize_window_size_too_small() {
	process_module_test(
		&["pick aaa comment"],
		ViewState {
			size: (1, 1),
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let mut modules = Modules::new(test_context.display, test_context.config);
			process.state = State::List;
			let result = ProcessResult::new().input(Input::Resize);
			process.handle_process_result(&mut modules, &result);
		},
	);
}

#[test]
#[serial_test::serial]
fn error() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let mut modules = Modules::new(test_context.display, test_context.config);
			let result = ProcessResult::new().error(anyhow!("Test error"));
			process.handle_process_result(&mut modules, &result);
		},
	);
}

#[test]
#[serial_test::serial]
fn help_start() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let mut modules = Modules::new(test_context.display, test_context.config);
			let result = ProcessResult::new().input(Input::Help);
			process.handle_process_result(&mut modules, &result);
		},
	);
}

#[test]
#[serial_test::serial]
fn help_exit() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let mut modules = Modules::new(test_context.display, test_context.config);
			process.state = State::Help;
			let result = ProcessResult::new().input(Input::Help);
			process.handle_process_result(&mut modules, &result);
		},
	);
}

#[test]
#[serial_test::serial]
fn other_input() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.view,
				test_context.input_handler,
			);
			let mut modules = Modules::new(test_context.display, test_context.config);
			let result = ProcessResult::new().input(Input::Character('a'));
			process.handle_process_result(&mut modules, &result);
		},
	);
}
