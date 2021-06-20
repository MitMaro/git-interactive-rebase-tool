use std::env::var;

use config::Color;

use super::{color_mode::ColorMode, Colors, CrosstermColor};

pub(super) fn detect_color_mode(number_of_colors: u16) -> ColorMode {
	// respect COLORTERM being truecolor or 24bit
	if let Ok(color_term) = var("COLORTERM") {
		if color_term == "truecolor" || color_term == "24bit" {
			return ColorMode::TrueColor;
		}
	}

	// VTE based terms should all be setting COLORTERM, but just in case
	if let Ok(vte_version) = var("VTE_VERSION") {
		let vte_version = vte_version.parse::<i32>().unwrap_or(0);

		if vte_version >= 3600 {
			// version 0.36.00
			return ColorMode::TrueColor;
		}
		else if vte_version > 0 {
			return ColorMode::EightBit;
		}
	}

	// Assume terminals with `-256` are 8bit
	if let Ok(term) = var("TERM") {
		if term.contains("-256") {
			return ColorMode::EightBit;
		}
	}

	// Windows 10 Terminal sets WT_SESSION
	if var("WT_SESSION").is_ok() {
		return ColorMode::TrueColor;
	}

	// at this point there is no way to detect truecolor support, so the best we can get is 8bit
	match number_of_colors {
		n if n >= 256 => ColorMode::EightBit,
		n if n >= 16 => ColorMode::FourBit,
		n if n >= 8 => ColorMode::ThreeBit,
		_ => ColorMode::TwoTone,
	}
}

pub fn register_selectable_color_pairs(
	color_mode: ColorMode,
	foreground: Color,
	background: Color,
	selected_background: Color,
) -> (Colors, Colors) {
	let fg = find_color(color_mode, foreground);
	let bg = find_color(color_mode, background);
	let standard_pair = Colors::new(fg, bg);
	if color_mode.has_minimum_four_bit_color() {
		(
			standard_pair,
			Colors::new(fg, find_color(color_mode, selected_background)),
		)
	}
	else {
		// when there is not enough color pairs to support selected
		(standard_pair, standard_pair)
	}
}

// Modified version from gyscos/cursive (https://github.com/gyscos/cursive)
// Copyright (c) 2015 Alexandre Bury - MIT License
fn find_color(color_mode: ColorMode, color: Color) -> CrosstermColor {
	match color {
		Color::Default => CrosstermColor::Reset,
		Color::LightBlack => CrosstermColor::DarkGrey,
		Color::LightWhite | Color::LightGrey => CrosstermColor::White,
		Color::LightBlue => CrosstermColor::Blue,
		Color::LightCyan => CrosstermColor::Cyan,
		Color::LightGreen => CrosstermColor::Green,
		Color::LightMagenta => CrosstermColor::Magenta,
		Color::LightRed => CrosstermColor::Red,
		Color::LightYellow => CrosstermColor::Yellow,
		// for dark colors, just the light color when there isn't deep enough color support
		Color::DarkBlack => CrosstermColor::Black,
		Color::DarkBlue => {
			if color_mode.has_minimum_four_bit_color() {
				CrosstermColor::DarkBlue
			}
			else {
				CrosstermColor::Blue
			}
		},
		Color::DarkCyan => {
			if color_mode.has_minimum_four_bit_color() {
				CrosstermColor::DarkCyan
			}
			else {
				CrosstermColor::Cyan
			}
		},
		Color::DarkGreen => {
			if color_mode.has_minimum_four_bit_color() {
				CrosstermColor::DarkGreen
			}
			else {
				CrosstermColor::Green
			}
		},
		Color::DarkMagenta => {
			if color_mode.has_minimum_four_bit_color() {
				CrosstermColor::DarkMagenta
			}
			else {
				CrosstermColor::Magenta
			}
		},
		Color::DarkRed => {
			if color_mode.has_minimum_four_bit_color() {
				CrosstermColor::DarkRed
			}
			else {
				CrosstermColor::Red
			}
		},
		Color::DarkYellow => {
			if color_mode.has_minimum_four_bit_color() {
				CrosstermColor::DarkYellow
			}
			else {
				CrosstermColor::Yellow
			}
		},
		Color::DarkWhite => {
			if color_mode.has_minimum_four_bit_color() {
				CrosstermColor::Grey
			}
			else {
				CrosstermColor::White
			}
		},
		Color::DarkGrey => {
			if color_mode.has_minimum_four_bit_color() {
				CrosstermColor::DarkGrey
			}
			else {
				CrosstermColor::Grey
			}
		},
		// for indexed colored we assume 8bit color
		Color::Index(i) => CrosstermColor::AnsiValue(i),
		Color::Rgb { red, green, blue } if color_mode.has_true_color() => CrosstermColor::from((red, green, blue)),
		Color::Rgb { red, green, blue } if color_mode.has_minimum_four_bit_color() => {
			// If red, green and blue are equal then we assume a grey scale color
			// shades less than 8 should go to pure black, while shades greater than 247 should go to pure white
			if red == green && green == blue && red >= 8 && red < 247 {
				// The grayscale palette says the colors 232 + n are: (red = green = blue) = 8 + 10 * n
				// With 0 <= n <= 23. This gives: (red - 8) / 10 = n
				CrosstermColor::AnsiValue(232 + (red - 8) / 10)
			}
			else {
				// Generic RGB
				let r = 6 * u16::from(red) / 256;
				let g = 6 * u16::from(green) / 256;
				let b = 6 * u16::from(blue) / 256;
				CrosstermColor::AnsiValue((16 + 36 * r + 6 * g + b) as u8)
			}
		},
		Color::Rgb { red, green, blue } => {
			// Have to hack it down to 8 colors.
			let r = if red > 127 { 1 } else { 0 };
			let g = if green > 127 { 1 } else { 0 };
			let b = if blue > 127 { 1 } else { 0 };
			CrosstermColor::AnsiValue((r + 2 * g + 4 * b) as u8)
		},
	}
}

