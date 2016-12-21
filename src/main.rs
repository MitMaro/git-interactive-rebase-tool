extern crate pancurses;

use std::env;

// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const COLOR_TABLE: [i16; 8] = [
	pancurses::COLOR_WHITE,
	pancurses::COLOR_YELLOW,
	pancurses::COLOR_BLUE,
	pancurses::COLOR_GREEN,
	pancurses::COLOR_CYAN,
	pancurses::COLOR_MAGENTA,
	pancurses::COLOR_RED,
	pancurses::COLOR_BLACK
];

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
	
	let mut v: Vec<Vec<&str>> = s
		.lines()
		.filter(|l| !l.starts_with("#") && !l.is_empty())
		.map(|l| l.splitn(3, " ").collect())
		.collect();
	
	let v_len: i16 = v.len() as i16;
	
	/* Setup pancurses. */
	let window = pancurses::initscr();
	
	pancurses::curs_set(0);
	pancurses::noecho();
	window.keypad(true);
	
	if pancurses::has_colors() {
		pancurses::start_color();
	}
	
	pancurses::use_default_colors();
	for (i, color) in COLOR_TABLE.into_iter().enumerate() {
		pancurses::init_pair(i as i16, *color, -1);
	}
	
	let mut selected_line: i16 = 0;
	
	loop {
		window.clear();
		window.attrset(pancurses::COLOR_PAIR(0));
		window.addstr("Git Interactive Rebase       Type ? for help\n\n");
		window.attrset(pancurses::COLOR_PAIR(0));
		window.refresh();
		let mut i: i16 = 0;
		for ss in &v {
			if ss[0] == "pick" {
				window.attrset(pancurses::COLOR_PAIR(0));
			} else if ss[0] == "reword" {
				window.attrset(pancurses::COLOR_PAIR(1));
			} else if ss[0] == "edit" {
				window.attrset(pancurses::COLOR_PAIR(2));
			} else if ss[0] == "squash" {
				window.attrset(pancurses::COLOR_PAIR(4));
			} else if ss[0] == "fixup" {
				window.attrset(pancurses::COLOR_PAIR(5));
			} else if ss[0] == "exec" {
				window.attrset(pancurses::COLOR_PAIR(6));
			} else if ss[0] == "drop" {
				window.attrset(pancurses::COLOR_PAIR(7));
			}
			if i == selected_line {
				window.attrset(pancurses::COLOR_PAIR(0));
				window.attron(pancurses::A_STANDOUT);
			}
			window.addstr(&format!(" {:6} {} {}\n", &ss[0], &ss[1], &ss[2]).as_ref());
			window.attroff(pancurses::A_STANDOUT);
			i += 1;
		}
		window.attrset(pancurses::COLOR_PAIR(0));
		window.refresh();
		match window.getch() {
			Some(pancurses::Input::Character(q)) if q == 'q' || q == 'Q' => {
				window.clear();
				window.attrset(pancurses::COLOR_PAIR(0));
				window.addstr("Git Interactive Rebase - Abort Rebase\n\n");
				window.addstr("Are you sure you want to abort? (y/n)");
				window.refresh();
				match window.getch() {
					Some(pancurses::Input::Character(c)) if c == 'y' || c == 'Y' => {
						pancurses::curs_set(1);
						pancurses::endwin();
						// empty file
						v.clear();
						break;
					},
					_ => {}
				}
			},
			Some(pancurses::Input::Character(q)) if q == 'w' || q == 'W' => {
				window.clear();
				window.attrset(pancurses::COLOR_PAIR(0));
				window.addstr("Git Interactive Rebase - Confirm Rebase\n\n");
				window.addstr("Are you sure you want to rebase? (y/n)");
				window.refresh();
				match window.getch() {
					Some(pancurses::Input::Character(c)) if c == 'y' || c == 'Y' => {
						pancurses::curs_set(1);
						pancurses::endwin();
						break;
					},
					_ => {}
				}
			},
			Some(pancurses::Input::Character(c)) if c == '?' || c == 'h' || c == 'H' => {
				window.clear();
				window.attrset(pancurses::COLOR_PAIR(0));
				window.addstr("Git Interactive Rebase - Help\n\n");
				window.addstr(" Up and Down arrow keys to move selection\n");
				window.addstr(" q, abort interactive rebase\n");
				window.addstr(" w, write and continue interactive rebase\n");
				window.addstr(" ?, show this help message\n");
				window.addstr(" j, move selected commit up\n");
				window.addstr(" k, move selected commit down\n");
				window.addstr(" p, pick: use commit\n");
				window.addstr(" r, reword: use commit, but edit the commit message\n");
				window.addstr(" e, edit: use commit, but stop for amending\n");
				window.addstr(" s, squash: use commit, but meld into previous commit\n");
				window.addstr(" f, fixup: like 'squash', but discard this commit's log message\n");
				window.addstr(" x, exec: run command (the rest of the line) using shell\n");
				window.addstr(" d, drop: remove commit\n");
				window.addstr("\n\nHit any key to close help");
				window.refresh();
				window.getch();
			},
			Some(pancurses::Input::Character(c)) if c == 'p' || c == 'P' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "pick");
			},
			Some(pancurses::Input::Character(c)) if c == 'r' || c == 'R' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "reword");
			},
			Some(pancurses::Input::Character(c)) if c == 'e' || c == 'E' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "edit");
			},
			Some(pancurses::Input::Character(c)) if c == 's' || c == 'S' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "squash");
			},
			Some(pancurses::Input::Character(c)) if c == 'f' || c == 'F' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "fixup");
			},
			Some(pancurses::Input::Character(c)) if c == 'x' || c == 'X' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "exec");
			},
			Some(pancurses::Input::Character(c)) if c == 'd' || c == 'D' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "drop");
			},
			Some(pancurses::Input::Character(c)) if c == 'd' || c == 'D' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "drop");
			},
			Some(pancurses::Input::Character(c)) if c == 'j' || c == 'J' => {
				if selected_line != 0 {
					v.swap(selected_line as usize, selected_line as usize - 1);
					selected_line -= 1;
				}
			},
			Some(pancurses::Input::Character(c)) if c == 'k' || c == 'K' => {
				if selected_line != v_len - 1 {
					v.swap(selected_line as usize, selected_line as usize + 1);
					selected_line += 1;
				}
			},
			Some(pancurses::Input::KeyUp) => {
				selected_line -= 1;
				if selected_line < 0 {
					selected_line = 0;
				}
			},
			Some(pancurses::Input::KeyDown) => {
				selected_line += 1;
				if selected_line > v_len - 1 {
					selected_line = v_len - 1;
				}
			},
			_ => {}
		}
	}
	
	let mut outfile = match File::create(path) {
		Err(why) => panic!("couldn't create {}: {}", display, why.description()),
		Ok(outfile) => outfile,
	};

	for ss in &v {
		if let Err(e) = writeln!(outfile, "{} {} {}", &ss[0], &ss[1], &ss[2]) {
			println!("{}", e);
		}
	}
}

