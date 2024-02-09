/// The state of the `CrossTerm` instance.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum State {
	/// The TUI is new and unchanged.
	New,
	/// The TUI is in the normal mode.
	Normal,
	/// The TUI has ended.
	Ended,
}
