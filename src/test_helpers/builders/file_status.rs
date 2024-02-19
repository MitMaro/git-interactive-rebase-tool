use std::path::{Path, PathBuf};

use crate::git::{Delta, FileMode, FileStatus, Status};

/// Builder for creating a new reference.
#[derive(Debug)]
pub(crate) struct FileStatusBuilder {
	deltas: Vec<Delta>,
	destination_is_binary: bool,
	destination_mode: FileMode,
	destination_path: PathBuf,
	source_is_binary: bool,
	source_mode: FileMode,
	source_path: PathBuf,
	status: Status,
}

impl FileStatusBuilder {
	/// Create a new instance of the builder. The new instance will default to an empty file status.
	#[must_use]
	pub(crate) fn new() -> Self {
		Self {
			deltas: vec![],
			destination_is_binary: false,
			destination_mode: FileMode::Normal,
			destination_path: PathBuf::default(),
			source_is_binary: false,
			source_mode: FileMode::Normal,
			source_path: PathBuf::default(),
			status: Status::Added,
		}
	}

	/// Push a `Delta`.
	#[must_use]
	pub(crate) fn push_delta(mut self, delta: Delta) -> Self {
		self.deltas.push(delta);
		self
	}

	/// Set if the destination is binary.
	#[must_use]
	pub(crate) const fn destination_is_binary(mut self, binary: bool) -> Self {
		self.destination_is_binary = binary;
		self
	}

	/// Set the destination file mode.
	#[must_use]
	pub(crate) const fn destination_mode(mut self, mode: FileMode) -> Self {
		self.destination_mode = mode;
		self
	}

	/// Set the destination file path.
	#[must_use]
	pub(crate) fn destination_path<F: AsRef<Path>>(mut self, path: F) -> Self {
		self.destination_path = PathBuf::from(path.as_ref());
		self
	}

	/// Set if the source is binary.
	#[must_use]
	pub(crate) const fn source_is_binary(mut self, binary: bool) -> Self {
		self.source_is_binary = binary;
		self
	}

	/// Set if the source file mode.
	#[must_use]
	pub(crate) const fn source_mode(mut self, mode: FileMode) -> Self {
		self.source_mode = mode;
		self
	}

	/// Set the destination file path.
	#[must_use]
	pub(crate) fn source_path<F: AsRef<Path>>(mut self, path: F) -> Self {
		self.source_path = PathBuf::from(path.as_ref());
		self
	}

	/// Set the status.
	#[must_use]
	pub(crate) const fn status(mut self, status: Status) -> Self {
		self.status = status;
		self
	}

	/// Build the `FileStatus`
	#[must_use]
	pub(crate) fn build(self) -> FileStatus {
		let mut file_status = FileStatus::new(
			self.source_path,
			self.source_mode,
			self.source_is_binary,
			self.destination_path,
			self.destination_mode,
			self.destination_is_binary,
			self.status,
		);
		for delta in self.deltas {
			file_status.add_delta(delta);
		}

		file_status
	}
}
