use anyhow::Result;
use crossterm::{
	event::{Event, KeyCode, KeyEvent},
	style::Colors,
};

use crate::{ColorMode, Size, Tui};

/// A version of the `TUI` that provides defaults for all trait methods. This can be used to create
/// mocked versions of the `TUI` interface, without needing to define all methods provided by the
/// interface.
#[allow(missing_docs, clippy::missing_errors_doc)]
pub trait MockableTui: Tui {
	#[inline]
	fn get_color_mode(&self) -> ColorMode {
		ColorMode::TwoTone
	}

	#[inline]
	fn reset(&mut self) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn flush(&mut self) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn print(&mut self, _s: &str) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn set_color(&mut self, _colors: Colors) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn set_dim(&mut self, _dim: bool) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn set_underline(&mut self, _underline: bool) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn set_reverse(&mut self, _reverse: bool) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn read_event() -> Result<Option<Event>>
	where Self: Sized {
		Ok(Some(Event::Key(KeyEvent::from(KeyCode::Null))))
	}

	#[inline]
	fn get_size(&self) -> Size {
		Size::new(100, 100)
	}

	#[inline]
	fn move_to_column(&mut self, _x: u16) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn move_next_line(&mut self) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn start(&mut self) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn end(&mut self) -> Result<()> {
		Ok(())
	}
}

impl<T: MockableTui> Tui for T {
	#[inline]
	fn get_color_mode(&self) -> ColorMode {
		<T as MockableTui>::get_color_mode(self)
	}

	#[inline]
	fn reset(&mut self) -> Result<()> {
		<T as MockableTui>::reset(self)
	}

	#[inline]
	fn flush(&mut self) -> Result<()> {
		<T as MockableTui>::flush(self)
	}

	#[inline]
	fn print(&mut self, s: &str) -> Result<()> {
		<T as MockableTui>::print(self, s)
	}

	#[inline]
	fn set_color(&mut self, colors: Colors) -> Result<()> {
		<T as MockableTui>::set_color(self, colors)
	}

	#[inline]
	fn set_dim(&mut self, dim: bool) -> Result<()> {
		<T as MockableTui>::set_dim(self, dim)
	}

	#[inline]
	fn set_underline(&mut self, underline: bool) -> Result<()> {
		<T as MockableTui>::set_underline(self, underline)
	}

	#[inline]
	fn set_reverse(&mut self, reverse: bool) -> Result<()> {
		<T as MockableTui>::set_reverse(self, reverse)
	}

	#[inline]
	fn read_event() -> Result<Option<Event>>
	where Self: Sized {
		<T as MockableTui>::read_event()
	}

	#[inline]
	fn get_size(&self) -> Size {
		<T as MockableTui>::get_size(self)
	}

	#[inline]
	fn move_to_column(&mut self, x: u16) -> Result<()> {
		<T as MockableTui>::move_to_column(self, x)
	}

	#[inline]
	fn move_next_line(&mut self) -> Result<()> {
		<T as MockableTui>::move_next_line(self)
	}

	#[inline]
	fn start(&mut self) -> Result<()> {
		<T as MockableTui>::start(self)
	}

	#[inline]
	fn end(&mut self) -> Result<()> {
		<T as MockableTui>::end(self)
	}
}
