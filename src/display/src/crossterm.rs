use std::{
	io::{stdout, BufWriter, Stdout, Write},
	time::Duration,
};

use crossterm::{
	cursor::{Hide, MoveTo, MoveToColumn, MoveToNextLine, Show},
	event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event},
	style::{available_color_count, Attribute, Colors, Print, ResetColor, SetAttribute, SetColors},
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

use super::{color_mode::ColorMode, size::Size, tui::Tui, utils::detect_color_mode};
use crate::error::DisplayError;

/// A thin wrapper over the [Crossterm library](https://github.com/crossterm-rs/crossterm).
#[derive(Debug)]
pub struct CrossTerm {
	color_mode: ColorMode,
	window: BufWriter<Stdout>,
}

impl Tui for CrossTerm {
	#[inline]
	fn get_color_mode(&self) -> ColorMode {
		self.color_mode
	}

	#[inline]
	fn reset(&mut self) -> Result<(), DisplayError> {
		self.queue_command(ResetColor)?;
		self.queue_command(SetAttribute(Attribute::Reset))?;
		self.queue_command(Clear(ClearType::All))?;
		self.queue_command(MoveTo(0, 0))
	}

	#[inline]
	fn flush(&mut self) -> Result<(), DisplayError> {
		self.window.flush().map_err(DisplayError::Unexpected)
	}

	#[inline]
	fn print(&mut self, s: &str) -> Result<(), DisplayError> {
		self.queue_command(Print(s))
	}

	#[inline]
	fn set_color(&mut self, colors: Colors) -> Result<(), DisplayError> {
		self.queue_command(SetColors(colors))
	}

	#[inline]
	fn set_dim(&mut self, dim: bool) -> Result<(), DisplayError> {
		self.queue_command(SetAttribute(
			if dim {
				Attribute::Dim
			}
			else {
				Attribute::NormalIntensity
			},
		))
	}

	#[inline]
	fn set_underline(&mut self, underline: bool) -> Result<(), DisplayError> {
		self.queue_command(SetAttribute(
			if underline {
				Attribute::Underlined
			}
			else {
				Attribute::NoUnderline
			},
		))
	}

	#[inline]
	fn set_reverse(&mut self, reverse: bool) -> Result<(), DisplayError> {
		self.queue_command(SetAttribute(
			if reverse {
				Attribute::Reverse
			}
			else {
				Attribute::NoReverse
			},
		))
	}

	#[inline]
	fn read_event() -> Result<Option<Event>, DisplayError> {
		if poll(Duration::from_millis(20)).unwrap_or(false) {
			read().map(Some).map_err(DisplayError::Unexpected)
		}
		else {
			Ok(None)
		}
	}

	#[inline]
	fn get_size(&self) -> Size {
		size().map_or_else(
			|_| Size::new(0, 0),
			|(width, height)| Size::new(usize::from(width), usize::from(height)),
		)
	}

	#[inline]
	fn move_to_column(&mut self, x: u16) -> Result<(), DisplayError> {
		self.queue_command(MoveToColumn(x))
	}

	#[inline]
	fn move_next_line(&mut self) -> Result<(), DisplayError> {
		self.queue_command(MoveToNextLine(1))
	}

	#[inline]
	fn start(&mut self) -> Result<(), DisplayError> {
		self.queue_command(EnterAlternateScreen)?;
		self.queue_command(DisableLineWrap)?;
		self.queue_command(Hide)?;
		self.queue_command(EnableMouseCapture)?;
		enable_raw_mode().map_err(DisplayError::Unexpected)?;
		self.flush()
	}

	#[inline]
	fn end(&mut self) -> Result<(), DisplayError> {
		self.queue_command(DisableMouseCapture)?;
		self.queue_command(Show)?;
		self.queue_command(EnableLineWrap)?;
		self.queue_command(LeaveAlternateScreen)?;
		disable_raw_mode().map_err(DisplayError::Unexpected)?;
		self.flush()
	}
}

impl CrossTerm {
	/// Create a new instance.
	#[inline]
	#[must_use]
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self {
			window: BufWriter::new(stdout()),
			color_mode: detect_color_mode(available_color_count()),
		}
	}

	fn queue_command(&mut self, command: impl Command) -> Result<(), DisplayError> {
		let _result = self.window.queue(command).map_err(DisplayError::Unexpected)?;
		Ok(())
	}
}
