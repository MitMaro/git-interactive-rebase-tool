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

impl Status {
	pub(super) fn new_from_git_delta(delta: Delta) -> Self {
		match delta {
			Delta::Added => Status::Added,
			Delta::Conflicted => Status::Other,
			Delta::Copied => Status::Copied,
			Delta::Deleted => Status::Deleted,
			Delta::Ignored => Status::Other,
			Delta::Modified => Status::Modified,
			Delta::Renamed => Status::Renamed,
			Delta::Typechange => Status::Typechange,
			Delta::Unmodified => Status::Other,
			Delta::Unreadable => Status::Other,
			Delta::Untracked => Status::Other,
		}
	}
}
