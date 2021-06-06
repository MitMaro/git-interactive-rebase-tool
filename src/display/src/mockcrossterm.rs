use anyhow::Result;
use crossterm::{
	event::Event,
	style::{Attribute, Attributes, Color, Colors},
};

use super::{color_mode::ColorMode, size::Size, tui::Tui, utils::detect_color_mode};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
	New,
	Normal,
	Ended,
}

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
	fn get_color_mode(&self) -> ColorMode {
		self.color_mode
	}

	fn reset(&mut self) -> Result<()> {
		self.attributes = Attributes::from(Attribute::Reset);
		self.colors = Colors::new(Color::Reset, Color::Reset);
		self.output.clear();
		self.state = State::Normal;
		Ok(())
	}

	fn flush(&mut self) -> Result<()> {
		self.dirty = false;
		Ok(())
	}

	fn print(&mut self, s: &str) -> Result<()> {
		self.output.push(String::from(s));
		Ok(())
	}

	fn set_color(&mut self, colors: Colors) -> Result<()> {
		self.colors = colors;
		Ok(())
	}

	fn set_dim(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Dim);
		}
		else {
			self.attributes.set(Attribute::NormalIntensity);
		}
		Ok(())
	}

	fn set_underline(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Underlined);
		}
		else {
			self.attributes.set(Attribute::NoUnderline);
		}
		Ok(())
	}

	fn set_reverse(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Reverse);
		}
		else {
			self.attributes.set(Attribute::NoReverse);
		}
		Ok(())
	}

	fn read_event() -> Result<Option<Event>> {
		Ok(None)
	}

	fn get_size(&self) -> Size {
		self.size
	}

	fn move_to_column(&mut self, x: u16) -> Result<()> {
		self.position.0 = x;
		Ok(())
	}

	fn move_next_line(&mut self) -> Result<()> {
		self.output.push(String::from("\n"));
		self.position.0 = 0;
		self.position.1 += 1;
		Ok(())
	}

	fn start(&mut self) -> Result<()> {
		self.state = State::Normal;
		Ok(())
	}

	fn end(&mut self) -> Result<()> {
		self.state = State::Ended;
		Ok(())
	}
}

impl CrossTerm {
	#[must_use]
	pub fn new() -> Self {
		Self {
			attributes: Attributes::from(Attribute::Reset),
			color_mode: detect_color_mode(16),
			colors: Colors::new(Color::Reset, Color::Reset),
			dirty: true,
			output: vec![],
			position: (0, 0),
			size: Size::new(10, 10),
			state: State::New,
		}
	}

	#[must_use]
	pub const fn get_output(&self) -> &Vec<String> {
		&self.output
	}

	#[must_use]
	pub const fn get_state(&self) -> State {
		self.state
	}

	#[must_use]
	pub fn is_colors_enabled(&self, colors: Colors) -> bool {
		self.colors == colors
	}

	#[must_use]
	pub fn is_dimmed(&self) -> bool {
		self.attributes.has(Attribute::Dim)
	}

	#[must_use]
	pub fn is_reverse(&self) -> bool {
		self.attributes.has(Attribute::Reverse)
	}

	#[must_use]
	pub fn is_underline(&self) -> bool {
		self.attributes.has(Attribute::Underlined)
	}

	pub fn set_size(&mut self, size: Size) {
		self.size = size;
	}

	#[must_use]
	pub const fn get_position(&self) -> (u16, u16) {
		self.position
	}

	#[must_use]
	pub const fn is_dirty(&self) -> bool {
		self.dirty
	}
}
