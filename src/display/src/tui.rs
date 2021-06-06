use anyhow::Result;
use crossterm::{event::Event, style::Colors};

use super::{color_mode::ColorMode, Size};

pub trait Tui {
	fn get_color_mode(&self) -> ColorMode;
	fn reset(&mut self) -> Result<()>;
	fn flush(&mut self) -> Result<()>;
	fn print(&mut self, s: &str) -> Result<()>;
	fn set_color(&mut self, colors: Colors) -> Result<()>;
	fn set_dim(&mut self, dim: bool) -> Result<()>;
	fn set_underline(&mut self, underline: bool) -> Result<()>;
	fn set_reverse(&mut self, reverse: bool) -> Result<()>;
	fn read_event() -> Result<Option<Event>>
	where Self: Sized;
	fn get_size(&self) -> Size;
	fn move_to_column(&mut self, x: u16) -> Result<()>;
	fn move_next_line(&mut self) -> Result<()>;
	fn start(&mut self) -> Result<()>;
	fn end(&mut self) -> Result<()>;
}