#[cfg(test)]
mod tests {
	use std::env::{remove_var, set_var};

	use rstest::rstest;
	use serial_test::serial;

	use super::*;

	fn clear_env() {
		remove_var("COLORTERM");
		remove_var("TERM");
		remove_var("VTE_VERSION");
		remove_var("WT_SESSION");
	}

	#[test]
	#[serial]
	fn detect_color_mode_no_env_2_colors() {
		clear_env();
		assert_eq!(detect_color_mode(2), ColorMode::TwoTone);
	}

	#[test]
	#[serial]
	fn detect_color_mode_no_env_8_colors() {
		clear_env();
		assert_eq!(detect_color_mode(8), ColorMode::ThreeBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_no_env_less_8_colors() {
		clear_env();
		assert_eq!(detect_color_mode(7), ColorMode::TwoTone);
	}

	#[test]
	#[serial]
	fn detect_color_mode_no_env_16_colors() {
		clear_env();
		assert_eq!(detect_color_mode(16), ColorMode::FourBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_no_env_less_16_colors() {
		clear_env();
		assert_eq!(detect_color_mode(15), ColorMode::ThreeBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_no_env_256_colors() {
		clear_env();
		assert_eq!(detect_color_mode(256), ColorMode::EightBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_no_env_less_256_colors() {
		clear_env();
		assert_eq!(detect_color_mode(255), ColorMode::FourBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_no_env_more_256_colors() {
		clear_env();
		assert_eq!(detect_color_mode(257), ColorMode::EightBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_term_env_no_256() {
		clear_env();
		set_var("TERM", "XTERM");
		assert_eq!(detect_color_mode(0), ColorMode::TwoTone);
	}

	#[test]
	#[serial]
	fn detect_color_mode_term_env_with_256() {
		clear_env();
		set_var("TERM", "XTERM-256");
		assert_eq!(detect_color_mode(0), ColorMode::EightBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_vte_version_0_36_00() {
		clear_env();
		set_var("VTE_VERSION", "3600");
		assert_eq!(detect_color_mode(0), ColorMode::TrueColor);
	}

	#[test]
	#[serial]
	fn detect_color_mode_vte_version_greater_0_36_00() {
		clear_env();
		set_var("VTE_VERSION", "3601");
		assert_eq!(detect_color_mode(0), ColorMode::TrueColor);
	}

	#[test]
	#[serial]
	fn detect_color_mode_vte_version_less_0_36_00() {
		clear_env();
		set_var("VTE_VERSION", "1");
		assert_eq!(detect_color_mode(0), ColorMode::EightBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_vte_version_0() {
		clear_env();
		set_var("VTE_VERSION", "0");
		assert_eq!(detect_color_mode(0), ColorMode::TwoTone);
	}
	#[test]
	#[serial]
	fn detect_color_mode_vte_version_invalid() {
		clear_env();
		set_var("VTE_VERSION", "invalid");
		assert_eq!(detect_color_mode(0), ColorMode::TwoTone);
	}

	#[test]
	#[serial]
	fn detect_color_mode_colorterm_env_is_truecolor() {
		clear_env();
		set_var("COLORTERM", "truecolor");
		assert_eq!(detect_color_mode(0), ColorMode::TrueColor);
	}

	#[test]
	#[serial]
	fn detect_color_mode_colorterm_env_is_24bit() {
		clear_env();
		set_var("COLORTERM", "24bit");
		assert_eq!(detect_color_mode(0), ColorMode::TrueColor);
	}

	#[test]
	#[serial]
	fn detect_color_mode_colorterm_env_is_other() {
		clear_env();
		set_var("COLORTERM", "other");
		assert_eq!(detect_color_mode(0), ColorMode::TwoTone);
	}

	#[test]
	#[serial]
	fn detect_color_mode_wt_session_env_iterm() {
		clear_env();
		// WT_SESSION is generally a GUID of some sort
		set_var("WT_SESSION", "32a25081-6745-4b65-909d-e8257bdbe852");
		assert_eq!(detect_color_mode(0), ColorMode::TrueColor);
	}
	#[rstest(
		red,
		green,
		blue,
		expected_index,
		case::black(0, 0, 0, 0),
		case::black(0, 0, 127, 0),
		case::black(0, 127, 0, 0),
		case::black(127, 0, 0, 0),
		case::black(127, 0, 127, 0),
		case::black(127, 127, 0, 0),
		case::black(127, 127, 127, 0),
		case::red(128, 0, 0, 1),
		case::red(255, 0, 0, 1),
		case::green(0, 128, 0, 2),
		case::green(0, 255, 0, 2),
		case::blue(0, 0, 128, 4),
		case::blue(0, 0, 255, 4),
		case::yellow(128, 128, 0, 3),
		case::yellow(128, 255, 0, 3),
		case::yellow(255, 255, 0, 3),
		case::cyan(0, 128, 128, 6),
		case::cyan(0, 128, 255, 6),
		case::cyan(0, 255, 255, 6),
		case::magenta(128, 0, 128, 5),
		case::magenta(128, 0, 255, 5),
		case::magenta(255, 0, 255, 5),
		case::white(128, 128, 128, 7),
		case::white(128, 128, 255, 7),
		case::white(128, 255, 128, 7),
		case::white(128, 255, 255, 7),
		case::white(155, 128, 128, 7),
		case::white(155, 128, 255, 7),
		case::white(155, 255, 128, 7),
		case::white(155, 255, 255, 7)
	)]
	fn find_color_three_bit_rgb(red: u8, green: u8, blue: u8, expected_index: u8) {
		let color = Color::Rgb { red, green, blue };
		assert_eq!(
			find_color(ColorMode::ThreeBit, color),
			CrosstermColor::AnsiValue(expected_index)
		);
	}

	#[rstest(
		color,
		expected,
		case::dark_black(Color::DarkBlack, CrosstermColor::Black),
		case::dark_blue(Color::DarkBlue, CrosstermColor::Blue),
		case::dark_cyan(Color::DarkCyan, CrosstermColor::Cyan),
		case::dark_green(Color::DarkGreen, CrosstermColor::Green),
		case::dark_magenta(Color::DarkMagenta, CrosstermColor::Magenta),
		case::dark_red(Color::DarkRed, CrosstermColor::Red),
		case::dark_white(Color::DarkWhite, CrosstermColor::White),
		case::dark_white(Color::DarkGrey, CrosstermColor::Grey),
		case::dark_yellow(Color::DarkYellow, CrosstermColor::Yellow),
		case::light_black(Color::LightBlack, CrosstermColor::DarkGrey),
		case::light_grey(Color::LightGrey, CrosstermColor::White),
		case::light_blue(Color::LightBlue, CrosstermColor::Blue),
		case::light_cyan(Color::LightCyan, CrosstermColor::Cyan),
		case::light_green(Color::LightGreen, CrosstermColor::Green),
		case::light_magenta(Color::LightMagenta, CrosstermColor::Magenta),
		case::light_red(Color::LightRed, CrosstermColor::Red),
		case::light_white(Color::LightWhite, CrosstermColor::White),
		case::light_yellow(Color::LightYellow, CrosstermColor::Yellow),
		case::default(Color::Default, CrosstermColor::Reset)
	)]
	fn find_color_three_bit_color(color: Color, expected: CrosstermColor) {
		assert_eq!(find_color(ColorMode::ThreeBit, color), expected);
	}

	#[rstest(
		red,
		green,
		blue,
		expected_index,
		case::black(0, 0, 0, 16),
		case::black(1, 1, 1, 16),
		case::black(4, 4, 4, 16),
		case::black(7, 7, 7, 16),
		case::grey(8, 8, 8, 232),
		case::grey(16, 16, 16, 232),
		case::grey(32, 32, 32, 234),
		case::grey(64, 64, 64, 237),
		case::grey(128, 128, 128, 244),
		case::grey(246, 246, 246, 255),
		case::white(247, 247, 247, 231),
		case::white(248, 248, 248, 231),
		case::white(253, 253, 253, 231),
		case::white(255, 255, 255, 231),
		case::base(255, 0, 0, 196),
		case::base(0, 255, 0, 46),
		case::base(0, 0, 255, 21),
		case::base(0, 255, 255, 51),
		case::base(255, 0, 255, 201),
		case::base(255, 255, 0, 226),
		case::sample(127, 0, 0, 88),
		case::sample(0, 127, 0, 28),
		case::sample(0, 0, 127, 18),
		case::sample(127, 0, 127, 90),
		case::sample(255, 95, 0, 208)
	)]
	fn find_color_four_bit_rgb(red: u8, green: u8, blue: u8, expected_index: u8) {
		let color = Color::Rgb { red, green, blue };
		assert_eq!(
			find_color(ColorMode::FourBit, color),
			CrosstermColor::AnsiValue(expected_index)
		);
	}

	#[rstest(
		color,
		expected,
		case::dark_black(Color::DarkBlack, CrosstermColor::Black),
		case::dark_blue(Color::DarkBlue, CrosstermColor::DarkBlue),
		case::dark_cyan(Color::DarkCyan, CrosstermColor::DarkCyan),
		case::dark_green(Color::DarkGreen, CrosstermColor::DarkGreen),
		case::dark_magenta(Color::DarkMagenta, CrosstermColor::DarkMagenta),
		case::dark_red(Color::DarkRed, CrosstermColor::DarkRed),
		case::dark_white(Color::DarkWhite, CrosstermColor::Grey),
		case::dark_white(Color::DarkGrey, CrosstermColor::DarkGrey),
		case::dark_yellow(Color::DarkYellow, CrosstermColor::DarkYellow),
		case::light_black(Color::LightBlack, CrosstermColor::DarkGrey),
		case::light_grey(Color::LightGrey, CrosstermColor::White),
		case::light_blue(Color::LightBlue, CrosstermColor::Blue),
		case::light_cyan(Color::LightCyan, CrosstermColor::Cyan),
		case::light_green(Color::LightGreen, CrosstermColor::Green),
		case::light_magenta(Color::LightMagenta, CrosstermColor::Magenta),
		case::light_red(Color::LightRed, CrosstermColor::Red),
		case::light_white(Color::LightWhite, CrosstermColor::White),
		case::light_yellow(Color::LightYellow, CrosstermColor::Yellow),
		case::default(Color::Default, CrosstermColor::Reset)
	)]
	fn find_color_four_bit_color(color: Color, expected: CrosstermColor) {
		assert_eq!(find_color(ColorMode::FourBit, color), expected);
	}

	#[rstest(
		red,
		green,
		blue,
		case::black(0, 0, 0),
		case::grey(128, 128, 128),
		case::white(255, 255, 255),
		case::base(255, 0, 0),
		case::base(0, 255, 0),
		case::base(0, 0, 255),
		case::base(0, 255, 255),
		case::base(255, 0, 255),
		case::base(255, 255, 0),
		case::sample(127, 0, 0),
		case::sample(0, 127, 0),
		case::sample(0, 0, 127),
		case::sample(127, 0, 127),
		case::sample(255, 95, 0)
	)]
	fn find_color_true_rgb(red: u8, green: u8, blue: u8) {
		let color = Color::Rgb { red, green, blue };
		assert_eq!(find_color(ColorMode::TrueColor, color), CrosstermColor::Rgb {
			r: red,
			g: green,
			b: blue
		});
	}

	#[test]
	fn action_register_selectable_color_pairs_true_color() {
		let (color, selected) = register_selectable_color_pairs(
			ColorMode::TrueColor,
			Color::LightRed,
			Color::LightYellow,
			Color::LightBlue,
		);
		assert_eq!(color, Colors::new(CrosstermColor::Red, CrosstermColor::Yellow));
		assert_eq!(selected, Colors::new(CrosstermColor::Red, CrosstermColor::Blue));
	}

	#[test]
	fn action_register_selectable_color_pairs_two_tone() {
		let (color, selected) = register_selectable_color_pairs(
			ColorMode::TwoTone,
			Color::LightRed,
			Color::LightYellow,
			Color::LightBlue,
		);
		assert_eq!(color, Colors::new(CrosstermColor::Red, CrosstermColor::Yellow));
		assert_eq!(selected, Colors::new(CrosstermColor::Red, CrosstermColor::Yellow));
	}
}
