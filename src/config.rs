use color::Color;
use std::ffi::OsString;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
	pub comment_char: String,
	pub foreground_color: Color,
	pub indicator_color: Color,
	pub error_color: Color,
	pub diff_add_color: Color,
	pub diff_remove_color: Color,
	pub pick_color: Color,
	pub reword_color: Color,
	pub edit_color: Color,
	pub exec_color: Color,
	pub squash_color: Color,
	pub fixup_color: Color,
	pub drop_color: Color,
	pub auto_select_next: bool,
	pub editor: OsString,
}

fn get_string(config: &git2::Config, name: &str, default: &str) -> Result<String, String> {
	match config.get_string(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(String::from(default)),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

fn get_os_string(config: &git2::Config, name: &str, default: OsString) -> Result<OsString, String> {
	match config.get_string(name) {
		Ok(v) => Ok(OsString::from(v)),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

fn get_bool(config: &git2::Config, name: &str, default: bool) -> Result<bool, String> {
	match config.get_bool(name) {
		Ok(v) => Ok(v),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

fn get_color(config: &git2::Config, name: &str, default_color: Color) -> Result<Color, String> {
	match config.get_string(name) {
		Ok(v) => Color::try_from(v.to_lowercase().as_str()),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(default_color),
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

fn editor_from_env() -> OsString {
	env::var_os("VISUAL")
		.or_else(|| env::var_os("EDITOR"))
		.unwrap_or_else(|| OsString::from("vi"))
}

fn open_git_config() -> Result<git2::Config, String> {
	match git2::Repository::open_from_env() {
		Ok(f) => {
			match f.config() {
				Ok(c) => Ok(c),
				Err(e) => Err(format!("Error reading git config: {}", e))
			}
		},
		Err(e) => Err(format!("Error reading git config: {}", e)),
	}
}

impl Config {
	pub fn new() -> Result<Self, String> {
		let git_config = open_git_config()?;
		Ok(Config {
			comment_char: get_string(&git_config, "core.commentChar", "#")?,
			foreground_color: get_color(&git_config, "interactive-rebase-tool.foregroundColor", Color::White)?,
			indicator_color: get_color(&git_config, "interactive-rebase-tool.indicatorColor", Color::Cyan)?,
			error_color: get_color(&git_config, "interactive-rebase-tool.errorColor", Color::Red)?,
			diff_add_color: get_color(&git_config, "interactive-rebase-tool.diffAddColor", Color::Green)?,
			diff_remove_color: get_color(&git_config, "interactive-rebase-tool.diffRemoveColor", Color::Red)?,
			pick_color: get_color(&git_config, "interactive-rebase-tool.pickColor", Color::Green)?,
			reword_color: get_color(&git_config, "interactive-rebase-tool.rewordColor", Color::Yellow)?,
			edit_color: get_color(&git_config, "interactive-rebase-tool.editColor", Color::Blue)?,
			exec_color: get_color(&git_config, "interactive-rebase-tool.execColor", Color::White)?,
			squash_color: get_color(&git_config, "interactive-rebase-tool.squashColor", Color::Cyan)?,
			fixup_color: get_color(&git_config, "interactive-rebase-tool.fixupColor", Color::Magenta)?,
			drop_color: get_color(&git_config, "interactive-rebase-tool.dropColor", Color::Red)?,
			auto_select_next: get_bool(&git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			editor: get_os_string(&git_config, "core.editor", editor_from_env())?,
		})
	}
}
