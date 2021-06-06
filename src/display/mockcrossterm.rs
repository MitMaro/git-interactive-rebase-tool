use std::sync::Mutex;

use anyhow::{anyhow, Result};
use crossterm::style::{Attribute, Attributes};
pub use crossterm::{
	event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind},
	style::{Color, Colors},
};
use lazy_static::lazy_static;

use super::{color_mode::ColorMode, size::Size, tui::Tui, utils::detect_color_mode};

lazy_static! {
	static ref OUTPUT: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
	New,
	Normal,
	Ended,
}

pub struct CrossTerm {
	attributes: Attributes,
	colors: Colors,
	color_mode: ColorMode,
	position: (u16, u16),
	size: Size,
	state: State,
	dirty: bool,
}

impl Tui for CrossTerm {
	fn get_color_mode(&self) -> ColorMode {
		self.color_mode
	}

	fn reset(&mut self) -> Result<()> {
		self.attributes = Attributes::from(Attribute::Reset);
		self.colors = Colors::new(Color::Reset, Color::Reset);
		OUTPUT
			.lock()
			.map_err(|e| anyhow!("{}", e).context("Unable to lock output"))?
			.clear();
		self.state = State::Normal;
		Ok(())
	}

	fn flush(&mut self) -> Result<()> {
		self.dirty = false;
		Ok(())
	}

	fn print(&mut self, s: &str) -> Result<()> {
		OUTPUT
			.lock()
			.map_err(|e| anyhow!("{}", e).context("Unable to lock output"))?
			.push(String::from(s));
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
		OUTPUT
			.lock()
			.map_err(|e| anyhow!("{}", e).context("Unable to lock output"))?
			.push(String::from("\n"));
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
	pub(crate) fn new() -> Self {
		OUTPUT.lock().unwrap().clear();
		Self {
			attributes: Attributes::from(Attribute::Reset),
			colors: Colors::new(Color::Reset, Color::Reset),
			color_mode: detect_color_mode(16),
			dirty: true,
			position: (0, 0),
			size: Size::new(10, 10),
			state: State::New,
		}
	}

	// Start mock access functions
	pub(crate) fn get_output() -> Vec<String> {
		OUTPUT.lock().unwrap().clone()
	}

	pub(crate) const fn get_state(&self) -> State {
		self.state
	}

	pub(crate) fn is_colors_enabled(&self, colors: Colors) -> bool {
		self.colors == colors
	}

	pub(crate) fn is_dimmed(&self) -> bool {
		self.attributes.has(Attribute::Dim)
	}

	pub(crate) fn is_reverse(&self) -> bool {
		self.attributes.has(Attribute::Reverse)
	}

	pub(crate) fn is_underline(&self) -> bool {
		self.attributes.has(Attribute::Underlined)
	}

	pub(crate) fn set_size(&mut self, size: Size) {
		self.size = size;
	}

	pub(crate) const fn get_position(&self) -> (u16, u16) {
		self.position
	}

	pub(crate) const fn is_dirty(&self) -> bool {
		self.dirty
	}

	// End mock access functions
}
