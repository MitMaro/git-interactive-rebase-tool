// TODO:
// - Add execute command
extern crate pancurses;

use std::cmp;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use pancurses::Input;

macro_rules! print_err {
	($($arg:tt)*) => (
		{
			use std::io::prelude::*;
			if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
				panic!(
					"Failed to write to stderr.\n\
					Original error output: {}\n\
					Secondary error writing to stderr: {}", format!($($arg)*), e
				);
			}
		}
	)
}

enum Action {
	Pick,
	Reword,
	Edit,
	Squash,
	Fixup,
	Drop
}


fn action_from_str(s: &str) -> Action {
	match s {
		"pick" | "p" => Action::Pick,
		"reword" | "r" => Action::Reword,
		"edit" | "e" => Action::Edit,
		"squash" | "s" => Action::Squash,
		"fixup" | "f" => Action::Fixup,
		"drop" | "d" => Action::Drop,
		_ => Action::Pick
	}
}

fn action_to_str(action: &Action) -> String {
	String::from(match action {
		&Action::Pick => "pick",
		&Action::Reword => "reword",
		&Action::Edit => "edit",
		&Action::Squash => "squash",
		&Action::Fixup => "fixup",
		&Action::Drop => "drop"
	})
}

struct Line {
	action: Action,
	hash: String,
	comment: String
}

impl Line {
	fn new(input_line: &str) -> Result<Self, String> {
		let input: Vec<&str> = input_line.splitn(3, " ").collect();
		match input.len() {
			3 => Ok(Line {
				action: action_from_str(input[0]),
				hash: String::from(input[1]),
				comment: String::from(input[2])
			}),
			_ => Err(format!(
				"Invalid line {}", input_line
			))
		}
	}
}

struct GitInteractive<'a> {
	filepath: &'a Path,
	lines: Vec<Line>,
	selected_line: usize
}

impl<'a> GitInteractive<'a> {
	fn from_filepath(filepath: &'a str) -> Result<Option<Self>, String> {
		let path = Path::new(filepath);
		
		let mut file = match File::open(path) {
			Ok(file) => file,
			Err(why) => {
				return Err(format!(
					"Error opening file, {}\n\
					Reason: {}", path.display(), why.description()
				));
			}
		};
		
		let mut s = String::new();
		match file.read_to_string(&mut s) {
			Ok(_) => {},
			Err(why) => {
				return Err(format!(
					"Error reading file, {}\n\
					Reason: {}", path.display(), why.description()
				));
			}
		}
		
		if s.starts_with("noop") {
			return Ok(None)
		}
		
		let parsed_result: Result<Vec<Line>, String> = s
			.lines()
			.filter(|l| !l.starts_with("#") && !l.is_empty())
			.map(|l| Line::new(l))
			.collect();
		
		match parsed_result {
			Ok(lines) => Ok(
				Some(GitInteractive {
					filepath: path,
					lines: lines,
					selected_line: 1
				})
			),
			Err(e) => Err(format!(
				"Error reading file, {}\n\
				Reason: {}", path.display(), e
			))
		}
	}
	
	fn write_file(&self) -> Result<(), String> {
		let path = Path::new(self.filepath);
		
		let mut file = match File::create(path) {
			Ok(file) => file,
			Err(why) => {
				return Err(format!(
					"Error opening file, {}\n\
					Reason: {}", path.display(), why.description()
				));
			}
		};
		
		for line in &self.lines {
			match writeln!(file, "{} {} {}", action_to_str(&line.action), line.hash, line.comment) {
				Ok(_) => {},
				Err(why) => {
					return Err(format!(
						"Error writing to file, {}", why.description()
					));
				}
			}
		}
		Ok(())
	}
	
	fn move_cursor_up(&mut self) {
		self.selected_line = cmp::max(self.selected_line - 1, 1);
	}
	
	fn move_cursor_down(&mut self) {
		self.selected_line = cmp::min(self.selected_line + 1, self.lines.len());
	}
	
	fn swap_selected_up(&mut self) {
		if self.selected_line == 1 {
			return
		}
		self.lines.swap(self.selected_line - 1, self.selected_line - 2);
		self.move_cursor_up();
	}
	
	fn swap_selected_down(&mut self) {
		if self.selected_line == self.lines.len() {
			return
		}
		self.lines.swap(self.selected_line - 1, self.selected_line);
		self.move_cursor_down();
	}
	
