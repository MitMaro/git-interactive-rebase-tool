use crate::color::Color;
use std::convert::TryFrom;
use std::env;

pub(in crate::config) fn get_input(config: &git2::Config, name: &str, default: &str) -> Result<String, String> {
	let value = get_string(config, name, default)?;

	match value.to_lowercase().as_ref() {
		"left" => Ok(String::from("Left")),
		"right" => Ok(String::from("Right")),
		"down" => Ok(String::from("Down")),
		"up" => Ok(String::from("Up")),
		"pageup" => Ok(String::from("PageUp")),
		"pagedown" => Ok(String::from("PageDown")),
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

pub(in crate::config) fn get_string(config: &git2::Config, name: &str, default: &str) -> Result<String, String> {
	match config.get_string(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(String::from(default)),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

pub(in crate::config) fn get_bool(config: &git2::Config, name: &str, default: bool) -> Result<bool, String> {
	match config.get_bool(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

pub(in crate::config) fn get_color(config: &git2::Config, name: &str, default_color: Color) -> Result<Color, String> {
	match config.get_string(name) {
		Ok(v) => Color::try_from(v.to_lowercase().as_str()),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default_color),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

pub(in crate::config) fn editor_from_env() -> String {
	env::var("VISUAL")
		.or_else(|_| env::var("EDITOR"))
		.unwrap_or_else(|_| String::from("vi"))
}

pub(in crate::config) fn open_git_config() -> Result<git2::Config, String> {
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
