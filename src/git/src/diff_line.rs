use crate::Origin;

/// Represents a single line in a diff
#[derive(Debug, Clone, PartialEq)]
pub struct DiffLine {
	end_of_file: bool,
	line: String,
	new_line_number: Option<u32>,
	old_line_number: Option<u32>,
	origin: Origin,
}

impl DiffLine {
	/// Create a new `DiffLine`.
	#[inline]
	#[must_use]
	pub fn new(
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

	/// Get the `Origin` of the `DiffLine`
	#[inline]
	#[must_use]
	pub const fn origin(&self) -> Origin {
		self.origin
	}

	/// Get the line of the `DiffLine`.
	#[inline]
	#[must_use]
	pub fn line(&self) -> &str {
		self.line.as_str()
	}

	/// Get the old line number of the `DiffLine`, if it exists, else `None`.
	#[inline]
	#[must_use]
	pub const fn old_line_number(&self) -> Option<u32> {
		self.old_line_number
	}

	/// Get the new line number of the `DiffLine`, if it exists, else `None`.
	#[inline]
	#[must_use]
	pub const fn new_line_number(&self) -> Option<u32> {
		self.new_line_number
	}

	/// Returns `true` is this line was at the end of a file, else `false`.
	#[inline]
	#[must_use]
	pub const fn end_of_file(&self) -> bool {
		self.end_of_file
	}

	pub(crate) fn from(diff_line: &git2::DiffLine<'_>) -> Self {
		Self::new(
			Origin::from(diff_line.origin_value()),
			std::str::from_utf8(diff_line.content()).unwrap_or("<INVALID UTF8>"),
			diff_line.old_lineno(),
			diff_line.new_lineno(),
			diff_line.origin_value() == git2::DiffLineType::ContextEOFNL
				|| diff_line.origin_value() == git2::DiffLineType::AddEOFNL
				|| diff_line.origin_value() == git2::DiffLineType::DeleteEOFNL,
		)
	}
}

#[cfg(test)]
mod tests {
	use parking_lot::Mutex;

	use super::*;

	fn create_diff_line() -> DiffLine {
		DiffLine::new(Origin::Addition, "Line", Some(1), Some(2), false)
	}

	#[test]
	fn origin() {
		assert_eq!(create_diff_line().origin(), Origin::Addition);
	}

	#[test]
	fn line() {
		assert_eq!(create_diff_line().line(), "Line");
	}

	#[test]
	fn old_line_number() {
		assert_eq!(create_diff_line().old_line_number(), Some(1));
	}

	#[test]
	fn new_line_number() {
		assert_eq!(create_diff_line().new_line_number(), Some(2));
	}

	#[test]
	fn end_of_file() {
		assert_eq!(create_diff_line().end_of_file(), false);
	}

	#[test]
	fn new_without_end_of_file() {
		let diff_line = DiffLine::new(
			Origin::Addition,
			"This is a line\n\\ No newline at end of file\n",
			None,
			None,
			false,
		);
		assert_eq!(diff_line.line(), "This is a line\n\\ No newline at end of file\n");
		assert!(!diff_line.end_of_file());
	}

	#[test]
	fn new_with_end_of_file() {
		let diff_line = DiffLine::new(
			Origin::Addition,
			"This is a line\n\\ No newline at end of file\n",
			None,
			None,
			true,
		);
		assert_eq!(diff_line.line(), "This is a line");
		assert!(diff_line.end_of_file());
	}

	#[test]
	fn from_diff_lines() {
		let diff = git2::Diff::from_buffer(
			[
				"diff --git a/file.rs b/file.rs",
				"index 493c0dd..4e07a6e 100644",
				"--- a/file.rs",
				"+++ b/file.rs",
				"@@ -1,2 +1,2 @@ context",
				" context",
				"-deleted",
				"+added",
				"",
			]
			.join("\n")
			.as_bytes(),
		)
		.unwrap();

		let lines = Mutex::new(vec![]);
		diff.print(git2::DiffFormat::Patch, |_, _, diff_line| {
			lines.lock().push(DiffLine::from(&diff_line));
			true
		})
		.unwrap();

		let file_header = [
			"diff --git a/file.rs b/file.rs",
			"index 493c0dd..4e07a6e 100644",
			"--- a/file.rs",
			"+++ b/file.rs",
			"",
		]
		.join("\n");
		let expected = [
			DiffLine::new(Origin::Header, file_header.as_str(), None, None, false),
			DiffLine::new(Origin::Header, "@@ -1,2 +1,2 @@ context\n", None, None, false),
			DiffLine::new(Origin::Context, "context\n", Some(1), Some(1), false),
			DiffLine::new(Origin::Deletion, "deleted\n", Some(2), None, false),
			DiffLine::new(Origin::Addition, "added\n", None, Some(2), false),
		];
		assert_eq!(lines.into_inner(), expected);
	}
}
