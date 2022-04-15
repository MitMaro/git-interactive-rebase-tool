/// Represents an event that is not tied directly to a user input device.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum StandardEvent {
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
}
