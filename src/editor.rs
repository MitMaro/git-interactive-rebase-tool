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
pub(crate) fn run(args: Args, git_config_parameters: Vec<(String, String)>) -> Exit {
	let mut application: Application<Modules> = match Application::new(args, git_config_parameters, read_event, CrossTerm::new()) {
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
	use std::path::Path;

	use super::*;
	use crate::test_helpers::with_git_directory;

	#[test]
	fn successful_run() {
		with_git_directory("fixtures/simple", |path| {
			let todo_file = Path::new(path).join("rebase-todo-empty").into_os_string();
			let args = Args::from_os_strings(vec![todo_file]).unwrap();
			let git_config_parameters = Vec::new();
			assert_eq!(
				run(args, git_config_parameters).get_status(),
				&ExitStatus::Good
			);
		});
	}

	#[test]
	fn error_on_application_create() {
		with_git_directory("fixtures/simple", |path| {
			let todo_file = Path::new(path).join("does-not-exist").into_os_string();
			let args = Args::from_os_strings(vec![todo_file]).unwrap();
			let git_config_parameters = Vec::new();
			assert_eq!(
				run(args, git_config_parameters).get_status(),
				&ExitStatus::FileReadError
			);
		});
	}
}
