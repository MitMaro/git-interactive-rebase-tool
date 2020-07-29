use crate::display::color::Color;
use crate::display::color_mode::ColorMode;
use crate::display::utils::detect_color_mode;
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
	color_index: i16,
	color_lookup: HashMap<(i16, i16, i16), i16>,
	color_mode: ColorMode,
	color_pair_index: i16,
	window: pancurses::Window,
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
			pancurses::use_default_colors();

			// pair zero should always be default
			pancurses::init_pair(0, -1, -1);
		}

		let color_mode = if has_colors {
			detect_color_mode(pancurses::COLORS() as i16)
		}
		else {
			ColorMode::TwoTone
		};

		Self {
			color_index: 16, // we only create new colors in true color mode
			window,
			color_pair_index: 16, // skip the default color pairs
			color_lookup: HashMap::new(),
			color_mode,
		}
	}

	fn init_color(&mut self, red: i16, green: i16, blue: i16) -> i16 {
		if let Some(index) = self.color_lookup.get(&(red, green, blue)) {
			*index
		}
		else {
			let index = self.color_index;
			self.color_index += 1;
			pancurses::init_color(
				index,
				// convert from 0-255 range to 0 - 1000
				((f64::from(red) / 255.0) * 1000.0) as i16,
				((f64::from(green) / 255.0) * 1000.0) as i16,
				((f64::from(blue) / 255.0) * 1000.0) as i16,
			);
			self.color_lookup.insert((red, green, blue), index);
			index
		}
	}

	// Modified version from gyscos/cursive (https://github.com/gyscos/cursive)
	// Copyright (c) 2015 Alexandre Bury - MIT License
	fn find_color(&mut self, color: Color) -> i16 {
		match color {
			Color::Default => -1,
			Color::LightBlack => COLOR_BLACK,
			Color::LightBlue => COLOR_BLUE,
			Color::LightCyan => COLOR_CYAN,
			Color::LightGreen => COLOR_GREEN,
			Color::LightMagenta => COLOR_MAGENTA,
			Color::LightRed => COLOR_RED,
			Color::LightYellow => COLOR_YELLOW,
			Color::LightWhite => COLOR_WHITE,
			// for dark colors, just the light color when there isn't deep enough color support
			Color::DarkBlack => {
				if self.color_mode.has_minimum_four_bit_color() {
					COLOR_BLACK + 8
				}
				else {
					COLOR_BLACK
				}
			},
			Color::DarkBlue => {
				if self.color_mode.has_minimum_four_bit_color() {
					COLOR_BLUE + 8
				}
				else {
					COLOR_BLUE
				}
			},
			Color::DarkCyan => {
				if self.color_mode.has_minimum_four_bit_color() {
					COLOR_CYAN + 8
				}
				else {
					COLOR_CYAN
				}
			},
			Color::DarkGreen => {
				if self.color_mode.has_minimum_four_bit_color() {
					COLOR_GREEN + 8
				}
				else {
					COLOR_GREEN
				}
			},
			Color::DarkMagenta => {
				if self.color_mode.has_minimum_four_bit_color() {
					COLOR_MAGENTA + 8
				}
				else {
					COLOR_MAGENTA
				}
			},
			Color::DarkRed => {
				if self.color_mode.has_minimum_four_bit_color() {
					COLOR_RED + 8
				}
				else {
					COLOR_RED
				}
			},
			Color::DarkYellow => {
				if self.color_mode.has_minimum_four_bit_color() {
					COLOR_YELLOW + 8
				}
				else {
					COLOR_YELLOW
				}
			},
			Color::DarkWhite => {
				if self.color_mode.has_minimum_four_bit_color() {
					COLOR_WHITE + 8
				}
				else {
					COLOR_WHITE
				}
			},
			// for indexed colored we assume 8bit color
			Color::Index(i) => i,
			Color::RGB { red, green, blue } if self.color_mode.has_true_color() => self.init_color(red, green, blue),
			Color::RGB { red, green, blue } if self.color_mode.has_minimum_four_bit_color() => {
				// If red, green and blue are equal then we assume a grey scale color
				// shades less than 8 should go to pure black, while shades greater than 247 should go to pure white
				if red == green && green == blue && red >= 8 && red < 247 {
					// The grayscale palette says the colors 232 + n are: (red = green = blue) = 8 + 10 * n
					// With 0 <= n <= 23. This gives: (red - 8) / 10 = n
					let n = (red - 8) / 10;
					232 + n
				}
				else {
					// Generic RGB
					let r = 6 * red / 256;
					let g = 6 * green / 256;
					let b = 6 * blue / 256;
					16 + 36 * r + 6 * g + b
				}
			},
			Color::RGB { red, green, blue } => {
				// Have to hack it down to 8 colors.
				let r = if red > 127 { 1 } else { 0 };
				let g = if green > 127 { 1 } else { 0 };
				let b = if blue > 127 { 1 } else { 0 };
				(r + 2 * g + 4 * b) as i16
			},
		}
	}

	fn init_color_pair(&mut self, foreground: Color, background: Color) -> chtype {
		let index = self.color_pair_index;
		self.color_pair_index += 1;
		pancurses::init_pair(index, self.find_color(foreground), self.find_color(background));
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
		if self.color_mode.has_minimum_four_bit_color() {
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
