use crate::todo_file::action::Action;
use anyhow::{anyhow, Result};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
	action: Action,
	hash: String,
	command: String,
	comment: String,
	mutated: bool,
}

impl Line {
	fn new_noop() -> Self {
		Self {
			action: Action::Noop,
			command: String::from(""),
			comment: String::from(""),
			hash: String::from(""),
			mutated: false,
		}
	}

	pub(crate) fn new_break() -> Self {
		Self {
			action: Action::Break,
			command: String::from(""),
			comment: String::from(""),
			hash: String::from(""),
			mutated: false,
		}
	}

	pub(crate) fn new(input_line: &str) -> Result<Self> {
		if input_line.starts_with("noop") {
			return Ok(Self::new_noop());
		}
		else if input_line.starts_with("break") || input_line.starts_with('b') {
			return Ok(Self::new_break());
		}
		else if input_line.starts_with("exec") || input_line.starts_with('x') {
			let input: Vec<&str> = input_line.splitn(2, ' ').collect();
			if input.len() == 2 {
				return Ok(Self {
					action: Action::try_from(input[0])?,
					hash: String::from(""),
					command: String::from(input[1]),
					comment: String::from(""),
					mutated: false,
				});
			}
		}
		else {
			let input: Vec<&str> = input_line.splitn(3, ' ').collect();
			if input.len() >= 2 {
				return Ok(Self {
					action: Action::try_from(input[0])?,
					hash: String::from(input[1]),
					command: String::from(""),
					comment: if input.len() == 3 {
						String::from(input[2])
					}
					else {
						String::from("")
					},
					mutated: false,
				});
			}
		}

		Err(anyhow!("Invalid line: {}", input_line))
	}

	pub(crate) fn set_action(&mut self, action: Action) {
		if self.action != action {
			self.mutated = true;
			self.action = action;
		}
	}

	pub(crate) fn edit_content(&mut self, content: &str) {
		if let Action::Exec = self.action {
			self.command = String::from(content)
		}
	}

	pub(crate) fn get_edit_content(&self) -> &str {
		match self.action {
			Action::Exec => self.command.as_str(),
			_ => self.comment.as_str(),
		}
	}

	pub(crate) const fn get_action(&self) -> &Action {
		&self.action
	}

	pub(crate) fn get_command(&self) -> &str {
		self.command.as_str()
	}

	pub(crate) fn get_hash(&self) -> &str {
		self.hash.as_str()
	}

	pub(crate) fn get_comment(&self) -> &str {
		self.comment.as_str()
	}

