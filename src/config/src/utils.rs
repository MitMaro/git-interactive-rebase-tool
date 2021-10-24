use std::env;

use anyhow::{anyhow, Result};
use git::{Config, ErrorCode};

use super::{
	diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting,
	diff_show_whitespace_setting::DiffShowWhitespaceSetting,
	Color,
};

pub(super) fn get_input(config: Option<&Config>, name: &str, default: &str) -> Result<Vec<String>> {
	let mut values = vec![];
	for mut value in get_string(config, name, default)?.split_whitespace().map(String::from) {
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
				_ => {
					if value.len() > 1 {
						// allow F{number} values
						if value.to_lowercase().starts_with('f') && value[1..].parse::<u8>().is_ok() {
							value.to_uppercase()
						}
						else {
							return Err(anyhow!("{} must contain only one character per binding", name));
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

pub(super) fn get_string(config: Option<&Config>, name: &str, default: &str) -> Result<String> {
	let cfg = match config {
		None => return Ok(String::from(default)),
		Some(c) => c,
	};
	match cfg.get_string(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == ErrorCode::NotFound => Ok(String::from(default)),
		Err(e) => Err(anyhow!(String::from(e.message()))),
	}
	.map_err(|e| e.context(anyhow!("\"{}\" is not valid", name)))
}

pub(super) fn get_bool(config: Option<&Config>, name: &str, default: bool) -> Result<bool> {
	let cfg = match config {
		None => return Ok(default),
		Some(c) => c,
	};
	match cfg.get_bool(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == ErrorCode::NotFound => Ok(default),
		Err(e) => Err(anyhow!(String::from(e.message()))),
	}
	.map_err(|e| e.context(anyhow!("\"{}\" is not valid", name)))
}

#[allow(clippy::map_err_ignore)]
pub(super) fn get_unsigned_integer(config: Option<&Config>, name: &str, default: u32) -> Result<u32> {
	let cfg = match config {
		None => return Ok(default),
		Some(c) => c,
	};
	match cfg.get_i32(name) {
		Ok(v) => {
			v.try_into()
				.map_err(|_| anyhow!("\"{}\" is outside of valid range for an unsigned 32-bit integer", v))
		},
		Err(ref e) if e.code() == ErrorCode::NotFound => Ok(default),
		Err(e) => Err(anyhow!(String::from(e.message()))),
	}
	.map_err(|e| e.context(anyhow!("\"{}\" is not valid", name)))
}

pub(super) fn get_color(config: Option<&Config>, name: &str, default: Color) -> Result<Color> {
	let cfg = match config {
		None => return Ok(default),
		Some(c) => c,
	};
	match cfg.get_string(name) {
		Ok(v) => Color::try_from(v.to_lowercase().as_str()),
		Err(ref e) if e.code() == ErrorCode::NotFound => Ok(default),
		Err(e) => Err(anyhow!(String::from(e.message()))),
	}
	.map_err(|e| e.context(anyhow!("\"{}\" is not valid", name)))
}

pub(super) fn editor_from_env() -> String {
	env::var("VISUAL")
		.or_else(|_| env::var("EDITOR"))
		.unwrap_or_else(|_| String::from("vi"))
}

pub(super) fn get_diff_show_whitespace(git_config: Option<&Config>) -> Result<DiffShowWhitespaceSetting> {
	let diff_show_whitespace = get_string(git_config, "interactive-rebase-tool.diffShowWhitespace", "both")?;

	match diff_show_whitespace.to_lowercase().as_str() {
		"true" | "on" | "both" => Ok(DiffShowWhitespaceSetting::Both),
		"trailing" => Ok(DiffShowWhitespaceSetting::Trailing),
		"leading" => Ok(DiffShowWhitespaceSetting::Leading),
		"false" | "off" | "none" => Ok(DiffShowWhitespaceSetting::None),
		_ => {
			Err(anyhow!(
				"\"{}\" does not match one of \"true\", \"on\", \"both\", \"trailing\", \"leading\", \"false\", \
				 \"off\" or \"none\"",
				diff_show_whitespace
			)
			.context("\"interactive-rebase-tool.diffShowWhitespace\" is not valid"))
		},
	}
}

pub(super) fn get_diff_ignore_whitespace(git_config: Option<&Config>) -> Result<DiffIgnoreWhitespaceSetting> {
	let diff_ignore_whitespace = get_string(git_config, "interactive-rebase-tool.diffIgnoreWhitespace", "none")?;

	match diff_ignore_whitespace.to_lowercase().as_str() {
		"true" | "on" | "all" => Ok(DiffIgnoreWhitespaceSetting::All),
		"change" => Ok(DiffIgnoreWhitespaceSetting::Change),
		"false" | "off" | "none" => Ok(DiffIgnoreWhitespaceSetting::None),
		_ => {
			Err(anyhow!(
				"\"{}\" does not match one of \"true\", \"on\", \"all\", \"change\", \"false\", \"off\" or \"none\"",
				diff_ignore_whitespace
			)
			.context("\"interactive-rebase-tool.diffIgnoreWhitespace\" is not valid"))
		},
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
