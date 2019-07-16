#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod action;
mod cli;
mod color;
mod commit;
mod config;
mod confirm_abort;
mod confirm_rebase;
mod constants;
mod edit;
mod error;
mod exiting;
mod external_editor;
mod git_interactive;
mod help;
mod input;
mod line;
mod list;
mod process;
mod scroll;
mod show_commit;
mod view;
mod window;
mod window_size_error;

use crate::config::Config;
use crate::git_interactive::GitInteractive;
use crate::input::InputHandler;
use crate::process::{ExitStatus, Process};
use crate::view::View;
use crate::window::Window;

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

	let git_interactive = match GitInteractive::new_from_filepath(filepath, config.comment_char.as_str()) {
		Ok(gi) => gi,
		Err(message) => {
			return Err(Exit {
				message,
				status: ExitStatus::FileReadError,
			});
		},
	};

	if git_interactive.get_lines().is_empty() {
		return Err(Exit {
			message: String::from("Nothing to rebase"),
			status: ExitStatus::FileReadError,
		});
	}

	let window = Window::new(&config);

	let input_handler = InputHandler::new(&window, &config);

	let view = View::new(&window, &config);

	let mut process = Process::new(git_interactive, &view, &input_handler, &config);

	let result = process.run();
	window.end();

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
