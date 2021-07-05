#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum Origin {
	Context,
	Addition,
	Deletion,
}

impl From<char> for Origin {
	fn from(c: char) -> Self {
		match c {
			'+' | '>' => Self::Addition,
			'-' | '<' => Self::Deletion,
			// ' ',  '='
			_ => Self::Context,
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::space(' ', &Origin::Context)]
	#[case::equals('=', &Origin::Context)]
	#[case::plus('+', &Origin::Addition)]
	#[case::greater_than('>', &Origin::Addition)]
	#[case::minus('-', &Origin::Deletion)]
	#[case::less_than('-', &Origin::Deletion)]
	#[case::other('a', &Origin::Context)]
	fn from_char(#[case] input: char, #[case] expected: &Origin) {
		assert_eq!(&Origin::from(input), expected);
	}
}
