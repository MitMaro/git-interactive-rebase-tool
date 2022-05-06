use std::{io::Write, path::Path, sync::atomic::Ordering};

use anyhow::anyhow;
use config::Theme;
use display::{testutil::CrossTerm, Display, Size};
use input::EventHandler;
use tempfile::{Builder, NamedTempFile};
use view::{assert_rendered_output, render_line};

use super::*;
use crate::{
	modules::{Error, WindowSizeError},
	testutil::{create_test_keybindings, TestModule},
};

fn create_crossterm() -> CrossTerm {
	let mut crossterm = CrossTerm::new();
	crossterm.set_size(Size::new(100, 300));
	crossterm
}

fn create_modules() -> Modules<'static> {
	let mut modules = Modules::new(EventHandler::new(create_test_keybindings()));
	modules.register_module(State::Error, Error::new());
	modules.register_module(State::WindowSizeError, WindowSizeError::new());
	modules.register_module(State::List, TestModule::new());
	modules
}

fn create_todo_file() -> (TodoFile, NamedTempFile) {
	let todo_file_path = Builder::new()
		.prefix("git-rebase-todo-scratch")
		.suffix("")
		.tempfile()
		.unwrap();
	write!(todo_file_path.as_file(), "pick aaa comment").unwrap();
	let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), 1, "#");
	todo_file.load_file().unwrap();
	(todo_file, todo_file_path)
}

fn create_shadow_todo_file(todo_file: &TodoFile) -> TodoFile {
	TodoFile::new(todo_file.get_filepath(), 1, "#")
}

fn create_process(rebase_todo_file: TodoFile, events: &[Event]) -> Process {
	let crossterm = create_crossterm();
	let display = Display::new(crossterm, &Theme::new());
	let view = View::new(display, "~", "?");
	let process = Process::new(rebase_todo_file, view);
	for event in events {
		process.event_sender.enqueue_event(*event).unwrap();
	}
	process
		.event_sender
		.enqueue_event(Event::from(StandardEvent::Kill))
		.unwrap();
	process
}

#[test]
fn view_start_error() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let modules = create_modules();
	while process.view_sender.end().is_ok() {}
	assert_eq!(process.run(modules).unwrap(), ExitStatus::StateError);
}

#[test]
fn window_too_small_on_start() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[Event::Resize(1, 1)]);
	let modules = create_modules();
	let _ = process.run(modules).unwrap();
	assert_eq!(process.state, State::WindowSizeError);
}

#[test]
fn render_error() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let sender = process.view_sender.clone();
	let mut test_module = TestModule::new();
	test_module.view_data_callback(move |_| while sender.end().is_ok() {});
	modules.register_module(State::List, test_module);
	assert_eq!(process.run(modules).unwrap(), ExitStatus::StateError);
}

#[test]
fn view_sender_is_poisoned() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[Event::from(StandardEvent::Exit)]);
	let modules = create_modules();
	process.view_sender.clone_poisoned().store(true, Ordering::Release);
	assert_eq!(process.run(modules).unwrap(), ExitStatus::StateError);
}

#[test]
fn stop_error() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[Event::from(StandardEvent::Exit)]);
	let mut modules = create_modules();
	let mut test_module = TestModule::new();
	let sender = process.view_sender.clone();
	test_module.event_callback(move |event, _, _| {
		while sender.end().is_ok() {}
		let mut results = Results::new();
		results.event(event);
		results
	});
	modules.register_module(State::List, test_module);
	assert_eq!(process.run(modules).unwrap(), ExitStatus::StateError);
}

#[test]
fn handle_exit_event_that_is_not_kill() {
	let (mut rebase_todo_file, _file_path) = create_todo_file();
	rebase_todo_file.set_lines(vec![]);
	let mut shadow_rebase_file = create_shadow_todo_file(&rebase_todo_file);
	let mut process = create_process(rebase_todo_file, &[Event::from('q')]);
	let mut modules = create_modules();
	let mut test_module = TestModule::new();
	test_module.event_callback(|_, _, _| {
		let mut results = Results::new();
		results.event(Event::from(MetaEvent::Rebase));
		results.exit_status(ExitStatus::Good);
		results
	});
	modules.register_module(State::List, test_module);
	assert_eq!(process.run(modules).unwrap(), ExitStatus::Good);
	shadow_rebase_file.load_file().unwrap();
	assert!(shadow_rebase_file.is_empty());
}

#[test]
fn handle_exit_event_that_is_kill() {
	let (mut rebase_todo_file, _file_path) = create_todo_file();
	rebase_todo_file.set_lines(vec![]);
	let mut shadow_rebase_file = create_shadow_todo_file(&rebase_todo_file);
	let mut process = create_process(rebase_todo_file, &[Event::from('q')]);
	let mut modules = create_modules();
	let mut test_module = TestModule::new();
	test_module.event_callback(|_, _, _| {
		let mut results = Results::new();
		results.event(Event::from(StandardEvent::Kill));
		results.exit_status(ExitStatus::Kill);
		results
	});
	modules.register_module(State::List, test_module);
	assert_eq!(process.run(modules).unwrap(), ExitStatus::Kill);
	shadow_rebase_file.load_file().unwrap();
	assert!(!shadow_rebase_file.is_empty());
}

