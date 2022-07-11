use std::env;

use git::Config;

use crate::{
	errors::ConfigError,
	get_string,
	utils::{get_unsigned_integer, git_diff_renames},
};

fn editor_from_env() -> String {
	env::var("VISUAL")
		.or_else(|_| env::var("EDITOR"))
		.unwrap_or_else(|_| String::from("vi"))
}

/// Represents the git configuration options.
#[derive(Clone, Debug)]
#[non_exhaustive]
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

	pub(super) fn new_with_config(git_config: Option<&Config>) -> Result<Self, ConfigError> {
		let mut comment_char = get_string(git_config, "core.commentChar", "#")?;
		if comment_char.as_str().eq("auto") {
			comment_char = String::from("#");
		}

		let (diff_renames, diff_copies) = git_diff_renames(git_config, "diff.renames")?;

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
	type Error = ConfigError;

	#[inline]
	fn try_from(config: &Config) -> Result<Self, Self::Error> {
		Self::new_with_config(Some(config))
	}
}

#[cfg(test)]
mod tests {
	use std::env::{remove_var, set_var};

	use rstest::rstest;
	use testutils::assert_err_eq;

	use super::*;
	use crate::{
		testutils::{invalid_utf, with_git_config},
		ConfigErrorCause,
	};

	macro_rules! config_test {
		(
			$key:ident,
			$config_parent:literal,
			$config_name:literal,
			default $default:literal,
			$($value: literal => $expected: literal),*
		) => {
			let config = GitConfig::new();
			let value = config.$key;
			assert_eq!(
				value,
				$default,
				"Default value for '{}' was expected to be '{}' but '{}' was found",
				stringify!($key),
				$default,
				value
			);

			for (value, expected) in vec![$( ($value, $expected), )*] {
				let config_parent = format!("[{}]", $config_parent);
				let config_value = format!("{} = \"{}\"", $config_name, value);
				with_git_config(&[config_parent.as_str(), config_value.as_str()], |git_config| {
					let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
					assert_eq!(
						config.$key,
						expected,
						"Value for '{}' was expected to be '{}' but '{}' was found",
						stringify!($key),
						$default,
						value
					);
				});
			}
		};
	}

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

	#[rstest]
	fn config_values() {
		config_test!(comment_char, "core", "commentChar", default "#", ";" => ";", "auto" => "#");
		config_test!(diff_context, "diff", "context", default 3, "5" => 5);
		config_test!(diff_interhunk_lines, "diff", "interHunkContext", default 0, "5" => 5);
		config_test!(diff_interhunk_lines, "diff", "interHunkContext", default 0, "5" => 5);
		config_test!(diff_rename_limit, "diff", "renameLimit", default 200, "5" => 5);
		config_test!(diff_renames, "diff", "renames", default true, "true" => true, "false" => false, "copy" => true);
		config_test!(diff_copies, "diff", "renames",default false, "true" => false, "false" => false, "copy" => true);
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
	fn diff_rename_limit_invalid() {
		with_git_config(&["[diff]", "renameLimit = invalid"], |git_config| {
			assert_err_eq!(
				GitConfig::new_with_config(Some(&git_config)),
				ConfigError::new("diff.renameLimit", "invalid", ConfigErrorCause::InvalidUnsignedInteger),
			);
		});
	}

	#[test]
	fn diff_rename_limit_invalid_range() {
		with_git_config(&["[diff]", "renameLimit = -100"], |git_config| {
			assert_err_eq!(
				GitConfig::new_with_config(Some(&git_config)),
				ConfigError::new("diff.renameLimit", "-100", ConfigErrorCause::InvalidUnsignedInteger),
			);
		});
	}

	#[test]
	fn diff_renames_invalid() {
		with_git_config(&["[diff]", "renames = invalid"], |git_config| {
			assert_err_eq!(
				GitConfig::new_with_config(Some(&git_config)),
				ConfigError::new("diff.renames", "invalid", ConfigErrorCause::InvalidDiffRenames),
			);
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
				assert_err_eq!(
					GitConfig::new_with_config(Some(&git_config)),
					ConfigError::new_read_error("core.editor", ConfigErrorCause::InvalidUtf),
				);
			},
		);
	}

	#[test]
	fn comment_char_invalid() {
		with_git_config(
			&["[core]", format!("commentChar = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_err_eq!(
					GitConfig::new_with_config(Some(&git_config)),
					ConfigError::new_read_error("core.commentChar", ConfigErrorCause::InvalidUtf),
				);
			},
		);
	}

	#[test]
	fn diff_context_invalid() {
		with_git_config(&["[diff]", "context = invalid"], |git_config| {
			assert_err_eq!(
				GitConfig::new_with_config(Some(&git_config)),
				ConfigError::new("diff.context", "invalid", ConfigErrorCause::InvalidUnsignedInteger),
			);
		});
	}

	#[test]
	fn diff_context_invalid_range() {
		with_git_config(&["[diff]", "context = -100"], |git_config| {
			assert_err_eq!(
				GitConfig::new_with_config(Some(&git_config)),
				ConfigError::new("diff.context", "-100", ConfigErrorCause::InvalidUnsignedInteger),
			);
		});
	}

	#[test]
	fn diff_interhunk_lines_invalid() {
		with_git_config(&["[diff]", "interHunkContext = invalid"], |git_config| {
			assert_err_eq!(
				GitConfig::new_with_config(Some(&git_config)),
				ConfigError::new(
					"diff.interHunkContext",
					"invalid",
					ConfigErrorCause::InvalidUnsignedInteger
				),
			);
		});
	}

	#[test]
	fn diff_interhunk_lines_invalid_range() {
		with_git_config(&["[diff]", "interHunkContext = -100"], |git_config| {
			assert_err_eq!(
				GitConfig::new_with_config(Some(&git_config)),
				ConfigError::new(
					"diff.interHunkContext",
					"-100",
					ConfigErrorCause::InvalidUnsignedInteger
				),
			);
		});
	}
}
