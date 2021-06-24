use anyhow::Result;
use crossterm::{
	event::Event,
	style::{Attribute, Attributes, Color, Colors},
};

use crate::{testutil::State, ColorMode, Size, Tui};

/// A mocked version of `CrossTerm`, useful for testing.
#[derive(Debug)]
pub struct CrossTerm {
	attributes: Attributes,
	color_mode: ColorMode,
	colors: Colors,
	dirty: bool,
	output: Vec<String>,
	position: (u16, u16),
	size: Size,
	state: State,
}

impl Tui for CrossTerm {
	#[inline]
	fn get_color_mode(&self) -> ColorMode {
		self.color_mode
	}

	#[inline]
	fn reset(&mut self) -> Result<()> {
		self.attributes = Attributes::from(Attribute::Reset);
		self.colors = Colors::new(Color::Reset, Color::Reset);
		self.output.clear();
		self.state = State::Normal;
		Ok(())
	}

	#[inline]
	fn flush(&mut self) -> Result<()> {
		self.dirty = false;
		Ok(())
	}

	#[inline]
	fn print(&mut self, s: &str) -> Result<()> {
		self.output.push(String::from(s));
		Ok(())
	}

	#[inline]
	fn set_color(&mut self, colors: Colors) -> Result<()> {
		self.colors = colors;
		Ok(())
	}

	#[inline]
	fn set_dim(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Dim);
		}
		else {
			self.attributes.set(Attribute::NormalIntensity);
		}
		Ok(())
	}

	#[inline]
	fn set_underline(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Underlined);
		}
		else {
			self.attributes.set(Attribute::NoUnderline);
		}
		Ok(())
	}

	#[inline]
	fn set_reverse(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Reverse);
		}
		else {
			self.attributes.set(Attribute::NoReverse);
		}
		Ok(())
	}

	#[inline]
	fn read_event() -> Result<Option<Event>> {
		Ok(None)
	}

	#[inline]
	fn get_size(&self) -> Size {
		self.size
	}

	#[inline]
	fn move_to_column(&mut self, x: u16) -> Result<()> {
		self.position.0 = x;
		Ok(())
	}

	#[inline]
	fn move_next_line(&mut self) -> Result<()> {
		self.output.push(String::from("\n"));
		self.position.0 = 0;
		self.position.1 += 1;
		Ok(())
	}

	#[inline]
	fn start(&mut self) -> Result<()> {
		self.state = State::Normal;
		Ok(())
	}

	#[inline]
	fn end(&mut self) -> Result<()> {
		self.state = State::Ended;
		Ok(())
	}
}

impl CrossTerm {
	/// Create a new mocked version of `CrossTerm`.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
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
	#[inline]
	#[must_use]
	pub const fn get_output(&self) -> &Vec<String> {
		&self.output
	}

	/// Get the current state.
	#[inline]
	#[must_use]
	pub const fn get_state(&self) -> State {
		self.state
	}

	/// Are colors enabled.
	#[inline]
	#[must_use]
	pub fn is_colors_enabled(&self, colors: Colors) -> bool {
		self.colors == colors
	}

	/// Does the current style attributes contained dimmed.
	#[inline]
	#[must_use]
	pub fn is_dimmed(&self) -> bool {
		self.attributes.has(Attribute::Dim)
	}

	/// Does the current style attributes contained reverse.
	#[inline]
	#[must_use]
	pub fn is_reverse(&self) -> bool {
		self.attributes.has(Attribute::Reverse)
	}

	/// Does the current style attributes contained underlined.
	#[inline]
	#[must_use]
	pub fn is_underline(&self) -> bool {
		self.attributes.has(Attribute::Underlined)
	}

	/// Update the size.
	#[inline]
	pub fn set_size(&mut self, size: Size) {
		self.size = size;
	}

	/// Get the current cursor position.
	#[inline]
	#[must_use]
	pub const fn get_position(&self) -> (u16, u16) {
		self.position
	}

	/// Has the output been flushed.
	#[inline]
	#[must_use]
	pub const fn is_dirty(&self) -> bool {
		self.dirty
	}
}
