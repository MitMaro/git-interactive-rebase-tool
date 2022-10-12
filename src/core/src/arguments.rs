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

	fn create_args(args: &[&str]) -> Vec<OsString> {
		args.iter().map(OsString::from).collect()
	}

	#[test]
	fn mode_help() {
		assert_eq!(Args::try_from(create_args(&["-h"])).unwrap().mode(), &Mode::Help);
		assert_eq!(Args::try_from(create_args(&["--help"])).unwrap().mode(), &Mode::Help);
	}

	#[test]
	fn mode_version() {
		assert_eq!(Args::try_from(create_args(&["-v"])).unwrap().mode(), &Mode::Version);
		assert_eq!(
			Args::try_from(create_args(&["--version"])).unwrap().mode(),
			&Mode::Version
		);
	}

	#[test]
	fn mode_license() {
		assert_eq!(
			Args::try_from(create_args(&["--license"])).unwrap().mode(),
			&Mode::License
		);
	}

	#[test]
	fn todo_file_ok() {
		let args = Args::try_from(create_args(&["todofile"])).unwrap();
		assert_eq!(args.mode(), &Mode::Editor);
		assert_eq!(args.todo_file_path(), &Some(String::from("todofile")));
	}

	#[test]
	fn todo_file_missing() {
		let args = Args::try_from(create_args(&[])).unwrap();
		assert_eq!(args.mode(), &Mode::Editor);
		assert!(args.todo_file_path().is_none());
	}

	#[cfg(unix)]
	#[test]
	#[allow(unsafe_code)]
	fn todo_file_invalid() {
		let args = unsafe { vec![OsString::from(String::from_utf8_unchecked(vec![0xC3, 0x28]))] };
		let _ = Args::try_from(args).unwrap_err();
	}
}
