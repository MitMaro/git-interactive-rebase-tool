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
mod git_interactive;
mod input;
mod list;
mod process;
mod show_commit;
#[cfg(test)]
#[macro_use]
mod testutil;
mod view;

use crate::config::Config;
use crate::constants::{NAME, VERSION};
use crate::display::curses::Curses;
use crate::display::Display;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::process::exit_status::ExitStatus;
use crate::process::modules::Modules;
use crate::process::Process;
use crate::view::View;
use clap::{App, ArgMatches};

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
		.args_from_usage("<rebase-todo-filepath> 'The path to the git rebase todo file'");

	let matches = app.get_matches();

	match try_main(&matches) {
		Ok(code) => std::process::exit(code.to_code()),
		Err(err) => {
			eprintln!("{}", err.message);
			std::process::exit(err.status.to_code());
		},
	}
}
fn try_main(matches: &ArgMatches<'_>) -> Result<ExitStatus, Exit> {
	let filepath = matches.value_of("rebase-todo-filepath").unwrap();

	let config = Config::new().map_err(|err| {
		Exit {
			message: err.to_string(),
			status: ExitStatus::ConfigError,
		}
	})?;

	let git_interactive =
		GitInteractive::new_from_filepath(filepath, config.git.comment_char.as_str()).map_err(|err| {
			Exit {
				message: err.to_string(),
				status: ExitStatus::FileReadError,
			}
		})?;

	if git_interactive.is_noop() {
		return Err(Exit {
			message: String::from("A noop rebase was provided, skipping editing"),
			status: ExitStatus::Good,
		});
	}

	if git_interactive.get_lines().is_empty() {
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
	let mut process = Process::new(git_interactive, &view, &input_handler);
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
