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
	) -> (chtype, chtype) {
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
			#[allow(clippy::option_if_let_else)]
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
					232 + (red - 8) / 10
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::build_trace;
	use crate::display::color_mode::ColorMode;
	use crate::testutil::compare_trace;
	use concat_idents::concat_idents;

	fn _assert_indexed_color(color_mode: ColorMode, color: Color, expected_index: i32) {
		let mut color_manager = ColorManager::new();
		let mut curses = Curses::new();
		curses.set_color_mode(color_mode);
		color_manager.register_selectable_color_pairs(&mut curses, color, Color::Default, Color::Default);
		let trace = curses.get_function_trace();
		let expected_trace = vec![build_trace!(
			"init_color_pair",
			"16",
			format!("{}", expected_index),
			"-1"
		)];
		compare_trace(&trace, &expected_trace);
	}

	macro_rules! assert_three_bit_color {
		($name:ident, $expected_index:expr, $color:path) => {
			concat_idents!(test_name = action_register_selectable_color_pairs_three_bit_, $name {
				#[test]
				fn test_name() {
					_assert_indexed_color(ColorMode::ThreeBit, $color, $expected_index);
				}
			});
		};
		($expected_index:expr, $red:expr, $green:expr, $blue:expr) => {
			concat_idents!(test_name = action_register_selectable_color_pairs_three_bit_, $red, _, $green, _, $blue {
				#[test]
				fn test_name() {
					_assert_indexed_color(ColorMode::ThreeBit, Color::RGB {
						red: $red,
						green: $green,
						blue: $blue,
					}, $expected_index);
				}
			});
		};
	}

	macro_rules! assert_four_bit_color {
		($name:ident, $expected_index:expr, $color:path) => {
			concat_idents!(test_name = action_register_selectable_color_pairs_four_bit_, $name {
				#[test]
				fn test_name() {
					_assert_indexed_color(ColorMode::FourBit, $color, $expected_index);
				}
			});
		};
		($expected_index:expr, $red:expr, $green:expr, $blue:expr) => {
			concat_idents!(test_name = action_register_selectable_color_pairs_four_bit_, $red, _, $green, _, $blue {
				#[test]
				fn test_name() {
					_assert_indexed_color(ColorMode::FourBit, Color::RGB {
						red: $red,
						green: $green,
						blue: $blue,
					}, $expected_index);
				}
			});
		};
	}

	// three bit
	// black range
	assert_three_bit_color!(0, 0, 0, 0);
	assert_three_bit_color!(0, 0, 0, 127);
	assert_three_bit_color!(0, 0, 127, 0);
	assert_three_bit_color!(0, 0, 127, 127);
	assert_three_bit_color!(0, 127, 0, 0);
	assert_three_bit_color!(0, 127, 0, 127);
	assert_three_bit_color!(0, 127, 127, 0);
	assert_three_bit_color!(0, 127, 127, 127);

	// red range
	assert_three_bit_color!(1, 128, 0, 0);
	assert_three_bit_color!(1, 255, 0, 0);

	// green range
	assert_three_bit_color!(2, 0, 128, 0);
	assert_three_bit_color!(2, 0, 255, 0);

	// blue range
	assert_three_bit_color!(4, 0, 0, 128);
	assert_three_bit_color!(4, 0, 0, 255);

	// yellow range
	assert_three_bit_color!(3, 128, 128, 0);
	assert_three_bit_color!(3, 128, 255, 0);
	assert_three_bit_color!(3, 255, 255, 0);

	// cyan range
	assert_three_bit_color!(6, 0, 128, 128);
	assert_three_bit_color!(6, 0, 128, 255);
	assert_three_bit_color!(6, 0, 255, 255);

	// magenta color range
	assert_three_bit_color!(5, 128, 0, 128);
	assert_three_bit_color!(5, 128, 0, 255);
	assert_three_bit_color!(5, 255, 0, 255);

	// white color range
	assert_three_bit_color!(7, 128, 128, 128);
	assert_three_bit_color!(7, 128, 128, 255);
	assert_three_bit_color!(7, 128, 255, 128);
	assert_three_bit_color!(7, 128, 255, 255);
	assert_three_bit_color!(7, 255, 128, 128);
	assert_three_bit_color!(7, 255, 128, 255);
	assert_three_bit_color!(7, 255, 255, 128);
	assert_three_bit_color!(7, 255, 255, 255);

	// colors
	assert_three_bit_color!(light_white, 7, Color::LightWhite);
	assert_three_bit_color!(dark_white, 7, Color::DarkWhite);
	assert_three_bit_color!(light_black, 0, Color::LightBlack);
	assert_three_bit_color!(dark_black, 0, Color::DarkBlack);
	assert_three_bit_color!(light_blue, 4, Color::LightBlue);
	assert_three_bit_color!(dark_blue, 4, Color::DarkBlue);
	assert_three_bit_color!(light_cyan, 6, Color::LightCyan);
	assert_three_bit_color!(dark_cyan, 6, Color::DarkCyan);
	assert_three_bit_color!(light_green, 2, Color::LightGreen);
	assert_three_bit_color!(dark_green, 2, Color::DarkGreen);
	assert_three_bit_color!(light_magenta, 5, Color::LightMagenta);
	assert_three_bit_color!(dark_magenta, 5, Color::DarkMagenta);
	assert_three_bit_color!(light_red, 1, Color::LightRed);
	assert_three_bit_color!(dark_red, 1, Color::DarkRed);
	assert_three_bit_color!(light_yellow, 3, Color::LightYellow);
	assert_three_bit_color!(dark_yellow, 3, Color::DarkYellow);
	assert_three_bit_color!(default, -1, Color::Default);

	// four bit
	// black range
	assert_four_bit_color!(16, 0, 0, 0);
	assert_four_bit_color!(16, 1, 1, 1);
	assert_four_bit_color!(16, 4, 4, 4);
	assert_four_bit_color!(16, 7, 7, 7);

	// grey range
	assert_four_bit_color!(232, 8, 8, 8);
	assert_four_bit_color!(232, 16, 16, 16);
	assert_four_bit_color!(234, 32, 32, 32);
	assert_four_bit_color!(237, 64, 64, 64);
	assert_four_bit_color!(244, 128, 128, 128);
	assert_four_bit_color!(255, 246, 246, 246);

	// white range
	assert_four_bit_color!(231, 247, 247, 247);
	assert_four_bit_color!(231, 248, 248, 248);
	assert_four_bit_color!(231, 253, 253, 253);
	assert_four_bit_color!(231, 255, 255, 255);

	// base colors
	assert_four_bit_color!(196, 255, 0, 0);
	assert_four_bit_color!(46, 0, 255, 0);
	assert_four_bit_color!(21, 0, 0, 255);
	assert_four_bit_color!(51, 0, 255, 255);
	assert_four_bit_color!(201, 255, 0, 255);
	assert_four_bit_color!(226, 255, 255, 0);

	// random sample of colors
	assert_four_bit_color!(88, 127, 0, 0);
	assert_four_bit_color!(28, 0, 127, 0);
	assert_four_bit_color!(18, 0, 0, 127);
	assert_four_bit_color!(90, 127, 0, 127);
	assert_four_bit_color!(208, 255, 95, 0);

	// colors
	assert_four_bit_color!(light_white, 7, Color::LightWhite);
	assert_four_bit_color!(dark_white, 15, Color::DarkWhite);
	assert_four_bit_color!(light_black, 0, Color::LightBlack);
	assert_four_bit_color!(dark_black, 8, Color::DarkBlack);
	assert_four_bit_color!(light_blue, 4, Color::LightBlue);
	assert_four_bit_color!(dark_blue, 12, Color::DarkBlue);
	assert_four_bit_color!(light_cyan, 6, Color::LightCyan);
	assert_four_bit_color!(dark_cyan, 14, Color::DarkCyan);
	assert_four_bit_color!(light_green, 2, Color::LightGreen);
	assert_four_bit_color!(dark_green, 10, Color::DarkGreen);
	assert_four_bit_color!(light_magenta, 5, Color::LightMagenta);
	assert_four_bit_color!(dark_magenta, 13, Color::DarkMagenta);
	assert_four_bit_color!(light_red, 1, Color::LightRed);
	assert_four_bit_color!(dark_red, 9, Color::DarkRed);
	assert_four_bit_color!(light_yellow, 3, Color::LightYellow);
	assert_four_bit_color!(dark_yellow, 11, Color::DarkYellow);
	assert_four_bit_color!(default, -1, Color::Default);

	// true color
	#[test]
	fn action_register_selectable_color_pairs_true_color() {
		let mut color_manager = ColorManager::new();
		let mut curses = Curses::new();
		curses.set_color_mode(ColorMode::TrueColor);
		color_manager.register_selectable_color_pairs(
			&mut curses,
			Color::RGB {
				red: 100,
				green: 150,
				blue: 200,
			},
			Color::RGB {
				red: 100,
				green: 150,
				blue: 200,
			},
			Color::RGB {
				red: 120,
				green: 170,
				blue: 220,
			},
		);
		let trace = curses.get_function_trace();
		let expected_trace = vec![
			build_trace!("init_color", "16", "392", "588", "784"),
			build_trace!("init_color", "17", "470", "666", "862"),
		];
		compare_trace(&trace, &expected_trace);
	}

	#[test]
	fn action_register_selectable_color_pairs_two_tone() {
		let mut color_manager = ColorManager::new();
		let mut curses = Curses::new();
		curses.set_color_mode(ColorMode::TwoTone);
		let (standard_color, selected_color) = color_manager.register_selectable_color_pairs(
			&mut curses,
			Color::DarkBlue,
			Color::LightYellow,
			Color::DarkMagenta,
		);
		let trace = curses.get_function_trace();
		let expected_trace = vec![build_trace!("init_color_pair", "16", "4", "3")];
		compare_trace(&trace, &expected_trace);
		assert_eq!(standard_color, selected_color);
	}

	#[test]
	fn action_register_selectable_color_pairs_register_multiple_pairs() {
		let mut color_manager = ColorManager::new();
		let mut curses = Curses::new();
		curses.set_color_mode(ColorMode::TwoTone);
		color_manager.register_selectable_color_pairs(
			&mut curses,
			Color::DarkBlue,
			Color::LightYellow,
			Color::DarkMagenta,
		);
		color_manager.register_selectable_color_pairs(
			&mut curses,
			Color::DarkCyan,
			Color::LightMagenta,
			Color::DarkRed,
		);
		let trace = curses.get_function_trace();
		let expected_trace = vec![
			build_trace!("init_color_pair", "16", "4", "3"),
			build_trace!("init_color_pair", "17", "6", "5"),
		];
		compare_trace(&trace, &expected_trace);
	}
}
