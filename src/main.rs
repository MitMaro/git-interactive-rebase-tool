#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;
extern crate pad;
extern crate pancurses;

mod action;
mod application;
mod cli;
mod commit;
mod git_interactive;
mod line;
mod window;
#[cfg(test)]
mod mocks;

use application::Application;
use git_interactive::GitInteractive;
use std::process;
use window::Window;

fn main() {
	let matches = cli::build_cli().get_matches();

	let filepath = matches.value_of("rebase-todo-filepath").unwrap();
	
	let git_interactive = match GitInteractive::new_from_filepath(filepath) {
		Ok(gi) => gi,
		Err(msg) => {
			eprintln!("{}", msg);
			process::exit(1)
		}
	};

	if git_interactive.get_lines().is_empty() {
		eprintln!("{}", &"Nothing to rebase");
		process::exit(0);
	}

	let window = Window::new();

	let mut application = Application::new(git_interactive, window);

	while application.exit_code == None {
		application.draw();
		application.process_input()
	}

	match application.end() {
		Ok(_) => {},
		Err(msg) => {
			eprintln!("{}", msg);
			process::exit(1);
		}
	};
	process::exit(match application.exit_code {
		None => 0,
		Some(code) => code
	});
}
