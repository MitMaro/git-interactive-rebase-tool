use std::env;

use crate::{
	config::{
		utils::{get_string, get_unsigned_integer, git_diff_renames},
		ConfigError,
	},
	git::Config,
};

fn editor_from_env() -> String {
	env::var("VISUAL")
		.or_else(|_| env::var("EDITOR"))
		.unwrap_or_else(|_| String::from("vi"))
}

/// Represents the git configuration options.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub(crate) struct GitConfig {
	/// The Git comment character, from [`core.commentChar`](
	///     https://git-scm.com/docs/git-config#Documentation/git-config.txt-corecommentChar
	/// ).
	pub(crate) comment_char: String,
	/// Number of context lines, from [`diff.context`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffcontext
	/// ).
	pub(crate) diff_context: u32,
	/// Number of interhunk lines, from [`diff.interhunk_lines`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffinterHunkContext
	/// ).
	pub(crate) diff_interhunk_lines: u32,
	/// The limit for detecting renames, from [`diff.renameLimit`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffrenameLimit
	/// ).
	pub(crate) diff_rename_limit: u32,
	/// If to detect renames, from [`diff.renames`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffrenames
	/// ).
	pub(crate) diff_renames: bool,
	/// If to detect copies, from [`diff.renames`](
	///     https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffrenames
	/// ).
	pub(crate) diff_copies: bool,
	/// The Git editor, from [`core.editor`](
	///     https://git-scm.com/docs/git-config#Documentation/git-config.txt-coreeditor
	/// ).
	pub(crate) editor: String,
}

impl GitConfig {
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

impl TryFrom<&Config> for GitConfig {
	type Error = ConfigError;

	fn try_from(config: &Config) -> Result<Self, Self::Error> {
		Self::new_with_config(Some(config))
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_err_eq, assert_ok};
	use rstest::rstest;

	use super::*;
	use crate::{
		config::ConfigErrorCause,
		test_helpers::{invalid_utf, with_env_var, with_git_config, EnvVarAction},
	};

	macro_rules! config_test {
		(
			$key:ident,
			$config_parent:literal,
			$config_name:literal,
			default $default:literal,
			$($value: literal => $expected: literal),*
		) => {
			let config = GitConfig::new_with_config(None).unwrap();
			let value = config.$key;
			assert_eq!(
				value,
				$default,
				"Default value for '{}' was expected to be '{}' but '{}' was found",
				stringify!($key),
				$default,
				value
			);

			for (value, expected) in [$( ($value, $expected), )*] {
				let config_parent = format!("[{}]", $config_parent);
				let config_value = format!("{} = \"{value}\"", $config_name);
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
	fn try_from_git_config() {
		with_git_config(&[], |git_config| {
			assert_ok!(GitConfig::try_from(&git_config));
		});
	}

	#[test]
	fn try_from_git_config_error() {
		with_git_config(&["[diff]", "renames = invalid"], |git_config| {
			_ = GitConfig::try_from(&git_config).unwrap_err();
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
	fn git_editor_default_no_env() {
		with_env_var(
			&[EnvVarAction::Remove("VISUAL"), EnvVarAction::Remove("EDITOR")],
			|| {
				let config = GitConfig::new_with_config(None).unwrap();
				assert_eq!(config.editor, "vi");
			},
		);
	}

	#[test]
	fn git_editor_default_visual_env() {
		with_env_var(
			&[
				EnvVarAction::Remove("EDITOR"),
				EnvVarAction::Set("VISUAL", String::from("visual-editor")),
			],
			|| {
				let config = GitConfig::new_with_config(None).unwrap();
				assert_eq!(config.editor, "visual-editor");
			},
		);
	}

	#[test]
	fn git_editor_default_editor_env() {
		with_env_var(
			&[
				EnvVarAction::Remove("VISUAL"),
				EnvVarAction::Set("EDITOR", String::from("editor")),
			],
			|| {
				let config = GitConfig::new_with_config(None).unwrap();
				assert_eq!(config.editor, "editor");
			},
		);
	}

	#[test]
	fn git_editor() {
		with_env_var(
			&[EnvVarAction::Remove("VISUAL"), EnvVarAction::Remove("EDITOR")],
			|| {
				with_git_config(&["[core]", "editor = custom"], |git_config| {
					let config = GitConfig::new_with_config(Some(&git_config)).unwrap();
					assert_eq!(config.editor, "custom");
				});
			},
		);
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
	fn git_editor_invalid() {
		with_env_var(
			&[EnvVarAction::Remove("VISUAL"), EnvVarAction::Remove("EDITOR")],
			|| {
				with_git_config(
					&["[core]", format!("editor = {}", invalid_utf()).as_str()],
					|git_config| {
						assert_err_eq!(
							GitConfig::new_with_config(Some(&git_config)),
							ConfigError::new_read_error("core.editor", ConfigErrorCause::InvalidUtf),
						);
					},
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
