use crate::display::color::Color;
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
use std::env::var;

pub(crate) struct Curses {
	color_lookup: HashMap<(i16, i16, i16), i16>,
	color_index: i16,
	color_pair_index: i16,
	window: pancurses::Window,
	selected_line_enabled: bool,
}

impl Curses {
	pub(crate) fn new() -> Self {
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

		let number_of_colors = pancurses::COLORS() as usize;

		Self {
			window,
			color_lookup: HashMap::new(),
			color_index: 8,
			color_pair_index: 1,
			// Terminal.app on MacOS doesn't not properly support the color pairs needed for selected line
			selected_line_enabled: number_of_colors > 16 && var("TERM_PROGRAM").unwrap_or_default() != "Apple_Terminal",
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

	fn init_color_pair(&mut self, foreground: Color, background: Color) -> chtype {
		let index = self.color_pair_index;
		self.color_pair_index += 1;
		pancurses::init_pair(index, self.init_color(foreground), self.init_color(background));
		// curses seems to init a pair for i16 but read with u64
		pancurses::COLOR_PAIR(index as chtype)
	}

	pub(super) fn register_selectable_color_pairs(
		&mut self,
		foreground: Color,
		background: Color,
		selected_background: Color,
	) -> (chtype, chtype)
	{
		let standard_pair = self.init_color_pair(foreground, background);
		if self.selected_line_enabled {
			return (standard_pair, self.init_color_pair(foreground, selected_background));
		}
		// when there is not enough color pairs to support selected
		(standard_pair, standard_pair)
	}

	pub(super) fn erase(&self) {
		self.window.erase();
	}

	pub(super) fn refresh(&self) {
		self.window.refresh();
	}

	pub(super) fn addstr(&self, s: &str) {
		self.window.addstr(s);
	}

	pub(super) fn attrset<T: Into<chtype>>(&self, attributes: T) {
		self.window.attrset(attributes);
	}

	pub(super) fn attron<T: Into<chtype>>(&self, attributes: T) {
		self.window.attron(attributes);
	}

	pub(super) fn attroff<T: Into<chtype>>(&self, attributes: T) {
		self.window.attroff(attributes);
	}

	pub(super) fn getch(&self) -> Option<Input> {
		self.window.getch()
	}

	pub(super) fn get_max_y(&self) -> i32 {
		self.window.get_max_y()
	}

	pub(super) fn get_max_x(&self) -> i32 {
		self.window.get_max_x()
	}

	pub(super) fn def_prog_mode(&self) {
		pancurses::def_prog_mode();
	}

	pub(super) fn reset_prog_mode(&self) {
		pancurses::reset_prog_mode();
	}

	pub(super) fn endwin(&self) {
		pancurses::endwin();
	}
}
