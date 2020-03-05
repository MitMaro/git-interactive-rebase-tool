use crate::commit::status::Status;

/// Represents a file change within a Git repository
#[derive(Debug, PartialEq)]
pub(crate) struct FileStat {
	status: Status,
	to_name: String,
	from_name: String,
}

impl FileStat {
	/// Create a new FileStat
	///
	/// The `from_name` should be the source file name, the `to_name` the destination file name.
	/// When the file change is not a copy or rename, `from_name` and `to_name` should be equal.
	pub(super) fn new(from_name: String, to_name: String, status: Status) -> Self {
		FileStat {
			status,
			to_name,
			from_name,
		}
	}

	/// Get the status of this file change
	pub(crate) fn get_status(&self) -> &Status {
		&self.status
	}

	/// Get the destination file name for this change.
	pub(crate) fn get_to_name(&self) -> &String {
		&self.to_name
	}

	/// Get the source file name for this change.
	pub(crate) fn get_from_name(&self) -> &String {
		&self.from_name
	}
}

#[cfg(test)]
mod tests {
	use crate::commit::file_stat::FileStat;
	use crate::commit::status::Status;

	#[test]
	fn commit_user_file_stat() {
		let file_stat = FileStat::new("/from/path".to_string(), "/to/path".to_string(), Status::Renamed);
		assert_eq!(*file_stat.get_status(), Status::Renamed);
		assert_eq!(file_stat.get_from_name(), "/from/path");
		assert_eq!(file_stat.get_to_name(), "/to/path");
	}
}
