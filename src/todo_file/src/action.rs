use anyhow::{anyhow, Error};

/// Describes an rebase action.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum Action {
	/// A break action.
	Break,
	/// A drop action.
	Drop,
	/// An edit action.
	Edit,
	/// An exec action.
	Exec,
	/// A fixup action.
	Fixup,
	/// A noop action.
	Noop,
	/// A pick action.
	Pick,
	/// A reword action.
	Reword,
	/// A squash action.
	Squash,
	/// A label for a merge block.
	Label,
	/// A reset for a merge block.
	Reset,
	/// A merge action.
	Merge,
}

impl Action {
	/// Get the full string version of the action.
	#[must_use]
	#[inline]
	pub fn as_string(self) -> String {
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

	/// Get the abbreviated version of the action.
	#[must_use]
	#[inline]
	pub fn to_abbreviation(self) -> String {
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

	/// Can the action be changed.
	#[must_use]
	#[inline]
	pub const fn is_static(self) -> bool {
		match self {
			Self::Break | Self::Exec | Self::Noop | Self::Reset | Self::Label | Self::Merge => true,
			Self::Drop | Self::Edit | Self::Fixup | Self::Pick | Self::Reword | Self::Squash => false,
		}
	}
}

impl TryFrom<&str> for Action {
	type Error = Error;

	#[inline]
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

	#[rstest]
	#[case::break_str(Action::Break, "break")]
	#[case::drop(Action::Drop, "drop")]
	#[case::edit(Action::Edit, "edit")]
	#[case::exec(Action::Exec, "exec")]
	#[case::fixup(Action::Fixup, "fixup")]
	#[case::noop(Action::Noop, "noop")]
	#[case::pick(Action::Pick, "pick")]
	#[case::reword(Action::Reword, "reword")]
	#[case::squash(Action::Squash, "squash")]
	#[case::label(Action::Label, "label")]
	#[case::reset(Action::Reset, "reset")]
	#[case::merge(Action::Merge, "merge")]
	fn to_string(#[case] action: Action, #[case] expected: &str) {
		assert_eq!(action.as_string(), expected);
	}

	#[rstest]
	#[case::b("b", Action::Break)]
	#[case::break_str("break", Action::Break)]
	#[case::d("d", Action::Drop)]
	#[case::drop("drop", Action::Drop)]
	#[case::e("e", Action::Edit)]
	#[case::edit("edit", Action::Edit)]
	#[case::x("x", Action::Exec)]
	#[case::exec("exec", Action::Exec)]
	#[case::f("f", Action::Fixup)]
	#[case::fixup("fixup", Action::Fixup)]
	#[case::n("n", Action::Noop)]
	#[case::noop("noop", Action::Noop)]
	#[case::p("p", Action::Pick)]
	#[case::pick("pick", Action::Pick)]
	#[case::r("r", Action::Reword)]
	#[case::reword("reword", Action::Reword)]
	#[case::s("s", Action::Squash)]
	#[case::squash("squash", Action::Squash)]
	#[case::l("l", Action::Label)]
	#[case::label("label", Action::Label)]
	#[case::t("t", Action::Reset)]
	#[case::reset("reset", Action::Reset)]
	#[case::m("m", Action::Merge)]
	#[case::merge("merge", Action::Merge)]
	fn try_from(#[case] action_str: &str, #[case] expected: Action) {
		assert_eq!(Action::try_from(action_str).unwrap(), expected);
	}

	#[test]
	fn action_try_from_invalid() {
		assert_eq!(
			Action::try_from("invalid").unwrap_err().to_string(),
			"Invalid action: invalid"
		);
	}

	#[rstest]
	#[case::b(Action::Break, "b")]
	#[case::d(Action::Drop, "d")]
	#[case::e(Action::Edit, "e")]
	#[case::x(Action::Exec, "x")]
	#[case::f(Action::Fixup, "f")]
	#[case::n(Action::Noop, "n")]
	#[case::p(Action::Pick, "p")]
	#[case::r(Action::Reword, "r")]
	#[case::s(Action::Squash, "s")]
	#[case::l(Action::Label, "l")]
	#[case::t(Action::Reset, "t")]
	#[case::m(Action::Merge, "m")]
	fn to_abbreviation(#[case] action: Action, #[case] expected: &str) {
		assert_eq!(action.to_abbreviation(), expected);
	}

	#[rstest]
	#[case::break_action(Action::Break, true)]
	#[case::drop(Action::Drop, false)]
	#[case::edit(Action::Edit, false)]
	#[case::exec(Action::Exec, true)]
	#[case::fixup(Action::Fixup, false)]
	#[case::noop(Action::Noop, true)]
	#[case::pick(Action::Pick, false)]
	#[case::reword(Action::Reword, false)]
	#[case::squash(Action::Squash, false)]
	#[case::squash(Action::Label, true)]
	#[case::squash(Action::Reset, true)]
	#[case::squash(Action::Merge, true)]
	fn module_lifecycle(#[case] action: Action, #[case] expected: bool) {
		assert_eq!(action.is_static(), expected);
	}
}
