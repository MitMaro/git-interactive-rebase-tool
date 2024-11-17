//! Git Interactive Rebase Tool - Configuration Module
//!
//! # Description
//! This module is used to handle the loading of configuration from the Git config system.
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance should only be used in test code.
mod color;
mod diff_ignore_whitespace_setting;
mod diff_show_whitespace_setting;
mod errors;
mod git_config;
mod key_bindings;
mod theme;
mod utils;

use self::utils::{get_bool, get_diff_ignore_whitespace, get_diff_show_whitespace, get_string, get_unsigned_integer};
pub(crate) use self::{
	color::Color,
	diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting,
	diff_show_whitespace_setting::DiffShowWhitespaceSetting,
	git_config::GitConfig,
	key_bindings::KeyBindings,
	theme::Theme,
};
use crate::{
	config::{
		errors::{ConfigError, ConfigErrorCause, InvalidColorError},
		utils::get_optional_string,
	},
	git::Repository,
};

const DEFAULT_SPACE_SYMBOL: &str = "\u{b7}"; // ·
const DEFAULT_TAB_SYMBOL: &str = "\u{2192}"; // →

/// Represents the configuration options.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub(crate) struct Config {
	/// If to select the next line in the list after performing an action.
	pub auto_select_next: bool,
	/// How to handle whitespace when calculating diffs.
	pub diff_ignore_whitespace: DiffIgnoreWhitespaceSetting,
	/// If to ignore blank lines when calculating diffs.
	pub diff_ignore_blank_lines: bool,
	/// How to show whitespace in diffs.
	pub diff_show_whitespace: DiffShowWhitespaceSetting,
	/// The symbol used to replace space characters.
	pub diff_space_symbol: String,
	/// The symbol used to replace tab characters.
	pub diff_tab_symbol: String,
	/// The display width of the tab character.
	pub diff_tab_width: u32,
	/// If set, automatically add an exec line with the command after every modified line
	pub post_modified_line_exec_command: Option<String>,
	/// The maximum number of undo steps.
	pub undo_limit: u32,
	/// Configuration options loaded directly from Git.
	pub git: GitConfig,
	/// Key binding configuration.
	pub key_bindings: KeyBindings,
	/// Theme configuration.
	pub theme: Theme,
}

impl Config {
	pub(crate) fn new_with_config(git_config: Option<&crate::git::Config>) -> Result<Self, ConfigError> {
		Ok(Self {
			auto_select_next: get_bool(git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			diff_ignore_whitespace: get_diff_ignore_whitespace(
				git_config,
				"interactive-rebase-tool.diffIgnoreWhitespace",
			)?,
			diff_ignore_blank_lines: get_bool(git_config, "interactive-rebase-tool.diffIgnoreBlankLines", false)?,
			diff_show_whitespace: get_diff_show_whitespace(git_config, "interactive-rebase-tool.diffShowWhitespace")?,
			diff_space_symbol: get_string(
				git_config,
				"interactive-rebase-tool.diffSpaceSymbol",
				DEFAULT_SPACE_SYMBOL,
			)?,
			diff_tab_symbol: get_string(git_config, "interactive-rebase-tool.diffTabSymbol", DEFAULT_TAB_SYMBOL)?,
			diff_tab_width: get_unsigned_integer(git_config, "interactive-rebase-tool.diffTabWidth", 4)?,
			undo_limit: get_unsigned_integer(git_config, "interactive-rebase-tool.undoLimit", 5000)?,
			post_modified_line_exec_command: get_optional_string(
				git_config,
				"interactive-rebase-tool.postModifiedLineExecCommand",
			)?,
			git: GitConfig::new_with_config(git_config)?,
			key_bindings: KeyBindings::new_with_config(git_config)?,
			theme: Theme::new_with_config(git_config)?,
		})
	}
}

impl TryFrom<&Repository> for Config {
	type Error = ConfigError;

	/// Creates a new Config instance loading the Git Config using [`crate::git::Repository`].
	///
	/// # Errors
	///
	/// Will return an `Err` if there is a problem loading the configuration.
	fn try_from(repo: &Repository) -> Result<Self, Self::Error> {
		let config = repo
			.load_config()
			.map_err(|e| ConfigError::new_read_error("", ConfigErrorCause::GitError(e)))?;
		Self::new_with_config(Some(&config))
	}
}

impl TryFrom<&crate::git::Config> for Config {
	type Error = ConfigError;

