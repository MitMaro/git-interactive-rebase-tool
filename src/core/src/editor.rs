#[cfg(test)]
use display::testutil::CrossTerm;
#[cfg(not(test))]
use display::CrossTerm;
use input::read_event;

use crate::{
	application::Application,
	arguments::Args,
	exit::Exit,
	module::{ExitStatus, Modules},
};

#[cfg(not(tarpaulin_include))]
pub(crate) fn run(args: &Args) -> Exit {
	let application: Application<Modules, _, _> = match Application::new(args, read_event, CrossTerm::new()) {
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
	use crate::testutil::set_git_directory;

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
