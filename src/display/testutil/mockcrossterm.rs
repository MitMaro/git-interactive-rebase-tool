use ::crossterm::style::{Attribute, Attributes, Color, Colors};

use crate::display::{
	testutil::{MockableTui, State},
	ColorMode,
	DisplayError,
	Size,
};

/// A mocked version of `CrossTerm`, useful for testing.
#[derive(Debug)]
pub(crate) struct CrossTerm {
	attributes: Attributes,
	color_mode: ColorMode,
	colors: Colors,
	dirty: bool,
	output: Vec<String>,
	position: (u16, u16),
	size: Size,
	state: State,
}

impl MockableTui for CrossTerm {
	fn get_color_mode(&self) -> ColorMode {
		self.color_mode
	}

	fn reset(&mut self) -> Result<(), DisplayError> {
		self.attributes = Attributes::from(Attribute::Reset);
		self.colors = Colors::new(Color::Reset, Color::Reset);
		self.output.clear();
		self.state = State::Normal;
		Ok(())
	}

	fn flush(&mut self) -> Result<(), DisplayError> {
		self.dirty = false;
		Ok(())
	}

	fn print(&mut self, s: &str) -> Result<(), DisplayError> {
		self.output.push(String::from(s));
		Ok(())
	}

	fn set_color(&mut self, colors: Colors) -> Result<(), DisplayError> {
		self.colors = colors;
		Ok(())
	}

	fn set_dim(&mut self, dim: bool) -> Result<(), DisplayError> {
		if dim {
			self.attributes.set(Attribute::Dim);
		}
		else {
			self.attributes.set(Attribute::NormalIntensity);
		}
		Ok(())
	}

	fn set_underline(&mut self, dim: bool) -> Result<(), DisplayError> {
		if dim {
			self.attributes.set(Attribute::Underlined);
		}
		else {
			self.attributes.set(Attribute::NoUnderline);
		}
		Ok(())
	}

	fn set_reverse(&mut self, dim: bool) -> Result<(), DisplayError> {
		if dim {
			self.attributes.set(Attribute::Reverse);
		}
		else {
			self.attributes.set(Attribute::NoReverse);
		}
		Ok(())
	}

	fn get_size(&self) -> Size {
		self.size
	}

	fn move_to_column(&mut self, x: u16) -> Result<(), DisplayError> {
		self.position.0 = x;
		Ok(())
	}

	fn move_next_line(&mut self) -> Result<(), DisplayError> {
		self.output.push(String::from("\n"));
		self.position.0 = 0;
		self.position.1 += 1;
		Ok(())
	}

	fn start(&mut self) -> Result<(), DisplayError> {
		self.state = State::Normal;
		Ok(())
	}

	fn end(&mut self) -> Result<(), DisplayError> {
		self.state = State::Ended;
		Ok(())
	}
}

impl CrossTerm {
	/// Create a new mocked version of `CrossTerm`.
	#[must_use]
	pub(crate) fn new() -> Self {
		Self {
			attributes: Attributes::from(Attribute::Reset),
			color_mode: ColorMode::FourBit,
			colors: Colors::new(Color::Reset, Color::Reset),
			dirty: true,
			output: vec![],
			position: (0, 0),
			size: Size::new(10, 10),
			state: State::New,
		}
	}

	/// Get a representation of the rendered output.
	#[must_use]
	pub(crate) const fn get_output(&self) -> &Vec<String> {
		&self.output
	}

	/// Get the current state.
	#[must_use]
	pub(crate) const fn get_state(&self) -> State {
		self.state
	}

	/// Are colors enabled.
	#[must_use]
	pub(crate) fn is_colors_enabled(&self, colors: Colors) -> bool {
		self.colors == colors
	}

	/// Does the current style attributes contained dimmed.
	#[must_use]
	pub(crate) const fn is_dimmed(&self) -> bool {
		self.attributes.has(Attribute::Dim)
	}

	/// Does the current style attributes contained reverse.
	#[must_use]
	pub(crate) const fn is_reverse(&self) -> bool {
		self.attributes.has(Attribute::Reverse)
	}

	/// Does the current style attributes contained underlined.
	#[must_use]
	pub(crate) const fn is_underline(&self) -> bool {
		self.attributes.has(Attribute::Underlined)
	}

	/// Update the size.
	pub(crate) fn set_size(&mut self, size: Size) {
		self.size = size;
	}

	/// Get the current cursor position.
	#[must_use]
	pub(crate) const fn get_position(&self) -> (u16, u16) {
		self.position
	}

	/// Has the output been flushed.
	#[must_use]
	pub(crate) const fn is_dirty(&self) -> bool {
		self.dirty
	}
}
