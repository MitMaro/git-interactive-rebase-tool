use std::{
	io,
	io::{stdout, BufWriter, Stdout, Write},
	time::Duration,
};

use anyhow::{anyhow, Error, Result};
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

#[derive(Debug)]
pub struct CrossTerm {
	color_mode: ColorMode,
	window: BufWriter<Stdout>,
}

impl Tui for CrossTerm {
	fn get_color_mode(&self) -> ColorMode {
		self.color_mode
	}

	fn reset(&mut self) -> Result<()> {
		self.queue_command(ResetColor)?;
		self.queue_command(SetAttribute(Attribute::Reset))?;
		self.queue_command(Clear(ClearType::All))?;
		self.queue_command(MoveTo(0, 0))
	}

	fn flush(&mut self) -> Result<()> {
		self.window
			.flush()
			.map_err(|err| anyhow!("{:#}", err).context("Unexpected Error"))
	}

	fn print(&mut self, s: &str) -> Result<()> {
		self.queue_command(Print(s))
	}

	fn set_color(&mut self, colors: Colors) -> Result<()> {
		self.queue_command(SetColors(colors))
	}

	fn set_dim(&mut self, dim: bool) -> Result<()> {
		self.queue_command(SetAttribute(
			if dim {
				Attribute::Dim
			}
			else {
				Attribute::NormalIntensity
			},
		))
	}

	fn set_underline(&mut self, underline: bool) -> Result<()> {
		self.queue_command(SetAttribute(
			if underline {
				Attribute::Underlined
			}
			else {
				Attribute::NoUnderline
			},
		))
	}

	fn set_reverse(&mut self, reverse: bool) -> Result<()> {
		self.queue_command(SetAttribute(
			if reverse {
				Attribute::Reverse
			}
			else {
				Attribute::NoReverse
			},
		))
	}

	fn read_event() -> Result<Option<Event>> {
		if poll(Duration::from_millis(20)).unwrap_or(false) {
			read().map(Some).map_err(Self::map_err)
		}
		else {
			Ok(None)
		}
	}

	fn get_size(&self) -> Size {
		size().map_or_else(
			|_| Size::new(0, 0),
			|(width, height)| Size::new(usize::from(width), usize::from(height)),
		)
	}

	fn move_to_column(&mut self, x: u16) -> Result<()> {
		self.queue_command(MoveToColumn(x))
	}

	fn move_next_line(&mut self) -> Result<()> {
		self.queue_command(MoveToNextLine(1))
	}

	fn start(&mut self) -> Result<()> {
		self.queue_command(EnterAlternateScreen)?;
		self.queue_command(DisableLineWrap)?;
		self.queue_command(Hide)?;
		self.queue_command(EnableMouseCapture)?;
		enable_raw_mode().map_err(Self::map_err)?;
		self.flush()
	}

	fn end(&mut self) -> Result<()> {
		self.queue_command(DisableMouseCapture)?;
		self.queue_command(Show)?;
		self.queue_command(EnableLineWrap)?;
		self.queue_command(LeaveAlternateScreen)?;
		disable_raw_mode().map_err(Self::map_err)?;
		self.flush()
	}
}

impl CrossTerm {
	#[must_use]
	pub fn new() -> Self {
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
		let _result = self.window.queue(command).map_err(Self::map_err)?;
		Ok(())
	}
}
