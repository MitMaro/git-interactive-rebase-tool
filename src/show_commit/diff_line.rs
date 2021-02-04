use crate::show_commit::origin::Origin;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DiffLine {
	end_of_file: bool,
	line: String,
	new_line_number: Option<u32>,
	old_line_number: Option<u32>,
	origin: Origin,
}

impl DiffLine {
	pub(super) fn new(
		origin: Origin,
		line: &str,
		old_line_number: Option<u32>,
		new_line_number: Option<u32>,
		end_of_file: bool,
	) -> Self {
		Self {
			end_of_file,
			// remove the end of file marker from diff
			line: if end_of_file {
				line.replace("\n\\ No newline at end of file\n", "")
			}
			else {
				String::from(line)
			},
			new_line_number,
			old_line_number,
			origin,
		}
	}

	pub(crate) fn line(&self) -> &str {
		self.line.as_str()
	}

	pub(crate) const fn new_line_number(&self) -> Option<u32> {
		self.new_line_number
	}

	pub(crate) const fn old_line_number(&self) -> Option<u32> {
		self.old_line_number
	}

	pub(crate) const fn origin(&self) -> &Origin {
		&self.origin
	}

	pub(crate) const fn end_of_file(&self) -> bool {
		self.end_of_file
	}
}

#[cfg(test)]
mod tests {
	use super::{super::origin::Origin, *};

	#[test]
	fn new_without_end_of_file() {
		let diff_line = DiffLine::new(
			Origin::Addition,
			"This is a line\n\\ No newline at end of file\n",
			Some(1),
			Some(2),
			false,
		);
		assert_eq!(diff_line.line(), "This is a line\n\\ No newline at end of file\n");
		assert_eq!(diff_line.old_line_number(), Some(1));
		assert_eq!(diff_line.new_line_number(), Some(2));
		assert_eq!(diff_line.origin(), &Origin::Addition);
		assert!(!diff_line.end_of_file());
	}

	#[test]
	fn new_with_end_of_file() {
		let diff_line = DiffLine::new(
			Origin::Addition,
			"This is a line\n\\ No newline at end of file\n",
			Some(1),
			Some(2),
			true,
		);
		assert_eq!(diff_line.line(), "This is a line");
		assert_eq!(diff_line.old_line_number(), Some(1));
		assert_eq!(diff_line.new_line_number(), Some(2));
		assert_eq!(diff_line.origin(), &Origin::Addition);
		assert!(diff_line.end_of_file());
	}
}
