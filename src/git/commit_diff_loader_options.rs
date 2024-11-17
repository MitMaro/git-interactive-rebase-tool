/// Options for loading a commit with diff
#[derive(Copy, Clone, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct CommitDiffLoaderOptions {
	pub context_lines: u32,
	pub copies: bool,
	pub ignore_whitespace: bool,
	pub ignore_whitespace_change: bool,
	pub ignore_blank_lines: bool,
	pub interhunk_context: u32,
	pub rename_limit: u32,
	pub renames: bool,
}

impl CommitDiffLoaderOptions {
	/// Create a new default instance.
	#[must_use]
	pub(crate) const fn new() -> Self {
		Self {
			context_lines: 0,
			copies: false,
			ignore_whitespace: false,
			ignore_whitespace_change: false,
			ignore_blank_lines: false,
			interhunk_context: 0,
			rename_limit: 0,
			renames: false,
		}
	}

	/// Set the number of context lines.
	#[must_use]
	pub(crate) const fn context_lines(mut self, context_lines: u32) -> Self {
		self.context_lines = context_lines;
		self
	}

	/// Set the number of interhunk lines.
	#[must_use]
	pub(crate) const fn interhunk_context(mut self, interhunk_context: u32) -> Self {
		self.interhunk_context = interhunk_context;
		self
	}

	/// Set if to detect copies or not.
	#[must_use]
	pub(crate) const fn copies(mut self, copies: bool) -> Self {
		self.copies = copies;
		self
	}

	/// Set if to ignore whitespace.
	#[must_use]
	pub(crate) const fn ignore_whitespace(mut self, ignore_whitespace: bool) -> Self {
		self.ignore_whitespace = ignore_whitespace;
		self
	}

	/// Set if to ignore changes in whitespace.
	#[must_use]
	pub(crate) const fn ignore_whitespace_change(mut self, ignore_whitespace_change: bool) -> Self {
		self.ignore_whitespace_change = ignore_whitespace_change;
		self
	}

	/// Set if to ignore blank lines.
	#[must_use]
	pub(crate) const fn ignore_blank_lines(mut self, ignore_blank_lines: bool) -> Self {
		self.ignore_blank_lines = ignore_blank_lines;
		self
	}

	/// Set if to detect renames, as well as the file rename limit.
	#[must_use]
	pub(crate) const fn renames(mut self, renames: bool, limit: u32) -> Self {
		self.rename_limit = limit;
		self.renames = renames;
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn context_lines() {
		assert_eq!(CommitDiffLoaderOptions::new().context_lines(42).context_lines, 42);
	}

	#[test]
	fn interhunk_lines() {
		assert_eq!(
			CommitDiffLoaderOptions::new().interhunk_context(42).interhunk_context,
			42
		);
	}

	#[test]
	fn copies() {
		assert!(CommitDiffLoaderOptions::new().copies(true).copies);
	}

	#[test]
	fn ignore_whitespace() {
		assert!(CommitDiffLoaderOptions::new().ignore_whitespace(true).ignore_whitespace);
	}

	#[test]
	fn ignore_whitespace_change() {
		assert!(
			CommitDiffLoaderOptions::new()
				.ignore_whitespace_change(true)
				.ignore_whitespace_change
		);
	}

	#[test]
	fn ignore_blank_lines() {
		assert!(
			CommitDiffLoaderOptions::new()
				.ignore_blank_lines(true)
				.ignore_blank_lines
		);
	}

	#[test]
	fn renames() {
		let load_commit_diff_options = CommitDiffLoaderOptions::new().renames(true, 42);
		assert!(load_commit_diff_options.renames);
		assert_eq!(load_commit_diff_options.rename_limit, 42);
	}
}
