// TODO:
// - Add execute command
extern crate pancurses;
extern crate pad;

use std::env;
use std::process;

mod commit;
mod action;
mod application;
mod git_interactive;
mod line;
#[macro_use]
mod utils;
mod window;
#[cfg(test)]
mod mocks;

use application::Application;
use git_interactive::GitInteractive;
use window::Window;

fn main() {
	
	let filepath = match env::args().nth(1) {
		Some(filepath) => filepath,
		None => {
			print_err!(
				"Must provide a filepath.\n\n\
				Usage: git-interactive <filepath>"
			);
			process::exit(1);
		}
	};
	
	let git_interactive = match GitInteractive::new_from_filepath(&filepath) {
		Ok(gi) => gi,
		Err(msg) => {
			print_err!("{}", msg);
			process::exit(1);
		}
	};

	if git_interactive.get_lines().is_empty() {
		print_err!("{}", &"Nothing to rebase");
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
			print_err!("{}", msg);
			process::exit(1);
		}
	};
	process::exit(match application.exit_code {
		None => 0,
		Some(code) => code
	});
}
