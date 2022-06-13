use crate::{commit::Commit, file_status::FileStatus};

/// Represents a commit with a diff
#[derive(Debug)]
pub struct CommitDiff {
	pub(crate) commit: Commit,
	pub(crate) parent: Option<Commit>,
	pub(crate) file_statuses: Vec<FileStatus>,
	pub(crate) number_files_changed: usize,
	pub(crate) number_insertions: usize,
	pub(crate) number_deletions: usize,
}

impl CommitDiff {
	/// The commit of the diff
	#[inline]
	#[must_use]
	pub const fn commit(&self) -> &Commit {
		&self.commit
	}

	/// The parent commit for the diff
	#[inline]
	#[must_use]
	pub const fn parent(&self) -> &Option<Commit> {
		&self.parent
	}

	/// The file statuses
	#[inline]
	#[must_use]
	pub const fn file_statuses(&self) -> &Vec<FileStatus> {
		&self.file_statuses
	}

	/// The total number of files changed in the diff
	#[inline]
	#[must_use]
	pub const fn number_files_changed(&self) -> usize {
		self.number_files_changed
	}

	/// The total number of insertions in the diff
	#[inline]
	#[must_use]
	pub const fn number_insertions(&self) -> usize {
		self.number_insertions
	}

	/// The total number of deletions in the diff
	#[inline]
	#[must_use]
	pub const fn number_deletions(&self) -> usize {
		self.number_deletions
	}
}

#[cfg(test)]
mod tests {
	use claim::assert_some_eq;

	use crate::{
		delta::Delta,
		diff_line::DiffLine,
		file_mode::FileMode,
		file_status::FileStatus,
		file_status_builder::FileStatusBuilder,
		status::Status,
		testutil::{CommitBuilder, CommitDiffBuilder},
		Origin,
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
		assert_eq!(diff.file_statuses()[0].source_path.to_string_lossy(), "foo");
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
