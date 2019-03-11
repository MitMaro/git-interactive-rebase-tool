#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;
extern crate git2;
extern crate pad;
extern crate pancurses;

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
#[cfg(test)]
mod mocks;

use application::Application;
use git_interactive::GitInteractive;
use std::process;
use window::Window;
use config::Config;
use constants::NAME;

fn error_handler(msg: &str, code: i32) -> ! {
	eprintln!("{}: {}", NAME, msg);
	process::exit(code)
}

fn main() {
	let matches = cli::build_cli().get_matches();

	let filepath = matches.value_of("rebase-todo-filepath").unwrap();

	let config = match Config::new() {
		Ok(c) => c,
		Err(msg) => error_handler(&format!("Error reading git config: {}", msg), 1),
	};

	let git_interactive = match GitInteractive::new_from_filepath(filepath, &config.comment_char) {
		Ok(gi) => gi,
		Err(msg) => error_handler(&msg, 1),
	};

	if git_interactive.get_lines().is_empty() {
		error_handler("Nothing to rebase", 1);
	}

	let window = Window::new(config.clone()); // TODO: shouldn't need two copies of `config`

	let mut application = Application::new(git_interactive, window, config);

	while application.exit_code == None {
		application.draw();
		application.process_input()
	}

	match application.end() {
		Ok(_) => {},
		Err(msg) => error_handler(&msg, 1),
	};

	process::exit(match application.exit_code {
		None => 0,
		Some(code) => code
	});
}
