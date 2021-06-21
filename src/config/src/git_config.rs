use anyhow::{anyhow, Result};
use git2::Config;

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
	pub(super) fn new(git_config: &Config) -> Result<Self> {
		let comment_char = get_string(git_config, "core.commentChar", "#")?;
		let comment_char = if comment_char.as_str().eq("auto") {
			String::from("#")
		}
		else {
			comment_char
		};

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
