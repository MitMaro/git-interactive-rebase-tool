use crate::config::utils::{editor_from_env, get_string};
use git2::Config;

#[derive(Clone, Debug)]
pub(crate) struct GitConfig {
	pub(crate) comment_char: String,
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

		Ok(Self {
			comment_char,
			editor: get_string(&git_config, "core.editor", editor_from_env().as_str())?,
		})
	}
}
