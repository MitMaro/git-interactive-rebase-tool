/// The state of the `CrossTerm` instance.
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum State {
	/// The TUI is new and unchanged.
	New,
	/// The TUI is in the normal mode.
	Normal,
	/// The TUI has ended.
	Ended,
}