#[test]
fn handle_none_event() {
	let (mut rebase_todo_file, _file_path) = create_todo_file();
	rebase_todo_file.set_lines(vec![]);
	let mut process = create_process(rebase_todo_file, &[Event::None, Event::from('q')]);
	let mut modules = create_modules();
	let mut test_module = TestModule::new();
	test_module.event_callback(|event, _, _| {
		let mut results = Results::new();
		if event != Event::None {
			results.event(Event::from(MetaEvent::Rebase));
		}
		results.exit_status(ExitStatus::Good);
		results
	});
	modules.register_module(State::List, test_module);
	assert_eq!(process.run(modules).unwrap(), ExitStatus::Good);
}

#[test]
fn handle_results_error() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.error(anyhow!("Test error"));
	process.handle_results(&mut modules, results);
	assert_eq!(process.state, State::Error);
}

#[test]
fn handle_results_with_return_error() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.error_with_return(anyhow!("Test error"), State::ExternalEditor);
	process.handle_results(&mut modules, results);
	assert_eq!(process.state, State::Error);
}

#[test]
fn handle_results_change_state() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.state(State::Error);
	process.handle_results(&mut modules, results);
	assert_eq!(process.state, State::Error);
}

#[test]
fn handle_results_state_same() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.state(State::List);
	process.handle_results(&mut modules, results);
	assert_eq!(process.state, State::List);
}

#[test]
fn handle_results_exit_event() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.event(Event::from(StandardEvent::Exit));
	process.handle_results(&mut modules, results);
	assert_eq!(process.exit_status, Some(ExitStatus::Abort));
}

#[test]
fn handle_results_kill_event() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.event(Event::from(StandardEvent::Kill));
	process.handle_results(&mut modules, results);
	assert_eq!(process.exit_status, Some(ExitStatus::Kill));
}

#[test]
fn handle_results_resize_event_not_too_small() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.event(Event::Resize(100, 200));
	process.handle_results(&mut modules, results);
	assert_eq!(process.render_context.width(), 100);
	assert_eq!(process.render_context.height(), 200);
	assert_eq!(process.state, State::List);
}

#[test]
fn handle_results_resize_event_too_small() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.event(Event::Resize(10, 20));
	process.handle_results(&mut modules, results);
	assert_eq!(process.render_context.width(), 10);
	assert_eq!(process.render_context.height(), 20);
	assert_eq!(process.state, State::WindowSizeError);
}

#[test]
fn handle_results_other_event() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let mut results = Results::new();
	results.event(Event::from('a'));
	process.handle_results(&mut modules, results);
	assert_eq!(process.exit_status, None);
	assert_eq!(process.state, State::List);
}

#[test]
fn handle_results_external_command_not_executable() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let command = String::from(
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("..")
			.join("..")
			.join("test")
			.join("not-executable.sh")
			.to_str()
			.unwrap(),
	);
	let mut results = Results::new();
	results.external_command(command.clone(), vec![]);
	process.handle_results(&mut modules, results);
	assert_eq!(process.state, State::Error);
	let view_data = modules.build_view_data(process.state, &process.render_context, &process.rebase_todo);
	assert_rendered_output!(
		view_data,
		"{TITLE}",
		"{BODY}",
		format!("{{Normal}}Unable to run {}", command),
		if cfg!(windows) {
			"{Normal}%1 is not a valid Win32 application. (os error 193)"
		}
		else {
			"{Normal}Permission denied (os error 13)"
		},
		"{TRAILING}",
		"{IndicatorColor}Press any key to continue"
	);
}

#[test]
fn handle_results_external_command_executable_not_found() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let command = String::from(
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("not-found.sh")
			.to_str()
			.unwrap(),
	);
	let mut results = Results::new();
	results.external_command(command.clone(), vec![]);
	process.handle_results(&mut modules, results);
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
}

#[test]
fn handle_results_external_command_status_success() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let command = String::from("true");
	let mut results = Results::new();
	results.external_command(command.clone(), vec![]);
	process.handle_results(&mut modules, results);
	process.event_sender.end().unwrap();
	let mut last_event = Event::None;
	while !process.event_sender.is_poisoned() {}
	loop {
		let event = process.event_sender.read_event();
		if event == Event::None {
			break;
		}
		last_event = event
	}
	assert_eq!(last_event, Event::from(MetaEvent::ExternalCommandSuccess));
}

#[test]
fn handle_results_external_command_status_error() {
	let (rebase_todo_file, _file_path) = create_todo_file();
	let mut process = create_process(rebase_todo_file, &[]);
	let mut modules = create_modules();
	let command = String::from("false");
	let mut results = Results::new();
	results.external_command(command.clone(), vec![]);
	process.handle_results(&mut modules, results);
	let mut last_event = Event::None;
	process.event_sender.end().unwrap();
	while !process.event_sender.is_poisoned() {}
	loop {
		let event = process.event_sender.read_event();
		if event == Event::None {
			break;
		}
		last_event = event
	}
	assert_eq!(last_event, Event::from(MetaEvent::ExternalCommandError));
}
