use crate::git::{Commit, FileStatus};

/// Represents a commit with a diff
#[derive(Debug)]
pub(crate) struct CommitDiff {
	commit: Commit,
	parent: Option<Commit>,
	file_statuses: Vec<FileStatus>,
	number_files_changed: usize,
	number_insertions: usize,
	number_deletions: usize,
}

impl CommitDiff {
	pub(crate) fn new(
		commit: Commit,
		parent: Option<Commit>,
		file_statuses: Vec<FileStatus>,
		number_files_changed: usize,
		number_insertions: usize,
		number_deletions: usize,
	) -> Self {
		CommitDiff {
			commit,
			parent,
			file_statuses,
			number_files_changed,
			number_insertions,
			number_deletions,
		}
	}

	/// The commit of the diff
	#[must_use]
	pub(crate) const fn commit(&self) -> &Commit {
		&self.commit
	}

	/// The parent commit for the diff
	#[must_use]
	#[allow(dead_code)]
	pub(crate) const fn parent(&self) -> &Option<Commit> {
		&self.parent
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
}

#[cfg(test)]
mod tests {
	use claims::assert_some_eq;

	use crate::{
		git::{Delta, DiffLine, FileMode, FileStatus, FileStatusBuilder, Origin, Status},
		test_helpers::builders::{CommitBuilder, CommitDiffBuilder},
	};

	#[test]
	fn commit() {
		let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789ABCDEF").build()).build();
		assert_eq!(diff.commit(), &CommitBuilder::new("0123456789ABCDEF").build());
	}

	#[test]
	fn parent() {
		let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789ABCDEF").build())
			.parent(CommitBuilder::new("ABCDEF0123456789").build())
			.build();
		assert_some_eq!(diff.parent(), &CommitBuilder::new("ABCDEF0123456789").build());
	}

	#[test]
	fn file_statuses() {
		let mut builder = FileStatusBuilder::new();
		builder.add_file_stat(FileStatus::new(
			"foo",
			FileMode::Normal,
			false,
			"foo",
			FileMode::Normal,
			false,
			Status::Modified,
		));
		builder.add_delta(Delta::new("name", 0, 0, 0, 1));
		builder.add_diff_line(DiffLine::new(Origin::Addition, "line", None, Some(1), false));
		let file_statuses = builder.build();
		let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789ABCDEF").build())
			.file_statuses(file_statuses)
			.build();
		assert_eq!(diff.file_statuses()[0].source_path().to_string_lossy(), "foo");
	}

	#[test]
	fn number_files_changed() {
		let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789ABCDEF").build())
			.number_files_changed(1)
			.build();
		assert_eq!(diff.number_files_changed(), 1);
	}

	#[test]
	fn number_insertions() {
		let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789ABCDEF").build())
			.number_insertions(2)
			.build();
		assert_eq!(diff.number_insertions(), 2);
	}

	#[test]
	fn number_deletions() {
		let diff = CommitDiffBuilder::new(CommitBuilder::new("0123456789ABCDEF").build())
			.number_deletions(3)
			.build();
		assert_eq!(diff.number_deletions(), 3);
	}
}
