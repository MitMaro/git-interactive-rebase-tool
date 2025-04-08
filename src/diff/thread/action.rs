use std::fmt::{Debug, Formatter};

#[derive(PartialEq)]
pub(crate) enum Action {
	StatusChange,
	Load(String),
}

impl Debug for Action {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::StatusChange => write!(f, "StatusChange"),
			Self::Load(ref hash) => write!(f, "Load({hash})"),
		}
	}
}
#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::status_change(Action::StatusChange, "StatusChange")]
	#[case::cont(Action::Load(String::from("abc123")), "Load(abc123)")]
	fn debug(#[case] action: Action, #[case] expected: &str) {
		assert_eq!(format!("{action:?}"), expected);
	}
}
