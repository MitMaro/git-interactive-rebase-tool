use git2::Delta;

#[derive(Debug, PartialEq)]
pub(crate) struct FileStat {
	status: Delta,
	to_name: String,
	from_name: String,
}

impl FileStat {
	pub(super) fn new(from_name: String, to_name: String, status: Delta) -> Self {
		FileStat {
			status,
			to_name,
			from_name,
		}
	}

	pub(crate) fn get_status(&self) -> &Delta {
		&self.status
	}

	pub(crate) fn get_to_name(&self) -> &String {
		&self.to_name
	}

	pub(crate) fn get_from_name(&self) -> &String {
		&self.from_name
	}
}
