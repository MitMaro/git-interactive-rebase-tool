use std::{env::set_var, fs::File, path::Path};

use display::{testutil::CrossTerm, Tui};
use input::{Event, EventHandler, KeyBindings, MetaEvent};

use super::*;
use crate::core::{
	main::{load_config, load_todo_file, run_process},
	module::ExitStatus,
};

fn set_git_directory(repo: &str) -> String {
	let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test").join(repo);
	set_var("GIT_DIR", path.to_str().unwrap());
	String::from(path.to_str().unwrap())
}

fn args(args: &[&str]) -> Vec<OsString> {
	args.iter().map(OsString::from).collect()
}

#[test]
#[serial_test::serial]
fn load_config_error_loading() {
	let path = set_git_directory("fixtures/invalid-config");
	assert_eq!(
		run(args(&["rebase-todo"])),
		Exit::new(
			ExitStatus::ConfigError,
			format!("Error loading git config: could not find repository from '{}'", path).as_str(),
		)
	);
}

#[test]
#[serial_test::serial]
fn load_todo_file_error_loading_file() {
	let path = set_git_directory("fixtures/simple");
	let todo_file = Path::new(path.as_str()).join("does-not-exist");
	assert_eq!(
		run(args(&[todo_file.to_str().unwrap()])),
		Exit::new(ExitStatus::FileReadError, "No such file or directory (os error 2)")
	);
}

#[test]
#[serial_test::serial]
fn load_todo_file_error_noop() {
	let path = set_git_directory("fixtures/simple");
	let todo_file = Path::new(path.as_str()).join("rebase-todo-noop");
	assert_eq!(
		run(args(&[todo_file.to_str().unwrap()])),
		Exit::new(ExitStatus::Good, "A noop rebase was provided, skipping editing")
	);
}

#[test]
#[serial_test::serial]
fn load_todo_file_error_empty_file() {
	let path = set_git_directory("fixtures/simple");
	let todo_file = Path::new(path.as_str()).join("rebase-todo-empty");
	assert_eq!(
		run(args(&[todo_file.to_str().unwrap()])),
		Exit::new(ExitStatus::Good, "An empty rebase was provided, nothing to edit")
	);
}

#[test]
#[serial_test::serial]
fn run_with_no_rebase_todo_filepath() {
	let exit = run(args(&[]));
	assert_eq!(exit.get_status(), &ExitStatus::StateError);
	assert!(exit
		.get_message()
		.as_ref()
		.unwrap()
		.contains("A todo file path must be provided."));
}

#[test]
#[serial_test::serial]
fn run_with_argument_version_long() {
	let exit = run(args(&["--version"]));
	assert_eq!(exit.get_status(), &ExitStatus::Good);
	assert!(exit
		.get_message()
		.as_ref()
		.unwrap()
		.contains("interactive-rebase-tool "));
}

#[test]
#[serial_test::serial]
fn run_with_argument_version_short() {
	let exit = run(args(&["-v"]));
	assert_eq!(exit.get_status(), &ExitStatus::Good);
	assert!(exit
		.get_message()
		.as_ref()
		.unwrap()
		.contains("interactive-rebase-tool "));
}

#[test]
#[serial_test::serial]
fn run_with_argument_help_long() {
	let exit = run(args(&["--help"]));
	assert_eq!(exit.get_status(), &ExitStatus::Good);
	assert!(exit.get_message().as_ref().unwrap().contains("USAGE:"));
}

#[test]
#[serial_test::serial]
fn run_with_argument_help_short() {
	let exit = run(args(&["-h"]));
	assert_eq!(exit.get_status(), &ExitStatus::Good);
	assert!(exit.get_message().as_ref().unwrap().contains("USAGE:"));
}

#[test]
#[serial_test::serial]
fn run_with_argument_license() {
	let exit = run(args(&["--license"]));
	assert_eq!(exit.get_status(), &ExitStatus::Good);
	assert!(exit
		.get_message()
		.as_ref()
		.unwrap()
		.contains("This program is free software: you can redistribute it and/or modify"));
}

#[test]
#[serial_test::serial]
fn run_process_error() {
	let path = set_git_directory("fixtures/simple");
	let todo_file_path = Path::new(path.as_str()).join("rebase-todo-readonly");
	let todo_file = File::open(todo_file_path.as_path()).unwrap();
	let mut permissions = todo_file.metadata().unwrap().permissions();
	permissions.set_readonly(true);
	todo_file.set_permissions(permissions).unwrap();
	let config = load_config().unwrap();
	let rebase_todo_file = load_todo_file(todo_file_path.to_str().unwrap(), &config).unwrap();
	let event_handler = EventHandler::new(CrossTerm::read_event, KeyBindings::new(&config.key_bindings));
	event_handler.push_event(Event::from(MetaEvent::Exit));
	assert_eq!(
		run_process(rebase_todo_file, event_handler, &config),
		Exit::new(
			ExitStatus::FileWriteError,
			format!("Error opening file: {}", todo_file_path.to_str().unwrap()).as_str()
		)
	);
}

#[test]
#[serial_test::serial]
fn run_process_success() {
	let path = set_git_directory("fixtures/simple");
	let todo_file = Path::new(path.as_str()).join("rebase-todo");
	let config = load_config().unwrap();
	let rebase_todo_file = load_todo_file(todo_file.to_str().unwrap(), &config).unwrap();
	let event_handler = EventHandler::new(CrossTerm::read_event, KeyBindings::new(&config.key_bindings));
	event_handler.push_event(Event::from(MetaEvent::Exit));
	assert_eq!(
		run_process(rebase_todo_file, event_handler, &config),
		Exit::from(ExitStatus::Abort)
	);
}
