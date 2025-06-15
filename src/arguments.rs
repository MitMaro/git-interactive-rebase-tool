use std::ffi::OsString;

use pico_args::Arguments;

use crate::{exit::Exit, module::ExitStatus};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Mode {
	Editor,
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

	pub(crate) fn todo_file_path(&self) -> Option<&str> {
		self.todo_file_path.as_deref()
	}

	#[cfg(test)]
	pub(crate) fn from_strs(args: impl IntoIterator<Item: AsRef<str>>) -> Result<Self, Exit> {
		let args = args.into_iter().map(|it| OsString::from(it.as_ref())).collect();
		Self::from_os_strings(args)
	}

	pub(crate) fn from_os_strings(args: Vec<OsString>) -> Result<Self, Exit> {
		let mut pargs = Arguments::from_vec(args);

		let mode = if pargs.contains(["-h", "--help"]) {
			Mode::Help
		}
		else if pargs.contains(["-v", "--version"]) {
			Mode::Version
		}
		else if pargs.contains("--license") {
			Mode::License
		}
		else {
			Mode::Editor
		};

		let todo_file_path = pargs
			.opt_free_from_str()
			.map_err(|err| Exit::new(ExitStatus::StateError, err.to_string().as_str()))?;

		Ok(Self { mode, todo_file_path })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mode_help() {
		assert_eq!(Args::from_strs(["-h"]).unwrap().mode(), &Mode::Help);
		assert_eq!(Args::from_strs(["--help"]).unwrap().mode(), &Mode::Help);
	}

	#[test]
	fn mode_version() {
		assert_eq!(Args::from_strs(["-v"]).unwrap().mode(), &Mode::Version);
		assert_eq!(
			Args::from_strs(["--version"]).unwrap().mode(),
			&Mode::Version
		);
	}

	#[test]
	fn mode_license() {
		assert_eq!(
			Args::from_strs(["--license"]).unwrap().mode(),
			&Mode::License
		);
	}

	#[test]
	fn todo_file_ok() {
		let args = Args::from_strs(["todofile"]).unwrap();
		assert_eq!(args.mode(), &Mode::Editor);
		assert_eq!(args.todo_file_path(), Some("todofile"));
	}

	#[test]
	fn todo_file_missing() {
		let args = Args::from_os_strings(Vec::new()).unwrap();
		assert_eq!(args.mode(), &Mode::Editor);
		assert!(args.todo_file_path().is_none());
	}

	#[cfg(unix)]
	#[test]
	#[expect(unsafe_code)]
	fn todo_file_invalid() {
		let args = unsafe { vec![OsString::from(String::from_utf8_unchecked(vec![0xC3, 0x28]))] };
		_ = Args::from_os_strings(args).unwrap_err();
	}
}
