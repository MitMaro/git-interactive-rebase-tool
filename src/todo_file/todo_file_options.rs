/// Options for `TodoFile`
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TodoFileOptions {
	pub(crate) comment_prefix: String,
	pub(crate) line_changed_command: Option<String>,
	pub(crate) undo_limit: u32,
}

impl TodoFileOptions {
	/// Create a new instance of `TodoFileOptions`
	#[must_use]
	pub(crate) fn new(undo_limit: u32, comment_prefix: &str) -> Self {
		Self {
			comment_prefix: String::from(comment_prefix),
			line_changed_command: None,
			undo_limit,
		}
	}

	/// Set a command to be added after each changed line
	pub(crate) fn line_changed_command(&mut self, command: &str) {
		self.line_changed_command = Some(String::from(command));
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_none, assert_some_eq};

	use super::*;

	#[test]
	fn new() {
		let options = TodoFileOptions::new(10, "#");

		assert_eq!(options.undo_limit, 10);
		assert_eq!(options.comment_prefix, "#");
		assert_none!(options.line_changed_command);
	}

	#[test]
	fn line_changed_command() {
		let mut options = TodoFileOptions::new(10, "#");

		options.line_changed_command("command");

		assert_some_eq!(options.line_changed_command, "command");
	}
}
