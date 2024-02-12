use std::{ops::Deref, sync::Arc};

use ::crossterm::style::{Attribute, Attributes, Color, Colors};
use parking_lot::RwLock;

use crate::display::{
	testutil::{MockableTui, State},
	ColorMode,
	DisplayError,
	Size,
};

#[derive(Debug)]
struct CrossTermInternalState {
	attributes: Attributes,
	color_mode: ColorMode,
	colors: Colors,
	dirty: bool,
	output: Vec<String>,
	position: (u16, u16),
	size: Size,
	state: State,
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
		state.state = State::Normal;
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
		self.state.write().state = State::Normal;
		Ok(())
	}

	fn end(&mut self) -> Result<(), DisplayError> {
		self.state.write().state = State::Ended;
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
				state: State::New,
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
	pub(crate) fn get_state(&self) -> State {
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
