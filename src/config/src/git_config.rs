use std::convert::TryFrom;

use anyhow::{anyhow, Error, Result};
use git::Config;

use super::utils::{editor_from_env, get_string, get_unsigned_integer};

/// Represents the git configuration options.
#[derive(Clone, Debug)]
pub struct GitConfig {
	/// The Git comment character, from [`core.commentChar`](
	///     https://git-scm.com/docs/git-config#Documentation/git-config.txt-corecommentChar
	/// ).
	pub comment_char: String,
	/// Number of context lines, from [`diff.context`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffcontext
	/// ).
	pub diff_context: u32,
	/// Number of interhunk lines, from [`diff.interhunk_lines`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffinterHunkContext
	/// ).
	pub diff_interhunk_lines: u32,
	/// The limit for detecting renames, from [`diff.renameLimit`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffrenameLimit
	/// ).
	pub diff_rename_limit: u32,
	/// If to detect renames, from [`diff.renames`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffrenames
	/// ).
	pub diff_renames: bool,
	/// If to detect copies, from [`diff.renames`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffrenames
	/// ).
	pub diff_copies: bool,
	/// The Git editor, from [`core.editor`](
	///     https://git-scm.com/docs/git-config#Documentation/git-config.txt-coreeditor
	/// ).
	pub editor: String,
}

impl GitConfig {
	/// Create a new configuration with default values.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self::new_with_config(None).expect("Panic without git config instance") // should never error with None config
	}

	pub(super) fn new_with_config(git_config: Option<&Config>) -> Result<Self> {
		let mut comment_char = get_string(git_config, "core.commentChar", "#")?;
		if comment_char.as_str().eq("auto") {
			comment_char = String::from("#");
		}

		let git_diff_renames = get_string(git_config, "diff.renames", "true")?.to_lowercase();
		let (diff_renames, diff_copies) = match git_diff_renames.to_lowercase().as_str() {
			"true" => (true, false),
			"false" => (false, false),
			"copy" | "copies" => (true, true),
			v => {
				return Err(anyhow!(
					"\"{}\" does not match one of \"true\", \"false\", \"copy\" or \"copies\"",
					v
				)
				.context("\"diff.renames\" is not valid"));
			},
		};

		Ok(Self {
			comment_char,
			diff_context: get_unsigned_integer(git_config, "diff.context", 3)?,
			diff_interhunk_lines: get_unsigned_integer(git_config, "diff.interHunkContext", 0)?,
			diff_rename_limit: get_unsigned_integer(git_config, "diff.renameLimit", 200)?,
			diff_renames,
			diff_copies,
			editor: get_string(git_config, "core.editor", editor_from_env().as_str())?,
		})
	}
}

impl Default for GitConfig {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl TryFrom<&Config> for GitConfig {
	type Error = Error;

	#[inline]
	fn try_from(config: &Config) -> core::result::Result<Self, Error> {
		Self::new_with_config(Some(config))
	}
}

#[cfg(test)]
mod tests {
	use std::env::{remove_var, set_var};

	use super::*;
	use crate::testutils::{assert_error, invalid_utf, with_git_config};

	#[test]
	fn new() {
		let _config = GitConfig::new();
	}

	#[test]
	fn default() {
		let _config = GitConfig::default();
	}

	#[test]
	fn try_from_git_config() {
		with_git_config(&[], |git_config| {
			assert!(GitConfig::try_from(&git_config).is_ok());
		});
	}

	#[test]
	fn try_from_git_config_error() {
		with_git_config(&["[diff]", "renames = invalid"], |git_config| {
			assert!(GitConfig::try_from(&git_config).is_err());
		});
	}

	#[test]
	fn comment_char_default() {
		let config = GitConfig::new();
		assert_eq!(config.comment_char, "#");
	}

	#[test]
	fn comment_char_auto() {
		with_git_config(&["[core]", "commentChar = auto"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert_eq!(config.comment_char, "#");
		});
	}

	#[test]
	fn comment_char() {
		with_git_config(&["[core]", "commentChar = \";\""], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert_eq!(config.comment_char, ";");
		});
	}

	#[test]
	fn comment_char_invalid() {
		with_git_config(
			&["[core]", format!("commentChar = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_error(
					GitConfig::new_with_config(Some(&git_config)),
					"\"core.commentChar\" is not valid: configuration value is not valid utf8",
				);
			},
		);
	}

	#[test]
	fn diff_context_default() {
		let config = GitConfig::new();
		assert_eq!(config.diff_context, 3);
	}

	#[test]
	fn diff_context() {
		with_git_config(&["[diff]", "context = 5"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert_eq!(config.diff_context, 5);
		});
	}

	#[test]
	fn diff_context_invalid() {
		with_git_config(&["[diff]", "context = invalid"], |git_config| {
			assert_error(
				GitConfig::new_with_config(Some(&git_config)),
				"\"diff.context\" is not valid: failed to parse \'invalid\' as a 32-bit integer",
			);
		});
	}

	#[test]
	fn diff_context_invalid_range() {
		with_git_config(&["[diff]", "context = -100"], |git_config| {
			assert_error(
				GitConfig::new_with_config(Some(&git_config)),
				"\"diff.context\" is not valid: \"-100\" is outside of valid range for an unsigned 32-bit integer",
			);
		});
	}

