use super::delta::Delta;
use super::status::Status;

/// Represents a file change within a Git repository
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct FileStat {
	pub(super) status: Status,
	pub(super) to_name: String,
	pub(super) from_name: String,
	pub(super) largest_old_line_number: u32,
	pub(super) largest_new_line_number: u32,
	pub(super) deltas: Vec<Delta>,
}

impl FileStat {
	/// Create a new `FileStat`
	pub(super) fn new(from_name: &str, to_name: &str, status: Status) -> Self {
		Self {
			status,
			to_name: String::from(to_name),
			from_name: String::from(from_name),
			largest_old_line_number: 0,
			largest_new_line_number: 0,
			deltas: vec![],
		}
	}

	pub(super) fn add_delta(&mut self, delta: Delta) {
		let last_old_line_number = delta.old_start() + delta.old_lines();
		if self.largest_old_line_number < last_old_line_number {
			self.largest_old_line_number = last_old_line_number;
		}
		let last_new_line_number = delta.new_start() + delta.new_lines();
		if self.largest_new_line_number < last_new_line_number {
			self.largest_new_line_number = last_new_line_number;
		}
		self.deltas.push(delta);
	}

	/// Get the status of this file change
	pub(super) const fn get_status(&self) -> &Status {
		&self.status
	}

	/// Get the destination file name for this change.
	pub(super) fn get_to_name(&self) -> &str {
		self.to_name.as_str()
	}

	/// Get the source file name for this change.
	pub(super) fn get_from_name(&self) -> &str {
		self.from_name.as_str()
	}

	pub(crate) const fn largest_old_line_number(&self) -> u32 {
		self.largest_old_line_number
	}

	pub(crate) const fn deltas(&self) -> &Vec<Delta> {
		&self.deltas
	}

	pub(crate) const fn largest_new_line_number(&self) -> u32 {
		self.largest_new_line_number
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_deltas() {
		let file_stat = FileStat::new("/from/path", "/to/path", Status::Renamed);
		assert_eq!(*file_stat.get_status(), Status::Renamed);
		assert_eq!(file_stat.get_from_name(), "/from/path");
		assert_eq!(file_stat.get_to_name(), "/to/path");
		assert_eq!(file_stat.largest_old_line_number(), 0);
		assert_eq!(file_stat.largest_new_line_number(), 0);
		assert!(file_stat.deltas().is_empty());
	}

	#[test]
	fn add_delta() {
		let mut file_stat = FileStat::new("/from/path", "/to/path", Status::Renamed);
		file_stat.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4));
		assert_eq!(file_stat.largest_old_line_number(), 13);
		assert_eq!(file_stat.largest_new_line_number(), 16);
		assert_eq!(file_stat.deltas().len(), 1);
	}

	#[test]
	fn add_delta_with_larger_old_line_number() {
		let mut file_stat = FileStat::new("/from/path", "/to/path", Status::Renamed);
		file_stat.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4));
		file_stat.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 14, 12, 3, 4));
		assert_eq!(file_stat.largest_old_line_number(), 17);
		assert_eq!(file_stat.largest_new_line_number(), 16);
		assert_eq!(file_stat.deltas().len(), 2);
	}

	#[test]
	fn add_delta_with_larger_new_line_number() {
		let mut file_stat = FileStat::new("/from/path", "/to/path", Status::Renamed);
		file_stat.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4));
		file_stat.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 17, 3, 4));
		assert_eq!(file_stat.largest_old_line_number(), 13);
		assert_eq!(file_stat.largest_new_line_number(), 21);
		assert_eq!(file_stat.deltas().len(), 2);
	}

	#[test]
	fn add_delta_with_larger_new_and_old_line_number() {
		let mut file_stat = FileStat::new("/from/path", "/to/path", Status::Renamed);
		file_stat.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4));
		file_stat.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 14, 12, 3, 4));
		file_stat.add_delta(Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 17, 3, 4));
		assert_eq!(file_stat.largest_old_line_number(), 17);
		assert_eq!(file_stat.largest_new_line_number(), 21);
		assert_eq!(file_stat.deltas().len(), 3);
	}
}
