
#[derive(PartialEq, Debug)]
pub enum Action {
	Pick,
	Reword,
	Edit,
	Squash,
	Fixup,
	Drop
}

pub fn action_from_str(s: &str) -> Result<Action, String> {
	match s {
		"pick" | "p" => Ok(Action::Pick),
		"reword" | "r" => Ok(Action::Reword),
		"edit" | "e" => Ok(Action::Edit),
		"squash" | "s" => Ok(Action::Squash),
		"fixup" | "f" => Ok(Action::Fixup),
		"drop" | "d" => Ok(Action::Drop),
		_ => Err(format!("Invalid action: {}", s))
	}
}

pub fn action_to_str(action: &Action) -> String {
	String::from(match *action {
		Action::Pick => "pick",
		Action::Reword => "reword",
		Action::Edit => "edit",
		Action::Squash => "squash",
		Action::Fixup => "fixup",
		Action::Drop => "drop"
	})
}

#[cfg(test)]
mod tests {
	use super::{
		Action,
		action_from_str,
		action_to_str
	};

	#[test]
	fn action_to_str_all() {
		assert_eq!(action_to_str(&Action::Pick), "pick");
		assert_eq!(action_to_str(&Action::Reword), "reword");
		assert_eq!(action_to_str(&Action::Edit), "edit");
		assert_eq!(action_to_str(&Action::Squash), "squash");
		assert_eq!(action_to_str(&Action::Fixup), "fixup");
		assert_eq!(action_to_str(&Action::Drop), "drop");
	}
	
	#[test]
	fn action_from_str_all() {
		assert_eq!(action_from_str("pick"), Ok(Action::Pick));
		assert_eq!(action_from_str("p"), Ok(Action::Pick));
		assert_eq!(action_from_str("reword"), Ok(Action::Reword));
		assert_eq!(action_from_str("r"), Ok(Action::Reword));
		assert_eq!(action_from_str("edit"), Ok(Action::Edit));
		assert_eq!(action_from_str("e"), Ok(Action::Edit));
		assert_eq!(action_from_str("squash"), Ok(Action::Squash));
		assert_eq!(action_from_str("s"), Ok(Action::Squash));
		assert_eq!(action_from_str("fixup"), Ok(Action::Fixup));
		assert_eq!(action_from_str("f"), Ok(Action::Fixup));
		assert_eq!(action_from_str("drop"), Ok(Action::Drop));
		assert_eq!(action_from_str("d"), Ok(Action::Drop));
	}
}

