use crate::show_commit::diff_line::DiffLine;

#[derive(Debug, Clone)]
pub struct Delta {
	old_start: u32,
	old_lines: u32,
	new_start: u32,
	new_lines: u32,
	context: String,
	lines: Vec<DiffLine>,
}

impl Delta {
	pub(super) fn new(header: &str, old_start: u32, new_start: u32, old_lines: u32, new_lines: u32) -> Self {
		let context = header.splitn(3, '@').nth(2).unwrap_or("").trim();
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

	pub(crate) const fn lines(&self) -> &Vec<DiffLine> {
		&self.lines
	}

	pub(crate) const fn old_start(&self) -> u32 {
		self.old_start
	}

	pub(crate) const fn old_lines(&self) -> u32 {
		self.old_lines
	}

	pub(crate) const fn new_start(&self) -> u32 {
		self.new_start
	}

	pub(crate) const fn new_lines(&self) -> u32 {
		self.new_lines
	}
}

#[cfg(test)]
mod tests {
	use super::super::origin::Origin;
	use super::*;

	#[test]
	fn new_with_correctly_formatted_context() {
		let delta = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4);
		assert_eq!(delta.context(), "impl Delta {");
		assert_eq!(delta.old_start(), 10);
		assert_eq!(delta.new_start(), 12);
		assert_eq!(delta.old_lines(), 3);
		assert_eq!(delta.new_lines(), 4);
	}

	#[test]
	fn new_with_at_symbol_in_context() {
		let delta = Delta::new("@ path:1 @ Context@", 10, 12, 3, 4);
		assert_eq!(delta.context(), "Context@");
		assert_eq!(delta.old_start(), 10);
		assert_eq!(delta.new_start(), 12);
		assert_eq!(delta.old_lines(), 3);
		assert_eq!(delta.new_lines(), 4);
	}

	#[test]
	fn new_with_incorrectly_formatted_context() {
		let delta = Delta::new("@invalid", 10, 12, 3, 4);
		assert_eq!(delta.context(), "");
		assert_eq!(delta.old_start(), 10);
		assert_eq!(delta.new_start(), 12);
		assert_eq!(delta.old_lines(), 3);
		assert_eq!(delta.new_lines(), 4);
	}

	#[test]
	fn add_line() {
		let mut delta = Delta::new("@ src/show_commit/delta.rs:56 @ impl Delta {", 10, 12, 3, 4);
		delta.add_line(DiffLine::new(
			Origin::Addition,
			"this is a line",
			Some(10),
			Some(12),
			false,
		));
		assert_eq!(delta.lines().len(), 1);
	}
}