	#[test]
	fn diff_interhunk_lines_default() {
		let config = GitConfig::new();
		assert_eq!(config.diff_interhunk_lines, 0);
	}

	#[test]
	fn diff_interhunk_lines() {
		with_git_config(&["[diff]", "interHunkContext = 5"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert_eq!(config.diff_interhunk_lines, 5);
		});
	}

	#[test]
	fn diff_interhunk_lines_invalid() {
		with_git_config(&["[diff]", "interHunkContext = invalid"], |git_config| {
			assert_error(
				GitConfig::new_with_config(Some(&git_config)),
				"\"diff.interHunkContext\" is not valid: failed to parse \'invalid\' as a 32-bit integer",
			);
		});
	}

	#[test]
	fn diff_interhunk_lines_invalid_range() {
		with_git_config(&["[diff]", "interHunkContext = -100"], |git_config| {
			assert_error(
				GitConfig::new_with_config(Some(&git_config)),
				"\"diff.interHunkContext\" is not valid: \"-100\" is outside of valid range for an unsigned 32-bit \
				 integer",
			);
		});
	}

	#[test]
	fn diff_rename_limit_default() {
		let config = GitConfig::new();
		assert_eq!(config.diff_rename_limit, 200);
	}

	#[test]
	fn diff_rename_limit() {
		with_git_config(&["[diff]", "renameLimit = 5"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert_eq!(config.diff_rename_limit, 5);
		});
	}

	#[test]
	fn diff_rename_limit_invalid() {
		with_git_config(&["[diff]", "renameLimit = invalid"], |git_config| {
			assert_error(
				GitConfig::new_with_config(Some(&git_config)),
				"\"diff.renameLimit\" is not valid: failed to parse \'invalid\' as a 32-bit integer",
			);
		});
	}

	#[test]
	fn diff_rename_limit_invalid_range() {
		with_git_config(&["[diff]", "renameLimit = -100"], |git_config| {
			assert_error(
				GitConfig::new_with_config(Some(&git_config)),
				"\"diff.renameLimit\" is not valid: \"-100\" is outside of valid range for an unsigned 32-bit integer",
			);
		});
	}

	#[test]
	fn diff_renames_default() {
		let config = GitConfig::new();
		assert!(config.diff_renames);
		assert!(!config.diff_copies);
	}

	#[test]
	fn diff_renames_true() {
		with_git_config(&["[diff]", "renames = true"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert!(config.diff_renames);
			assert!(!config.diff_copies);
		});
	}

	#[test]
	fn diff_renames_false() {
		with_git_config(&["[diff]", "renames = false"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert!(!config.diff_renames);
			assert!(!config.diff_copies);
		});
	}

	#[test]
	fn diff_renames_copy() {
		with_git_config(&["[diff]", "renames = copy"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert!(config.diff_renames);
			assert!(config.diff_copies);
		});
	}

	#[test]
	fn diff_renames_copies() {
		with_git_config(&["[diff]", "renames = copies"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert!(config.diff_renames);
			assert!(config.diff_copies);
		});
	}

	#[test]
	fn diff_renames_mixed_case() {
		with_git_config(&["[diff]", "renames = cOpIeS"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert!(config.diff_renames);
			assert!(config.diff_copies);
		});
	}

	#[test]
	fn diff_renames_invalid() {
		with_git_config(&["[diff]", "renames = invalid"], |git_config| {
			assert_error(
				GitConfig::new_with_config(Some(&git_config)),
				"\"diff.renames\" is not valid: \"invalid\" does not match one of \"true\", \"false\", \"copy\" or \
				 \"copies\"",
			);
		});
	}

	#[test]
	#[serial_test::serial]
	fn git_editor_default_no_env() {
		remove_var("VISUAL");
		remove_var("EDITOR");
		let config = GitConfig::new();
		assert_eq!(config.editor, "vi");
	}

	#[test]
	#[serial_test::serial]
	fn git_editor_default_visual_env() {
		remove_var("EDITOR");
		set_var("VISUAL", "visual-editor");
		let config = GitConfig::new();
		assert_eq!(config.editor, "visual-editor");
	}

	#[test]
	#[serial_test::serial]
	fn git_editor_default_editor_env() {
		remove_var("VISUAL");
		set_var("EDITOR", "editor");

		let config = GitConfig::new();
		assert_eq!(config.editor, "editor");
	}

	#[test]
	#[serial_test::serial]
	fn git_editor() {
		remove_var("VISUAL");
		remove_var("EDITOR");
		with_git_config(&["[core]", "editor = custom"], |git_config| {
			let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
			assert_eq!(config.editor, "custom");
		});
	}

	#[test]
	#[serial_test::serial]
	fn git_editor_invalid() {
		remove_var("VISUAL");
		remove_var("EDITOR");
		with_git_config(
			&["[core]", format!("editor = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_error(
					GitConfig::new_with_config(Some(&git_config)),
					"\"core.editor\" is not valid: configuration value is not valid utf8",
				);
			},
		);
	}
}
