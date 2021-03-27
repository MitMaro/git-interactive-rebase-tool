#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
	Exec,
	Label,
	Reset,
	Merge,
	Cancel,
}

impl ToString for LineType {
	fn to_string(&self) -> String {
		match *self {
			Self::Exec => String::from("exec"),
			Self::Label => String::from("label"),
			Self::Reset => String::from("reset"),
			Self::Merge => String::from("merge"),
			Self::Cancel => String::from("<cancel>"),
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest(
		line_type,
		expected,
		case::exec(&LineType::Exec, "exec"),
		case::exec(&LineType::Label, "label"),
		case::exec(&LineType::Reset, "reset"),
		case::exec(&LineType::Merge, "merge"),
		case::exec(&LineType::Cancel, "<cancel>"),
	)]
	fn to_string(line_type: &LineType, expected: &str) {
		assert_eq!(line_type.to_string(), String::from(expected));
	}
}
