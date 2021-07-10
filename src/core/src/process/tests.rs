use std::{path::Path, sync::atomic::Ordering};

use anyhow::anyhow;
use config::Theme;
use display::{testutil::CrossTerm, Display, Size};
use input::InputOptions;
use view::{assert_rendered_output, render_line, ViewData};

use super::*;
use crate::{
	module::Module,
	modules::{Error, WindowSizeError},
	testutil::module_test,
};

struct TestModule {
	event_callback: Box<dyn Fn(Event, &ViewSender, &mut TodoFile) -> ProcessResult>,
	view_data: ViewData,
	view_data_callback: Box<dyn Fn(&mut ViewData)>,
}

impl TestModule {
	fn new() -> Self {
		Self {
			event_callback: Box::new(|_, _, _| ProcessResult::new().event(Event::from(MetaEvent::Kill))),
			view_data: ViewData::new(|_| {}),
			view_data_callback: Box::new(|_| {}),
		}
	}
}

impl Module for TestModule {
	fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData {
		(self.view_data_callback)(&mut self.view_data);
		&self.view_data
	}

	fn handle_event(&mut self, event: Event, view_sender: &ViewSender, todo_file: &mut TodoFile) -> ProcessResult {
		(self.event_callback)(event, view_sender, todo_file)
	}
}

fn create_crossterm() -> CrossTerm {
	let mut crossterm = CrossTerm::new();
	crossterm.set_size(Size::new(100, 300));
	crossterm
}

fn create_modules() -> Modules<'static> {
	let mut modules = Modules::new();
	modules.register_module(State::Error, Error::new());
	modules.register_module(State::WindowSizeError, WindowSizeError::new());
	modules
}

#[test]
fn view_start_error() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		modules.register_module(State::List, TestModule::new());
		while process.view_sender.end().is_ok() {}
		assert_eq!(process.run(modules).unwrap(), ExitStatus::StateError);
	});
}

#[test]
fn window_too_small_on_start() {
	module_test(&["pick aaa comment"], &[Event::from(MetaEvent::Exit)], |test_context| {
		let mut crossterm = create_crossterm();
		crossterm.set_size(Size::new(1, 1));
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let modules = create_modules();
		let _ = process.run(modules).unwrap();
		assert_eq!(process.state, State::WindowSizeError);
	});
}

#[test]
fn render_error() {
	module_test(&["pick aaa comment"], &[Event::from(MetaEvent::Exit)], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		let mut test_module = TestModule::new();
		let sender = process.view_sender.clone();
		test_module.view_data_callback = Box::new(move |_| while sender.end().is_ok() {});
		modules.register_module(State::List, test_module);
		assert_eq!(process.run(modules).unwrap(), ExitStatus::StateError);
	});
}

#[test]
fn view_sender_is_poisoned() {
	module_test(&["pick aaa comment"], &[Event::from(MetaEvent::Exit)], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		let test_module = TestModule::new();
		process.view_sender.clone_poisoned().store(true, Ordering::Relaxed);
		modules.register_module(State::List, test_module);
		assert_eq!(process.run(modules).unwrap(), ExitStatus::StateError);
	});
}

#[test]
fn stop_error() {
	module_test(&["pick aaa comment"], &[Event::from(MetaEvent::Exit)], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		let mut test_module = TestModule::new();
		let sender = process.view_sender.clone();
		test_module.event_callback = Box::new(move |_, _, _| {
			while sender.end().is_ok() {}
			ProcessResult::new().event(Event::from(MetaEvent::Exit))
		});
		modules.register_module(State::List, test_module);
		assert_eq!(process.run(modules).unwrap(), ExitStatus::StateError);
	});
}

#[test]
fn handle_exit_event_that_is_not_kill() {
	module_test(&["pick aaa comment"], &[], |mut test_context| {
		test_context.rebase_todo_file.write_file().unwrap();
		test_context.rebase_todo_file.set_lines(vec![]);
		let mut shadow_rebase_file = test_context.new_todo_file();
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		let mut test_module = TestModule::new();
		test_module.event_callback = Box::new(|_, _, _| {
			ProcessResult::new()
				.event(Event::from(MetaEvent::Rebase))
				.exit_status(ExitStatus::Good)
		});
		modules.register_module(State::List, test_module);
		assert_eq!(process.run(modules).unwrap(), ExitStatus::Good);
		shadow_rebase_file.load_file().unwrap();
		assert!(shadow_rebase_file.is_empty());
	});
}

