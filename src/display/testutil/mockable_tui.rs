use std::io;

use crossterm::style::Colors;

use crate::display::{ColorMode, DisplayError, Size, Tui};

/// Create an instance of a `DisplayError::Unexpected` error with an other IO error.
#[must_use]
#[inline]
pub(crate) fn create_unexpected_error() -> DisplayError {
	DisplayError::Unexpected(io::Error::from(io::ErrorKind::Other))
}

/// A version of the `TUI` that provides defaults for all trait methods. This can be used to create
/// mocked versions of the `TUI` interface, without needing to define all methods provided by the
/// interface.
#[allow(missing_docs, clippy::missing_errors_doc)]
pub(crate) trait MockableTui: Tui {
	#[inline]
	fn get_color_mode(&self) -> ColorMode {
		ColorMode::TwoTone
	}

	#[inline]
	fn reset(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn flush(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn print(&mut self, _s: &str) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn set_color(&mut self, _colors: Colors) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn set_dim(&mut self, _dim: bool) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn set_underline(&mut self, _underline: bool) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn set_reverse(&mut self, _reverse: bool) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn get_size(&self) -> Size {
		Size::new(100, 100)
	}

	#[inline]
	fn move_to_column(&mut self, _x: u16) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn move_next_line(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn start(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}

	#[inline]
	fn end(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}
}

impl<T: MockableTui> Tui for T {
	#[inline]
	fn get_color_mode(&self) -> ColorMode {
		<T as MockableTui>::get_color_mode(self)
	}

	#[inline]
	fn reset(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::reset(self)
	}

	#[inline]
	fn flush(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::flush(self)
	}

	#[inline]
	fn print(&mut self, s: &str) -> Result<(), DisplayError> {
		<T as MockableTui>::print(self, s)
	}

	#[inline]
	fn set_color(&mut self, colors: Colors) -> Result<(), DisplayError> {
		<T as MockableTui>::set_color(self, colors)
	}

	#[inline]
	fn set_dim(&mut self, dim: bool) -> Result<(), DisplayError> {
		<T as MockableTui>::set_dim(self, dim)
	}

	#[inline]
	fn set_underline(&mut self, underline: bool) -> Result<(), DisplayError> {
		<T as MockableTui>::set_underline(self, underline)
	}

	#[inline]
	fn set_reverse(&mut self, reverse: bool) -> Result<(), DisplayError> {
		<T as MockableTui>::set_reverse(self, reverse)
	}

	#[inline]
	fn get_size(&self) -> Size {
		<T as MockableTui>::get_size(self)
	}

	#[inline]
	fn move_to_column(&mut self, x: u16) -> Result<(), DisplayError> {
		<T as MockableTui>::move_to_column(self, x)
	}

	#[inline]
	fn move_next_line(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::move_next_line(self)
	}

	#[inline]
	fn start(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::start(self)
	}

	#[inline]
	fn end(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::end(self)
	}
}
