/// Configuration option for how to ignore whitespace during diff calculation.
#[derive(Clone, Copy, PartialEq, Debug)]
#[non_exhaustive]
pub enum DiffIgnoreWhitespaceSetting {
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