	fn set_selected_line_action(&mut self, action: Action) {
		self.lines[self.selected_line - 1].action = action;
	}
}

const COLOR_TABLE: [i16; 7] = [
	pancurses::COLOR_WHITE,
	pancurses::COLOR_YELLOW,
	pancurses::COLOR_BLUE,
	pancurses::COLOR_GREEN,
	pancurses::COLOR_CYAN,
	pancurses::COLOR_MAGENTA,
	pancurses::COLOR_RED
];

enum Color {
	White,
	Yellow,
	Blue,
	Green,
	Cyan,
	Magenta,
	Red
}

struct Window {
	window: pancurses::Window,
	top: usize
}

impl Window {
	fn new() -> Self {
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
		
		Window{
			window: window,
			top: 0
		}
	}
	fn draw(&self, git_interactive: &GitInteractive) {
		self.window.clear();
		self.draw_title();
		// 4 removed for other UI lines
		let window_height = (self.window.get_max_y() - 4) as usize;
		
		let mut index: usize = self.top + 1;
		for line in git_interactive
			.lines
			.iter()
			.skip(self.top)
			.take(window_height)
		{
			self.draw_line(&line, index == git_interactive.selected_line);
			index += 1;
		}
		self.draw_footer();
		self.window.refresh();
	}
	
	fn draw_title(&self) {
		self.set_color(Color::White);
		self.set_dim(true);
		self.set_underline(true);
		self.window.addstr("Git Interactive Rebase                       ? for help\n\n");
		self.set_underline(false);
		self.set_dim(false);
	}
	
	fn draw_line(&self, line: &Line, selected: bool) {
		self.set_color(Color::White);
		if selected {
			self.window.addstr(" > ");
		}
		else {
			self.window.addstr("   ");
		}
		match line.action {
			Action::Pick => self.set_color(Color::Green),
			Action::Reword => self.set_color(Color::Yellow),
			Action::Edit => self.set_color(Color::Blue),
			Action::Squash => self.set_color(Color::Cyan),
			Action::Fixup => self.set_color(Color::Magenta),
			Action::Drop => self.set_color(Color::Red)
		}
		self.window.addstr(&format!("{:6}", action_to_str(&line.action)).as_ref());
		self.set_color(Color::White);
		self.window.addstr(&format!(" {} {}\n", line.hash, line.comment).as_ref());
	}
	
	fn draw_footer(&self) {
		self.set_color(Color::White);
		self.set_dim(true);
		self.window.mvaddstr(
			self.window.get_max_y() - 1,
			0,
			"Actions: [ up, down, q/Q, w/W, j, k, p, r, e, s, f, d, ? ]"
		);
		self.set_dim(false);
	}
	
	fn draw_help(&self) {
		self.window.clear();
		self.draw_title();
		self.set_color(Color::White);
		self.window.addstr(" Key        Action\n");
		self.window.addstr(" --------------------------------------------------\n");
		self.draw_help_command("Up", "Move selection up");
		self.draw_help_command("Down", "Move selection Down");
		self.draw_help_command("q", "Abort interactive rebase");
		self.draw_help_command("Q", "Immediately abort interactive rebase");
		self.draw_help_command("w", "Write interactive rebase file");
		self.draw_help_command("W", "Immediately write interactive rebase file");
		self.draw_help_command("?", "Show help");
		self.draw_help_command("j", "Move selected commit down");
		self.draw_help_command("k", "Move selected commit up");
		self.draw_help_command("p", "Set selected commit to be picked");
		self.draw_help_command("r", "Set selected commit to be reworded");
		self.draw_help_command("e", "Set selected commit to be edited");
		self.draw_help_command("s", "Set selected commit to be squashed");
		self.draw_help_command("f", "Set selected commit to be fixed-up");
		self.draw_help_command("d", "Set selected commit to be dropped");
		self.window.addstr("\n\nHit any key to close help");
		self.window.refresh();
		self.window.getch();
	}
	
	fn draw_help_command(&self, command: &str, help: &str) {
		self.set_color(Color::Blue);
		self.window.addstr(&format!(" {:4}    ", command));
		self.set_color(Color::White);
		self.window.addstr(&format!("{}\n", help));
	}
	
