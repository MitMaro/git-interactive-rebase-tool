use crate::constants::{NAME, VERSION};
use clap::App;

pub fn build_cli() -> App<'static, 'static> {
	App::new(NAME)
		.version(VERSION)
		.about("Full feature terminal based sequence editor for git interactive rebase.")
		.author("Tim Oram <dev@mitmaro.ca>")
		.args_from_usage("<rebase-todo-filepath> 'The path to the git rebase todo file'")
}
