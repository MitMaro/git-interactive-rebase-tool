use crate::show_commit::delta::Delta;
use crate::show_commit::status::Status;

/// Represents a file change within a Git repository
#[derive(Debug, Clone)]
pub struct FileStat {
	status: Status,
	to_name: String,
	from_name: String,
	largest_old_line_number: u32,
	largest_new_line_number: u32,
	deltas: Vec<Delta>,
}

impl FileStat {
	/// Create a new `FileStat`
	pub(super) const fn new(from_name: String, to_name: String, status: Status) -> Self {
		Self {
			status,
			to_name,
			from_name,
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
	use crate::show_commit::file_stat::FileStat;
	use crate::show_commit::status::Status;

	#[test]
	fn commit_user_file_stat() {
		let file_stat = FileStat::new("/from/path".to_string(), "/to/path".to_string(), Status::Renamed);
		assert_eq!(*file_stat.get_status(), Status::Renamed);
		assert_eq!(file_stat.get_from_name(), "/from/path");
		assert_eq!(file_stat.get_to_name(), "/to/path");
	}
}
