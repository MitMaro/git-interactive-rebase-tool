#![deny(warnings)]
#![deny(anonymous_parameters)]
#![deny(bare_trait_objects)]
#![allow(box_pointers)]
#![allow(elided_lifetimes_in_paths)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![allow(missing_docs)]
#![allow(single_use_lifetimes)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![allow(unused_results)]
#![deny(variant_size_differences)]

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
mod show_commit;
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

	let display = Display::new(&mut curses, &config);

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
