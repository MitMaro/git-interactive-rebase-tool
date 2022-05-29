use crate::events::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub(crate) enum MetaEvent {
	/// The abort meta event.
	Abort,
	/// The force abort meta event.
	ForceAbort,
	/// The rebase meta event.
	Rebase,
	/// The force rebase meta event.
	ForceRebase,
	/// The break action meta event.
	ActionBreak,
	/// The drop action meta event.
	ActionDrop,
	/// The edit action meta event.
	ActionEdit,
	/// The fixup action meta event.
	ActionFixup,
	/// The pick action meta event.
	ActionPick,
	/// The reword action meta event.
	ActionReword,
	/// The squash action meta event.
	ActionSquash,
	/// The move cursor down meta event.
	MoveCursorDown,
	/// The move cursor to end meta event.
	MoveCursorEnd,
	/// The move cursor to home meta event.
	MoveCursorHome,
	/// The move cursor left meta event.
	MoveCursorLeft,
	/// The move cursor page down meta event.
	MoveCursorPageDown,
	/// The move cursor page up meta event.
	MoveCursorPageUp,
	/// The move cursor right meta event.
	MoveCursorRight,
	/// The move cursor up meta event.
	MoveCursorUp,
	/// The delete meta event.
	Delete,
	/// The edit meta event.
	Edit,
	/// The open in editor meta event.
	OpenInEditor,
	/// The show commit meta event.
	ShowCommit,
	/// The show diff meta event.
	ShowDiff,
	/// The swap selection down meta event.
	SwapSelectedDown,
	/// The swap selection up meta event.
	SwapSelectedUp,
	/// The toggle visual mode meta event.
	ToggleVisualMode,
	/// The help meta event.
	Help,
	/// The insert line meta event.
	InsertLine,
	/// The no meta event.
	No,
	/// The yes meta event.
	Yes,
	/// The external command was successful meta event.
	ExternalCommandSuccess,
	/// The external command was an error meta event.
	ExternalCommandError,
}

impl input::CustomEvent for MetaEvent {}

impl From<MetaEvent> for Event {
	#[inline]
	fn from(event: MetaEvent) -> Self {
		Self::MetaEvent(event)
	}
}
