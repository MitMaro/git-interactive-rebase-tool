use std::sync::Arc;

use ::crossterm::style::{Attribute, Attributes, Color, Colors};
use parking_lot::RwLock;

use crate::display::{ColorMode, DisplayError, Size, Tui};

/// The state of the `CrossTerm` instance.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum CrosstermMockState {
	/// The TUI is new and unchanged.
	New,
	/// The TUI is in the normal mode.
	Normal,
	/// The TUI has ended.
	Ended,
}

/// A version of the `TUI` that provides defaults for all trait methods. This can be used to create
/// mocked versions of the `TUI` interface, without needing to define all methods provided by the
/// interface.
pub(crate) trait MockableTui: Tui {
	fn get_color_mode(&self) -> ColorMode {
		ColorMode::TwoTone
	}

	fn reset(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}

	fn flush(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}

	fn print(&mut self, _s: &str) -> Result<(), DisplayError> {
		Ok(())
	}

	fn set_color(&mut self, _colors: Colors) -> Result<(), DisplayError> {
		Ok(())
	}

	fn set_dim(&mut self, _dim: bool) -> Result<(), DisplayError> {
		Ok(())
	}

	fn set_underline(&mut self, _underline: bool) -> Result<(), DisplayError> {
		Ok(())
	}

	fn set_reverse(&mut self, _reverse: bool) -> Result<(), DisplayError> {
		Ok(())
	}

	fn get_size(&self) -> Size {
		Size::new(100, 100)
	}

	fn move_to_column(&mut self, _x: u16) -> Result<(), DisplayError> {
		Ok(())
	}

	fn move_next_line(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}

	fn start(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}

	fn end(&mut self) -> Result<(), DisplayError> {
		Ok(())
	}
}

impl<T: MockableTui> Tui for T {
	fn get_color_mode(&self) -> ColorMode {
		<T as MockableTui>::get_color_mode(self)
	}

	fn reset(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::reset(self)
	}

	fn flush(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::flush(self)
	}

	fn print(&mut self, s: &str) -> Result<(), DisplayError> {
		<T as MockableTui>::print(self, s)
	}

	fn set_color(&mut self, colors: Colors) -> Result<(), DisplayError> {
		<T as MockableTui>::set_color(self, colors)
	}

	fn set_dim(&mut self, dim: bool) -> Result<(), DisplayError> {
		<T as MockableTui>::set_dim(self, dim)
	}

	fn set_underline(&mut self, underline: bool) -> Result<(), DisplayError> {
		<T as MockableTui>::set_underline(self, underline)
	}

	fn set_reverse(&mut self, reverse: bool) -> Result<(), DisplayError> {
		<T as MockableTui>::set_reverse(self, reverse)
	}

	fn get_size(&self) -> Size {
		<T as MockableTui>::get_size(self)
	}

	fn move_to_column(&mut self, x: u16) -> Result<(), DisplayError> {
		<T as MockableTui>::move_to_column(self, x)
	}

	fn move_next_line(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::move_next_line(self)
	}

	fn start(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::start(self)
	}

	fn end(&mut self) -> Result<(), DisplayError> {
		<T as MockableTui>::end(self)
	}
}

#[derive(Debug)]
struct CrossTermInternalState {
	attributes: Attributes,
	color_mode: ColorMode,
	colors: Colors,
	dirty: bool,
	output: Vec<String>,
	position: (u16, u16),
	size: Size,
	state: CrosstermMockState,
}

/// A mocked version of `CrossTerm`, useful for testing.
#[derive(Debug)]
pub(crate) struct CrossTerm {
	state: Arc<RwLock<CrossTermInternalState>>,
}

impl MockableTui for CrossTerm {
	fn get_color_mode(&self) -> ColorMode {
		self.state.read().color_mode
	}

	fn reset(&mut self) -> Result<(), DisplayError> {
		let mut state = self.state.write();
		state.attributes = Attributes::from(Attribute::Reset);
		state.colors = Colors::new(Color::Reset, Color::Reset);
		state.output.clear();
		state.state = CrosstermMockState::Normal;
		Ok(())
	}

