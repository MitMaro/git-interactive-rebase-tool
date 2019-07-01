use crate::constants::{
	HEIGHT_ERROR_MESSAGE,
	MINIMUM_COMPACT_WINDOW_WIDTH,
	MINIMUM_WINDOW_HEIGHT,
	MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH,
	SHORT_ERROR_MESSAGE,
	SHORT_ERROR_MESSAGE_WIDTH,
	TITLE,
	TITLE_HELP_INDICATOR,
	TITLE_HELP_INDICATOR_LENGTH,
	TITLE_LENGTH,
	TITLE_SHORT,
	TITLE_SHORT_LENGTH,
};
use crate::scroll::{get_scroll_position, ScrollPosition};
use crate::view::{LineSegment, ViewLine};
use crate::window::Window;
use crate::window::WindowColor;

pub struct View<'v> {
	help_top: ScrollPosition,
	window: &'v Window<'v>,
}

impl<'v> View<'v> {
	pub fn new(window: &'v Window) -> Self {
		Self {
			help_top: ScrollPosition::new(3, 6, 3),
			window,
		}
	}

	pub fn draw_str(&self, s: &str) {
		self.window.draw_str(s);
	}

	pub fn set_color(&self, color: WindowColor) {
		self.window.color(color);
	}

	pub fn set_style(&self, dim: bool, underline: bool, reverse: bool) {
		self.window.set_style(dim, underline, reverse);
	}

	pub fn check_window_size(&self) -> bool {
		let (window_width, window_height) = self.window.get_window_size();
		!(window_width <= MINIMUM_COMPACT_WINDOW_WIDTH || window_height <= MINIMUM_WINDOW_HEIGHT)
	}

	pub fn draw_error(&self, message: &str) {
		self.draw_title(false);
		self.window.color(WindowColor::Foreground);
		self.window.set_style(false, false, false);
		self.window.draw_str(message);
		self.window.draw_str("\n");
		self.window.color(WindowColor::IndicatorColor);
		self.window.draw_str("Press any key to continue");
	}

	pub fn clear(&self) {
		self.window.clear();
	}

	pub fn get_view_size(&self) -> (usize, usize) {
		let (view_width, view_height) = self.window.get_window_size();
		(view_width as usize, view_height as usize)
	}

	pub fn refresh(&self) {
		self.window.refresh();
	}

	pub fn draw_window_size_error(&self) {
		let (window_width, window_height) = self.window.get_window_size();

		self.window.color(WindowColor::Foreground);
		if window_width <= MINIMUM_COMPACT_WINDOW_WIDTH {
			if window_width >= SHORT_ERROR_MESSAGE_WIDTH {
				self.window.draw_str(SHORT_ERROR_MESSAGE);
			}
			else {
				// not much to do if the window gets too narrow
				self.window.draw_str("Size!\n");
			}
			return;
		}

		if window_height <= MINIMUM_WINDOW_HEIGHT {
			if window_width >= MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH {
				self.window.draw_str(HEIGHT_ERROR_MESSAGE);
			}
			else if window_width >= SHORT_ERROR_MESSAGE_WIDTH {
				self.window.draw_str(SHORT_ERROR_MESSAGE);
			}
			else {
				// not much to do if the window gets too narrow
				self.window.draw_str("Size!\n");
			}
		}
	}

	pub fn draw_view_lines(&self, lines: Vec<ViewLine>, top: usize, height: usize) {
		let number_of_lines = lines.len();

		let scroll_indicator_index = get_scroll_position(top, number_of_lines, height);
		let show_scroll_bar = height < number_of_lines;

		let mut index: usize = 0;
		for line in lines.iter().skip(top).take(height) {
			self.draw_view_line(line, show_scroll_bar);
			if show_scroll_bar {
				self.window.color(WindowColor::Foreground);
				self.window.set_style(scroll_indicator_index != index, false, true);
				self.window.draw_str(" ");
			}
			index += 1;
		}

		if height > lines.len() {
			self.draw_vertical_spacer((height - index) as i32);
		}
	}

