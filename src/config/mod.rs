pub(crate) mod git_config;
pub(crate) mod key_bindings;
pub(crate) mod theme;
mod utils;

use crate::config::git_config::GitConfig;
use crate::config::key_bindings::KeyBindings;
use crate::config::theme::Theme;
use crate::config::utils::{get_bool, open_git_config};

#[derive(Clone, Debug)]
pub(crate) struct Config {
	pub(crate) auto_select_next: bool,
	pub(crate) git: GitConfig,
	pub(crate) key_bindings: KeyBindings,
	pub(crate) theme: Theme,
}

impl Config {
	pub(crate) fn new() -> Result<Self, String> {
		let git_config = open_git_config()?;

		Ok(Config {
			auto_select_next: get_bool(&git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			git: GitConfig::new(&git_config)?,
			key_bindings: KeyBindings::new(&git_config)?,
			theme: Theme::new(&git_config)?,
		})
	}
}
