extern crate pancurses;

use pancurses::*;

use std::env;

// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Prints each argument on a separate line
fn main() {
	let args: Vec<String> = env::args().collect();
	
	// Create a path to the desired file
	let path = Path::new(&args[1]);
	let display = path.display();
	
	// Open the path in read-only mode, returns `io::Result<File>`
	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why.description()),
		Ok(file) => file,
	};
	
	// Read the file contents into a string, returns `io::Result<usize>`
	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(why) => panic!("couldn't read {}: {}", display, why.description()),
		Ok(_) => {},
	}
	
	let v: Vec<&str> = s.split('\n').filter(|l| !l.starts_with("#") && !l.is_empty()).collect();

	let v_len: i16 = v.len() as i16;
	
	/* Setup ncurses. */
	let window = initscr();
	window.refresh();
	window.keypad(true);
	noecho();
	
	let mut selected_line: i16 = 0;
	
	loop {
		window.clear();
		window.addstr("Instructions: Up/Down - Change Selected | q - quit\n\n");
		let mut i: i16 = 0;
		for ss in &v {
			if i == selected_line {
				window.attron(A_STANDOUT);
			}
			window.addstr(&format!("{}\n", &ss).as_ref());
			window.attroff(A_STANDOUT);
			i += 1;
		}
		window.refresh();
		match window.getch() {
			Some(Input::Character(q)) if q == 'q' || q == 'Q' => {
				curs_set(1);
				endwin();
				return;
			},
			Some(Input::KeyUp) => {
				selected_line -= 1;
				if selected_line < 0 {
					selected_line = 0;
				}
			},
			Some(Input::KeyDown) => {
				selected_line += 1;
				if selected_line > v_len - 1 {
					selected_line = v_len - 1;
				}
			},
			_ => {}
		}
	}
}

