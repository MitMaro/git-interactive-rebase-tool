use crate::show_commit::diff_line::Origin::{Addition, Context, Deletion};

#[derive(Debug, Clone)]
pub enum Origin {
	Context,
	Addition,
	Deletion,
}

impl Origin {
	pub(crate) fn from_chr(c: char) -> Self {
		match c {
			' ' => Context,
			'+' => Addition,
			'-' => Deletion,
			'=' => Context,
			'>' => Addition,
			'<' => Deletion,
			_ => panic!("Invalid diff origin: {}", c),
		}
	}
}

#[derive(Debug, Clone)]
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
	) -> Self
	{
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

	pub(crate) fn new_line_number(&self) -> Option<u32> {
		self.new_line_number
	}

	pub(crate) fn old_line_number(&self) -> Option<u32> {
		self.old_line_number
	}

	pub(crate) fn origin(&self) -> &Origin {
		&self.origin
	}

	pub(crate) fn end_of_file(&self) -> bool {
		self.end_of_file
	}
}
