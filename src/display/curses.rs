use crate::display::Color;
use pancurses::{
	chtype,
	Input,
	COLOR_BLACK,
	COLOR_BLUE,
	COLOR_CYAN,
	COLOR_GREEN,
	COLOR_MAGENTA,
	COLOR_RED,
	COLOR_WHITE,
	COLOR_YELLOW,
};
use std::collections::HashMap;

pub struct Curses {
	color_lookup: HashMap<(i16, i16, i16), i16>,
	color_index: i16,
	color_pair_index: i16,
	pub number_of_colors: usize,
	window: pancurses::Window,
	pub has_colors: bool,
}

impl Curses {
	pub fn new() -> Self {
		let window = pancurses::initscr();
		window.keypad(true);

		pancurses::curs_set(0);
		pancurses::noecho();

		let has_colors = pancurses::has_colors();
		if has_colors {
			pancurses::start_color();
		}
		pancurses::use_default_colors();

		// pair zero should always be default
		pancurses::init_pair(0, -1, -1);

		Self {
			window,
			has_colors,
			number_of_colors: pancurses::COLORS() as usize,
			color_lookup: HashMap::new(),
			color_index: 8,
			color_pair_index: 1,
		}
	}

	fn init_color(&mut self, color: Color) -> i16 {
		match color {
			Color::Black => COLOR_BLACK,
			Color::Blue => COLOR_BLUE,
			Color::Cyan => COLOR_CYAN,
			Color::Green => COLOR_GREEN,
			Color::Magenta => COLOR_MAGENTA,
			Color::Red => COLOR_RED,
			Color::Yellow => COLOR_YELLOW,
			Color::White => COLOR_WHITE,
			Color::Default => -1,
			Color::RGB { red, green, blue } => {
				match self.color_lookup.get(&(red, green, blue)) {
					Some(index) => *index,
					None => {
						pancurses::init_color(self.color_index, red, green, blue);
						let index = self.color_index;
						self.color_index += 1;
						index
					},
				}
			},
		}
	}

	pub fn init_color_pair(&mut self, foreground: Color, background: Color) -> i16 {
		let index = self.color_pair_index;
		self.color_pair_index += 1;
		pancurses::init_pair(index, self.init_color(foreground), self.init_color(background));
		// curses seems to init a pair for i16 but read with u64
		pancurses::COLOR_PAIR(index as chtype) as i16
	}

	pub fn erase(&self) {
		self.window.erase();
	}

	pub fn refresh(&self) {
		self.window.refresh();
	}

	pub fn addstr(&self, s: &str) {
		self.window.addstr(s);
	}

	pub fn attrset<T: Into<chtype>>(&self, attributes: T) {
		self.window.attrset(attributes);
	}

	pub fn attron<T: Into<chtype>>(&self, attributes: T) {
		self.window.attron(attributes);
	}

	pub fn attroff<T: Into<chtype>>(&self, attributes: T) {
		self.window.attroff(attributes);
	}

	pub fn getch(&self) -> Option<Input> {
		self.window.getch()
	}

	pub fn get_max_y(&self) -> i32 {
		self.window.get_max_y()
	}

	pub fn get_max_x(&self) -> i32 {
		self.window.get_max_x()
	}

	pub fn def_prog_mode(&self) {
		pancurses::def_prog_mode();
	}

	pub fn reset_prog_mode(&self) {
		pancurses::reset_prog_mode();
	}

	pub fn endwin(&self) {
		pancurses::endwin();
	}
}
