use bitflags::bitflags;

bitflags! {
	/// Represents options for parsing input events.
	#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
	pub struct InputOptions: u8 {
		/// Enable movement input handling
		const MOVEMENT = 0b0000_0001;
		/// Enable terminal resize input handling
		const RESIZE = 0b0000_0010;
		/// Enable undo and redo input handling
		const UNDO_REDO = 0b0000_0100;
		/// Search handling
		const SEARCH = 0b0000_1000;
		/// Help input handling
		const HELP = 0b0001_0000;
	}
}