	fn try_from(config: &crate::git::Config) -> Result<Self, Self::Error> {
		Self::new_with_config(Some(config))
	}
}

#[cfg(test)]
mod tests {
	use std::fmt::Debug;

	use claims::{assert_err_eq, assert_ok};
	use rstest::rstest;

	use super::*;
	use crate::test_helpers::{invalid_utf, with_git_config, with_temp_bare_repository};

	#[test]
	fn try_from_repository() {
		with_temp_bare_repository(|repository| {
			assert_ok!(Config::try_from(&repository));
		});
	}

	#[test]
	fn try_from_git_config() {
		with_git_config(&[], |git_config| {
			assert_ok!(Config::try_from(&git_config));
		});
	}

	#[test]
	fn try_from_git_config_error() {
		with_git_config(
			&["[interactive-rebase-tool]", "autoSelectNext = invalid"],
			|git_config| {
				_ = Config::try_from(&git_config).unwrap_err();
			},
		);
	}

	#[rstest]
	#[case::auto_select_next_default("autoSelectNext", "", false, |config: Config| config.auto_select_next)]
	#[case::auto_select_next_false("autoSelectNext", "false", false, |config: Config| config.auto_select_next)]
	#[case::auto_select_next_true("autoSelectNext", "true", true, |config: Config| config.auto_select_next)]
	#[case::diff_ignore_whitespace_default(
		"diffIgnoreWhitespace",
		"",
		DiffIgnoreWhitespaceSetting::None,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_whitespace_true(
		"diffIgnoreWhitespace",
		"true",
		DiffIgnoreWhitespaceSetting::All,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_whitespace_on(
		"diffIgnoreWhitespace",
		"on",
		DiffIgnoreWhitespaceSetting::All,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_whitespace_all(
		"diffIgnoreWhitespace",
		"all",
		DiffIgnoreWhitespaceSetting::All,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_whitespace_change(
		"diffIgnoreWhitespace",
		"change",
		DiffIgnoreWhitespaceSetting::Change,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_whitespace_false(
		"diffIgnoreWhitespace",
		"false",
		DiffIgnoreWhitespaceSetting::None,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_whitespace_off(
		"diffIgnoreWhitespace",
		"off",
		DiffIgnoreWhitespaceSetting::None,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_whitespace_none(
		"diffIgnoreWhitespace",
		"none",
		DiffIgnoreWhitespaceSetting::None,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_whitespace_mixed_case(
		"diffIgnoreWhitespace",
		"ChAnGe",
		DiffIgnoreWhitespaceSetting::Change,
		|config: Config| config.diff_ignore_whitespace)
	]
	#[case::diff_ignore_blank_lines_default(
		"diffIgnoreBlankLines",
		"",
		false,
		|config: Config| config.diff_ignore_blank_lines
	)]
	#[case::diff_ignore_blank_lines_false(
		"diffIgnoreBlankLines",
		"false",
		false,
		|config: Config| config.diff_ignore_blank_lines
	)]
	#[case::diff_ignore_blank_lines_true(
		"diffIgnoreBlankLines",
		"true",
		true,
		|config: Config| config.diff_ignore_blank_lines
	)]
	#[case::diff_show_whitespace_default(
		"diffShowWhitespace",
		"",
		DiffShowWhitespaceSetting::Both,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_true(
		"diffShowWhitespace",
		"true",
		DiffShowWhitespaceSetting::Both,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_on(
		"diffShowWhitespace",
		"on",
		DiffShowWhitespaceSetting::Both,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_both(
		"diffShowWhitespace",
		"both",
		DiffShowWhitespaceSetting::Both,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_trailing(
		"diffShowWhitespace",
		"trailing",
		DiffShowWhitespaceSetting::Trailing,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_leading(
		"diffShowWhitespace",
		"leading",
		DiffShowWhitespaceSetting::Leading,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_false(
		"diffShowWhitespace",
		"false",
		DiffShowWhitespaceSetting::None,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_off(
		"diffShowWhitespace",
		"off",
		DiffShowWhitespaceSetting::None,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_none(
		"diffShowWhitespace",
		"none",
		DiffShowWhitespaceSetting::None,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_show_whitespace_mixed_case(
		"diffShowWhitespace",
		"tRaIlInG",
		DiffShowWhitespaceSetting::Trailing,
		|config: Config| config.diff_show_whitespace)
	]
	#[case::diff_tab_width_default("diffTabWidth", "", 4, |config: Config| config.diff_tab_width)]
	#[case::diff_tab_width("diffTabWidth", "42", 42, |config: Config| config.diff_tab_width)]
	#[case::diff_tab_symbol_default("diffTabSymbol", "", String::from("→"), |config: Config| config.diff_tab_symbol)]
	#[case::diff_tab_symbol("diffTabSymbol", "|", String::from("|"), |config: Config| config.diff_tab_symbol)]
	#[case::diff_space_symbol_default(
		"diffSpaceSymbol",
		"",
		String::from("·"),
		|config: Config| config.diff_space_symbol)
	]
	#[case::diff_space_symbol("diffSpaceSymbol", "-", String::from("-"), |config: Config| config.diff_space_symbol)]
	#[case::undo_limit_default("undoLimit", "", 5000, |config: Config| config.undo_limit)]
	#[case::undo_limit("undoLimit", "42", 42, |config: Config| config.undo_limit)]
	#[case::post_modified_line_exec_command(
		"postModifiedLineExecCommand",
		"command",
		Some(String::from("command")),
		|config: Config| config.post_modified_line_exec_command
	)]
	#[case::post_modified_line_exec_command_default(
		"postModifiedLineExecCommand",
		"",
		None,
		|config: Config| config.post_modified_line_exec_command
	)]
	pub(crate) fn config_test<F, T>(
		#[case] config_name: &str,
		#[case] config_value: &str,
		#[case] expected: T,
		#[case] access: F,
	) where
		F: Fn(Config) -> T + 'static,
		T: Debug + PartialEq,
	{
		let value = format!("{config_name} = \"{config_value}\"");
		let lines = if config_value.is_empty() {
			vec![]
		}
		else {
			vec!["[interactive-rebase-tool]", value.as_str()]
		};
		with_git_config(&lines, |config| {
			let config = Config::new_with_config(Some(&config)).unwrap();
			assert_eq!(access(config), expected);
		});
	}

	#[rstest]
	#[case::auto_select_next("autoSelectNext", "invalid", ConfigErrorCause::InvalidBoolean)]
	#[case::diff_ignore_whitespace("diffIgnoreWhitespace", "invalid", ConfigErrorCause::InvalidDiffIgnoreWhitespace)]
	#[case::diff_ignore_blank_lines("diffIgnoreBlankLines", "invalid", ConfigErrorCause::InvalidBoolean)]
	#[case::diff_show_whitespace("diffShowWhitespace", "invalid", ConfigErrorCause::InvalidShowWhitespace)]
	#[case::diff_tab_width_non_integer("diffTabWidth", "invalid", ConfigErrorCause::InvalidUnsignedInteger)]
	#[case::diff_tab_width_non_poitive_integer("diffTabWidth", "-100", ConfigErrorCause::InvalidUnsignedInteger)]
	#[case::undo_limit_non_integer("undoLimit", "invalid", ConfigErrorCause::InvalidUnsignedInteger)]
	#[case::undo_limit_non_positive_integer("undoLimit", "-100", ConfigErrorCause::InvalidUnsignedInteger)]
	fn value_parsing_invalid(#[case] config_name: &str, #[case] config_value: &str, #[case] cause: ConfigErrorCause) {
		with_git_config(
			&[
				"[interactive-rebase-tool]",
				format!("{config_name} = {config_value}").as_str(),
			],
			|git_config| {
				assert_err_eq!(
					Config::new_with_config(Some(&git_config)),
					ConfigError::new(
						format!("interactive-rebase-tool.{config_name}").as_str(),
						config_value,
						cause
					)
				);
			},
		);
	}

	#[rstest]
	#[case::diff_tab_symbol("diffIgnoreWhitespace")]
	#[case::diff_show_whitespace("diffShowWhitespace")]
	#[case::diff_tab_symbol("diffTabSymbol")]
	#[case::diff_space_symbol("diffSpaceSymbol")]
	#[case::post_modified_line_exec_command("postModifiedLineExecCommand")]
	fn value_parsing_invalid_utf(#[case] config_name: &str) {
		with_git_config(
			&[
				"[interactive-rebase-tool]",
				format!("{config_name} = {}", invalid_utf()).as_str(),
			],
			|git_config| {
				assert_err_eq!(
					Config::new_with_config(Some(&git_config)),
					ConfigError::new_read_error(
						format!("interactive-rebase-tool.{config_name}").as_str(),
						ConfigErrorCause::InvalidUtf
					)
				);
			},
		);
	}
}
