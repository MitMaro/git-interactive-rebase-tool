use crate::diff::{Commit, FileStatus};

/// Represents a commit with a diff
#[derive(Debug, Clone)]
pub(crate) struct CommitDiff {
	commit: Commit,
	parent: Option<Commit>,
	file_statuses: Vec<FileStatus>,
	number_files_changed: usize,
	number_insertions: usize,
	number_deletions: usize,
}

impl CommitDiff {
	pub(crate) fn new() -> Self {
		CommitDiff {
			commit: Commit::empty(),
			parent: None,
			file_statuses: vec![],
			number_files_changed: 0,
			number_insertions: 0,
			number_deletions: 0,
		}
	}

	/// The commit of the diff
	#[must_use]
	pub(crate) const fn commit(&self) -> &Commit {
		&self.commit
	}

	/// The parent commit for the diff
	#[must_use]
	#[expect(dead_code, reason = "Available for future use.")]
	pub(crate) const fn parent(&self) -> Option<&Commit> {
		self.parent.as_ref()
	}

	/// The file statuses
	#[must_use]
	pub(crate) const fn file_statuses(&self) -> &Vec<FileStatus> {
		&self.file_statuses
	}

	/// The total number of files changed in the diff
	#[must_use]
	pub(crate) const fn number_files_changed(&self) -> usize {
		self.number_files_changed
	}

	/// The total number of insertions in the diff
	#[must_use]
	pub(crate) const fn number_insertions(&self) -> usize {
		self.number_insertions
	}

	/// The total number of deletions in the diff
	#[must_use]
	pub(crate) const fn number_deletions(&self) -> usize {
		self.number_deletions
	}

	/// Update the details of the diff
	pub(crate) fn update(
		&mut self,
		file_statuses: Vec<FileStatus>,
		number_files_changed: usize,
		number_insertions: usize,
		number_deletions: usize,
	) {
		self.file_statuses = file_statuses;
		self.number_files_changed = number_files_changed;
		self.number_insertions = number_insertions;
		self.number_deletions = number_deletions;
	}

	/// Reset the diff back to an empty state
	pub(crate) fn reset(&mut self, commit: Commit, parent: Option<Commit>) {
		self.commit = commit;
		self.parent = parent;
		self.file_statuses.clear();
		self.number_files_changed = 0;
		self.number_insertions = 0;
		self.number_deletions = 0;
	}

	pub(crate) fn clear(&mut self) {
		self.commit = Commit::empty();
		self.parent = None;
		self.file_statuses.clear();
		self.number_files_changed = 0;
		self.number_insertions = 0;
		self.number_deletions = 0;
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_none, assert_some_eq};

	use crate::{
		assert_empty,
		assert_not_empty,
		diff::{Commit, CommitDiff, FileMode, FileStatus, Status},
	};

	#[test]
	fn new() {
		let diff = CommitDiff::new();
		assert_eq!(diff.commit(), &Commit::empty());
		assert_none!(diff.parent());
		assert_empty!(diff.file_statuses());
		assert_eq!(diff.number_files_changed(), 0);
		assert_eq!(diff.number_insertions(), 0);
		assert_eq!(diff.number_deletions(), 0);
	}

	#[test]
	fn reset_update() {
		let mut diff = CommitDiff::new();
		diff.reset(Commit::new_with_hash("abc123"), Some(Commit::new_with_hash("def456")));
		diff.update(
			vec![FileStatus::new(
				"foo",
				FileMode::Normal,
				false,
				"foo",
				FileMode::Normal,
				false,
				Status::Modified,
			)],
			11,
			22,
			33,
		);
		assert_eq!(diff.commit().hash(), "abc123");
		assert_some_eq!(diff.parent().map(|d| d.hash()), "def456");
		assert_not_empty!(diff.file_statuses());
		assert_eq!(diff.number_files_changed(), 11);
		assert_eq!(diff.number_insertions(), 22);
		assert_eq!(diff.number_deletions(), 33);
	}

	#[test]
	fn reset_clear() {
		let mut diff = CommitDiff::new();
		diff.reset(Commit::new_with_hash("abc123"), Some(Commit::new_with_hash("def456")));
		diff.update(
			vec![FileStatus::new(
				"foo",
				FileMode::Normal,
				false,
				"foo",
				FileMode::Normal,
				false,
				Status::Modified,
			)],
			11,
			22,
			33,
		);
		diff.clear();
		assert_eq!(diff.commit(), &Commit::empty());
		assert_none!(diff.parent());
		assert_empty!(diff.file_statuses());
		assert_eq!(diff.number_files_changed(), 0);
		assert_eq!(diff.number_insertions(), 0);
		assert_eq!(diff.number_deletions(), 0);
	}
}
