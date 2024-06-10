use std::path::Path;

use super::*;
use crate::{module::ExitStatus, test_helpers::with_git_directory};

fn args(args: &[&str]) -> Vec<OsString> {
	args.iter().map(OsString::from).collect::<Vec<OsString>>()
}

#[test]
#[serial_test::serial]
fn successful_run_help() {
	let exit = run(args(&["--help"]));
	assert!(exit.get_message().as_ref().unwrap().contains("USAGE:"));
	assert_eq!(exit.get_status(), &ExitStatus::Good);
}

#[test]
#[serial_test::serial]
fn successful_run_version() {
	let exit = run(args(&["--version"]));
	assert!(
		exit.get_message()
			.as_ref()
			.unwrap()
			.starts_with("interactive-rebase-tool")
	);
	assert_eq!(exit.get_status(), &ExitStatus::Good);
}

#[test]
#[serial_test::serial]
fn successful_run_license() {
	let exit = run(args(&["--license"]));
	assert!(
		exit.get_message()
			.as_ref()
			.unwrap()
			.contains("Sequence Editor for Git Interactive Rebase")
	);
	assert_eq!(exit.get_status(), &ExitStatus::Good);
}

#[test]
fn successful_run_editor() {
	with_git_directory("fixtures/simple", |path| {
		let todo_file = Path::new(path).join("rebase-todo-empty");
		assert_eq!(
			run(args(&[todo_file.to_str().unwrap()])).get_status(),
			&ExitStatus::Good
		);
	});
}

#[cfg(unix)]
#[test]
#[serial_test::serial]
#[allow(unsafe_code)]
fn error() {
	let args = unsafe { vec![OsString::from(String::from_utf8_unchecked(vec![0xC3, 0x28]))] };
	assert_eq!(run(args).get_status(), &ExitStatus::StateError);
}
