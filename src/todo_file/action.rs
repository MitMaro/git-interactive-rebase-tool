use std::convert::TryFrom;

use anyhow::{anyhow, Error};

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
	Label,
	Reset,
	Merge,
}

impl Action {
	pub(crate) fn as_string(self) -> String {
		String::from(match self {
			Self::Break => "break",
			Self::Drop => "drop",
			Self::Edit => "edit",
			Self::Exec => "exec",
			Self::Fixup => "fixup",
			Self::Label => "label",
			Self::Merge => "merge",
			Self::Noop => "noop",
			Self::Pick => "pick",
			Self::Reset => "reset",
			Self::Reword => "reword",
			Self::Squash => "squash",
		})
	}

	pub(crate) fn to_abbreviation(self) -> String {
		String::from(match self {
			Self::Break => "b",
			Self::Drop => "d",
			Self::Edit => "e",
			Self::Exec => "x",
			Self::Fixup => "f",
			Self::Label => "l",
			Self::Merge => "m",
			Self::Noop => "n",
			Self::Pick => "p",
			Self::Reset => "t",
			Self::Reword => "r",
			Self::Squash => "s",
		})
	}

	pub const fn is_static(self) -> bool {
		match self {
			Self::Break | Self::Exec | Self::Noop | Self::Reset | Self::Label | Self::Merge => true,
			Self::Drop | Self::Edit | Self::Fixup | Self::Pick | Self::Reword | Self::Squash => false,
		}
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
			"label" | "l" => Ok(Self::Label),
			"reset" | "t" => Ok(Self::Reset),
			"merge" | "m" => Ok(Self::Merge),
			_ => Err(anyhow!("Invalid action: {}", s)),
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

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
	test_action_to_string!(label, Action::Label, "label");
	test_action_to_string!(reset, Action::Reset, "reset");
	test_action_to_string!(merge, Action::Merge, "merge");

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
	test_action_try_from!(l, "l", Action::Label);
	test_action_try_from!(label, "label", Action::Label);
	test_action_try_from!(t, "t", Action::Reset);
	test_action_try_from!(reset, "reset", Action::Reset);
	test_action_try_from!(m, "m", Action::Merge);
	test_action_try_from!(merge, "merge", Action::Merge);

	#[test]
	fn action_try_from_invalid() {
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
	test_action_to_abbreviation!(l, Action::Label, "l");
	test_action_to_abbreviation!(t, Action::Reset, "t");
	test_action_to_abbreviation!(m, Action::Merge, "m");

	#[rstest(
		action,
		expected,
		case::break_action(Action::Break, true),
		case::drop(Action::Drop, false),
		case::edit(Action::Edit, false),
		case::exec(Action::Exec, true),
		case::fixup(Action::Fixup, false),
		case::noop(Action::Noop, true),
		case::pick(Action::Pick, false),
		case::reword(Action::Reword, false),
		case::squash(Action::Squash, false),
		case::squash(Action::Label, true),
		case::squash(Action::Reset, true),
		case::squash(Action::Merge, true)
	)]
	fn module_lifecycle(action: Action, expected: bool) {
		assert_eq!(action.is_static(), expected);
	}
}
