mod action;
pub mod line_segment;
pub mod render_context;
mod render_slice;
mod scroll_position;
mod sender;
mod thread;
mod util;
pub mod view_data;
mod view_data_updater;
pub mod view_line;

#[cfg(test)]
mod tests;
#[cfg(test)]
pub mod testutil;

use anyhow::Result;

use self::{render_slice::RenderSlice, view_line::ViewLine};
pub use self::{
	sender::Sender as ViewSender,
	thread::spawn_view_thread,
	util::handle_view_data_scroll,
	view_data::ViewData,
	view_data_updater::ViewDataUpdater,
};
use crate::display::{Display, DisplayColor, Size};

const TITLE: &str = "Git Interactive Rebase Tool";
const TITLE_SHORT: &str = "Git Rebase";
const TITLE_HELP_INDICATOR_LABEL: &str = "Help: ";

pub struct View {
	character_vertical_spacing: String,
	display: Display,
	help_indicator_key: String,
	last_render_version: u32,
}

impl View {
	pub(crate) fn new(display: Display, character_vertical_spacing: &str, help_indicator_key: &str) -> Self {
		Self {
			character_vertical_spacing: String::from(character_vertical_spacing),
			display,
			help_indicator_key: String::from(help_indicator_key),
			last_render_version: u32::MAX,
		}
	}

	pub(crate) fn start(&mut self) -> Result<()> {
		self.display.start()
	}

	pub(crate) fn end(&mut self) -> Result<()> {
		self.display.end()
	}

	pub(crate) fn get_view_size(&self) -> Size {
		self.display.get_window_size()
	}

	pub(crate) fn render(&mut self, render_slice: &RenderSlice) -> Result<()> {
		let current_render_version = render_slice.get_version();
		if self.last_render_version == current_render_version {
			return Ok(());
		}
		self.last_render_version = current_render_version;
		let view_size = self.get_view_size();
		let window_height = view_size.height();

		self.display.clear()?;

		self.display.ensure_at_line_start()?;
		if render_slice.show_title() {
			self.display.ensure_at_line_start()?;
			self.draw_title(render_slice.show_help())?;
			self.display.next_line()?;
		}

		let lines = render_slice.get_lines();
		let leading_line_count = render_slice.get_leading_lines_count();
		let trailing_line_count = render_slice.get_trailing_lines_count();
		let lines_count = lines.len() - leading_line_count - trailing_line_count;
		let show_scroll_bar = render_slice.should_show_scroll_bar();
		let scroll_indicator_index = render_slice.get_scroll_index();
		let view_height = window_height - leading_line_count - trailing_line_count;

		let leading_lines_iter = lines.iter().take(leading_line_count);
		let lines_iter = lines.iter().skip(leading_line_count).take(lines_count);
		let trailing_lines_iter = lines.iter().skip(leading_line_count + lines_count);

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
				self.display
					.draw_str(if scroll_indicator_index == index { "█" } else { " " })?;
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
						.draw_str(format!("{}{}", TITLE_HELP_INDICATOR_LABEL, self.help_indicator_key).as_str())?;
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
