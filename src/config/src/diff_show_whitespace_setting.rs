/// Configuration option for how to show whitespace when displaying diffs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiffShowWhitespaceSetting {
	/// Do not show whitespace characters.
	None,
	/// Show only trailing whitespace characters.
	Trailing,
	/// Show only leading whitespace characters.
	Leading,
	/// Show both leading and trailing whitespace characters.
	Both,
}
