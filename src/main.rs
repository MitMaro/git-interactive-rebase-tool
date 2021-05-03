// Make rustc's built-in lints more strict and set clippy into a whitelist-based configuration
#![deny(
	warnings,
	nonstandard_style,
	unused,
	future_incompatible,
	rust_2018_idioms,
	unsafe_code
)]
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::as_conversions)]
#![allow(clippy::blocks_in_if_conditions)] // sometimes rustfmt makes blocks out of simple statements
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::else_if_without_else)]
#![allow(clippy::expect_used)]
#![allow(clippy::float_arithmetic)]
#![allow(clippy::implicit_return)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::integer_division)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::panic)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::wildcard_enum_match_arm)]
#![allow(clippy::similar_names)]
#![allow(clippy::unreachable)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::redundant_closure_for_method_calls)] // too many false positives
#![allow(clippy::default_numeric_fallback)]

mod components;
mod config;
mod confirm_abort;
mod confirm_rebase;
mod display;
mod external_editor;
mod input;
mod insert;
mod list;
mod process;
mod show_commit;
mod todo_file;
mod view;

use clap::{App, Arg};

use crate::{
	config::Config,
	display::{CrossTerm, Display},
	input::{EventHandler, KeyBindings},
	process::{exit_status::ExitStatus, modules::Modules, Process},
	todo_file::TodoFile,
	view::View,
};

const NAME: &str = "interactive-rebase-tool";
#[cfg(not(feature = "dev"))]
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "dev")]
const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-dev");

#[derive(Debug)]
pub struct Exit {
	message: String,
	status: ExitStatus,
}

// TODO use the termination trait once rust-lang/rust#43301 is stable
#[allow(clippy::exit, clippy::print_stderr)]
fn main() {
	let app = App::new(NAME)
		.version(VERSION)
		.about("Full feature terminal based sequence editor for git interactive rebase.")
		.author("Tim Oram <dev@mitmaro.ca>")
		.arg(
			Arg::with_name("license")
				.long("license")
				.help("Print license information and exit")
				.conflicts_with("<rebase-todo-filepath>"),
		)
		.arg(
			Arg::with_name("rebase-todo-filepath")
				.index(1)
				.help("The path to the git rebase todo file")
				.required_unless_one(&["license"]),
		);

	let matches = app.get_matches();

	if matches.is_present("license") {
		print_license();
		std::process::exit(ExitStatus::Good.to_code());
	}

	let filepath = matches.value_of("rebase-todo-filepath").unwrap();

	match try_main(filepath) {
		Ok(code) => std::process::exit(code.to_code()),
		Err(err) => {
			eprintln!("{}", err.message);
			std::process::exit(err.status.to_code());
		},
	}
}

#[allow(clippy::print_stdout)]
fn print_license() {
	println!(
		r#"
Sequence Editor for Git Interactive Rebase

Copyright (C) 2017-2020 Tim Oram and Contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

A list of open source software and the license terms can be found at
<https://gitrebasetool.mitmaro.ca/licenses.html>
		"#
	);
}

fn try_main(filepath: &str) -> Result<ExitStatus, Exit> {
	let config = Config::new().map_err(|err| {
		Exit {
			message: format!("{:#}", err),
			status: ExitStatus::ConfigError,
		}
	})?;

	let mut todo_file = TodoFile::new(filepath, config.undo_limit, config.git.comment_char.as_str());
	todo_file.load_file().map_err(|err| {
		Exit {
			message: err.to_string(),
			status: ExitStatus::FileReadError,
		}
	})?;

	if todo_file.is_noop() {
		return Err(Exit {
			message: String::from("A noop rebase was provided, skipping editing"),
			status: ExitStatus::Good,
		});
	}

	if todo_file.is_empty() {
		return Err(Exit {
			message: String::from("An empty rebase was provided, nothing to edit"),
			status: ExitStatus::Good,
		});
	}

	let mut crossterm = CrossTerm::new();
	let display = Display::new(&mut crossterm, &config.theme);
	let modules = Modules::new(&config);
	let mut process = Process::new(
		todo_file,
		EventHandler::new(KeyBindings::new(&config.key_bindings)),
		View::new(display, &config),
	);
	let result = process.run(modules);

	result
		.map_err(|err| {
			Exit {
				message: err.to_string(),
				status: ExitStatus::FileWriteError,
			}
		})
		.map(|exit_code| exit_code.unwrap_or(ExitStatus::Good))
}

#[cfg(all(unix, test))]
mod tests {
	use std::{env::set_var, fs::File, path::Path};

	use super::*;
	use crate::{
		assert_exit_status,
		input::{testutil::setup_mocked_events, Event, MetaEvent},
	};

	fn set_git_directory(repo: &str) -> String {
		let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test").join(repo);
		set_var("GIT_DIR", path.to_str().unwrap());
		String::from(path.to_str().unwrap())
	}

	#[test]
	#[serial_test::serial]
	fn error_loading_config() {
		let path = set_git_directory("fixtures/invalid-config");
		assert_exit_status!(
			try_main("does-not-exist"),
			message = format!("Error loading git config: could not find repository from '{}'", path),
			status = ExitStatus::ConfigError
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_loading_file() {
		set_git_directory("fixtures/simple");
		assert_exit_status!(
			try_main("does-not-exist"),
			message = "No such file or directory (os error 2)",
			status = ExitStatus::FileReadError
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_noop() {
		let path = set_git_directory("fixtures/simple");
		let todo_file = Path::new(path.as_str()).join("rebase-todo-noop");
		assert_exit_status!(
			try_main(todo_file.to_str().unwrap()),
			message = "A noop rebase was provided, skipping editing",
			status = ExitStatus::Good
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_empty_file() {
		let path = set_git_directory("fixtures/simple");
		let todo_file = Path::new(path.as_str()).join("rebase-todo-empty");
		assert_exit_status!(
			try_main(todo_file.to_str().unwrap()),
			message = "An empty rebase was provided, nothing to edit",
			status = ExitStatus::Good
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_process() {
		let path = set_git_directory("fixtures/simple");
		let todo_file_path = Path::new(path.as_str()).join("rebase-todo-readonly");
		let todo_file = File::open(todo_file_path.as_path()).unwrap();
		let mut permissions = todo_file.metadata().unwrap().permissions();
		setup_mocked_events(&[Event::from(MetaEvent::Exit)]);
		permissions.set_readonly(true);
		todo_file.set_permissions(permissions).unwrap();
		assert_exit_status!(
			try_main(todo_file_path.to_str().unwrap()),
			message = format!("Error opening file: {}", todo_file_path.to_str().unwrap()),
			status = ExitStatus::FileWriteError
		);
	}
	#[test]
	#[serial_test::serial]
	fn success() {
		let path = set_git_directory("fixtures/simple");
		let todo_file = Path::new(path.as_str()).join("rebase-todo");
		setup_mocked_events(&[Event::from(MetaEvent::Exit)]);
		assert_exit_status!(try_main(todo_file.to_str().unwrap()), status = ExitStatus::Abort);
	}
}
