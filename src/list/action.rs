use anyhow::{anyhow, Error};
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
	type Error = Error;

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
			_ => Err(anyhow!("Invalid action: {}", s)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Action;
	use super::TryFrom;

	macro_rules! test_action_to_string {
		($name:ident, $action:expr, $expected:expr) => {
			concat_idents::concat_idents!(test_name = action_as_string_, $name {
				#[test]
				fn test_name() {
					assert_eq!($action.as_string(), $expected);
				}
			});
		}
	}

	macro_rules! test_action_try_from {
		($name:ident, $action_string:expr, $expected:expr) => {
			concat_idents::concat_idents!(test_name = action_try_from_, $name {
				#[test]
				fn test_name() {
					assert_eq!(Action::try_from($action_string).unwrap(), $expected);
				}
			});
		}
	}

	macro_rules! test_action_to_abbreviation {
		($name:ident, $action:expr, $expected:expr) => {
			concat_idents::concat_idents!(test_name = action_to_abbreviation_, $name {
				#[test]
				fn test_name() {
					assert_eq!($action.to_abbreviation(), $expected);
				}
			});
		}
	}

	test_action_to_string!(break_str, Action::Break, "break");
	test_action_to_string!(drop, Action::Drop, "drop");
	test_action_to_string!(edit, Action::Edit, "edit");
	test_action_to_string!(exec, Action::Exec, "exec");
	test_action_to_string!(fixup, Action::Fixup, "fixup");
	test_action_to_string!(noop, Action::Noop, "noop");
	test_action_to_string!(pick, Action::Pick, "pick");
	test_action_to_string!(reword, Action::Reword, "reword");
	test_action_to_string!(squash, Action::Squash, "squash");

	test_action_try_from!(b, "b", Action::Break);
	test_action_try_from!(break_str, "break", Action::Break);
	test_action_try_from!(d, "d", Action::Drop);
	test_action_try_from!(drop, "drop", Action::Drop);
	test_action_try_from!(e, "e", Action::Edit);
	test_action_try_from!(edit, "edit", Action::Edit);
	test_action_try_from!(x, "x", Action::Exec);
	test_action_try_from!(exec, "exec", Action::Exec);
	test_action_try_from!(f, "f", Action::Fixup);
	test_action_try_from!(fixup, "fixup", Action::Fixup);
	test_action_try_from!(n, "n", Action::Noop);
	test_action_try_from!(noop, "noop", Action::Noop);
	test_action_try_from!(p, "p", Action::Pick);
	test_action_try_from!(pick, "pick", Action::Pick);
	test_action_try_from!(r, "r", Action::Reword);
	test_action_try_from!(reword, "reword", Action::Reword);
	test_action_try_from!(s, "s", Action::Squash);
	test_action_try_from!(squash, "squash", Action::Squash);

	#[test]
	fn action_try_from_() {
		assert_eq!(
			Action::try_from("invalid").unwrap_err().to_string(),
			"Invalid action: invalid"
		);
	}

	test_action_to_abbreviation!(b, Action::Break, "b");
	test_action_to_abbreviation!(d, Action::Drop, "d");
	test_action_to_abbreviation!(e, Action::Edit, "e");
	test_action_to_abbreviation!(x, Action::Exec, "x");
	test_action_to_abbreviation!(f, Action::Fixup, "f");
	test_action_to_abbreviation!(n, Action::Noop, "n");
	test_action_to_abbreviation!(p, Action::Pick, "p");
	test_action_to_abbreviation!(r, Action::Reword, "r");
	test_action_to_abbreviation!(s, Action::Squash, "s");
}
