extern crate pancurses;

use pancurses::*;

use std::env;

// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const COLOR_TABLE: [i16; 7] = [
	COLOR_WHITE,
	COLOR_YELLOW,
	COLOR_BLUE,
	COLOR_GREEN,
	COLOR_CYAN,
	COLOR_MAGENTA,
	COLOR_RED
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
	
	/* Setup ncurses. */
	let window = initscr();
	window.refresh();
	window.keypad(true);
	noecho();
	
	if has_colors() {
		start_color();
	}
	
	for (i, color) in COLOR_TABLE.into_iter().enumerate() {
		init_pair(i as i16, *color, COLOR_BLACK);
	}
	
	let mut selected_line: i16 = 0;
	
	loop {
		window.clear();
		window.addstr("git interactive rebase tool\n\n");
		let mut i: i16 = 0;
		for ss in &v {
			if ss[0] == "pick" {
				window.attrset(COLOR_PAIR(0));
			} else if ss[0] == "reword" {
				window.attrset(COLOR_PAIR(1));
			} else if ss[0] == "edit" {
				window.attrset(COLOR_PAIR(2));
			} else if ss[0] == "squash" {
				window.attrset(COLOR_PAIR(4));
			} else if ss[0] == "fixup" {
				window.attrset(COLOR_PAIR(5));
			} else if ss[0] == "exec" {
				window.attrset(COLOR_PAIR(6));
			} else if ss[0] == "drop" {
				window.attrset(COLOR_PAIR(7) | A_BOLD);
			}
			if i == selected_line {
				window.attron(A_STANDOUT);
			}
			window.addstr(&format!("{:6}", &ss[0]).as_ref());
			window.addstr(&format!(" {} {}\n", &ss[1], &ss[2]).as_ref());
			window.attroff(A_STANDOUT);
			i += 1;
		}
		window.addstr("\n\n");
		window.addstr("Up - Move selection up, Down - Move selection down\n");
		window.addstr("j - Move selected line up, k - Move selected line down\n");
		window.addstr("(p)ick, (r)eword, (e)dit, (s)quash, (f)ixup, e(x)ec, (d)rop\n");
		window.refresh();
		match window.getch() {
			Some(Input::Character(q)) if q == 'q' || q == 'Q' => {
				curs_set(1);
				endwin();
				break;
			},
			Some(Input::Character(c)) if c == 'p' || c == 'P' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "pick");
			},
			Some(Input::Character(c)) if c == 'r' || c == 'R' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "reword");
			},
			Some(Input::Character(c)) if c == 'e' || c == 'E' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "edit");
			},
			Some(Input::Character(c)) if c == 's' || c == 'S' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "squash");
			},
			Some(Input::Character(c)) if c == 'f' || c == 'F' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "fixup");
			},
			Some(Input::Character(c)) if c == 'x' || c == 'X' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "exec");
			},
			Some(Input::Character(c)) if c == 'd' || c == 'D' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "drop");
			},
			Some(Input::Character(c)) if c == 'd' || c == 'D' => {
				v[selected_line as usize].remove(0);
				v[selected_line as usize].insert(0, "drop");
			},
			Some(Input::Character(c)) if c == 'j' || c == 'J' => {
				if selected_line != 0 {
					v.swap(selected_line as usize, selected_line as usize - 1);
					selected_line -= 1;
				}
			},
			Some(Input::Character(c)) if c == 'k' || c == 'K' => {
				if selected_line != v_len - 1 {
					v.swap(selected_line as usize, selected_line as usize + 1);
					selected_line += 1;
				}
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
			}
			_ => {}
		}
	}
	
	// Create a path to the desired file
	let outfilepath = Path::new("git-rebase-output.txt");
	let outfiledisplay = path.display();
	
	// Open a file in write-only mode, returns `io::Result<File>`
	let mut outfile = match File::create(outfilepath) {
		Err(why) => panic!("couldn't create {}: {}", outfiledisplay, why.description()),
		Ok(outfile) => outfile,
	};
	
	for ss in &v {
		if let Err(e) = writeln!(outfile, "{} {} {}", &ss[0], &ss[1], &ss[2]) {
			println!("{}", e);
		}
	}
}

