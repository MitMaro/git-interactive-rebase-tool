use crate::git::DiffLine;

/// Represents a single set of changes
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Delta {
	old_lines_start: u32,
	old_number_lines: u32,
	new_lines_start: u32,
	new_number_lines: u32,
	context: String,
	lines: Vec<DiffLine>,
}

impl Delta {
	/// Create a new `Delta`.
	#[must_use]
	pub(crate) fn new(
		header: &str,
		old_lines_start: u32,
		new_lines_start: u32,
		old_number_lines: u32,
		new_number_lines: u32,
	) -> Self {
		let context = header.splitn(3, "@@").nth(2).unwrap_or("").trim();
		Self {
			old_lines_start,
			old_number_lines,
			new_lines_start,
			new_number_lines,
			context: String::from(context),
			lines: vec![],
		}
	}

	/// Add a `DiffLine`.
	pub(crate) fn add_line(&mut self, diff_line: DiffLine) {
		self.lines.push(diff_line);
	}

	/// Get the diff context.
	#[must_use]
	pub(crate) fn context(&self) -> &str {
		self.context.as_str()
	}

	/// Get the lines.
	#[must_use]
	pub(crate) const fn lines(&self) -> &Vec<DiffLine> {
		&self.lines
	}

	/// Get the old lines start.
	#[must_use]
	pub(crate) const fn old_lines_start(&self) -> u32 {
		self.old_lines_start
	}

	/// Get the old number of lines
	#[must_use]
	pub(crate) const fn old_number_lines(&self) -> u32 {
		self.old_number_lines
	}

	/// Get the new lines start.
	#[must_use]
	pub(crate) const fn new_lines_start(&self) -> u32 {
		self.new_lines_start
	}

	/// Get the new number of lines.
	#[must_use]
	pub(crate) const fn new_number_lines(&self) -> u32 {
		self.new_number_lines
	}

	pub(crate) fn from(diff_hunk: &git2::DiffHunk<'_>) -> Self {
		Self::new(
			std::str::from_utf8(diff_hunk.header()).unwrap_or("<INVALID UTF8>"),
			diff_hunk.old_start(),
			diff_hunk.new_start(),
			diff_hunk.old_lines(),
			diff_hunk.new_lines(),
		)
	}
}

#[cfg(test)]
mod tests {
	use claims::assert_err;

	use super::*;
	use crate::git::Origin;

	#[test]
	fn new_with_correctly_formatted_context() {
		let delta = Delta::new("@@ path/to/file.rs:56 @@ impl Delta {", 10, 12, 3, 4);
		assert_eq!(delta.context(), "impl Delta {");
		assert_eq!(delta.old_lines_start(), 10);
		assert_eq!(delta.new_lines_start(), 12);
		assert_eq!(delta.old_number_lines(), 3);
		assert_eq!(delta.new_number_lines(), 4);
	}

	#[test]
	fn new_with_at_symbol_in_context() {
		let delta = Delta::new("@@ path:1 @@ Context@@", 10, 12, 3, 4);
		assert_eq!(delta.context(), "Context@@");
		assert_eq!(delta.old_lines_start(), 10);
		assert_eq!(delta.new_lines_start(), 12);
		assert_eq!(delta.old_number_lines(), 3);
		assert_eq!(delta.new_number_lines(), 4);
	}

	#[test]
	fn new_with_incorrectly_formatted_context() {
		let delta = Delta::new("@@invalid", 10, 12, 3, 4);
		assert_eq!(delta.context(), "");
		assert_eq!(delta.old_lines_start(), 10);
		assert_eq!(delta.new_lines_start(), 12);
		assert_eq!(delta.old_number_lines(), 3);
		assert_eq!(delta.new_number_lines(), 4);
	}

	#[test]
	fn add_line() {
		let mut delta = Delta::new("@@ path/to/file.rs:56 @@ impl Delta {", 10, 12, 3, 4);
		delta.add_line(DiffLine::new(
			Origin::Addition,
			"this is a line",
			Some(10),
			Some(12),
			false,
		));
		assert_eq!(delta.lines().len(), 1);
	}

	#[test]
	fn from_diff_hunk() {
		let diff = git2::Diff::from_buffer(
			[
				"diff --git a/src/git/src/status.rs b/src/git/src/status.rs",
				"index 493c0dd..4e07a6e 100644",
				"--- a/src/git/src/status.rs",
				"+++ b/src/git/src/status.rs",
				"@@ -4,3 +4,3 @@ use git2::Delta;",
				" #[derive(Debug, Copy, Clone, PartialEq)]",
				"-#[allow(clippy::exhaustive_enums)]",
				"+#[non_exhaustive]",
				" pub enum Status {",
				"",
			]
			.join("\n")
			.as_bytes(),
		)
		.unwrap();

		// using err return to ensure assert ran
		assert_err!(diff.print(git2::DiffFormat::Patch, |_, diff_hunk, _| {
			if diff_hunk.is_none() {
				return true;
			}
			assert_eq!(
				Delta::from(&diff_hunk.unwrap()),
				Delta::new("@@ -4,3 +4,3 @@ use git2::Delta;", 4, 4, 3, 3)
			);
			false
		}));
	}
}
