use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
	Break,
	Drop,
	Edit,
	Exec,
	Fixup,
	Noop,
	Pick,
	Reword,
	Squash,
}

impl Action {
	pub(crate) fn as_string(self) -> String {
		String::from(match self {
			Self::Break => "break",
			Self::Drop => "drop",
			Self::Edit => "edit",
			Self::Exec => "exec",
			Self::Fixup => "fixup",
			Self::Noop => "noop",
			Self::Pick => "pick",
			Self::Reword => "reword",
			Self::Squash => "squash",
		})
	}

	pub(super) fn to_abbreviation(self) -> String {
		String::from(match self {
			Self::Break => "b",
			Self::Drop => "d",
			Self::Edit => "e",
			Self::Exec => "x",
			Self::Fixup => "f",
			Self::Noop => "n",
			Self::Pick => "p",
			Self::Reword => "r",
			Self::Squash => "s",
		})
	}
}

impl TryFrom<&str> for Action {
	type Error = String;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s {
			"break" | "b" => Ok(Self::Break),
			"drop" | "d" => Ok(Self::Drop),
			"edit" | "e" => Ok(Self::Edit),
			"exec" | "x" => Ok(Self::Exec),
			"fixup" | "f" => Ok(Self::Fixup),
			"noop" | "n" => Ok(Self::Noop),
			"pick" | "p" => Ok(Self::Pick),
			"reword" | "r" => Ok(Self::Reword),
			"squash" | "s" => Ok(Self::Squash),
			_ => Err(format!("Invalid action: {}", s)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Action;
	use super::TryFrom;

	#[test]
	fn action_to_str_break() {
		assert_eq!(Action::Break.as_string(), "break");
	}

	#[test]
	fn action_to_str_drop() {
		assert_eq!(Action::Drop.as_string(), "drop");
	}

	#[test]
	fn action_to_str_edit() {
		assert_eq!(Action::Edit.as_string(), "edit");
	}

	#[test]
	fn action_to_str_exec() {
		assert_eq!(Action::Exec.as_string(), "exec");
	}

	#[test]
	fn action_to_str_fixup() {
		assert_eq!(Action::Fixup.as_string(), "fixup");
	}

	#[test]
	fn action_to_str_noop() {
		assert_eq!(Action::Noop.as_string(), "noop");
	}

	#[test]
	fn action_to_str_pick() {
		assert_eq!(Action::Pick.as_string(), "pick");
	}

	#[test]
	fn action_to_str_reword() {
		assert_eq!(Action::Reword.as_string(), "reword");
	}

	#[test]
	fn action_to_str_squash() {
		assert_eq!(Action::Squash.as_string(), "squash");
	}

	#[test]
	fn action_from_str_b() {
		assert_eq!(Action::try_from("b").unwrap(), Action::Break);
	}

	#[test]
	fn action_from_str_break() {
		assert_eq!(Action::try_from("break").unwrap(), Action::Break);
	}

	#[test]
	fn action_from_str_d() {
		assert_eq!(Action::try_from("d").unwrap(), Action::Drop);
	}

	#[test]
	fn action_from_str_drop() {
		assert_eq!(Action::try_from("drop").unwrap(), Action::Drop);
	}

	#[test]
	fn action_from_str_e() {
		assert_eq!(Action::try_from("e").unwrap(), Action::Edit);
	}

	#[test]
	fn action_from_str_edit() {
		assert_eq!(Action::try_from("edit").unwrap(), Action::Edit);
	}

	#[test]
	fn action_from_str_x() {
		assert_eq!(Action::try_from("x").unwrap(), Action::Exec);
	}

	#[test]
	fn action_from_str_exec() {
		assert_eq!(Action::try_from("exec").unwrap(), Action::Exec);
	}

	#[test]
	fn action_from_str_f() {
		assert_eq!(Action::try_from("f").unwrap(), Action::Fixup);
	}

	#[test]
	fn action_from_str_fixup() {
		assert_eq!(Action::try_from("fixup").unwrap(), Action::Fixup);
	}

	#[test]
	fn action_from_str_n() {
		assert_eq!(Action::try_from("n").unwrap(), Action::Noop);
	}

	#[test]
	fn action_from_str_noop() {
		assert_eq!(Action::try_from("noop").unwrap(), Action::Noop);
	}

	#[test]
	fn action_from_str_p() {
		assert_eq!(Action::try_from("p").unwrap(), Action::Pick);
	}

	#[test]
	fn action_from_str_pick() {
		assert_eq!(Action::try_from("pick").unwrap(), Action::Pick);
	}

	#[test]
	fn action_from_str_r() {
		assert_eq!(Action::try_from("r").unwrap(), Action::Reword);
	}

	#[test]
	fn action_from_str_reword() {
		assert_eq!(Action::try_from("reword").unwrap(), Action::Reword);
	}

	#[test]
	fn action_from_str_s() {
		assert_eq!(Action::try_from("s").unwrap(), Action::Squash);
	}

	#[test]
	fn action_from_str_squash() {
		assert_eq!(Action::try_from("squash").unwrap(), Action::Squash);
	}

	#[test]
	fn action_from_str_invalid_action() {
		assert_eq!(Action::try_from("invalid").unwrap_err(), "Invalid action: invalid");
	}

	#[test]
	fn action_to_abbreviation_break() {
		assert_eq!(Action::Break.to_abbreviation(), "b");
	}

	#[test]
	fn action_to_abbreviation_drop() {
		assert_eq!(Action::Drop.to_abbreviation(), "d");
	}

	#[test]
	fn action_to_abbreviation_edit() {
		assert_eq!(Action::Edit.to_abbreviation(), "e");
	}

	#[test]
	fn action_to_abbreviation_exec() {
		assert_eq!(Action::Exec.to_abbreviation(), "x");
	}

	#[test]
	fn action_to_abbreviation_fixup() {
		assert_eq!(Action::Fixup.to_abbreviation(), "f");
	}

	#[test]
	fn action_to_abbreviation_noop() {
		assert_eq!(Action::Noop.to_abbreviation(), "n");
	}

	#[test]
	fn action_to_abbreviation_pick() {
		assert_eq!(Action::Pick.to_abbreviation(), "p");
	}

	#[test]
	fn action_to_abbreviation_reword() {
		assert_eq!(Action::Reword.to_abbreviation(), "r");
	}

	#[test]
	fn action_to_abbreviation_squash() {
		assert_eq!(Action::Squash.to_abbreviation(), "s");
	}
}
