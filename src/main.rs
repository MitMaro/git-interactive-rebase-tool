#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod cli;
mod commit;
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
mod help;
mod input;
mod list;
mod process;
mod scroll;
mod show_commit;
mod view;
mod window_size_error;

use crate::config::Config;
use crate::display::{Curses, Display};
use crate::git_interactive::GitInteractive;
use crate::input::InputHandler;
use crate::process::{ExitStatus, Process};
use crate::view::View;

struct Exit {
	message: String,
	status: ExitStatus,
}

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

	let mut curses = Curses::new();

	let git_interactive = match GitInteractive::new_from_filepath(filepath, config.comment_char.as_str()) {
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
			status: ExitStatus::FileReadError,
		});
	}

	let display = Display::new(&mut curses, &config);

	let input_handler = InputHandler::new(&display, &config);
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