	pub(crate) fn to_text(&self) -> String {
		match self.action {
			Action::Exec => format!("exec {}", self.command),
			Action::Break => String::from("break"),
			_ => format!("{} {} {}", self.action.as_string(), self.hash, self.comment),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest(
		line,
		expected,
		case::pick_action("pick aaa comment", &Line {
			action: Action::Pick,
			hash: String::from("aaa"),
			command: String::from(""),
			comment: String::from("comment"),
			mutated: false,
		}),
		case::reword_action("reword aaa comment", &Line {
			action: Action::Reword,
			hash: String::from("aaa"),
			command: String::from(""),
			comment: String::from("comment"),
			mutated: false,
		}),
		case::edit_action("edit aaa comment", &Line {
			action: Action::Edit,
			hash: String::from("aaa"),
			command: String::from(""),
			comment: String::from("comment"),
			mutated: false,
		}),
		case::squash_action("squash aaa comment", &Line {
			action: Action::Squash,
			hash: String::from("aaa"),
			command: String::from(""),
			comment: String::from("comment"),
			mutated: false,
		}),
		case::fixup_action("fixup aaa comment", &Line {
			action: Action::Fixup,
			hash: String::from("aaa"),
			command: String::from(""),
			comment: String::from("comment"),
			mutated: false,
		}),
		case::drop_action("drop aaa comment", &Line {
			action: Action::Drop,
			hash: String::from("aaa"),
			command: String::from(""),
			comment: String::from("comment"),
			mutated: false,
		}),
		case::action_without_comment("pick aaa", &Line {
			action: Action::Pick,
			hash: String::from("aaa"),
			command: String::from(""),
			comment: String::from(""),
			mutated: false,
		}),
		case::exec_action("exec command", &Line {
			action: Action::Exec,
			hash: String::from(""),
			command: String::from("command"),
			comment: String::from(""),
			mutated: false,
		}),
		case::break_action("break", &Line {
			action: Action::Break,
			hash: String::from(""),
			command: String::from(""),
			comment: String::from(""),
			mutated: false,
		}),
		case::nnop( "noop", &Line {
			action: Action::Noop,
			hash: String::from(""),
			command: String::from(""),
			comment: String::from(""),
			mutated: false,
		}),
	)]
	fn new(line: &str, expected: &Line) {
		assert_eq!(&Line::new(line).unwrap(), expected);
	}

	#[test]
	fn line_new_break() {
		assert_eq!(Line::new_break(), Line {
			action: Action::Break,
			hash: String::from(""),
			command: String::from(""),
			comment: String::from(""),
			mutated: false,
		});
	}

	#[rstest(
		line,
		expected_err,
		case::invalid_action("invalid aaa comment", "Invalid action: invalid"),
		case::invalid_line_only("invalid", "Invalid line: invalid"),
		case::pick_line_only("pick", "Invalid line: pick"),
		case::reword_line_only("reword", "Invalid line: reword"),
		case::edit_line_only("edit", "Invalid line: edit"),
		case::squash_line_only("squash", "Invalid line: squash"),
		case::fixup_line_only("fixup", "Invalid line: fixup"),
		case::exec_line_only("exec", "Invalid line: exec"),
		case::drop_line_only("drop", "Invalid line: drop")
	)]
	fn new_err(line: &str, expected_err: &str) {
		assert_eq!(Line::new(line).unwrap_err().to_string(), expected_err);
	}

	#[test]
	fn set_to_new_action_with_changed_action() {
		let mut line = Line::new("pick aaa comment").unwrap();
		line.set_action(Action::Fixup);
		assert_eq!(line.action, Action::Fixup);
		assert_eq!(line.mutated, true);
	}

	#[test]
	fn set_to_new_action_with_unchanged_action() {
		let mut line = Line::new("pick aaa comment").unwrap();
		line.set_action(Action::Pick);
		assert_eq!(line.action, Action::Pick);
		assert_eq!(line.mutated, false);
	}

	#[rstest(
		line,
		expected,
		case::break_action("break", ""),
		case::drop("drop aaa comment", "comment"),
		case::edit("edit aaa comment", "comment"),
		case::exec("exec git commit --amend 'foo'", "new"),
		case::fixup("fixup aaa comment", "comment"),
		case::pick("pick aaa comment", "comment"),
		case::reword("reword aaa comment", "comment"),
		case::squash("squash aaa comment", "comment")
	)]
	fn edit_content(line: &str, expected: &str) {
		let mut line = Line::new(line).unwrap();
		line.edit_content("new");
		assert_eq!(line.get_edit_content(), expected);
	}

	#[rstest(
		line,
		expected,
		case::break_action("break", ""),
		case::drop("drop aaa comment", "comment"),
		case::edit("edit aaa comment", "comment"),
		case::exec("exec git commit --amend 'foo'", "git commit --amend 'foo'"),
		case::fixup("fixup aaa comment", "comment"),
		case::pick("pick aaa comment", "comment"),
		case::reword("reword aaa comment", "comment"),
		case::squash("squash aaa comment", "comment")
	)]
	fn get_edit_content(line: &str, expected: &str) {
		assert_eq!(Line::new(line).unwrap().get_edit_content(), expected);
	}

	#[rstest(
		line,
		expected,
		case::break_action("break", Action::Break),
		case::drop("drop aaa comment", Action::Drop),
		case::edit("edit aaa comment", Action::Edit),
		case::exec("exec git commit --amend 'foo'", Action::Exec),
		case::fixup("fixup aaa comment", Action::Fixup),
		case::pick("pick aaa comment", Action::Pick),
		case::reword("reword aaa comment", Action::Reword),
		case::squash("squash aaa comment", Action::Squash)
	)]
	fn get_action(line: &str, expected: Action) {
		assert_eq!(Line::new(line).unwrap().get_action(), &expected);
	}

	#[rstest(
		line,
		expected,
		case::break_action("break", ""),
		case::drop("drop aaa comment", ""),
		case::edit("edit aaa comment", ""),
		case::exec("exec git commit --amend 'foo'", "git commit --amend 'foo'"),
		case::fixup("fixup aaa comment", ""),
		case::pick("pick aaa comment", ""),
		case::reword("reword aaa comment", ""),
		case::squash("squash aaa comment", "")
	)]
	fn get_command(line: &str, expected: &str) {
		assert_eq!(Line::new(line).unwrap().get_command(), expected);
	}

	#[rstest(
		line,
		expected,
		case::break_action("break", ""),
		case::drop("drop aaa comment", "aaa"),
		case::edit("edit aaa comment", "aaa"),
		case::exec("exec git commit --amend 'foo'", ""),
		case::fixup("fixup aaa comment", "aaa"),
		case::pick("pick aaa comment", "aaa"),
		case::reword("reword aaa comment", "aaa"),
		case::squash("squash aaa comment", "aaa")
	)]
	fn get_hash(line: &str, expected: &str) {
		assert_eq!(Line::new(line).unwrap().get_hash(), expected);
	}

	#[rstest(
		line,
		expected,
		case::break_action("break", ""),
		case::drop("drop aaa comment", "comment"),
		case::edit("edit aaa comment", "comment"),
		case::exec("exec git commit --amend 'foo'", ""),
		case::fixup("fixup aaa comment", "comment"),
		case::pick("pick aaa comment", "comment"),
		case::reword("reword aaa comment", "comment"),
		case::squash("squash aaa comment", "comment")
	)]
	fn get_comment(line: &str, expected: &str) {
		assert_eq!(Line::new(line).unwrap().get_comment(), expected);
	}

	#[rstest(
		line,
		case::break_action("break"),
		case::drop("drop aaa comment"),
		case::edit("edit aaa comment"),
		case::exec("exec git commit --amend 'foo'"),
		case::fixup("fixup aaa comment"),
		case::pick("pick aaa comment"),
		case::reword("reword aaa comment"),
		case::squash("squash aaa comment")
	)]
	fn to_text(line: &str) {
		assert_eq!(Line::new(line).unwrap().to_text(), line);
	}
}
