use anyhow::anyhow;

use super::*;
use crate::{
	display::size::Size,
	process::testutil::{process_module_test, TestContext, ViewState},
	todo_file::line::Line,
};

#[test]
fn window_too_small() {
	process_module_test(
		&["pick aaa comment"],
		ViewState { size: Size::new(1, 1) },
		&[Event::from(MetaEvent::Exit)],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(process.run(modules).unwrap().unwrap(), ExitStatus::Abort);
		},
	);
}

#[test]
fn force_abort() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(MetaEvent::ForceAbort)],
		|test_context: TestContext<'_>| {
			let mut shadow_rebase_file = test_context.new_todo_file();
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(process.run(modules).unwrap().unwrap(), ExitStatus::Good);
			shadow_rebase_file.load_file().unwrap();
			assert!(shadow_rebase_file.is_empty());
		},
	);
}

#[test]
fn force_rebase() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(MetaEvent::ForceRebase)],
		|test_context: TestContext<'_>| {
			let mut shadow_rebase_file = test_context.new_todo_file();
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(process.run(modules).unwrap().unwrap(), ExitStatus::Good);
			shadow_rebase_file.load_file().unwrap();
			assert_eq!(shadow_rebase_file.get_lines_owned(), vec![Line::new(
				"pick aaa comment"
			)
			.unwrap()]);
		},
	);
}

#[test]
fn error_write_todo() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::from(MetaEvent::ForceRebase)],
		|test_context: TestContext<'_>| {
			let todo_path = test_context.get_todo_file_path();
			test_context.set_todo_file_readonly();
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(
				process.run(modules).unwrap_err().to_string(),
				format!("Error opening file: {}", todo_path)
			);
		},
	);
}

#[test]
fn resize_window_size_okay() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[Event::Resize(100, 100), Event::from(MetaEvent::Exit)],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(process.run(modules).unwrap().unwrap(), ExitStatus::Abort);
		},
	);
}

#[test]
fn resize_window_size_too_small() {
	process_module_test(
		&["pick aaa comment"],
		ViewState { size: Size::new(1, 1) },
		&[],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let mut modules = Modules::new(test_context.config);
			process.state = State::List;
			let result = ProcessResult::new().event(Event::Resize(0, 0));
			process.handle_process_result(&mut modules, &result);
		},
	);
}

#[test]
fn error() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let mut modules = Modules::new(test_context.config);
			let result = ProcessResult::new().error(anyhow!("Test error"));
			process.handle_process_result(&mut modules, &result);
		},
	);
}

#[test]
fn handle_exit_event() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut modules = Modules::new(test_context.config);
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let result = ProcessResult::new().event(Event::from(MetaEvent::Exit));
			process.handle_process_result(&mut modules, &result);
			assert_eq!(process.exit_status, Some(ExitStatus::Abort));
		},
	);
}

#[test]
fn handle_kill_event() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut modules = Modules::new(test_context.config);
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let result = ProcessResult::new().event(Event::from(MetaEvent::Kill));
			process.handle_process_result(&mut modules, &result);
			assert_eq!(process.exit_status, Some(ExitStatus::Kill));
		},
	);
}

#[test]
fn other_event() {
	process_module_test(
		&["pick aaa comment"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				test_context.view,
			);
			let mut modules = Modules::new(test_context.config);
			let result = ProcessResult::new().event(Event::from('a'));
			process.handle_process_result(&mut modules, &result);
		},
	);
}
