use crate::show_commit::diff_line::DiffLine;

#[derive(Debug, Clone)]
pub(crate) struct Delta {
	old_start: u32,
	old_lines: u32,
	new_start: u32,
	new_lines: u32,
	context: String,
	lines: Vec<DiffLine>,
}

impl Delta {
	pub(super) fn new(header: &str, old_start: u32, new_start: u32, old_lines: u32, new_lines: u32) -> Self {
		let context = header.split('@').last().unwrap_or("").trim();
		Self {
			old_start,
			old_lines,
			new_start,
			new_lines,
			context: String::from(context),
			lines: vec![],
		}
	}

	pub(crate) fn add_line(&mut self, diff_line: DiffLine) {
		self.lines.push(diff_line);
	}

	pub(crate) fn context(&self) -> &str {
		self.context.as_str()
	}

	pub(crate) fn lines(&self) -> &Vec<DiffLine> {
		&self.lines
	}

	pub(crate) fn old_start(&self) -> u32 {
		self.old_start
	}

	pub(crate) fn old_lines(&self) -> u32 {
		self.old_lines
	}

	pub(crate) fn new_start(&self) -> u32 {
		self.new_start
	}

	pub(crate) fn new_lines(&self) -> u32 {
		self.new_lines
	}
}
