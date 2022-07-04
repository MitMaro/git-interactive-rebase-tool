use std::env;

use git::{Config, ErrorCode};

use super::{
	diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting,
	diff_show_whitespace_setting::DiffShowWhitespaceSetting,
	Color,
};
use crate::errors::{ConfigError, ConfigErrorCause};

fn _get_string(config: Option<&Config>, name: &str) -> Result<Option<String>, ConfigError> {
	let cfg = match config {
		None => return Ok(None),
		Some(c) => c,
	};
	match cfg.get_string(name) {
		Ok(v) => Ok(Some(v)),
		Err(e) if e.code() == ErrorCode::NotFound => Ok(None),
		// detecting a UTF-8 error is tricky
		Err(e) if e.message() == "configuration value is not valid utf8" => {
			Err(ConfigError::new_read_error(name, ConfigErrorCause::InvalidUtf))
		},
		Err(e) => {
			Err(ConfigError::new_read_error(
				name,
				ConfigErrorCause::UnknownError(e.to_string()),
			))
		},
	}
}

#[allow(clippy::string_slice, clippy::indexing_slicing)]
pub(super) fn get_input(config: Option<&Config>, name: &str, default: &str) -> Result<Vec<String>, ConfigError> {
	let mut values = vec![];
	let input = get_string(config, name, default)?;
	for mut value in input.split_whitespace().map(String::from) {
		let mut modifiers = vec![];

		if let Some(index) = value.to_lowercase().find("shift+") {
			modifiers.push("Shift");
			value.replace_range(index..index + 6, "");
		}
		if let Some(index) = value.to_lowercase().find("control+") {
			modifiers.push("Control");
			value.replace_range(index..index + 8, "");
		}
		if let Some(index) = value.to_lowercase().find("alt+") {
			modifiers.push("Alt");
			value.replace_range(index..index + 4, "");
		}

		values.push(format!(
			"{}{}",
			modifiers.join(""),
			match value.to_lowercase().as_ref() {
				"backspace" => String::from("Backspace"),
				"backtab" => String::from("BackTab"),
				"delete" => String::from("Delete"),
				"down" => String::from("Down"),
				"end" => String::from("End"),
				"enter" => String::from("Enter"),
				"esc" => String::from("Esc"),
				"home" => String::from("Home"),
				"insert" => String::from("Insert"),
				"left" => String::from("Left"),
				"pagedown" => String::from("PageDown"),
				"pageup" => String::from("PageUp"),
				"right" => String::from("Right"),
				"tab" => String::from("Tab"),
				"up" => String::from("Up"),
				v => {
					if v.len() > 1 {
						// allow F{number} values
						if v.starts_with('f') && v[1..].parse::<u8>().is_ok() {
							v.to_uppercase()
						}
						else {
							return Err(ConfigError::new(
								name,
								input.as_str(),
								ConfigErrorCause::InvalidKeyBinding,
							));
						}
					}
					else {
						value
					}
				},
			}
		));
	}
	Ok(values)
}

pub(super) fn get_string(config: Option<&Config>, name: &str, default: &str) -> Result<String, ConfigError> {
	Ok(_get_string(config, name)?.unwrap_or_else(|| String::from(default)))
}

pub(super) fn get_bool(config: Option<&Config>, name: &str, default: bool) -> Result<bool, ConfigError> {
	if let Some(cfg) = config {
		match cfg.get_bool(name) {
			Ok(v) => Ok(v),
			Err(e) if e.code() == ErrorCode::NotFound => Ok(default),
			Err(e) if e.message().contains("failed to parse") => {
				Err(ConfigError::new_with_optional_input(
					name,
					_get_string(config, name).ok().flatten(),
					ConfigErrorCause::InvalidBoolean,
				))
			},
			Err(e) => {
				Err(ConfigError::new_with_optional_input(
					name,
					_get_string(config, name).ok().flatten(),
					ConfigErrorCause::UnknownError(e.to_string()),
				))
			},
		}
	}
	else {
		Ok(default)
	}
}

