#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate chrono;
extern crate clap;
extern crate git2;
extern crate pancurses;
extern crate unicode_segmentation;

mod action;
mod application;
mod cli;
mod color;
mod commit;
mod config;
mod constants;
mod git_interactive;
mod input;
mod line;
mod window;
mod view;

use application::Application;
use git_interactive::GitInteractive;
use std::process;
use window::Window;
use config::Config;
use view::View;
use constants::{
	EXIT_CODE_CONFIG_ERROR,
	EXIT_CODE_FILE_READ_ERROR,
	EXIT_CODE_FILE_WRITE_ERROR
};

struct Exit {
	message: String,
	code: i32,
}

fn main() {
	match try_main() {
		Ok(code) => process::exit(code),
		Err(err) => {
			eprintln!("{}", err.message);
			process::exit(err.code);
		}
	}
}

fn try_main() -> Result<i32, Exit> {
	let matches = cli::build_cli().get_matches();

	let filepath = matches.value_of("rebase-todo-filepath").unwrap();

	let config = match Config::new() {
		Ok(c) => c,
		Err(message) => return Err(Exit {message, code: EXIT_CODE_CONFIG_ERROR}),
	};

	let git_interactive = match GitInteractive::new_from_filepath(filepath, config.comment_char.as_str()) {
		Ok(gi) => gi,
		Err(message) => return Err(Exit {message, code: EXIT_CODE_FILE_READ_ERROR}),
	};

	if git_interactive.get_lines().is_empty() {
		return Err(Exit {message: String::from("Nothing to rebase"), code: EXIT_CODE_FILE_READ_ERROR});
	}

	let window = Window::new(&config);

	let mut application = Application::new(
		git_interactive,
		View::new(&window),
		&window,
		&config
    );

	let exit_code = match application.run() {
		Ok(c) => c,
		Err(message) => return Err(Exit {message, code: EXIT_CODE_FILE_WRITE_ERROR}),
	};

	Ok(exit_code.unwrap_or(0))
}