	pub fn draw_view_line(&self, line: &ViewLine, scrollbar: bool) {
		let (window_width, _) = self.window.get_window_size();
		let window_width = if scrollbar { window_width - 1 } else { window_width } as usize;

		let mut start = 0;
		for segment in line.get_segments() {
			start += segment.draw(window_width - start, &self.window);
			if start >= window_width {
				break;
			}
		}

		if start < window_width {
			let padding = " ".repeat(window_width - start);
			self.window.draw_str(padding.as_str());
		}
	}

	pub fn draw_title(&self, show_help: bool) {
		self.window.color(WindowColor::Foreground);
		self.window.set_style(false, true, true);
		let (window_width, _) = self.window.get_window_size();

		if window_width >= TITLE_LENGTH {
			self.window.draw_str(TITLE);
			// only draw help if there is room
			if window_width > TITLE_LENGTH + TITLE_HELP_INDICATOR_LENGTH {
				if (window_width - TITLE_LENGTH - TITLE_HELP_INDICATOR_LENGTH) > 0 {
					let padding = " ".repeat((window_width - TITLE_LENGTH - TITLE_HELP_INDICATOR_LENGTH) as usize);
					self.window.draw_str(padding.as_str());
				}
				if show_help {
					self.window.draw_str(TITLE_HELP_INDICATOR);
				}
				else {
					let padding = " ".repeat(TITLE_HELP_INDICATOR_LENGTH as usize);
					self.window.draw_str(padding.as_str());
				}
			}
			else if (window_width - TITLE_LENGTH) > 0 {
				let padding = " ".repeat((window_width - TITLE_LENGTH) as usize);
				self.window.draw_str(padding.as_str());
			}
		}
		else {
			self.window.draw_str(TITLE_SHORT);
			if (window_width - TITLE_SHORT_LENGTH) > 0 {
				let padding = " ".repeat((window_width - TITLE_SHORT_LENGTH) as usize);
				self.window.draw_str(padding.as_str());
			}
		}
	}

	fn draw_vertical_spacer(&self, repeat: i32) {
		self.window.color(WindowColor::Foreground);
		self.window.set_style(false, false, false);
		for _x in 0..repeat {
			self.window.draw_vertical_space_character();
		}
	}

	pub fn update_help_top(&self, scroll_up: bool, reset: bool, help_lines: &[(&str, &str)]) {
		let (_, window_height) = self.window.get_window_size();
		if reset {
			self.help_top.reset();
		}
		else if scroll_up {
			self.help_top.scroll_up(window_height as usize, help_lines.len());
		}
		else {
			self.help_top.scroll_down(window_height as usize, help_lines.len());
		}
	}

	pub fn draw_help(&self, help_lines: &[(&str, &str)]) {
		let (window_width, window_height) = self.window.get_window_size();
		let view_height = window_height as usize - 3;

		let mut view_lines: Vec<ViewLine> = vec![];

		for line in help_lines {
			view_lines.push(ViewLine::new(vec![
				LineSegment::new_with_color(format!(" {:4} ", line.0).as_str(), WindowColor::IndicatorColor),
				LineSegment::new(line.1),
			]));
		}

		self.window.set_style(false, false, false);
		self.window.clear();
		self.draw_title(false);

		self.window.color(WindowColor::Foreground);
		self.window.set_style(false, true, false);
		self.window.draw_str(" Key   Action");
		if window_width as usize > 13 {
			let padding = " ".repeat(window_width as usize - 13);
			self.window.draw_str(padding.as_str());
		}

		self.draw_view_lines(view_lines, self.help_top.get_position(), view_height);

		self.window.color(WindowColor::IndicatorColor);
		self.window.draw_str("Any key to close");
	}

	pub fn draw_prompt(&self, message: &str) {
		self.draw_title(false);
		self.window.set_style(false, false, false);
		self.window.draw_str(&format!("\n{} ", message));
	}

	pub fn draw_confirm(&self, message: &str) {
		self.draw_prompt(&format!("{} (y/n)? ", message));
	}
}
