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

mod cli;
mod config;
mod confirm_abort;
mod confirm_rebase;
mod constants;
mod display;
mod edit;
mod error;
mod exiting;
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
mod window_size_error;

use crate::config::Config;
use crate::display::curses::Curses;
use crate::display::Display;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::process::exit_status::ExitStatus;
use crate::process::Process;
use crate::view::View;

struct Exit {
	message: String,
	status: ExitStatus,
}

// TODO use the termination trait once rust-lang/rust#43301 is stable
#[allow(clippy::exit)]
fn main() {
	match try_main() {
		Ok(code) => std::process::exit(code.to_code()),
		Err(err) => {
			eprintln!("{}", err.message);
			std::process::exit(err.status.to_code());
		},
	}
}

fn try_main() -> Result<ExitStatus, Exit> {
	let matches = cli::build_cli().get_matches();

	let filepath = matches.value_of("rebase-todo-filepath").unwrap();

	let config = match Config::new() {
		Ok(c) => c,
		Err(message) => {
			return Err(Exit {
				message,
				status: ExitStatus::ConfigError,
			});
		},
	};

	let git_interactive = match GitInteractive::new_from_filepath(filepath, config.git.comment_char.as_str()) {
		Ok(gi) => gi,
		Err(message) => {
			return Err(Exit {
				message,
				status: ExitStatus::FileReadError,
			});
		},
	};

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

	let mut process = Process::new(git_interactive, &view, &display, &input_handler, &config);

	let result = process.run();
	display.end();

	let exit_code = match result {
		Ok(c) => c,
		Err(message) => {
			return Err(Exit {
				message,
				status: ExitStatus::FileWriteError,
			});
		},
	};

	Ok(exit_code.unwrap_or(ExitStatus::Good))
}
