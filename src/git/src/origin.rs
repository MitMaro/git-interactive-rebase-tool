#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_enums)]
/// The origin of a diff line
pub enum Origin {
	/// A diff line that has been added
	Addition,
	/// A binary file line
	Binary,
	/// A diff line that provides context
	Context,
	/// A diff line that has been deleted
	Deletion,
	/// Diff line header content
	Header,
}

impl From<git2::DiffLineType> for Origin {
	#[inline]
	fn from(diff_line_type: git2::DiffLineType) -> Self {
		match diff_line_type {
			git2::DiffLineType::Context | git2::DiffLineType::ContextEOFNL => Self::Context,
			git2::DiffLineType::Addition | git2::DiffLineType::AddEOFNL => Self::Addition,
			git2::DiffLineType::Deletion | git2::DiffLineType::DeleteEOFNL => Self::Deletion,
			git2::DiffLineType::FileHeader | git2::DiffLineType::HunkHeader => Self::Header,
			git2::DiffLineType::Binary => Self::Binary,
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::context(git2::DiffLineType::Context, Origin::Context)]
	#[case::context_eof(git2::DiffLineType::ContextEOFNL, Origin::Context)]
	#[case::addition(git2::DiffLineType::Addition, Origin::Addition)]
	#[case::addition_eof(git2::DiffLineType::AddEOFNL, Origin::Addition)]
	#[case::deletion(git2::DiffLineType::Deletion, Origin::Deletion)]
	#[case::deletion_eof(git2::DiffLineType::DeleteEOFNL, Origin::Deletion)]
	#[case::file_header(git2::DiffLineType::FileHeader, Origin::Header)]
	#[case::hunk_header(git2::DiffLineType::HunkHeader, Origin::Header)]
	#[case::binary(git2::DiffLineType::Binary, Origin::Binary)]
	fn from_char(#[case] input: git2::DiffLineType, #[case] expected: Origin) {
		assert_eq!(Origin::from(input), expected);
	}
}
