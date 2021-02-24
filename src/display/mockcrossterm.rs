use std::sync::Mutex;

use anyhow::{anyhow, Result};
use crossterm::style::{Attribute, Attributes};
pub use crossterm::{
	event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind},
	style::{Color, Colors},
};
use lazy_static::lazy_static;

use crate::{
	create_key_event,
	display::{color_mode::ColorMode, size::Size, utils::detect_color_mode},
};

lazy_static! {
	static ref INPUT: Mutex<Vec<Event>> = Mutex::new(vec![create_key_event!('c', "Control")]);
}

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

	pub(crate) fn set_inputs(mut input: Vec<Event>) {
		input.reverse();
		INPUT.lock().unwrap().clear();
		INPUT.lock().unwrap().append(&mut input);
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

	pub(super) const fn get_color_mode(&self) -> ColorMode {
		self.color_mode
	}

	pub(super) fn reset(&mut self) -> Result<()> {
		self.attributes = Attributes::from(Attribute::Reset);
		self.colors = Colors::new(Color::Reset, Color::Reset);
		OUTPUT
			.lock()
			.map_err(|e| anyhow!("{}", e).context("Unable to lock output"))?
			.clear();
		self.state = State::Normal;
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(super) fn flush(&mut self) -> Result<()> {
		self.dirty = false;
		Ok(())
	}

	#[allow(clippy::unused_self, clippy::unnecessary_wraps)]
	pub(super) fn print(&mut self, s: &str) -> Result<()> {
		OUTPUT
			.lock()
			.map_err(|e| anyhow!("{}", e).context("Unable to lock output"))?
			.push(String::from(s));
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(super) fn set_color(&mut self, colors: Colors) -> Result<()> {
		self.colors = colors;
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(super) fn set_dim(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Dim);
		}
		else {
			self.attributes.set(Attribute::NormalIntensity);
		}
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(super) fn set_underline(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Underlined);
		}
		else {
			self.attributes.set(Attribute::NoUnderline);
		}
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(super) fn set_reverse(&mut self, dim: bool) -> Result<()> {
		if dim {
			self.attributes.set(Attribute::Reverse);
		}
		else {
			self.attributes.set(Attribute::NoReverse);
		}
		Ok(())
	}

	pub(crate) fn read_event() -> Result<Event> {
		if let Some(input) = INPUT
			.lock()
			.map_err(|e| anyhow!("{}", e).context("Unable to lock output"))?
			.pop()
		{
			Ok(input)
		}
		else {
			Err(anyhow!("Error"))
		}
	}

	#[allow(clippy::missing_const_for_fn)]
	pub(super) fn get_size(&self) -> Size {
		self.size
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(crate) fn move_to_column(&mut self, x: u16) -> Result<()> {
		self.position.0 = x;
		Ok(())
	}

	pub(crate) fn move_next_line(&mut self) -> Result<()> {
		OUTPUT
			.lock()
			.map_err(|e| anyhow!("{}", e).context("Unable to lock output"))?
			.push(String::from("\n"));
		self.position.0 = 0;
		self.position.1 += 1;
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(crate) fn start(&mut self) -> Result<()> {
		self.state = State::Normal;
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(crate) fn end(&mut self) -> Result<()> {
		self.state = State::Ended;
		Ok(())
	}
}
