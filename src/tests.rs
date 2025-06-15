use std::path::Path;

use super::*;
use crate::{module::ExitStatus, test_helpers::with_git_directory};

#[test]
#[serial_test::serial]
fn successful_run_help() {
	let args = ["--help"].into_iter().map(OsString::from).collect();
	let git_config_parameters = Vec::new();
	let exit = run(args, git_config_parameters);
	assert!(exit.get_message().unwrap().contains("USAGE:"));
	assert_eq!(exit.get_status(), &ExitStatus::Good);
}

#[test]
#[serial_test::serial]
fn successful_run_version() {
	let args = ["--version"].into_iter().map(OsString::from).collect();
	let git_config_parameters = Vec::new();
	let exit = run(args, git_config_parameters);
	assert!(exit.get_message().unwrap().starts_with("interactive-rebase-tool"));
	assert_eq!(exit.get_status(), &ExitStatus::Good);
}

#[test]
#[serial_test::serial]
fn successful_run_license() {
	let args = ["--license"].into_iter().map(OsString::from).collect();
	let git_config_parameters = Vec::new();
	let exit = run(args, git_config_parameters);
	assert!(
		exit.get_message()
			.unwrap()
			.contains("Sequence Editor for Git Interactive Rebase")
	);
	assert_eq!(exit.get_status(), &ExitStatus::Good);
}

#[test]
fn successful_run_editor() {
	with_git_directory("fixtures/simple", |path| {
		let todo_file = Path::new(path).join("rebase-todo-empty").into_os_string();
		let args = vec![todo_file];
		let git_config_parameters = Vec::new();
		assert_eq!(
			run(args, git_config_parameters).get_status(),
			&ExitStatus::Good
		);
	});
}

#[cfg(unix)]
#[test]
#[serial_test::serial]
#[expect(unsafe_code)]
fn error() {
	let args = unsafe { vec![OsString::from(String::from_utf8_unchecked(vec![0xC3, 0x28]))] };
	let git_config_parameters = Vec::new();
	assert_eq!(run(args, git_config_parameters).get_status(), &ExitStatus::StateError);
}