#[test]
fn handle_exit_event_that_is_kill() {
	module_test(&["pick aaa comment"], &[], |mut test_context| {
		test_context.rebase_todo_file.write_file().unwrap();
		test_context.rebase_todo_file.set_lines(vec![]);
		let mut shadow_rebase_file = test_context.new_todo_file();
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		let mut test_module = TestModule::new();
		test_module.event_callback = Box::new(|_, _, _| {
			ProcessResult::new()
				.event(Event::from(MetaEvent::Kill))
				.exit_status(ExitStatus::Kill)
		});
		modules.register_module(State::List, test_module);
		assert_eq!(process.run(modules).unwrap(), ExitStatus::Kill);
		shadow_rebase_file.load_file().unwrap();
		assert!(!shadow_rebase_file.is_empty());
	});
}

#[test]
fn handle_process_result_error() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		let result = ProcessResult::new().error(anyhow!("Test error"));
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.state, State::Error);
	});
}

#[test]
fn handle_process_result_new_state() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		modules.register_module(State::List, TestModule::new());
		let result = ProcessResult::new().state(State::Error);
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.state, State::Error);
	});
}

#[test]
fn handle_process_result_state_same() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		modules.register_module(State::List, TestModule::new());
		let result = ProcessResult::new().state(State::List);
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.state, State::List);
	});
}

#[test]
fn handle_process_result_exit_event() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut modules = create_modules();
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
fn handle_process_result_kill_event() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut modules = create_modules();
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
fn handle_process_result_resize_event_not_too_small() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut modules = create_modules();
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let result = ProcessResult::new().event(Event::Resize(100, 200));
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.render_context.width(), 100);
		assert_eq!(process.render_context.height(), 200);
		assert_eq!(process.state, State::List);
	});
}
#[test]
fn handle_process_result_resize_event_too_small() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut modules = create_modules();
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let result = ProcessResult::new().event(Event::Resize(10, 20));
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.render_context.width(), 10);
		assert_eq!(process.render_context.height(), 20);
		assert_eq!(process.state, State::WindowSizeError);
	});
}

#[test]
fn handle_process_result_other_event() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let mut modules = create_modules();
		let result = ProcessResult::new().event(Event::from('a'));
		process.handle_process_result(&mut modules, &result);
		assert_eq!(process.exit_status, None);
		assert_eq!(process.state, State::List);
	});
}

#[test]
fn handle_process_result_external_command_not_executable() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut modules = create_modules();
		let mut process = Process::new(
			test_context.rebase_todo_file,
			test_context.event_handler_context.event_handler,
			view,
		);
		let command = String::from(
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("..")
				.join("..")
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
			format!("{{Normal}}Unable to run {}", command),
			if cfg!(windows) {
				render_line!(StartsWith "{Normal}%1 is not a valid Win32 application.")
			}
			else {
				render_line!(StartsWith "{Normal}Permission denied")
			},
			"{TRAILING}",
			"{IndicatorColor}Press any key to continue"
		);
	});
}

#[test]
fn handle_process_result_external_command_executable_not_found() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut modules = create_modules();
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
			format!("{{Normal}}Unable to run {}", command),
			if cfg!(windows) {
				render_line!(StartsWith "{Normal}The system cannot find the path specified.")
			}
			else {
				render_line!(StartsWith "{Normal}No such file or directory")
			},
			"{TRAILING}",
			"{IndicatorColor}Press any key to continue"
		);
	});
}

#[test]
fn handle_process_result_external_command_status_success() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut modules = create_modules();
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
fn handle_process_result_external_command_status_error() {
	module_test(&["pick aaa comment"], &[], |test_context| {
		let crossterm = create_crossterm();
		let display = Display::new(crossterm, &Theme::new());
		let view = View::new(display, "~", "?");
		let mut modules = create_modules();
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
