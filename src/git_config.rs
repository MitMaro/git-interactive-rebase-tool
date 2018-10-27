use git2::ConfigLevel;
use git2::Config;
use std::env;
use std::path::Path;

pub struct GitConfig {
	pub comment_char: String,
	pub foreground_color: String,
	pub indicator_color: String,
	pub error_color: String,
	pub diff_add_color: String,
	pub diff_remove_color: String,
	pub pick_color: String,
	pub reword_color: String,
	pub edit_color: String,
	pub squash_color: String,
	pub fixup_color: String,
	pub drop_color: String,
}

impl GitConfig {
	pub fn new() -> Result<Self, String> {
		let cfg = Config::open_default();

		match cfg {
			Ok(mut config) => {
				if let Some(val) = env::var_os("GIT_DIR") {
					match val.into_string() {
						Ok(s) => {
							let mut p = s.to_owned();
							p.push_str("/config");
							match config.add_file(Path::new(&p), ConfigLevel::Local, false) {
								Ok(_v) => {},
								Err(_e) => {}
							}
						},
						Err(_e) => {}
					}
				}

				Ok(GitConfig {
					comment_char: match config.get_string("core.commentChar") {
						Ok(comment_char_value) => comment_char_value,
						Err(_msg) => String::from("#")
					},
					foreground_color: match config.get_string("interactive-rebase-tool.foregroundColor") {
						Ok(foreground_color_value) => foreground_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					indicator_color: match config.get_string("interactive-rebase-tool.indicatorColor") {
						Ok(indicator_color_value) => indicator_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					error_color: match config.get_string("interactive-rebase-tool.errorColor") {
						Ok(error_color_value) => error_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					diff_add_color: match config.get_string("interactive-rebase-tool.diffAddColor") {
						Ok(diff_add_color_value) => diff_add_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					diff_remove_color: match config.get_string("interactive-rebase-tool.diffRemoveColor") {
						Ok(diff_remove_color_value) => diff_remove_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					pick_color: match config.get_string("interactive-rebase-tool.pickColor") {
						Ok(pick_color_value) => pick_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					reword_color: match config.get_string("interactive-rebase-tool.rewordColor") {
						Ok(reword_color_value) => reword_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					edit_color: match config.get_string("interactive-rebase-tool.editColor") {
						Ok(edit_color_value) => edit_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					squash_color: match config.get_string("interactive-rebase-tool.squashColor") {
						Ok(squash_color_value) => squash_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					fixup_color: match config.get_string("interactive-rebase-tool.fixupColor") {
						Ok(fixup_color_value) => fixup_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
					drop_color: match config.get_string("interactive-rebase-tool.dropColor") {
						Ok(drop_color_value) => drop_color_value.to_lowercase(),
						Err(_msg) => String::from("")
					},
				})
			},
			Err(msg) => {
				Err(format!(
					"Error reading git config, Reason {}\n", msg
				))
			}
		}
	}
}