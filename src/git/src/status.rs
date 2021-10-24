/// Represents the type of change of a diff entry
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum Status {
	/// Entry does not exist in old version
	Added,
	/// Entry does not exist in new version
	Deleted,
	/// Entry content changed between old and new
	Modified,
	/// Entry was renamed between old and new
	Renamed,
	/// Entry was copied from another old entry
	Copied,
	/// Type of entry changed between old and new
	Typechange,
	/// Other type of change not normally found in a rebase
	Other,
}

impl Status {
	/// Create a new status for a `git2::Delta`.
	#[must_use]
	#[inline]
	pub const fn from(delta: git2::Delta) -> Self {
		match delta {
			git2::Delta::Added => Self::Added,
			git2::Delta::Copied => Self::Copied,
			git2::Delta::Deleted => Self::Deleted,
			git2::Delta::Modified => Self::Modified,
			git2::Delta::Renamed => Self::Renamed,
			git2::Delta::Typechange => Self::Typechange,
			git2::Delta::Ignored
			| git2::Delta::Conflicted
			| git2::Delta::Unmodified
			| git2::Delta::Unreadable
			| git2::Delta::Untracked => Self::Other,
		}
	}
}

#[cfg(test)]
mod tests {
	use git2::Delta;
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::added(Delta::Added, Status::Added)]
	#[case::copied(Delta::Copied, Status::Copied)]
	#[case::deleted(Delta::Deleted, Status::Deleted)]
	#[case::modified(Delta::Modified, Status::Modified)]
	#[case::renamed(Delta::Renamed, Status::Renamed)]
	#[case::typechange(Delta::Typechange, Status::Typechange)]
	#[case::ignored(Delta::Ignored, Status::Other)]
	#[case::conflicted(Delta::Conflicted, Status::Other)]
	#[case::unmodified(Delta::Unmodified, Status::Other)]
	#[case::unreadable(Delta::Unreadable, Status::Other)]
	#[case::untracked(Delta::Untracked, Status::Other)]
	fn from_delta(#[case] input: Delta, #[case] expected: Status) {
		assert_eq!(Status::from(input), expected);
	}
}