#[allow(clippy::map_err_ignore)]
pub(super) fn get_unsigned_integer(config: Option<&Config>, name: &str, default: u32) -> Result<u32, ConfigError> {
	if let Some(cfg) = config {
		// TODO check for overflow of i32 value
		match cfg.get_i32(name) {
			Ok(v) => {
				v.try_into().map_err(|_| {
					ConfigError::new_with_optional_input(
						name,
						_get_string(config, name).ok().flatten(),
						ConfigErrorCause::InvalidUnsignedInteger,
					)
				})
			},
			Err(e) if e.code() == ErrorCode::NotFound => Ok(default),
			Err(e) if e.message().contains("failed to parse") => {
				Err(ConfigError::new_with_optional_input(
					name,
					_get_string(config, name).ok().flatten(),
					ConfigErrorCause::InvalidUnsignedInteger,
				))
			},
			Err(e) => {
				Err(ConfigError::new_with_optional_input(
					name,
					_get_string(config, name).ok().flatten(),
					ConfigErrorCause::UnknownError(e.to_string()),
				))
			},
		}
	}
	else {
		Ok(default)
	}
}

pub(super) fn get_color(config: Option<&Config>, name: &str, default: Color) -> Result<Color, ConfigError> {
	if let Some(value) = _get_string(config, name)? {
		Color::try_from(value.to_lowercase().as_str()).map_err(|invalid_color_error| {
			ConfigError::new(
				name,
				value.as_str(),
				ConfigErrorCause::InvalidColor(invalid_color_error),
			)
		})
	}
	else {
		Ok(default)
	}
}

pub(super) fn editor_from_env() -> String {
	env::var("VISUAL")
		.or_else(|_| env::var("EDITOR"))
		.unwrap_or_else(|_| String::from("vi"))
}

pub(super) fn get_diff_show_whitespace(
	git_config: Option<&Config>,
	name: &str,
) -> Result<DiffShowWhitespaceSetting, ConfigError> {
	match get_string(git_config, name, "both")?.to_lowercase().as_str() {
		"true" | "on" | "both" => Ok(DiffShowWhitespaceSetting::Both),
		"trailing" => Ok(DiffShowWhitespaceSetting::Trailing),
		"leading" => Ok(DiffShowWhitespaceSetting::Leading),
		"false" | "off" | "none" => Ok(DiffShowWhitespaceSetting::None),
		input => Err(ConfigError::new(name, input, ConfigErrorCause::InvalidShowWhitespace)),
	}
}

pub(super) fn get_diff_ignore_whitespace(
	git_config: Option<&Config>,
	name: &str,
) -> Result<DiffIgnoreWhitespaceSetting, ConfigError> {
	match get_string(git_config, name, "none")?.to_lowercase().as_str() {
		"true" | "on" | "all" => Ok(DiffIgnoreWhitespaceSetting::All),
		"change" => Ok(DiffIgnoreWhitespaceSetting::Change),
		"false" | "off" | "none" => Ok(DiffIgnoreWhitespaceSetting::None),
		input => {
			Err(ConfigError::new(
				name,
				input,
				ConfigErrorCause::InvalidDiffIgnoreWhitespace,
			))
		},
	}
}

pub(super) fn git_diff_renames(git_config: Option<&Config>, name: &str) -> Result<(bool, bool), ConfigError> {
	match get_string(git_config, name, "true")?.to_lowercase().as_str() {
		"true" => Ok((true, false)),
		"false" => Ok((false, false)),
		"copy" | "copies" => Ok((true, true)),
		input => Err(ConfigError::new(name, input, ConfigErrorCause::InvalidDiffRenames)),
	}
}

pub(super) fn map_single_ascii_to_lower(s: &str) -> String {
	if s.is_ascii() && s.len() == 1 {
		s.to_lowercase()
	}
	else {
		String::from(s)
	}
}
