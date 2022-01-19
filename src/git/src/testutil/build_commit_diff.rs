use crate::{Commit, CommitDiff, FileStatus};

/// Builder for creating a new commit diff.
#[derive(Debug)]
pub struct CommitDiffBuilder {
	commit_diff: CommitDiff,
}

impl CommitDiffBuilder {
	/// Create a new instance.
	#[inline]
	#[must_use]
	pub const fn new(commit: Commit) -> Self {
		Self {
			commit_diff: CommitDiff {
				commit,
				parent: None,
				file_statuses: vec![],
				number_files_changed: 0,
				number_insertions: 0,
				number_deletions: 0,
			},
		}
	}

	/// Set the commit.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn commit(mut self, commit: Commit) -> Self {
		self.commit_diff.commit = commit;
		self
	}

	/// Set the parent commit.
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn parent(mut self, parent: Commit) -> Self {
		self.commit_diff.parent = Some(parent);
		self
	}

	/// Set the `FileStatus`es.
	#[inline]
	#[must_use]
	pub fn file_statuses(mut self, statuses: Vec<FileStatus>) -> Self {
		self.commit_diff.file_statuses = statuses;
		self
	}

	/// Set the number of files changed.
	#[inline]
	#[must_use]
	pub const fn number_files_changed(mut self, count: usize) -> Self {
		self.commit_diff.number_files_changed = count;
		self
	}

	/// Set the number of line insertions.
	#[inline]
	#[must_use]
	pub const fn number_insertions(mut self, count: usize) -> Self {
		self.commit_diff.number_insertions = count;
		self
	}

	/// Set the number of line deletions.
	#[inline]
	#[must_use]
	pub const fn number_deletions(mut self, count: usize) -> Self {
		self.commit_diff.number_deletions = count;
		self
	}

	/// Return the built `CommitDiff`
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn build(self) -> CommitDiff {
		self.commit_diff
	}
}
