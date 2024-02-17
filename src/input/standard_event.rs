/// Represents an event that is not tied directly to a user input device.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub(crate) enum StandardEvent {
	/// The exit meta event.
	Exit,
	/// The kill meta event.
	Kill,
	/// The undo meta event.
	Undo,
	/// The redo meta event.
	Redo,
	/// The scroll bottom meta event.
	ScrollBottom,
	/// The scroll bottom meta event.
	ScrollDown,
	/// The scroll to bottom meta event.
	ScrollJumpDown,
	/// The scroll jump down meta event.
	ScrollJumpUp,
	/// The scroll jump up meta event.
	ScrollLeft,
	/// The scroll left meta event.
	ScrollRight,
	/// The scroll right meta event.
	ScrollTop,
	/// The scroll to top meta event.
	ScrollUp,
	/// The help meta event.
	Help,
	/// Start a search meta event.
	SearchStart,
	/// Next search result meta event.
	SearchNext,
	/// Previous search result meta event.
	SearchPrevious,
	/// Cancel search mode meta event.
	SearchCancel,
	/// Finish search mode meta event.
	SearchFinish,
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
	/// The insert line meta event.
	InsertLine,
	/// Fixup specific action to toggle the c option.
	FixupKeepMessage,
	/// Fixup specific action to toggle the C option.
	FixupKeepMessageWithEditor,
	/// The no meta event.
	No,
	/// The yes meta event.
	Yes,
	/// The external command was successful meta event.
	ExternalCommandSuccess,
	/// The external command was an error meta event.
	ExternalCommandError,
	/// Search was updated
	SearchUpdate,
}
