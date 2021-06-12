use std::{
	io,
	io::{stdout, BufWriter, Stdout, Write},
	time::Duration,
};

use anyhow::{anyhow, Error, Result};
use crossterm::{
	cursor::{Hide, MoveTo, MoveToColumn, MoveToNextLine, Show},
	event::{poll, read, DisableMouseCapture, EnableMouseCapture},
	style::{available_color_count, Attribute, Print, ResetColor, SetAttribute, SetColors},
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
		size,
		Clear,
		ClearType,
		DisableLineWrap,
		EnableLineWrap,
		EnterAlternateScreen,
		LeaveAlternateScreen,
	},
	Command,
	QueueableCommand,
};
pub use crossterm::{
	event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind},
	style::{Color, Colors},
};

use crate::display::{color_mode::ColorMode, size::Size, utils::detect_color_mode};

pub struct CrossTerm {
	color_mode: ColorMode,
	window: BufWriter<Stdout>,
}

impl CrossTerm {
	pub(crate) fn new() -> Self {
		Self {
			window: BufWriter::new(stdout()),
			color_mode: detect_color_mode(available_color_count()),
		}
	}

	#[allow(clippy::needless_pass_by_value)]
	fn map_err(err: io::Error) -> Error {
		anyhow!("{:#}", err).context("Unexpected Error")
	}

	fn queue_command(&mut self, command: impl Command) -> Result<()> {
		self.window.queue(command).map_err(Self::map_err)?;
		Ok(())
	}

	pub(super) const fn get_color_mode(&self) -> ColorMode {
		self.color_mode
	}

	pub(super) fn reset(&mut self) -> Result<()> {
		self.queue_command(ResetColor)?;
		self.queue_command(SetAttribute(Attribute::Reset))?;
		self.queue_command(Clear(ClearType::All))?;
		self.queue_command(MoveTo(0, 0))
	}

	pub(super) fn flush(&mut self) -> Result<()> {
		self.window
			.flush()
			.map_err(|err| anyhow!("{:#}", err).context("Unexpected Error"))
	}

	pub(super) fn print(&mut self, s: &str) -> Result<()> {
		self.queue_command(Print(s))
	}

	pub(super) fn set_color(&mut self, colors: Colors) -> Result<()> {
		self.queue_command(SetColors(colors))
	}

	pub(super) fn set_dim(&mut self, dim: bool) -> Result<()> {
		self.queue_command(SetAttribute(
			if dim {
				Attribute::Dim
			}
			else {
				Attribute::NormalIntensity
			},
		))
	}

	pub(super) fn set_underline(&mut self, underline: bool) -> Result<()> {
		self.queue_command(SetAttribute(
			if underline {
				Attribute::Underlined
			}
			else {
				Attribute::NoUnderline
			},
		))
	}

	pub(super) fn set_reverse(&mut self, reverse: bool) -> Result<()> {
		self.queue_command(SetAttribute(
			if reverse {
				Attribute::Reverse
			}
			else {
				Attribute::NoReverse
			},
		))
	}

	pub(crate) fn read_event() -> Result<Option<Event>> {
		if poll(Duration::from_millis(20)).unwrap_or(false) {
			read().map(Some).map_err(Self::map_err)
		}
		else {
			Ok(None)
		}
	}

	#[allow(clippy::unused_self)]
	pub(super) fn get_size(&self) -> Size {
		size().map_or_else(
			|_| Size::new(0, 0),
			|(width, height)| Size::new(usize::from(width), usize::from(height)),
		)
	}

	pub(crate) fn move_to_column(&mut self, x: u16) -> Result<()> {
		self.queue_command(MoveToColumn(x))
	}

	pub(crate) fn move_next_line(&mut self) -> Result<()> {
		self.queue_command(MoveToNextLine(1))
	}

	pub(crate) fn start(&mut self) -> Result<()> {
		self.queue_command(EnterAlternateScreen)?;
		self.queue_command(DisableLineWrap)?;
		self.queue_command(Hide)?;
		self.queue_command(EnableMouseCapture)?;
		enable_raw_mode().map_err(Self::map_err)?;
		self.flush()
	}

	pub(super) fn end(&mut self) -> Result<()> {
		self.queue_command(DisableMouseCapture)?;
		self.queue_command(Show)?;
		self.queue_command(EnableLineWrap)?;
		self.queue_command(LeaveAlternateScreen)?;
		disable_raw_mode().map_err(Self::map_err)?;
		self.flush()
	}
}
