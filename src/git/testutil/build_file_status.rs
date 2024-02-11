use std::path::{Path, PathBuf};

use crate::git::{Delta, FileMode, FileStatus, Status};

/// Builder for creating a new reference.
#[derive(Debug)]
pub(crate) struct FileStatusBuilder {
	file_status: FileStatus,
}

impl FileStatusBuilder {
	/// Create a new instance of the builder. The new instance will default to an empty file status.
	#[must_use]
	pub(crate) fn new() -> Self {
		Self {
			file_status: FileStatus {
				deltas: vec![],
				destination_is_binary: false,
				destination_mode: FileMode::Normal,
				destination_path: PathBuf::default(),
				largest_new_line_number: 0,
				largest_old_line_number: 0,
				source_is_binary: false,
				source_mode: FileMode::Normal,
				source_path: PathBuf::default(),
				status: Status::Added,
			},
		}
	}

	/// Push a `Delta`.
	#[must_use]
	pub(crate) fn push_delta(mut self, delta: Delta) -> Self {
		self.file_status.add_delta(delta);
		self
	}

	/// Set if the destination is binary.
	#[must_use]
	pub(crate) const fn destination_is_binary(mut self, binary: bool) -> Self {
		self.file_status.destination_is_binary = binary;
		self
	}

	/// Set the destination file mode.
	#[must_use]
	pub(crate) const fn destination_mode(mut self, mode: FileMode) -> Self {
		self.file_status.destination_mode = mode;
		self
	}

	/// Set the destination file path.
	#[must_use]
	pub(crate) fn destination_path<F: AsRef<Path>>(mut self, path: F) -> Self {
		self.file_status.destination_path = PathBuf::from(path.as_ref());
		self
	}

	/// Set the largest new line number.
	#[must_use]
	pub(crate) const fn largest_new_line_number(mut self, largest_new_line_number: u32) -> Self {
		self.file_status.largest_new_line_number = largest_new_line_number;
		self
	}

	/// Set the largest old line number.
	#[must_use]
	pub(crate) const fn largest_old_line_number(mut self, largest_old_line_number: u32) -> Self {
		self.file_status.largest_old_line_number = largest_old_line_number;
		self
	}

	/// Set if the source is binary.
	#[must_use]
	pub(crate) const fn source_is_binary(mut self, binary: bool) -> Self {
		self.file_status.source_is_binary = binary;
		self
	}

	/// Set if the source file mode.
	#[must_use]
	pub(crate) const fn source_mode(mut self, mode: FileMode) -> Self {
		self.file_status.source_mode = mode;
		self
	}

	/// Set the destination file path.
	#[must_use]
	pub(crate) fn source_path<F: AsRef<Path>>(mut self, path: F) -> Self {
		self.file_status.source_path = PathBuf::from(path.as_ref());
		self
	}

	/// Set the status.
	#[must_use]
	pub(crate) const fn status(mut self, status: Status) -> Self {
		self.file_status.status = status;
		self
	}

	/// Build the `FileStatus`
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub(crate) fn build(self) -> FileStatus {
		self.file_status
	}
}
