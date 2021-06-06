use std::path::Path;

use anyhow::anyhow;

use super::*;
use crate::{
	assert_rendered_output,
	display::{testutil::CrossTerm, Display, Size},
	input::InputOptions,
	process::testutil::{process_module_test, TestContext},
	todo_file::line::Line,
};

fn create_crossterm() -> CrossTerm {
	let mut crossterm = CrossTerm::new();
	crossterm.set_size(Size::new(100, 300));
	crossterm
}

#[test]
fn window_too_small() {
	process_module_test(
		&["pick aaa comment"],
		&[Event::from(MetaEvent::Exit)],
		|test_context: TestContext<'_>| {
			let crossterm = create_crossterm();
			let display = Display::new(crossterm, &test_context.config.theme);
			let view = View::new(display, "~", "?");
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(process.run(modules).unwrap(), ExitStatus::Abort);
		},
	);
}

#[test]
fn force_abort() {
	process_module_test(
		&["pick aaa comment"],
		&[Event::from(MetaEvent::ForceAbort)],
		|test_context: TestContext<'_>| {
			let crossterm = create_crossterm();
			let display = Display::new(crossterm, &test_context.config.theme);
			let view = View::new(display, "~", "?");
			let mut shadow_rebase_file = test_context.new_todo_file();
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(process.run(modules).unwrap(), ExitStatus::Good);
			shadow_rebase_file.load_file().unwrap();
			assert!(shadow_rebase_file.is_empty());
		},
	);
}

#[test]
fn force_rebase() {
	process_module_test(
		&["pick aaa comment"],
		&[Event::from(MetaEvent::ForceRebase)],
		|test_context: TestContext<'_>| {
			let crossterm = create_crossterm();
			let display = Display::new(crossterm, &test_context.config.theme);
			let view = View::new(display, "~", "?");
			let mut shadow_rebase_file = test_context.new_todo_file();
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(process.run(modules).unwrap(), ExitStatus::Good);
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
		&[Event::from(MetaEvent::ForceRebase)],
		|test_context: TestContext<'_>| {
			let crossterm = create_crossterm();
			let display = Display::new(crossterm, &test_context.config.theme);
			let view = View::new(display, "~", "?");
			let todo_path = test_context.get_todo_file_path();
			test_context.set_todo_file_readonly();
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				view,
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
		&[Event::Resize(100, 100), Event::from(MetaEvent::Exit)],
		|test_context: TestContext<'_>| {
			let crossterm = create_crossterm();
			let display = Display::new(crossterm, &test_context.config.theme);
			let view = View::new(display, "~", "?");
			let mut process = Process::new(
				test_context.rebase_todo_file,
				test_context.event_handler_context.event_handler,
				view,
			);
			let modules = Modules::new(test_context.config);
			assert_eq!(process.run(modules).unwrap(), ExitStatus::Abort);
		},
	);
}

#[test]
fn resize_window_size_too_small() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = Modules::new(test_context.config);
		process.state = State::List;
		let result = ProcessResult::new().event(Event::Resize(0, 0));
		process.handle_process_result(&mut modules, &result);
	});
}

#[test]
fn error() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = Modules::new(test_context.config);
		let result = ProcessResult::new().error(anyhow!("Test error"));
		process.handle_process_result(&mut modules, &result);
	});
}

#[test]
fn handle_exit_event() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut modules = Modules::new(test_context.config);
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let result = ProcessResult::new().event(Event::from(MetaEvent::Exit));
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.exit_status, Some(ExitStatus::Abort));
	});
}

#[test]
fn handle_kill_event() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut modules = Modules::new(test_context.config);
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let result = ProcessResult::new().event(Event::from(MetaEvent::Kill));
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.exit_status, Some(ExitStatus::Kill));
	});
}

#[test]
fn other_event() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = Modules::new(test_context.config);
		let result = ProcessResult::new().event(Event::from('a'));
		process.handle_process_result(&mut modules, &result);
	});
}

#[test]
fn handle_external_command_not_executable() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut modules = Modules::new(test_context.config);
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let command = String::from(
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("test")
				.join("not-executable.sh")
				.to_str()
				.unwrap(),
		);
		let result = ProcessResult::new().external_command(command.clone(), vec![]);
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.state, State::Error);
		let view_data = modules.build_view_data(process.state, &process.render_context, &process.rebase_todo);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			format!("{{Normal}}Unable to run {} ", command),
			if cfg!(windows) {
				"{Normal}%1 is not a valid Win32 application. (os error 193)"
			}
			else {
				"{Normal}Permission denied (os error 13)"
			},
			"{TRAILING}",
			"{IndicatorColor}Press any key to continue"
		);
	});
}

#[test]
fn handle_external_command_executable_not_found() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut modules = Modules::new(test_context.config);
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let command = String::from(
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("test")
				.join("not-found.sh")
				.to_str()
				.unwrap(),
		);
		let result = ProcessResult::new().external_command(command.clone(), vec![]);
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.state, State::Error);
		let view_data = modules.build_view_data(process.state, &process.render_context, &process.rebase_todo);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{BODY}",
			format!("{{Normal}}Unable to run {} ", command),
			if cfg!(windows) {
				"{Normal}The system cannot find the file specified. (os error 2)"
			}
			else {
				"{Normal}No such file or directory (os error 2)"
			},
			"{TRAILING}",
			"{IndicatorColor}Press any key to continue"
		);
	});
}

#[test]
fn handle_external_command_status_success() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut modules = Modules::new(test_context.config);
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let command = String::from("true");
		let result = ProcessResult::new().external_command(command, vec![]);
		process.handle_process_result(&mut modules, &result);
		assert_eq!(
			process.event_handler.read_event(&InputOptions::new(), |e, _| e),
			Event::from(MetaEvent::ExternalCommandSuccess)
		);
	});
}

#[test]
fn handle_external_command_status_error() {
	process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &test_context.config.theme);
		let view = View::new(display, "~", "?");
		let mut modules = Modules::new(test_context.config);
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let command = String::from("false");
		let result = ProcessResult::new().external_command(command, vec![]);
		process.handle_process_result(&mut modules, &result);
		assert_eq!(
			process.event_handler.read_event(&InputOptions::new(), |e, _| e),
			Event::from(MetaEvent::ExternalCommandError)
		);
	});
}
