use std::ffi::OsString;

use pico_args::Arguments;

use crate::{exit::Exit, module::ExitStatus};

#[derive(Debug)]
pub(crate) enum Mode {
	Normal,
	Help,
	Version,
	License,
}

#[derive(Debug)]
pub(crate) struct Args {
	mode: Mode,
	todo_file_path: Option<String>,
}

impl Args {
	pub(crate) const fn mode(&self) -> &Mode {
		&self.mode
	}

	pub(crate) const fn todo_file_path(&self) -> &Option<String> {
		&self.todo_file_path
	}
}

impl TryFrom<Vec<OsString>> for Args {
	type Error = Exit;

	fn try_from(args: Vec<OsString>) -> Result<Self, Self::Error> {
		let mut pargs = Arguments::from_vec(args);

		let mode = if pargs.contains(["-h", "--help"]) {
			Mode::Help
		}
		else if pargs.contains(["-v", "--version"]) {
			Mode::Version
		}
		else if pargs.contains(["-v", "--license"]) {
			Mode::License
		}
		else {
			Mode::Normal
		};

		let todo_file_path = pargs
			.opt_free_from_str()
			.map_err(|err| Exit::new(ExitStatus::StateError, err.to_string().as_str()))?;

		Ok(Self { mode, todo_file_path })
	}
}
