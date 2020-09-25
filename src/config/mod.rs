pub mod diff_ignore_whitespace_setting;
pub mod diff_show_whitespace_setting;
pub mod git_config;
pub mod key_bindings;
pub mod theme;
mod utils;
use anyhow::Result;

#[cfg(test)]
mod tests;

use crate::config::diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting;
use crate::config::diff_show_whitespace_setting::DiffShowWhitespaceSetting;
use crate::config::git_config::GitConfig;
use crate::config::key_bindings::KeyBindings;
use crate::config::theme::Theme;
use crate::config::utils::{
	get_bool,
	get_diff_ignore_whitespace,
	get_diff_show_whitespace,
	get_string,
	get_unsigned_integer,
	open_git_config,
};

#[derive(Clone, Debug)]
pub struct Config {
	pub(crate) auto_select_next: bool,
	pub(crate) diff_ignore_whitespace: DiffIgnoreWhitespaceSetting,
	pub(crate) diff_show_whitespace: DiffShowWhitespaceSetting,
	pub(crate) diff_tab_width: u32,
	pub(crate) diff_tab_symbol: String,
	pub(crate) diff_space_symbol: String,
	pub(crate) git: GitConfig,
	pub(crate) key_bindings: KeyBindings,
	pub(crate) theme: Theme,
}

impl Config {
	pub(crate) fn new() -> Result<Self> {
		let config = open_git_config().map_err(|e| e.context("Error loading git config"))?;
		Self::new_from_config(&config).map_err(|e| e.context("Error reading git config"))
	}

	fn new_from_config(git_config: &git2::Config) -> Result<Self> {
		Ok(Self {
			auto_select_next: get_bool(git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			diff_ignore_whitespace: get_diff_ignore_whitespace(git_config)?,
			diff_show_whitespace: get_diff_show_whitespace(git_config)?,
			diff_tab_width: get_unsigned_integer(git_config, "interactive-rebase-tool.diffTabWidth", 4)?,
			diff_tab_symbol: get_string(git_config, "interactive-rebase-tool.diffTabSymbol", "→")?,
			diff_space_symbol: get_string(git_config, "interactive-rebase-tool.diffSpaceSymbol", "·")?,
			git: GitConfig::new(git_config)?,
			key_bindings: KeyBindings::new(git_config)?,
			theme: Theme::new(git_config)?,
		})
	}
}
