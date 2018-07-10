use git2::ConfigLevel;
use git2::Config;
use std::env;
use std::path::Path;

pub struct GitConfig {
	pub comment_char: String,
}

impl GitConfig {
	pub fn new() -> Result<Self, String> {
		let cfg = Config::open_default();

		match cfg {
			Ok(mut config) => {
				match env::var_os("GIT_DIR") {
					Some(val) => match val.into_string() {
						Ok(s) => {
							let mut p = s.to_owned();
							p.push_str("/config");
							match config.add_file(Path::new(&p), ConfigLevel::Local, false) {
								Ok(_v) => {}, Err(_e) => {}
							}
						},
						Err(_e) => {}
					},
					None => {}
				}

				Ok(GitConfig {
					comment_char: match config.get_string("core.commentChar") {
						Ok(comment_char_value) => comment_char_value,
						Err(_msg) => String::from("#")
					}
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