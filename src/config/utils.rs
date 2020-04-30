use crate::display::color::Color;
use std::convert::TryFrom;
use std::env;

pub(super) fn get_input(config: &git2::Config, name: &str, default: &str) -> Result<String, String> {
	let value = get_string(config, name, default)?;

	match value.to_lowercase().as_ref() {
		"backspace" => Ok(String::from("Backspace")),
		"delete" => Ok(String::from("Delete")),
		"down" => Ok(String::from("Down")),
		"end" => Ok(String::from("End")),
		"enter" => Ok(String::from("Enter")),
		"f0" => Ok(String::from("F0")),
		"f1" => Ok(String::from("F1")),
		"f2" => Ok(String::from("F2")),
		"f3" => Ok(String::from("F3")),
		"f4" => Ok(String::from("F4")),
		"f5" => Ok(String::from("F5")),
		"f6" => Ok(String::from("F6")),
		"f7" => Ok(String::from("F7")),
		"f8" => Ok(String::from("F8")),
		"f9" => Ok(String::from("F9")),
		"f10" => Ok(String::from("F10")),
		"f11" => Ok(String::from("F11")),
		"f12" => Ok(String::from("F12")),
		"f13" => Ok(String::from("F13")),
		"f14" => Ok(String::from("F14")),
		"f15" => Ok(String::from("F15")),
		"home" => Ok(String::from("Home")),
		"insert" => Ok(String::from("Insert")),
		"left" => Ok(String::from("Left")),
		"pagedown" => Ok(String::from("PageDown")),
		"pageup" => Ok(String::from("PageUp")),
		"right" => Ok(String::from("Right")),
		"shift+delete" => Ok(String::from("ShiftDelete")),
		"shift+down" => Ok(String::from("ShiftDown")),
		"shift+end" => Ok(String::from("ShiftEnd")),
		"shift+home" => Ok(String::from("ShiftHome")),
		"shift+left" => Ok(String::from("ShiftLeft")),
		"shift+right" => Ok(String::from("ShiftRight")),
		"shift+tab" => Ok(String::from("ShiftTab")),
		"shift+up" => Ok(String::from("ShiftUp")),
		"tab" => Ok(String::from("Tab")),
		"up" => Ok(String::from("Up")),
		_ => {
			if value.len() > 1 {
				return Err(format!(
					"Error reading git config: {} must contain only one character",
					name
				));
			}
			Ok(value)
		},
	}
}

pub(super) fn get_string(config: &git2::Config, name: &str, default: &str) -> Result<String, String> {
	match config.get_string(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(String::from(default)),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

pub(super) fn get_bool(config: &git2::Config, name: &str, default: bool) -> Result<bool, String> {
	match config.get_bool(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

pub(super) fn get_color(config: &git2::Config, name: &str, default_color: Color) -> Result<Color, String> {
	match config.get_string(name) {
		Ok(v) => Color::try_from(v.to_lowercase().as_str()),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default_color),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

pub(super) fn editor_from_env() -> String {
	env::var("VISUAL")
		.or_else(|_| env::var("EDITOR"))
		.unwrap_or_else(|_| String::from("vi"))
}

pub(super) fn open_git_config() -> Result<git2::Config, String> {
	match git2::Repository::open_from_env() {
		Ok(f) => {
			match f.config() {
				Ok(c) => Ok(c),
				Err(e) => Err(format!("Error reading git config: {}", e)),
			}
		},
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}
