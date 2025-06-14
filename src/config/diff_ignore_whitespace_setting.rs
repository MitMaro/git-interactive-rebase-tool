/// Configuration option for how to ignore whitespace during diff calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum DiffIgnoreWhitespaceSetting {
	/// Do not ignore whitespace when calculating diffs.
	None,
	/// Ignore all whitespace in diffs, same as the [`--ignore-all-space`](
	///     https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---ignore-all-space
	/// ) flag.
	All,
	/// Ignore changed whitespace in diffs, same as the [`--ignore-space-change`](
	///     https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---ignore-space-change
	/// ) flag.
	Change,
}

impl DiffIgnoreWhitespaceSetting {
	pub(crate) fn parse(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
			"true" | "on" | "all" => Some(DiffIgnoreWhitespaceSetting::All),
			"change" => Some(DiffIgnoreWhitespaceSetting::Change),
			"false" | "off" | "none" => Some(DiffIgnoreWhitespaceSetting::None),
			_ => None,
		}
	}
}
