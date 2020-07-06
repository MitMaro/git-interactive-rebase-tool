use crate::config::utils::{editor_from_env, get_string, get_unsigned_integer};
use git2::Config;

#[derive(Clone, Debug)]
pub(crate) struct GitConfig {
	pub(crate) comment_char: String,
	pub(crate) diff_rename_limit: u32,
	pub(crate) diff_renames: bool,
	pub(crate) diff_copies: bool,
	pub(crate) editor: String,
}

impl GitConfig {
	pub(super) fn new(git_config: &Config) -> Result<Self, String> {
		let comment_char = get_string(&git_config, "core.commentChar", "#")?;
		let comment_char = if comment_char.as_str().eq("auto") {
			String::from("#")
		}
		else {
			comment_char
		};

		let git_diff_renames = get_string(&git_config, "diff.renames", "true")?;
		let diff_renames = git_diff_renames.as_str() != "false";
		let diff_copies = git_diff_renames.as_str() == "copy" || git_diff_renames.as_str() == "copies";

		Ok(Self {
			comment_char,
			diff_rename_limit: get_unsigned_integer(&git_config, "diff.renameLimit", 200)?,
			diff_renames,
			diff_copies,
			editor: get_string(&git_config, "core.editor", editor_from_env().as_str())?,
		})
	}
}
