
#[derive(PartialEq, Debug)]
pub enum Action {
	Drop,
	Edit,
	Exec,
	Fixup,
	Pick,
	Reword,
	Squash,
}

pub fn action_from_str(s: &str) -> Result<Action, String> {
	match s {
		"drop" | "d" => Ok(Action::Drop),
		"edit" | "e" => Ok(Action::Edit),
		"exec" | "x" => Ok(Action::Exec),
		"fixup" | "f" => Ok(Action::Fixup),
		"pick" | "p" => Ok(Action::Pick),
		"reword" | "r" => Ok(Action::Reword),
		"squash" | "s" => Ok(Action::Squash),
		_ => Err(format!("Invalid action: {}", s))
	}
}

pub fn action_to_str(action: &Action) -> String {
	String::from(match *action {
		Action::Drop => "drop",
		Action::Edit => "edit",
		Action::Exec => "exec",
		Action::Fixup => "fixup",
		Action::Pick => "pick",
		Action::Reword => "reword",
		Action::Squash => "squash",
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
		assert_eq!(action_to_str(&Action::Drop), "drop");
		assert_eq!(action_to_str(&Action::Edit), "edit");
		assert_eq!(action_to_str(&Action::Exec), "exec");
		assert_eq!(action_to_str(&Action::Fixup), "fixup");
		assert_eq!(action_to_str(&Action::Pick), "pick");
		assert_eq!(action_to_str(&Action::Reword), "reword");
		assert_eq!(action_to_str(&Action::Squash), "squash");
	}
	
	#[test]
	fn action_from_str_all() {
		assert_eq!(action_from_str("d"), Ok(Action::Drop));
		assert_eq!(action_from_str("drop"), Ok(Action::Drop));
		assert_eq!(action_from_str("e"), Ok(Action::Edit));
		assert_eq!(action_from_str("edit"), Ok(Action::Edit));
		assert_eq!(action_from_str("x"), Ok(Action::Exec));
		assert_eq!(action_from_str("exec"), Ok(Action::Exec));
		assert_eq!(action_from_str("f"), Ok(Action::Fixup));
		assert_eq!(action_from_str("fixup"), Ok(Action::Fixup));
		assert_eq!(action_from_str("p"), Ok(Action::Pick));
		assert_eq!(action_from_str("pick"), Ok(Action::Pick));
		assert_eq!(action_from_str("r"), Ok(Action::Reword));
		assert_eq!(action_from_str("reword"), Ok(Action::Reword));
		assert_eq!(action_from_str("s"), Ok(Action::Squash));
		assert_eq!(action_from_str("squash"), Ok(Action::Squash));
	}
}

