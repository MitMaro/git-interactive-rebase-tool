use crate::display::color::Color;
use crate::display::curses::{
	chtype,
	Curses,
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

pub(super) struct ColorManager {
	color_index: i16,
	color_lookup: HashMap<(i16, i16, i16), i16>,
	color_pair_index: i16,
}

impl ColorManager {
	pub(super) fn new() -> Self {
		Self {
			color_lookup: HashMap::new(),
			color_index: 16,      // we only create new colors in true color mode
			color_pair_index: 16, // skip the default color pairs
		}
	}

	pub fn register_selectable_color_pairs(
		&mut self,
		curses: &mut Curses,
		foreground: Color,
		background: Color,
		selected_background: Color,
	) -> (chtype, chtype)
	{
		let fg = self.find_color(curses, foreground);
		let bg = self.find_color(curses, background);
		let standard_pair = curses.init_color_pair(self.color_pair_index, fg, bg);
		self.color_pair_index += 1;
		if curses.get_color_mode().has_minimum_four_bit_color() {
			let sbg = self.find_color(curses, selected_background);
			let index = self.color_pair_index;
			self.color_pair_index += 1;
			return (standard_pair, curses.init_color_pair(index, fg, sbg));
		}
		// when there is not enough color pairs to support selected
		(standard_pair, standard_pair)
	}

	// Modified version from gyscos/cursive (https://github.com/gyscos/cursive)
	// Copyright (c) 2015 Alexandre Bury - MIT License
	fn find_color(&mut self, curses: &mut Curses, color: Color) -> i16 {
		let color_mode = curses.get_color_mode();
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
				if color_mode.has_minimum_four_bit_color() {
					COLOR_BLACK + 8
				}
				else {
					COLOR_BLACK
				}
			},
			Color::DarkBlue => {
				if color_mode.has_minimum_four_bit_color() {
					COLOR_BLUE + 8
				}
				else {
					COLOR_BLUE
				}
			},
			Color::DarkCyan => {
				if color_mode.has_minimum_four_bit_color() {
					COLOR_CYAN + 8
				}
				else {
					COLOR_CYAN
				}
			},
			Color::DarkGreen => {
				if color_mode.has_minimum_four_bit_color() {
					COLOR_GREEN + 8
				}
				else {
					COLOR_GREEN
				}
			},
			Color::DarkMagenta => {
				if color_mode.has_minimum_four_bit_color() {
					COLOR_MAGENTA + 8
				}
				else {
					COLOR_MAGENTA
				}
			},
			Color::DarkRed => {
				if color_mode.has_minimum_four_bit_color() {
					COLOR_RED + 8
				}
				else {
					COLOR_RED
				}
			},
			Color::DarkYellow => {
				if color_mode.has_minimum_four_bit_color() {
					COLOR_YELLOW + 8
				}
				else {
					COLOR_YELLOW
				}
			},
			Color::DarkWhite => {
				if color_mode.has_minimum_four_bit_color() {
					COLOR_WHITE + 8
				}
				else {
					COLOR_WHITE
				}
			},
			// for indexed colored we assume 8bit color
			Color::Index(i) => i,
			Color::RGB { red, green, blue } if color_mode.has_true_color() => {
				if let Some(index) = self.color_lookup.get(&(red, green, blue)) {
					*index
				}
				else {
					let index = self.color_index;
					curses.init_color(
						index,
						((f64::from(red) / 255.0) * 1000.0) as i16,
						((f64::from(green) / 255.0) * 1000.0) as i16,
						((f64::from(blue) / 255.0) * 1000.0) as i16,
					);
					self.color_lookup.insert((red, green, blue), index);
					self.color_index += 1;
					index
				}
			},
			Color::RGB { red, green, blue } if color_mode.has_minimum_four_bit_color() => {
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
}
