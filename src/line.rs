use crate::action::Action;
use std::convert::TryFrom;

#[derive(PartialEq, Debug)]
pub struct Line {
	action: Action,
	hash: String,
	command: String,
	comment: String,
	mutated: bool,
}

impl Line {
	pub fn new_break() -> Self {
		Self {
			action: Action::Break,
			command: String::from(""),
			comment: String::from(""),
			hash: String::from(""),
			mutated: false,
		}
	}

	pub fn new(input_line: &str) -> Result<Self, String> {
		if input_line.starts_with("break") || input_line.starts_with('b') {
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

		Err(format!("Invalid line: {}", input_line))
	}

	pub fn set_action(&mut self, action: Action) {
		if self.action != action {
			self.mutated = true;
			self.action = action;
		}
	}

	pub fn get_action(&self) -> &Action {
		&self.action
	}

	pub fn get_command(&self) -> &String {
		&self.command
	}

	pub fn get_hash(&self) -> &String {
		&self.hash
	}

	pub fn get_comment(&self) -> &String {
		&self.comment
	}

	pub fn to_text(&self) -> String {
		match self.action {
			Action::Exec => format!("exec {}", self.command),
			Action::Break => String::from("break"),
			_ => format!("{} {} {}", self.action.as_string(), self.hash, self.comment),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Line;
	use crate::action::Action;

	#[test]
	fn new_with_pick_action() {
		let line = Line::new("pick aaa comment").unwrap();
		assert_eq!(line.action, Action::Pick);
		assert_eq!(line.get_hash(), &"aaa");
		assert_eq!(line.get_command(), &"");
		assert_eq!(line.get_comment(), &"comment");
		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_reword_action() {
		let line = Line::new("reword aaa comment").unwrap();
		assert_eq!(line.action, Action::Reword);
		assert_eq!(line.get_hash(), &"aaa");
		assert_eq!(line.get_command(), &"");
		assert_eq!(line.get_comment(), &"comment");
		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_edit_action() {
		let line = Line::new("edit aaa comment").unwrap();
		assert_eq!(line.action, Action::Edit);
		assert_eq!(line.get_hash(), &"aaa");
		assert_eq!(line.get_command(), &"");
		assert_eq!(line.get_comment(), &"comment");

		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_squash_action() {
		let line = Line::new("squash aaa comment").unwrap();
		assert_eq!(line.action, Action::Squash);
		assert_eq!(line.get_hash(), &"aaa");
		assert_eq!(line.get_command(), &"");
		assert_eq!(line.get_comment(), &"comment");
		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_fixup_action() {
		let line = Line::new("fixup aaa comment").unwrap();
		assert_eq!(line.action, Action::Fixup);
		assert_eq!(line.get_hash(), &"aaa");
		assert_eq!(line.get_command(), &"");
		assert_eq!(line.get_comment(), &"comment");
		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_drop_action() {
		let line = Line::new("drop aaa comment").unwrap();
		assert_eq!(line.action, Action::Drop);
		assert_eq!(line.get_hash(), &"aaa");
		assert_eq!(line.get_command(), &"");
		assert_eq!(line.get_comment(), &"comment");
		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_action_without_comment() {
		let line = Line::new("pick aaa").unwrap();
		assert_eq!(line.action, Action::Pick);
		assert_eq!(line.get_hash(), &"aaa");
		assert_eq!(line.get_command(), &"");
		assert_eq!(line.get_comment(), &"");
		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_exec_action() {
		let line = Line::new("exec command").unwrap();
		assert_eq!(line.action, Action::Exec);
		assert_eq!(line.get_hash(), &"");
		assert_eq!(line.get_command(), &"command");
		assert_eq!(line.get_comment(), &"");
		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_break_action() {
		let line = Line::new("break").unwrap();
		assert_eq!(line.action, Action::Break);
		assert_eq!(line.get_hash(), &"");
		assert_eq!(line.get_command(), &"");
		assert_eq!(line.get_comment(), &"");
		assert_eq!(line.mutated, false);
	}

	#[test]
	fn new_with_invalid_action() {
		assert_eq!(Line::new("invalid aaa comment").unwrap_err(), "Invalid action: invalid");
	}

	#[test]
	fn new_with_invalid_line() {
		assert_eq!(Line::new("invalid").unwrap_err(), "Invalid line: invalid");
		assert_eq!(Line::new("pick").unwrap_err(), "Invalid line: pick");
		assert_eq!(Line::new("reword").unwrap_err(), "Invalid line: reword");
		assert_eq!(Line::new("edit").unwrap_err(), "Invalid line: edit");
		assert_eq!(Line::new("squash").unwrap_err(), "Invalid line: squash");
		assert_eq!(Line::new("fixup").unwrap_err(), "Invalid line: fixup");
		assert_eq!(Line::new("exec").unwrap_err(), "Invalid line: exec");
		assert_eq!(Line::new("drop").unwrap_err(), "Invalid line: drop");
	}

	#[test]
	fn set_to_new_action() {
		let mut line = Line::new("pick aaa comment").unwrap();
		line.set_action(Action::Fixup);
		assert_eq!(line.action, Action::Fixup);
		assert_eq!(line.mutated, true);
	}

	#[test]
	fn to_text_pick_action() {
		let line = Line::new("pick aaa comment").unwrap();
		assert_eq!(line.to_text(), "pick aaa comment");
	}

	#[test]
	fn to_text_reword_action() {
		let line = Line::new("reword aaa comment").unwrap();
		assert_eq!(line.to_text(), "reword aaa comment");
	}

	#[test]
	fn to_text_edit_action() {
		let line = Line::new("edit aaa comment").unwrap();
		assert_eq!(line.to_text(), "edit aaa comment");
	}

	#[test]
	fn to_text_squash_action() {
		let line = Line::new("squash aaa comment").unwrap();
		assert_eq!(line.to_text(), "squash aaa comment");
	}

	#[test]
	fn to_text_fixup_action() {
		let line = Line::new("fixup aaa comment").unwrap();
		assert_eq!(line.to_text(), "fixup aaa comment");
	}

	#[test]
	fn to_text_exec_action() {
		let line = Line::new("exec command").unwrap();
		assert_eq!(line.to_text(), "exec command");
	}

	#[test]
	fn to_text_break_action() {
		let line = Line::new("break").unwrap();
		assert_eq!(line.to_text(), "break");
	}

	#[test]
	fn to_text_drop_action() {
		let line = Line::new("drop aaa comment").unwrap();
		assert_eq!(line.to_text(), "drop aaa comment");
	}
}
