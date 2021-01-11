use super::color_mode::ColorMode;
use std::env::var;

pub(super) fn detect_color_mode(number_of_colors: i16) -> ColorMode {
	// assume 16 colors on Windows
	if cfg!(windows) {
		// TODO Windows 10 build 14931 and higher support true color
		// TODO Windows 10 build 10586 and higher support 256 colors
		return ColorMode::ThreeBit;
	}

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

	// Apple has some special cases
	if let Ok(term_program) = var("TERM_PROGRAM") {
		// Apple Terminal sometimes pretends to support TrueColor, but it's 8bit
		// TODO iTerm does support truecolor, but does not support the way that colors are set
		if term_program == "Apple_Terminal" || term_program == "iTerm.app" {
			return ColorMode::EightBit;
		}
	}

	// Assume terminals with `-256` are 8bit, this is technically what curses does internally
	if let Ok(term) = var("TERM") {
		if term.contains("-256") {
			return ColorMode::EightBit;
		}
	}

	// at this point there is no way to detect truecolor support, so the best we can get is 8bit
	match number_of_colors {
		n if n >= 256 => ColorMode::EightBit,
		n if n >= 16 => ColorMode::FourBit,
		n if n >= 8 => ColorMode::ThreeBit,
		_ => ColorMode::TwoTone,
	}
}

#[cfg(all(windows, test))]
mod tests {
	use super::*;

	#[test]
	fn detect_color_mode_windows() {
		assert_eq!(detect_color_mode(2), ColorMode::ThreeBit);
	}
}

#[cfg(all(unix, test))]
mod tests {
	use super::*;
	use serial_test::serial;
	use std::env::{remove_var, set_var};

	fn clear_env() {
		remove_var("COLORTERM");
		remove_var("VTE_VERSION");
		remove_var("TERM_PROGRAM");
		remove_var("TERM");
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
	fn detect_color_mode_term_program_env_apple_terminal() {
		clear_env();
		set_var("TERM_PROGRAM", "Apple_Terminal");
		assert_eq!(detect_color_mode(0), ColorMode::EightBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_term_program_env_iterm() {
		clear_env();
		set_var("TERM_PROGRAM", "iTerm.app");
		assert_eq!(detect_color_mode(0), ColorMode::EightBit);
	}

	#[test]
	#[serial]
	fn detect_color_mode_term_program_env_other() {
		clear_env();
		set_var("TERM_PROGRAM", "other");
		assert_eq!(detect_color_mode(0), ColorMode::TwoTone);
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
}
