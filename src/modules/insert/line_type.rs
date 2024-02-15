use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LineType {
	Cancel,
	Pick,
	Exec,
	Label,
	Merge,
	Reset,
	UpdateRef,
}

impl Display for LineType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::Cancel => write!(f, "<cancel>"),
			Self::Pick => write!(f, "pick"),
			Self::Exec => write!(f, "exec"),
			Self::Label => write!(f, "label"),
			Self::Merge => write!(f, "merge"),
			Self::Reset => write!(f, "reset"),
			Self::UpdateRef => write!(f, "update-ref"),
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::cancel(&LineType::Cancel, "<cancel>")]
	#[case::pick(&LineType::Pick, "pick")]
	#[case::exec(&LineType::Exec, "exec")]
	#[case::label(&LineType::Label, "label")]
	#[case::merge(&LineType::Merge, "merge")]
	#[case::reset(&LineType::Reset, "reset")]
	#[case::update_ref(&LineType::UpdateRef, "update-ref")]
	fn to_string(#[case] line_type: &LineType, #[case] expected: &str) {
		assert_eq!(line_type.to_string(), String::from(expected));
	}
}
