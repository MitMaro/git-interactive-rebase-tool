use action::{
	Action,
	action_from_str,
	action_to_str
};

#[derive(PartialEq, Debug)]
pub struct Line {
	action: Action,
	hash: String,
	comment: String,
	mutated: bool
}

impl Line {
	pub fn new(input_line: &str) -> Result<Self, String> {
		let input: Vec<&str> = input_line.splitn(3, ' ').collect();
		match input.len() {
			3 => Ok(Line {
				action: action_from_str(input[0])?,
				hash: String::from(input[1]),
				comment: String::from(input[2]),
				mutated: false
			}),
			_ => Err(format!(
				"Invalid line: {}", input_line
			))
		}
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
	pub fn get_hash(&self) -> &String {
		&self.hash
	}
	pub fn get_comment(&self) -> &String {
		&self.comment
	}
	
	pub fn to_text(&self) -> String {
		format!("{} {} {}", action_to_str(&self.action), self.hash, self.comment)
	}
}

#[cfg(test)]
mod tests {
	use super::Line;
	use action::Action;
	
	#[test]
	fn new_with_valid_line() {
		let line = Line::new("pick aaa comment").unwrap();
		assert_eq!(line.action, Action::Pick);
		assert_eq!(line.hash, "aaa");
		assert_eq!(line.comment, "comment");
		assert_eq!(line.mutated, false);
	}
	
	#[test]
	fn new_with_invalid_action() {
		assert_eq!(Line::new("invalid aaa comment").unwrap_err(), "Invalid action: invalid");
	}
	
	#[test]
	fn new_with_invalid_line() {
		assert_eq!(Line::new("invalid").unwrap_err(), "Invalid line: invalid");
	}
	
	#[test]
	fn set_to_new_action() {
		let mut line = Line::new("pick aaa comment").unwrap();
		line.set_action(Action::Fixup);
		assert_eq!(line.action, Action::Fixup);
		assert_eq!(line.mutated, true);
	}
	
	#[test]
	fn getters() {
		let line = Line::new("pick aaa comment").unwrap();
		assert_eq!(line.get_action(), &Action::Pick);
		assert_eq!(line.get_hash(), &"aaa");
		assert_eq!(line.get_comment(), &"comment");
	}
	
	#[test]
	fn to_text() {
		let line = Line::new("pick aaa comment").unwrap();
		assert_eq!(line.to_text(), "pick aaa comment");
	}
}

