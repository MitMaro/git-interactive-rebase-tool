use crate::diff::{Commit, CommitDiff, FileStatus};

/// Builder for creating a new commit diff.
#[derive(Debug)]
pub(crate) struct CommitDiffBuilder {
	commit: Commit,
	parent: Option<Commit>,
	file_statuses: Vec<FileStatus>,
	number_files_changed: usize,
	number_insertions: usize,
	number_deletions: usize,
}

impl CommitDiffBuilder {
	/// Create a new instance.
	#[must_use]
	pub(crate) const fn new(commit: Commit) -> Self {
		Self {
			commit,
			parent: None,
			file_statuses: vec![],
			number_files_changed: 0,
			number_insertions: 0,
			number_deletions: 0,
		}
	}

	/// Set the commit.
	#[must_use]
	pub(crate) fn commit(mut self, commit: Commit) -> Self {
		self.commit = commit;
		self
	}

	/// Set the parent commit.
	#[must_use]
	pub(crate) fn parent(mut self, parent: Commit) -> Self {
		self.parent = Some(parent);
		self
	}

	/// Set the `FileStatus`es.
	#[must_use]
	pub(crate) fn file_statuses(mut self, statuses: Vec<FileStatus>) -> Self {
		self.file_statuses = statuses;
		self
	}

	/// Set the number of files changed.
	#[must_use]
	pub(crate) const fn number_files_changed(mut self, count: usize) -> Self {
		self.number_files_changed = count;
		self
	}

	/// Set the number of line insertions.
	#[must_use]
	pub(crate) const fn number_insertions(mut self, count: usize) -> Self {
		self.number_insertions = count;
		self
	}

	/// Set the number of line deletions.
	#[must_use]
	pub(crate) const fn number_deletions(mut self, count: usize) -> Self {
		self.number_deletions = count;
		self
	}

	/// Return the built `CommitDiff`
	#[must_use]
	pub(crate) fn build(self) -> CommitDiff {
		let mut diff = CommitDiff::new();
		diff.reset(self.commit, self.parent);
		diff.update(
			self.file_statuses,
			self.number_files_changed,
			self.number_insertions,
			self.number_deletions,
		);

		diff
	}
}
