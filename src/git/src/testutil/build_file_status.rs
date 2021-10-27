use std::path::{Path, PathBuf};

use crate::{Delta, FileMode, FileStatus, Status};

/// Builder for creating a new reference.
#[derive(Debug)]
pub struct FileStatusBuilder {
	file_status: FileStatus,
}

impl FileStatusBuilder {
	/// Create a new instance of the builder. The new instance will default to an empty file status.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
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
	#[inline]
	#[must_use]
	pub fn push_delta(mut self, delta: Delta) -> Self {
		self.file_status.add_delta(delta);
		self
	}

	/// Set if the destination is binary.
	#[inline]
	#[must_use]
	pub const fn destination_is_binary(mut self, binary: bool) -> Self {
		self.file_status.destination_is_binary = binary;
		self
	}

	/// Set the destination file mode.
	#[inline]
	#[must_use]
	pub const fn destination_mode(mut self, mode: FileMode) -> Self {
		self.file_status.destination_mode = mode;
		self
	}

	/// Set the destination file path.
	#[inline]
	#[must_use]
	pub fn destination_path<F: AsRef<Path>>(mut self, path: F) -> Self {
		self.file_status.destination_path = PathBuf::from(path.as_ref());
		self
	}

	/// Set the largest new line number.
	#[inline]
	#[must_use]
	pub const fn largest_new_line_number(mut self, largest_new_line_number: u32) -> Self {
		self.file_status.largest_new_line_number = largest_new_line_number;
		self
	}

	/// Set the largest old line number.
	#[inline]
	#[must_use]
	pub const fn largest_old_line_number(mut self, largest_old_line_number: u32) -> Self {
		self.file_status.largest_old_line_number = largest_old_line_number;
		self
	}

	/// Set if the source is binary.
	#[inline]
	#[must_use]
	pub const fn source_is_binary(mut self, binary: bool) -> Self {
		self.file_status.source_is_binary = binary;
		self
	}

	/// Set if the source file mode.
	#[inline]
	#[must_use]
	pub const fn source_mode(mut self, mode: FileMode) -> Self {
		self.file_status.source_mode = mode;
		self
	}

	/// Set the destination file path.
	#[inline]
	#[must_use]
	pub fn source_path<F: AsRef<Path>>(mut self, path: F) -> Self {
		self.file_status.source_path = PathBuf::from(path.as_ref());
		self
	}

	/// Set the status.
	#[inline]
	#[must_use]
	pub const fn status(mut self, status: Status) -> Self {
		self.file_status.status = status;
		self
	}

	/// Build the `FileStatus`
	#[inline]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn build(self) -> FileStatus {
		self.file_status
	}
}

impl Default for FileStatusBuilder {
	#[inline]
	#[must_use]
	fn default() -> Self {
		Self::new()
	}
}