	fn flush(&mut self) -> Result<(), DisplayError> {
		self.state.write().dirty = false;
		Ok(())
	}

	fn print(&mut self, s: &str) -> Result<(), DisplayError> {
		self.state.write().output.push(String::from(s));
		Ok(())
	}

	fn set_color(&mut self, colors: Colors) -> Result<(), DisplayError> {
		self.state.write().colors = colors;
		Ok(())
	}

	fn set_dim(&mut self, dim: bool) -> Result<(), DisplayError> {
		if dim {
			self.state.write().attributes.set(Attribute::Dim);
		}
		else {
			self.state.write().attributes.set(Attribute::NormalIntensity);
		}
		Ok(())
	}

	fn set_underline(&mut self, dim: bool) -> Result<(), DisplayError> {
		if dim {
			self.state.write().attributes.set(Attribute::Underlined);
		}
		else {
			self.state.write().attributes.set(Attribute::NoUnderline);
		}
		Ok(())
	}

	fn set_reverse(&mut self, dim: bool) -> Result<(), DisplayError> {
		if dim {
			self.state.write().attributes.set(Attribute::Reverse);
		}
		else {
			self.state.write().attributes.set(Attribute::NoReverse);
		}
		Ok(())
	}

	fn get_size(&self) -> Size {
		self.state.read().size
	}

	fn move_to_column(&mut self, x: u16) -> Result<(), DisplayError> {
		self.state.write().position.0 = x;
		Ok(())
	}

	fn move_next_line(&mut self) -> Result<(), DisplayError> {
		let mut state = self.state.write();
		state.output.push(String::from("\n"));
		state.position.0 = 0;
		state.position.1 += 1;
		Ok(())
	}

	fn start(&mut self) -> Result<(), DisplayError> {
		self.state.write().state = CrosstermMockState::Normal;
		Ok(())
	}

	fn end(&mut self) -> Result<(), DisplayError> {
		self.state.write().state = CrosstermMockState::Ended;
		Ok(())
	}
}

impl CrossTerm {
	/// Create a new mocked version of `CrossTerm`.
	#[must_use]
	pub(crate) fn new() -> Self {
		Self {
			state: Arc::new(RwLock::new(CrossTermInternalState {
				attributes: Attributes::from(Attribute::Reset),
				color_mode: ColorMode::FourBit,
				colors: Colors::new(Color::Reset, Color::Reset),
				dirty: true,
				output: vec![],
				position: (0, 0),
				size: Size::new(10, 10),
				state: CrosstermMockState::New,
			})),
		}
	}

	/// Get a representation of the rendered output.
	#[must_use]
	pub(crate) fn get_output(&self) -> Vec<String> {
		self.state.read().output.clone()
	}

	/// Get the current state.
	#[must_use]
	pub(crate) fn get_state(&self) -> CrosstermMockState {
		self.state.read().state
	}

	/// Are colors enabled.
	#[must_use]
	pub(crate) fn is_colors_enabled(&self, colors: Colors) -> bool {
		self.state.read().colors == colors
	}

	/// Does the current style attributes contained dimmed.
	#[must_use]
	pub(crate) fn is_dimmed(&self) -> bool {
		self.state.read().attributes.has(Attribute::Dim)
	}

	/// Does the current style attributes contained reverse.
	#[must_use]
	pub(crate) fn is_reverse(&self) -> bool {
		self.state.read().attributes.has(Attribute::Reverse)
	}

	/// Does the current style attributes contained underlined.
	#[must_use]
	pub(crate) fn is_underline(&self) -> bool {
		self.state.read().attributes.has(Attribute::Underlined)
	}

	/// Update the size.
	pub(crate) fn set_size(&mut self, size: Size) {
		self.state.write().size = size;
	}

	/// Get the current cursor position.
	#[must_use]
	pub(crate) fn get_position(&self) -> (u16, u16) {
		self.state.read().position
	}

	/// Has the output been flushed.
	#[must_use]
	pub(crate) fn is_dirty(&self) -> bool {
		self.state.read().dirty
	}
}

impl Clone for CrossTerm {
	fn clone(&self) -> Self {
		CrossTerm {
			state: Arc::clone(&self.state),
		}
	}
}
