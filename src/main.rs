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

mod config;
mod confirm_abort;
mod confirm_rebase;
mod constants;
mod display;
mod edit;
mod external_editor;
mod input;
mod list;
mod process;
mod show_commit;
mod todo_file;
mod view;

use crate::config::Config;
use crate::constants::{NAME, VERSION};
use crate::display::curses::Curses;
use crate::display::Display;
use crate::input::input_handler::InputHandler;
use crate::process::exit_status::ExitStatus;
use crate::process::modules::Modules;
use crate::process::Process;
use crate::todo_file::TodoFile;
use crate::view::View;
use clap::{App, Arg, ArgMatches};

struct Exit {
	message: String,
	status: ExitStatus,
}

// TODO use the termination trait once rust-lang/rust#43301 is stable
#[allow(clippy::exit)]
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

	match try_main(&matches) {
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

fn try_main(matches: &ArgMatches<'_>) -> Result<ExitStatus, Exit> {
	if matches.is_present("license") {
		print_license();
		return Ok(ExitStatus::Good);
	}

	let filepath = matches.value_of("rebase-todo-filepath").unwrap();

	let config = Config::new().map_err(|err| {
		Exit {
			message: format!("{:#}", err),
			status: ExitStatus::ConfigError,
		}
	})?;

	let mut todo_file = TodoFile::new(filepath, config.git.comment_char.as_str());
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

	if todo_file.get_lines().is_empty() {
		return Err(Exit {
			message: String::from("An empty rebase was provided, nothing to edit"),
			status: ExitStatus::Good,
		});
	}

	let mut curses = Curses::new();
	let display = Display::new(&mut curses, &config.theme);
	let input_handler = InputHandler::new(&display, &config.key_bindings);
	let view = View::new(&display, &config);
	let modules = Modules::new(&display, &config);
	let mut process = Process::new(todo_file, &view, &input_handler);
	let result = process.run(modules);
	display.end();

	result
		.map_err(|err| {
			Exit {
				message: err.to_string(),
				status: ExitStatus::FileWriteError,
			}
		})
		.map(|exit_code| exit_code.unwrap_or(ExitStatus::Good))
}
