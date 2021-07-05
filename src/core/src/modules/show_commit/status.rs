use git2::Delta;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Status {
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

impl From<Delta> for Status {
	fn from(delta: Delta) -> Self {
		match delta {
			Delta::Added => Self::Added,
			Delta::Copied => Self::Copied,
			Delta::Deleted => Self::Deleted,
			Delta::Modified => Self::Modified,
			Delta::Renamed => Self::Renamed,
			Delta::Typechange => Self::Typechange,
			Delta::Ignored | Delta::Conflicted | Delta::Unmodified | Delta::Unreadable | Delta::Untracked => {
				Self::Other
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest(
		input,
		expected,
		case::added(Delta::Added, &Status::Added),
		case::copied(Delta::Copied, &Status::Copied),
		case::deleted(Delta::Deleted, &Status::Deleted),
		case::modified(Delta::Modified, &Status::Modified),
		case::renamed(Delta::Renamed, &Status::Renamed),
		case::typechange(Delta::Typechange, &Status::Typechange),
		case::ignored(Delta::Ignored, &Status::Other),
		case::conflicted(Delta::Conflicted, &Status::Other),
		case::unmodified(Delta::Unmodified, &Status::Other),
		case::unreadable(Delta::Unreadable, &Status::Other),
		case::untracked(Delta::Untracked, &Status::Other)
	)]
	fn from_delta(input: Delta, expected: &Status) {
		assert_eq!(&Status::from(input), expected);
	}
}
