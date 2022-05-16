use std::{env::set_var, fs::File, path::Path};

use ::git::Repository;
use input::StandardEvent;
use todo_file::TodoFile;
use view::ViewSender;

use super::*;
use crate::{
	events::Event,
	module::{ExitStatus, Module},
	process::Results,
	run::{create_module_handler, create_process, load_config, load_todo_file, run_process},
	testutil::create_test_module_handler,
};

fn set_git_directory(repo: &str) -> String {
	let path = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("..")
		.join("..")
		.join("test")
		.join(repo)
		.canonicalize()
		.unwrap();
	set_var("GIT_DIR", path.to_str().unwrap());
	String::from(path.to_str().unwrap())
}

fn args(args: &[&str]) -> Vec<OsString> {
	args.iter().map(OsString::from).collect()
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
#[allow(unsafe_code)]
fn load_todo_file_error_non_utf_arg() {
	let todo_file = unsafe { String::from_utf8_unchecked(vec![0xC3, 0x28]) };
	assert_eq!(
		run(args(&[todo_file.as_str()])),
		Exit::new(ExitStatus::StateError, "argument is not a UTF-8 string")
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
fn module_handler() {
	let _ = set_git_directory("fixtures/simple");
	let repo = Repository::open_from_env().unwrap();
	let config = load_config(&repo).unwrap();
	let _ = create_module_handler(&config, repo);
}

#[test]
#[serial_test::serial]
fn run_process_error() {
	struct TestModule {}

	impl Module for TestModule {
		fn handle_event(&mut self, _: Event, _: &ViewSender, _: &mut TodoFile) -> Results {
			Results::from(Event::from(StandardEvent::Exit))
		}
	}

	let path = set_git_directory("fixtures/simple");
	let todo_file_path = Path::new(path.as_str()).join("rebase-todo-readonly");
	let todo_file = File::open(todo_file_path.as_path()).unwrap();
	let mut permissions = todo_file.metadata().unwrap().permissions();
	permissions.set_readonly(true);
	todo_file.set_permissions(permissions).unwrap();

	let repo = Repository::open_from_env().unwrap();
	let config = load_config(&repo).unwrap();
	let rebase_todo_file = load_todo_file(todo_file_path.to_str().unwrap(), &config).unwrap();
	let process = create_process(rebase_todo_file, &config);

	let test_module = TestModule {};
	let module_provider = create_test_module_handler(test_module);
	assert_eq!(
		run_process(process, module_provider),
		Exit::new(
			ExitStatus::FileWriteError,
			format!("Error opening file: {}", todo_file_path.to_str().unwrap()).as_str()
		)
	);
}

#[test]
#[serial_test::serial]
fn run_process_success() {
	struct TestModule {}

	impl Module for TestModule {
		fn handle_event(&mut self, _: Event, _: &ViewSender, _: &mut TodoFile) -> Results {
			Results::from(Event::from(StandardEvent::Exit))
		}
	}

	let path = set_git_directory("fixtures/simple");
	let todo_file = Path::new(path.as_str()).join("rebase-todo");
	let repo = Repository::open_from_env().unwrap();
	let config = load_config(&repo).unwrap();
	let rebase_todo_file = load_todo_file(todo_file.to_str().unwrap(), &config).unwrap();
	let process = create_process(rebase_todo_file, &config);

	let test_module = TestModule {};
	let module_provider = create_test_module_handler(test_module);
	assert_eq!(run_process(process, module_provider), Exit::from(ExitStatus::Abort));
}
