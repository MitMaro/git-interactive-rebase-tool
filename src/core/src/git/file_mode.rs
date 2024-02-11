/// Represents the mode of a file
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum FileMode {
	/// A normal type of file
	Normal,
	/// A file that is executable
	Executable,
	/// A file that is a link
	Link,
	/// Any other file types
	Other,
}

impl FileMode {
	pub(crate) const fn from(file_mode: git2::FileMode) -> Self {
		match file_mode {
			git2::FileMode::Commit | git2::FileMode::Tree | git2::FileMode::Unreadable => Self::Other,
			git2::FileMode::Blob | git2::FileMode::BlobGroupWritable => Self::Normal,
			git2::FileMode::BlobExecutable => Self::Executable,
			git2::FileMode::Link => Self::Link,
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::commit(git2::FileMode::Commit, FileMode::Other)]
	#[case::commit(git2::FileMode::Tree, FileMode::Other)]
	#[case::commit(git2::FileMode::Unreadable, FileMode::Other)]
	#[case::commit(git2::FileMode::Blob, FileMode::Normal)]
	#[case::commit(git2::FileMode::BlobExecutable, FileMode::Executable)]
	#[case::commit(git2::FileMode::Link, FileMode::Link)]
	fn from(#[case] git2_file_mode: git2::FileMode, #[case] expected_file_mode: FileMode) {
		assert_eq!(FileMode::from(git2_file_mode), expected_file_mode);
	}
}