	fn set_color(&self, color: Color) {
		match color {
			Color::White => self.window.attrset(pancurses::COLOR_PAIR(0)),
			Color::Yellow => self.window.attrset(pancurses::COLOR_PAIR(1)),
			Color::Blue => self.window.attrset(pancurses::COLOR_PAIR(2)),
			Color::Green => self.window.attrset(pancurses::COLOR_PAIR(3)),
			Color::Cyan => self.window.attrset(pancurses::COLOR_PAIR(4)),
			Color::Magenta => self.window.attrset(pancurses::COLOR_PAIR(5)),
			Color::Red => self.window.attrset(pancurses::COLOR_PAIR(6))
		};
	}
	
	fn set_dim(&self, on: bool) {
		if on {
			self.window.attron(pancurses::A_DIM);
		}
		else {
			self.window.attroff(pancurses::A_DIM);
		}
	}
	
	fn set_underline(&self, on: bool) {
		if on {
			self.window.attron(pancurses::A_UNDERLINE);
		}
		else {
			self.window.attroff(pancurses::A_UNDERLINE);
		}
	}
	
	fn confirm(&self, message: &str) -> bool  {
		self.window.clear();
		self.draw_title();
		self.window.addstr(&format!("{} (y/n)?", message));
		match self.window.getch() {
			Some(pancurses::Input::Character(c)) if c == 'y' || c == 'Y' => true,
			_ => false
		}
	}
	
	fn set_top(&mut self, git_interactive: &GitInteractive) {
		// 4 removed for other UI lines
		let window_height = (self.window.get_max_y() - 4) as usize;
		
		self.top = match git_interactive.selected_line {
			s if s == git_interactive.lines.len() => self.top,
			s if s <= self.top => s - 1,
			s if s >= self.top + window_height => s - window_height + 1,
			_ => self.top
		}
	}
	
	fn endwin(&self) {
		self.window.clear();
		self.window.refresh();
		pancurses::curs_set(1);
		pancurses::endwin();
	}
}

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
	
	let mut git_interactive = match GitInteractive::from_filepath(&filepath) {
		Ok(gi) => {
			match gi {
				Some(git_interactive) => git_interactive,
				None => {
					print_err!("{}", &"Nothing to edit");
					process::exit(0);
				}
			}
		},
		Err(msg) => {
			print_err!("{}", msg);
			process::exit(1);
		}
	};
	
	let mut window = Window::new();
	
	loop {
		window.draw(&git_interactive);
		match window.window.getch() {
			Some(Input::Character(c)) if c == 'q' => {
				if window.confirm("Are you sure you want to abort") {
					git_interactive.lines.clear();
					break;
				}
			},
			Some(Input::Character(c)) if c == 'Q' => {
				git_interactive.lines.clear();
				break;
			},
			Some(Input::Character(c)) if c == 'w' => {
				if window.confirm("Are you sure you want to rebase") {
					break;
				}
			},
			Some(Input::Character(c)) if c == 'W' => {
				break;
			},
			Some(Input::Character(c)) if c == '?' => window.draw_help(),
			Some(Input::Character(c)) if c == 'p' => git_interactive.set_selected_line_action(Action::Pick),
			Some(Input::Character(c)) if c == 'r' => git_interactive.set_selected_line_action(Action::Reword),
			Some(Input::Character(c)) if c == 'e' => git_interactive.set_selected_line_action(Action::Edit),
			Some(Input::Character(c)) if c == 's' => git_interactive.set_selected_line_action(Action::Squash),
			Some(Input::Character(c)) if c == 'f' => git_interactive.set_selected_line_action(Action::Fixup),
			Some(Input::Character(c)) if c == 'd' => git_interactive.set_selected_line_action(Action::Drop),
			Some(Input::Character(c)) if c == 'j' => {
				git_interactive.swap_selected_down();
				window.set_top(&git_interactive);
			},
			Some(Input::Character(c)) if c == 'k' => {
				git_interactive.swap_selected_up();
				window.set_top(&git_interactive);
			},
			Some(pancurses::Input::KeyUp) => {
				git_interactive.move_cursor_up();
				window.set_top(&git_interactive);
			},
			Some(pancurses::Input::KeyDown) => {
				git_interactive.move_cursor_down();
				window.set_top(&git_interactive);
			},
			Some(pancurses::Input::KeyResize) => window.set_top(&git_interactive),
			_ => {}
		}
	}
	
	window.endwin();
	match git_interactive.write_file() {
		Ok(_) => {},
		Err(msg) => {
			print_err!("{}", msg);
			process::exit(1);
		}
	};
}
