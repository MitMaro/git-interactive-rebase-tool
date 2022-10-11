use std::path::{Path, PathBuf};

use super::{delta::Delta, status::Status};
use crate::file_mode::FileMode;

/// Represents a file change within a Git repository
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStatus {
	pub(crate) deltas: Vec<Delta>,
	pub(crate) destination_is_binary: bool,
	pub(crate) destination_mode: FileMode,
	pub(crate) destination_path: PathBuf,
	pub(crate) largest_new_line_number: u32,
	pub(crate) largest_old_line_number: u32,
	pub(crate) source_is_binary: bool,
	pub(crate) source_mode: FileMode,
	pub(crate) source_path: PathBuf,
	pub(crate) status: Status,
}

impl FileStatus {
	/// Create a new `FileStat`.
	#[inline]
	#[must_use]
	pub(crate) fn new<F: AsRef<Path>>(
		source_path: F,
		source_mode: FileMode,
		source_is_binary: bool,
		destination_path: F,
		destination_mode: FileMode,
		destination_is_binary: bool,
		status: Status,
	) -> Self {
		Self {
			deltas: vec![],
			destination_is_binary,
			destination_mode,
			destination_path: PathBuf::from(destination_path.as_ref()),
			largest_new_line_number: 0,
			largest_old_line_number: 0,
			source_is_binary,
			source_mode,
			source_path: PathBuf::from(source_path.as_ref()),
			status,
		}
	}

	/// Add a delta to the change.
	#[inline]
	pub fn add_delta(&mut self, delta: Delta) {
		let last_old_line_number = delta.old_lines_start() + delta.old_number_lines();
		if self.largest_old_line_number < last_old_line_number {
			self.largest_old_line_number = last_old_line_number;
		}
		let last_new_line_number = delta.new_lines_start() + delta.new_number_lines();
		if self.largest_new_line_number < last_new_line_number {
			self.largest_new_line_number = last_new_line_number;
		}
		self.deltas.push(delta);
	}

	/// Get the status of this file change.
	#[inline]
	#[must_use]
	pub const fn status(&self) -> Status {
		self.status
	}

	/// Get the destination file path for this change.
	#[inline]
	#[must_use]
	pub fn destination_path(&self) -> &Path {
		self.destination_path.as_path()
	}

	/// Get the destination file mode for this change.
	#[inline]
	#[must_use]
	pub const fn destination_mode(&self) -> FileMode {
		self.destination_mode
	}

	/// Is the destination file a binary file.
	#[inline]
	#[must_use]
	pub const fn destination_is_binary(&self) -> bool {
		self.destination_is_binary
	}

	/// Get the source file path for this change.
	#[inline]
	#[must_use]
	pub fn source_path(&self) -> &Path {
		self.source_path.as_path()
	}

	/// Get the source file mode for this change.
	#[inline]
	#[must_use]
	pub const fn source_mode(&self) -> FileMode {
		self.source_mode
	}

	/// Is the source file a binary file.
	#[inline]
	#[must_use]
	pub const fn source_is_binary(&self) -> bool {
		self.source_is_binary
	}

	/// Get the deltas for this change.
	#[inline]
	#[must_use]
	pub const fn deltas(&self) -> &Vec<Delta> {
		&self.deltas
	}

	/// Get the line number of the last old changed line.
	#[inline]
	#[must_use]
	pub const fn last_old_line_number(&self) -> u32 {
		self.largest_old_line_number
	}

	/// Get the line number of the last new changed line.
	#[inline]
	#[must_use]
	pub const fn last_new_line_number(&self) -> u32 {
		self.largest_new_line_number
	}
}

#[cfg(test)]
mod tests {
	use testutils::assert_empty;

	use super::*;

	fn create_file_stat() -> FileStatus {
		FileStatus::new(
			Path::new("/from/path"),
			FileMode::Normal,
			false,
			Path::new("/to/path"),
			FileMode::Executable,
			false,
			Status::Modified,
		)
	}

	#[test]
	fn status() {
		assert_eq!(create_file_stat().status(), Status::Modified);
	}

	#[test]
	fn destination_path() {
		assert_eq!(create_file_stat().destination_path(), PathBuf::from("/to/path"));
	}

	#[test]
	fn destination_mode() {
		assert_eq!(create_file_stat().destination_mode(), FileMode::Executable);
	}

	#[test]
	fn destination_is_binary() {
		assert!(!create_file_stat().destination_is_binary());
	}

	#[test]
	fn source_path() {
		assert_eq!(create_file_stat().source_path(), PathBuf::from("/from/path"));
	}

	#[test]
	fn source_mode() {
		assert_eq!(create_file_stat().source_mode(), FileMode::Normal);
	}

	#[test]
	fn source_is_binary() {
		assert!(!create_file_stat().source_is_binary());
	}

	#[test]
	fn deltas_empty() {
		let file_stat = create_file_stat();
		assert_empty!(file_stat.deltas());
		assert_eq!(file_stat.last_old_line_number(), 0);
		assert_eq!(file_stat.last_new_line_number(), 0);
	}

	#[test]
	fn deltas_single() {
		let mut file_stat = create_file_stat();
		let delta = Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4);
		file_stat.add_delta(delta.clone());
		assert_eq!(file_stat.deltas(), &vec![delta]);
		assert_eq!(file_stat.last_old_line_number(), 13);
		assert_eq!(file_stat.last_new_line_number(), 16);
	}

	#[test]
	fn deltas_multiple() {
		let mut file_stat = create_file_stat();
		let delta1 = Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 12, 3, 4);
		let delta2 = Delta::new("@ path/to/file.rs:156 @ impl Delta {", 110, 2, 10, 3);
		file_stat.add_delta(delta1.clone());
		file_stat.add_delta(delta2.clone());
		assert_eq!(file_stat.deltas(), &vec![delta1, delta2]);
		assert_eq!(file_stat.last_old_line_number(), 120);
		assert_eq!(file_stat.last_new_line_number(), 16);
	}

	#[test]
	fn deltas_with_second_delta_with_larger_old_line_number() {
		let mut file_stat = create_file_stat();
		file_stat.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 20, 5, 5));
		file_stat.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 20, 20, 5, 5));
		assert_eq!(file_stat.last_old_line_number(), 25);
	}

	#[test]
	fn deltas_with_first_delta_with_larger_old_line_number() {
		let mut file_stat = create_file_stat();
		file_stat.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 20, 20, 5, 5));
		file_stat.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 20, 5, 5));
		assert_eq!(file_stat.last_old_line_number(), 25);
	}

	#[test]
	fn deltas_with_second_delta_with_larger_new_line_number() {
		let mut file_stat = create_file_stat();
		file_stat.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 10, 5, 5));
		file_stat.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 20, 5, 5));
		assert_eq!(file_stat.last_new_line_number(), 25);
	}

	#[test]
	fn deltas_with_first_delta_with_larger_new_line_number() {
		let mut file_stat = create_file_stat();
		file_stat.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 20, 5, 5));
		file_stat.add_delta(Delta::new("@ path/to/file.rs:56 @ impl Delta {", 10, 10, 5, 5));
		assert_eq!(file_stat.last_new_line_number(), 25);
	}
}
