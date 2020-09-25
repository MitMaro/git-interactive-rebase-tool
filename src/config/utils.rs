use crate::config::diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting;
use crate::config::diff_show_whitespace_setting::DiffShowWhitespaceSetting;
use crate::display::color::Color;
use anyhow::{anyhow, Result};
use git2::Config;
use std::convert::{TryFrom, TryInto};
use std::env;

pub(super) fn get_input(config: &Config, name: &str, default: &str) -> Result<String> {
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
				return Err(anyhow!("{} must contain only one character", name).context("Error reading git config"));
			}
			Ok(value)
		},
	}
}

pub(super) fn get_string(config: &Config, name: &str, default: &str) -> Result<String> {
	match config.get_string(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(String::from(default)),
		Err(e) => Err(anyhow!(String::from(e.message()))),
	}
	.map_err(|e| e.context(anyhow!("\"{}\" is not valid", name)))
}

pub(super) fn get_bool(config: &Config, name: &str, default: bool) -> Result<bool> {
	match config.get_bool(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default),
		Err(e) => Err(anyhow!(String::from(e.message()))),
	}
	.map_err(|e| e.context(anyhow!("\"{}\" is not valid", name)))
}

pub(super) fn get_unsigned_integer(config: &Config, name: &str, default: u32) -> Result<u32> {
	match config.get_i32(name) {
		Ok(v) => {
			v.try_into()
				.map_err(|_| anyhow!("\"{}\" is outside of valid range for an unsigned 32-bit integer", v))
		},
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default),
		Err(e) => Err(anyhow!(String::from(e.message()))),
	}
	.map_err(|e| e.context(anyhow!("\"{}\" is not valid", name)))
}

pub(super) fn get_color(config: &Config, name: &str, default_color: Color) -> Result<Color> {
	match config.get_string(name) {
		Ok(v) => Color::try_from(v.to_lowercase().as_str()),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default_color),
		Err(e) => Err(anyhow!(String::from(e.message()))),
	}
	.map_err(|e| e.context(anyhow!("\"{}\" is not valid", name)))
}

pub(super) fn editor_from_env() -> String {
	env::var("VISUAL")
		.or_else(|_| env::var("EDITOR"))
		.unwrap_or_else(|_| String::from("vi"))
}

pub(super) fn open_git_config() -> Result<Config> {
	git2::Repository::open_from_env()
		.map_err(|e| anyhow!(String::from(e.message())))?
		.config()
		.map_err(|e| anyhow!(String::from(e.message())))
}

pub(super) fn get_diff_show_whitespace(git_config: &Config) -> Result<DiffShowWhitespaceSetting> {
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

pub(super) fn get_diff_ignore_whitespace(git_config: &Config) -> Result<DiffIgnoreWhitespaceSetting> {
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
