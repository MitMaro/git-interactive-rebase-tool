#[cfg(not(test))]
use crate::display::CrossTerm;
#[cfg(test)]
use crate::test_helpers::mocks::CrossTerm;
use crate::{
	application::Application,
	arguments::Args,
	exit::Exit,
	input::read_event,
	module::{ExitStatus, Modules},
};

#[cfg(not(tarpaulin_include))]
pub(crate) fn run(args: &Args) -> Exit {
	let mut application: Application<Modules> = match Application::new(args, read_event, CrossTerm::new()) {
		Ok(app) => app,
		Err(exit) => return exit,
	};

	match application.run_until_finished() {
		Ok(..) => Exit::from(ExitStatus::Good),
		Err(exit) => exit,
	}
}

#[cfg(test)]
mod tests {
	use std::{ffi::OsString, path::Path};

	use super::*;
	use crate::test_helpers::set_git_directory;

	fn args(args: &[&str]) -> Args {
		Args::try_from(args.iter().map(OsString::from).collect::<Vec<OsString>>()).unwrap()
	}

	#[test]
	#[serial_test::serial]
	fn successful_run() {
		let path = set_git_directory("fixtures/simple");
		let todo_file = Path::new(path.as_str()).join("rebase-todo-empty");
		assert_eq!(
			run(&args(&[todo_file.to_str().unwrap()])).get_status(),
			&ExitStatus::Good
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_on_application_create() {
		let path = set_git_directory("fixtures/simple");
		let todo_file = Path::new(path.as_str()).join("does-not-exist");
		assert_eq!(
			run(&args(&[todo_file.to_str().unwrap()])).get_status(),
			&ExitStatus::FileReadError
		);
	}
}
