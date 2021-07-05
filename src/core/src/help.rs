use crate::{exit::Exit, version::VERSION};

const HELP_MESSAGE: &str = r#"
Git Interactive Rebase Editor ({{VERSION}})
Full feature terminal based sequence editor for git interactive rebase.

USAGE:
  interactive-rebase-tool [FLAGS] [REBASE-TODO-FILE]

FLAGS:
  -v, --version       Prints versioning information
  -h, --help          Prints help information
  --license           Prints Open Source Software licensing

ARGS:
  <REBASE-TODO-FILE>  The path to the Git rebase todo file
"#;

pub(crate) fn build_help(message: Option<String>) -> String {
	let help = HELP_MESSAGE.replace("{{VERSION}}", VERSION);
	if let Some(msg) = message {
		format!("{}\n\n{}", msg, help)
	}
	else {
		help
	}
}

pub(crate) fn run() -> Exit {
	Exit::from(build_help(None))
}
