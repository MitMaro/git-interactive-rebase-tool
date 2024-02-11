use crossterm::style::Colors;

use crate::display::{ColorMode, DisplayError, Size};

/// An interface that describes interactions with a terminal interface.
pub(crate) trait Tui {
	/// Get the supported color mode.
	fn get_color_mode(&self) -> ColorMode;

	/// Reset the terminal interface to a default state.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot be reset for any reason. In general this should not error, and if
	/// this does generate an error, the Tui should be considered to be in a non-recoverable state.
	fn reset(&mut self) -> Result<(), DisplayError>;

	/// Flush the contents printed to the terminal interface.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot be flushed for any reason. In general this should not error, and if
	/// this does generate an error, the Tui should be considered to be in a non-recoverable state.
	fn flush(&mut self) -> Result<(), DisplayError>;

	/// Print text to the terminal interface.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot be printed to for any reason. In general this should not error, and
	/// if this does generate an error, the Tui should be considered to be in a non-recoverable
	/// state.
	fn print(&mut self, s: &str) -> Result<(), DisplayError>;

	/// Set the color attribute of text printed to the terminal interface.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot set the color for any reason. In general this should not error, and
	/// if this does generate an error, the Tui should be considered to be in a non-recoverable
	/// state.
	fn set_color(&mut self, colors: Colors) -> Result<(), DisplayError>;

	/// Set the dimmed style attribute of text printed to the terminal interface.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot set the dimmed state for any reason. In general this should not
	/// error, and if this does generate an error, the Tui should be considered to be in a
	/// non-recoverable state.
	fn set_dim(&mut self, dim: bool) -> Result<(), DisplayError>;

	/// Set the underlined style attribute of text printed to the terminal interface.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot set the underline state for any reason. In general this should not
	/// error, and if this does generate an error, the Tui should be considered to be in a
	/// non-recoverable state.
	fn set_underline(&mut self, underline: bool) -> Result<(), DisplayError>;

	/// Set the reversed style attribute of text printed to the terminal interface.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot set the reversed state for any reason. In general this should not
	/// error, and if this does generate an error, the Tui should be considered to be in a
	/// non-recoverable state.
	fn set_reverse(&mut self, reverse: bool) -> Result<(), DisplayError>;

	/// Get the number of columns and rows of the terminal interface.
	fn get_size(&self) -> Size;

	/// Move the cursor position `x` characters from the start of the line.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot move to a column for any reason. In general this should not error,
	/// and if this does generate an error, the Tui should be considered to be in a non-recoverable
	/// state.
	fn move_to_column(&mut self, x: u16) -> Result<(), DisplayError>;

	/// Move the cursor to the next line.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot move to the next line for any reason. In general this should not
	/// error, and if this does generate an error, the Tui should be considered to be in a
	/// non-recoverable state.
	fn move_next_line(&mut self) -> Result<(), DisplayError>;

	/// Start the terminal interface interactions.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot move to a started state any reason. In general this should not
	/// error,and if this does generate an error, the Tui should be considered to be in a
	/// non-recoverable state.
	fn start(&mut self) -> Result<(), DisplayError>;

	/// End the terminal interface interactions.
	///
	/// # Errors
	///
	/// Errors if the Tui cannot move to an ended state any reason. In general this should not
	/// error,and if this does generate an error, the Tui should be considered to be in a
	/// non-recoverable state.
	fn end(&mut self) -> Result<(), DisplayError>;
}
