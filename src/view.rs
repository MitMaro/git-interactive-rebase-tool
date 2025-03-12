// TODO deny clippy::same_name_method again once bitflags/bitflags#374 is merged
// #![allow(clippy::same_name_method, clippy::as_conversions, clippy::integer_division)]
//! Git Interactive Rebase Tool - View Module
//!
//! # Description
//! This module is used to handle working with the view.
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance should only be used in test code.

mod line_segment;
mod render_context;
mod render_slice;
mod scroll_position;
mod thread;
mod view_data;
mod view_data_updater;
mod view_line;
mod view_lines;

#[cfg(test)]
mod tests;

use anyhow::{Error, Result};

#[cfg(test)]
pub(crate) use self::render_slice::RenderAction;
pub(crate) use self::{
	line_segment::{LineSegment, LineSegmentOptions},
	render_context::RenderContext,
	render_slice::RenderSlice,
	scroll_position::ScrollPosition,
	thread::{REFRESH_THREAD_NAME, State, Thread, ViewAction},
	view_data::ViewData,
	view_data_updater::ViewDataUpdater,
	view_line::ViewLine,
	view_lines::ViewLines,
};
use crate::display::{Display, DisplayColor, Tui};

const TITLE: &str = "Git Interactive Rebase Tool";
const TITLE_SHORT: &str = "Git Rebase";
const TITLE_HELP_INDICATOR_LABEL: &str = "Help: ";
const SCROLLBAR_INDICATOR_CHARACTER: &str = "\u{2588}"; // "█"

/// Represents a view.
#[derive(Debug)]
pub(crate) struct View<C: Tui> {
	character_vertical_spacing: String,
	display: Display<C>,
	help_indicator_key: String,
	last_render_version: u32,
}

impl<C: Tui> View<C> {
	/// Create a new instance of the view.
	pub(crate) fn new(display: Display<C>, character_vertical_spacing: &str, help_indicator_key: &str) -> Self {
		Self {
			character_vertical_spacing: String::from(character_vertical_spacing),
			display,
			help_indicator_key: String::from(help_indicator_key),
			last_render_version: u32::MAX,
		}
	}

	/// End processing of the view.
	///
	/// # Errors
	/// Results in an error if the terminal cannot be started.
	pub(crate) fn start(&mut self) -> Result<()> {
		self.display.start().map_err(Error::from)
	}

	/// End the view processing.
	///
	/// # Errors
	/// Results in an error if the terminal cannot be ended.
	pub(crate) fn end(&mut self) -> Result<()> {
		self.display.end().map_err(Error::from)
	}

	/// Render a slice.
	///
	/// # Errors
	/// Results in an error if there are errors with interacting with the terminal.
	pub(crate) fn render(&mut self, render_slice: &RenderSlice) -> Result<()> {
		let current_render_version = render_slice.get_version();
		if self.last_render_version == current_render_version {
			return Ok(());
		}
		self.last_render_version = current_render_version;
		let view_size = self.display.get_window_size();
		let window_height = view_size.height();

		self.display.clear()?;

		self.display.ensure_at_line_start()?;
		if render_slice.show_title() {
			self.display.ensure_at_line_start()?;
			self.draw_title(render_slice.show_help())?;
			self.display.next_line()?;
		}

		let view_lines = render_slice.view_lines();
		let leading_line_count = render_slice.get_leading_lines_count();
		let trailing_line_count = render_slice.get_trailing_lines_count();
		let lines_count = view_lines.count() as usize - leading_line_count - trailing_line_count;
		let show_scroll_bar = render_slice.should_show_scroll_bar();
		let scroll_indicator_index = render_slice.get_scroll_index();
		let view_height = window_height - leading_line_count - trailing_line_count;

		let leading_lines_iter = view_lines.iter().take(leading_line_count);
		let lines_iter = view_lines.iter().skip(leading_line_count).take(lines_count);
		let trailing_lines_iter = view_lines.iter().skip(leading_line_count + lines_count);

		for line in leading_lines_iter {
			self.display.ensure_at_line_start()?;
			self.draw_view_line(line)?;
			self.display.next_line()?;
		}

		for (index, line) in lines_iter.enumerate() {
			self.display.ensure_at_line_start()?;
			self.draw_view_line(line)?;
			if show_scroll_bar {
				self.display.move_from_end_of_line(1)?;
				self.display.color(DisplayColor::Normal, true)?;
				self.display.draw_str(
					if scroll_indicator_index == index {
						SCROLLBAR_INDICATOR_CHARACTER
					}
					else {
						" "
					},
				)?;
			}
			self.display.color(DisplayColor::Normal, false)?;
			self.display.set_style(false, false, false)?;
			self.display.next_line()?;
		}

		if view_height > lines_count {
			self.display.color(DisplayColor::Normal, false)?;
			self.display.set_style(false, false, false)?;
			let draw_height = view_height - lines_count - if render_slice.show_title() { 1 } else { 0 };
			self.display.ensure_at_line_start()?;
			for _x in 0..draw_height {
				self.display.draw_str(self.character_vertical_spacing.as_str())?;
				self.display.next_line()?;
			}
		}

		for line in trailing_lines_iter {
			self.display.ensure_at_line_start()?;
			self.draw_view_line(line)?;
			self.display.next_line()?;
		}
		self.display.refresh()?;
		Ok(())
	}

	fn draw_view_line(&mut self, line: &ViewLine) -> Result<()> {
		for segment in line.get_segments() {
			self.display.color(segment.get_color(), line.get_selected())?;
			self.display
				.set_style(segment.is_dimmed(), segment.is_underlined(), segment.is_reversed())?;
			self.display.draw_str(segment.get_content())?;
		}

		// reset style
		self.display.color(DisplayColor::Normal, false)?;
		self.display.set_style(false, false, false)?;
		Ok(())
	}

	fn draw_title(&mut self, show_help: bool) -> Result<()> {
		self.display.color(DisplayColor::Normal, false)?;
		self.display.set_style(false, true, false)?;
		let window_width = self.display.get_window_size().width();

		let title_help_indicator_total_length = TITLE_HELP_INDICATOR_LABEL.len() + self.help_indicator_key.len();

		if window_width >= TITLE.len() {
			self.display.draw_str(TITLE)?;
			// only draw help if there is room
			if window_width > TITLE.len() + title_help_indicator_total_length {
				if (window_width - TITLE.len() - title_help_indicator_total_length) > 0 {
					let padding = " ".repeat(window_width - TITLE.len() - title_help_indicator_total_length);
					self.display.draw_str(padding.as_str())?;
				}
				if show_help {
					self.display
						.draw_str(format!("{TITLE_HELP_INDICATOR_LABEL}{}", self.help_indicator_key).as_str())?;
				}
				else {
					let padding = " ".repeat(title_help_indicator_total_length);
					self.display.draw_str(padding.as_str())?;
				}
			}
			else if (window_width - TITLE.len()) > 0 {
				let padding = " ".repeat(window_width - TITLE.len());
				self.display.draw_str(padding.as_str())?;
			}
		}
		else {
			self.display.draw_str(TITLE_SHORT)?;
			if (window_width - TITLE_SHORT.len()) > 0 {
				let padding = " ".repeat(window_width - TITLE_SHORT.len());
				self.display.draw_str(padding.as_str())?;
			}
		}

		// reset style
		self.display.color(DisplayColor::Normal, false)?;
		self.display.set_style(false, false, false)?;
		Ok(())
	}
}
