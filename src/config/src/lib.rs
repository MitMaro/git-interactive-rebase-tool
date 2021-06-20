// Make rustc's built-in lints more strict and set clippy into a whitelist-based configuration
#![deny(
	warnings,
	nonstandard_style,
	unused,
	future_incompatible,
	rust_2018_idioms,
	unsafe_code
)]
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::as_conversions)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::implicit_return)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::wildcard_enum_match_arm)]

mod color;
mod diff_ignore_whitespace_setting;
mod diff_show_whitespace_setting;
mod git_config;
mod key_bindings;
pub mod testutil;
mod theme;
mod utils;

#[cfg(test)]
mod tests;

use anyhow::Result;

pub use self::{
	color::Color,
	diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting,
	diff_show_whitespace_setting::DiffShowWhitespaceSetting,
	key_bindings::KeyBindings,
	theme::Theme,
};
use self::{
	git_config::GitConfig,
	utils::{
		get_bool,
		get_diff_ignore_whitespace,
		get_diff_show_whitespace,
		get_string,
		get_unsigned_integer,
		open_git_config,
	},
};

#[derive(Clone, Debug)]
pub struct Config {
	pub auto_select_next: bool,
	pub diff_ignore_whitespace: DiffIgnoreWhitespaceSetting,
	pub diff_show_whitespace: DiffShowWhitespaceSetting,
	pub diff_space_symbol: String,
	pub diff_tab_symbol: String,
	pub diff_tab_width: u32,
	pub undo_limit: u32,
	pub git: GitConfig,
	pub key_bindings: KeyBindings,
	pub theme: Theme,
}

impl Config {
	#[inline]
	pub fn new() -> Result<Self> {
		let config = open_git_config().map_err(|e| e.context("Error loading git config"))?;
		Self::new_from_config(&config).map_err(|e| e.context("Error reading git config"))
	}

	fn new_from_config(git_config: &git2::Config) -> Result<Self> {
		Ok(Self {
			auto_select_next: get_bool(git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			diff_ignore_whitespace: get_diff_ignore_whitespace(git_config)?,
			diff_show_whitespace: get_diff_show_whitespace(git_config)?,
			diff_space_symbol: get_string(git_config, "interactive-rebase-tool.diffSpaceSymbol", "·")?,
			diff_tab_symbol: get_string(git_config, "interactive-rebase-tool.diffTabSymbol", "→")?,
			diff_tab_width: get_unsigned_integer(git_config, "interactive-rebase-tool.diffTabWidth", 4)?,
			undo_limit: get_unsigned_integer(git_config, "interactive-rebase-tool.undoLimit", 5000)?,
			git: GitConfig::new(git_config)?,
			key_bindings: KeyBindings::new(git_config)?,
			theme: Theme::new(git_config)?,
		})
	}
}
