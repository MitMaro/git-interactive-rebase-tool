use anyhow::Result;
use crossterm::{event::Event, style::Colors};

use super::{color_mode::ColorMode, Size};

/// An interface that describes interactions with a terminal interface.
pub trait Tui {
	/// Get the supported color mode.
	fn get_color_mode(&self) -> ColorMode;
	/// Reset the terminal interface to a default state.
	fn reset(&mut self) -> Result<()>;
	/// Flush the contents printed to the terminal interface.
	fn flush(&mut self) -> Result<()>;
	/// Print text to the terminal interface.
	fn print(&mut self, s: &str) -> Result<()>;
	/// Set the color attribute of text printed to the terminal interface.
	fn set_color(&mut self, colors: Colors) -> Result<()>;
	/// Set the dimmed style attribute of text printed to the terminal interface.
	fn set_dim(&mut self, dim: bool) -> Result<()>;
	/// Set the underlined style attribute of text printed to the terminal interface.
	fn set_underline(&mut self, underline: bool) -> Result<()>;
	/// Set the reversed style attribute of text printed to the terminal interface.
	fn set_reverse(&mut self, reverse: bool) -> Result<()>;
	/// Read the next input event from the terminal interface.
	fn read_event() -> Result<Option<Event>>
	where Self: Sized;
	/// Get the number of columns and rows of the terminal interface.
	fn get_size(&self) -> Size;
	/// Move the cursor position `x` characters from the start of the line.
	fn move_to_column(&mut self, x: u16) -> Result<()>;
	/// Move the cursor to the next line.
	fn move_next_line(&mut self) -> Result<()>;
	/// Start the terminal interface interactions.
	fn start(&mut self) -> Result<()>;
	/// End the terminal interface interactions.
	fn end(&mut self) -> Result<()>;
}
