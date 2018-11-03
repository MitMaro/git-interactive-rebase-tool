use std::ffi::OsString;
use std::env;
use git2;

#[derive(Debug)]
pub struct GitConfig {
	pub comment_char: String,
	pub foreground_color: String,
	pub indicator_color: String,
	pub error_color: String,
	pub diff_add_color: String,
	pub diff_remove_color: String,
	pub pick_color: String,
	pub reword_color: String,
	pub edit_color: String,
	pub exec_color: String,
	pub squash_color: String,
	pub fixup_color: String,
	pub drop_color: String,
	pub auto_select_next: bool,
	pub editor: OsString,
}

impl GitConfig {
	pub fn new() -> Result<Self, git2::Error> {
		let config = git2::Repository::open_from_env()?.config()?;
		Ok(GitConfig {
			comment_char: get_string(&config, "core.commentChar")?.unwrap_or_else(|| String::from("#")),
			foreground_color: get_color(&config, "interactive-rebase-tool.foregroundColor")?,
			indicator_color: get_color(&config, "interactive-rebase-tool.indicatorColor")?,
			error_color: get_color(&config, "interactive-rebase-tool.errorColor")?,
			diff_add_color: get_color(&config, "interactive-rebase-tool.diffAddColor")?,
			diff_remove_color: get_color(&config, "interactive-rebase-tool.diffRemoveColor")?,
			pick_color: get_color(&config, "interactive-rebase-tool.pickColor")?,
			reword_color: get_color(&config, "interactive-rebase-tool.rewordColor")?,
			edit_color: get_color(&config, "interactive-rebase-tool.editColor")?,
			exec_color: get_color(&config, "interactive-rebase-tool.execColor")?,
			squash_color: get_color(&config, "interactive-rebase-tool.squashColor")?,
			fixup_color: get_color(&config, "interactive-rebase-tool.fixupColor")?,
			drop_color: get_color(&config, "interactive-rebase-tool.dropColor")?,
			auto_select_next: match config.get_bool("interactive-rebase-tool.autoSelectNext") {
				Ok(auto_select_next_value) => auto_select_next_value,
				Err(_msg) => false
			},
			editor: get_string(&config, "core.editor")?.map_or_else(
				editor_from_env, OsString::from),
		})
	}
}

fn get_string(config: &git2::Config, name: &str) -> Result<Option<String>, git2::Error> {
	match config.get_string(name) {
		Ok(v) => Ok(Some(v)),
		Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
		Err(e) => Err(e)
	}
}

fn get_color(config: &git2::Config, name: &str) -> Result<String, git2::Error> {
	// TODO: could make colors in GitConfig an Option<String> and move this to config.rs
	Ok(get_string(config, name)?.map(|s| s.to_lowercase())
		.unwrap_or_default())
}

fn editor_from_env() -> OsString {
	env::var_os("VISUAL")
		.or_else(|| env::var_os("EDITOR"))
		.unwrap_or_else(|| OsString::from("vi"))
}
