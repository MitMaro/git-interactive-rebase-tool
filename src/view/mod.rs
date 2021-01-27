pub mod line_segment;
pub mod scroll_position;
#[cfg(test)]
pub mod testutil;
pub mod view_data;
pub mod view_line;

use crate::constants::{TITLE, TITLE_HELP_INDICATOR_LENGTH, TITLE_LENGTH, TITLE_SHORT, TITLE_SHORT_LENGTH};
use crate::display::display_color::DisplayColor;
use crate::display::size::Size;
use crate::display::Display;
use crate::input::input_handler::InputMode;
use crate::input::Input;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::Config;
use anyhow::Result;

pub struct View<'v> {
	config: &'v Config,
	display: Display<'v>,
}

impl<'v> View<'v> {
	pub(crate) const fn new(display: Display<'v>, config: &'v Config) -> Self {
		Self { display, config }
	}

	pub(crate) fn start(&mut self) -> Result<()> {
		self.display.start()
	}

	pub(crate) fn end(&mut self) -> Result<()> {
		self.display.end()
	}

	pub(crate) fn get_input(&self, mode: InputMode) -> Input {
		self.display.get_input(mode)
	}

	pub(crate) fn get_view_size(&self) -> Size {
		self.display.get_window_size()
	}

	pub(crate) fn render(&mut self, view_data: &ViewData) -> Result<()> {
		self.display.clear()?;
		let window_height = self.display.get_window_size().height();

		self.display.ensure_at_line_start()?;
		if view_data.show_title() {
			self.display.ensure_at_line_start()?;
			self.draw_title(view_data.show_help())?;
			self.display.next_line()?;
		}

		if let Some(ref prompt) = *view_data.get_prompt() {
			self.display.set_style(false, false, false)?;
			self.display.next_line()?;
			self.display.draw_str(&format!(
				"{} ({}/{})? ",
				prompt, self.config.key_bindings.confirm_yes, self.config.key_bindings.confirm_no
			))?;
			self.display.next_line()?;
			self.display.refresh()?;
			return Ok(());
		}

		let leading_lines = view_data.get_leading_lines();
		let lines = view_data.get_lines();
		let trailing_lines = view_data.get_trailing_lines();

		let view_height = window_height - leading_lines.len() - trailing_lines.len();

		let show_scroll_bar = view_data.should_show_scroll_bar();
		let scroll_indicator_index = view_data.get_scroll_index();

		for line in leading_lines {
			self.display.ensure_at_line_start()?;
			self.draw_view_line(line)?;
			self.display.next_line()?;
		}

		for (index, line) in lines.iter().enumerate() {
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

		if view_height > lines.len() {
			self.display.color(DisplayColor::Normal, false)?;
			self.display.set_style(false, false, false)?;
			let draw_height = view_height - lines.len() - if view_data.show_title() { 1 } else { 0 };
			self.display.ensure_at_line_start()?;
			for _x in 0..draw_height {
				self.display
					.draw_str(self.config.theme.character_vertical_spacing.as_str())?;
				self.display.next_line()?;
			}
		}

		for line in trailing_lines {
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

		let title_help_indicator_total_length = TITLE_HELP_INDICATOR_LENGTH + self.config.key_bindings.help.len();

		if window_width >= TITLE_LENGTH {
			self.display.draw_str(TITLE)?;
			// only draw help if there is room
			if window_width > TITLE_LENGTH + title_help_indicator_total_length {
				if (window_width - TITLE_LENGTH - title_help_indicator_total_length) > 0 {
					let padding = " ".repeat(window_width - TITLE_LENGTH - title_help_indicator_total_length);
					self.display.draw_str(padding.as_str())?;
				}
				if show_help {
					self.display
						.draw_str(format!("Help: {}", self.config.key_bindings.help).as_str())?;
				}
				else {
					let padding = " ".repeat(title_help_indicator_total_length);
					self.display.draw_str(padding.as_str())?;
				}
			}
			else if (window_width - TITLE_LENGTH) > 0 {
				let padding = " ".repeat(window_width - TITLE_LENGTH);
				self.display.draw_str(padding.as_str())?;
			}
		}
		else {
			self.display.draw_str(TITLE_SHORT)?;
			if (window_width - TITLE_SHORT_LENGTH) > 0 {
				let padding = " ".repeat(window_width - TITLE_SHORT_LENGTH);
				self.display.draw_str(padding.as_str())?;
			}
		}

		// reset style
		self.display.color(DisplayColor::Normal, false)?;
		self.display.set_style(false, false, false)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::config::Config;
	use crate::display::CrossTerm;
	use crate::input::input_handler::InputHandler;
	use std::env::set_var;
	use std::path::Path;

	pub struct TestContext<'t> {
		pub view: View<'t>,
	}

	impl<'t> TestContext<'t> {
		fn assert_output(expected: &[&str]) {
			assert_eq!(CrossTerm::get_output().join(""), format!("{}\n", expected.join("\n")));
		}
	}

	pub fn view_module_test<F>(size: Size, callback: F)
	where F: FnOnce(TestContext<'_>) {
		set_var(
			"GIT_DIR",
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("test")
				.join("fixtures")
				.join("simple")
				.to_str()
				.unwrap(),
		);
		let config = Config::new().unwrap();
		let mut crossterm = CrossTerm::new();
		crossterm.set_size(size);
		let input_handler = InputHandler::new(&config.key_bindings);
		let display = Display::new(input_handler, &mut crossterm, &config.theme);
		let view = View::new(display, &config);
		callback(TestContext { view });
	}

	#[test]
	#[serial_test::serial]
	fn get_view_size() {
		view_module_test(Size::new(20, 10), |test_context| {
			assert_eq!(test_context.view.get_view_size(), Size::new(20, 10));
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_empty() {
		view_module_test(Size::new(20, 10), |mut test_context| {
			let view_data = ViewData::new();
			test_context.view.render(&view_data).unwrap();
			TestContext::assert_output(&["~"; 10]);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_title_full_width() {
		view_module_test(Size::new(35, 10), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.set_show_title(true);
			test_context.view.render(&view_data).unwrap();
			let mut expected = vec!["Git Interactive Rebase Tool        "];
			expected.extend(vec!["~"; 9]);
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_title_short_title() {
		view_module_test(Size::new(26, 10), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.set_show_title(true);
			test_context.view.render(&view_data).unwrap();
			let mut expected = vec!["Git Rebase                "];
			expected.extend(vec!["~"; 9]);
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_title_full_width_with_help() {
		view_module_test(Size::new(35, 10), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.set_show_title(true);
			view_data.set_show_help(true);
			test_context.view.render(&view_data).unwrap();
			let mut expected = vec!["Git Interactive Rebase Tool Help: ?"];
			expected.extend(vec!["~"; 9]);
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_title_full_width_with_help_enabled_but_not_enough_length() {
		view_module_test(Size::new(34, 10), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.set_show_title(true);
			view_data.set_show_help(true);
			test_context.view.render(&view_data).unwrap();
			let mut expected = vec!["Git Interactive Rebase Tool       "];
			expected.extend(vec!["~"; 9]);
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_prompt() {
		view_module_test(Size::new(35, 10), |mut test_context| {
			let view_data = ViewData::new_confirm("This is a prompt");
			test_context.view.render(&view_data).unwrap();
			let expected = vec!["Git Interactive Rebase Tool        ", "\nThis is a prompt (y/n)? "];
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_leading_lines() {
		view_module_test(Size::new(30, 10), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.push_leading_line(ViewLine::from("This is a leading line"));
			view_data.set_view_size(30, 10);
			test_context.view.render(&view_data).unwrap();
			let mut expected = vec!["This is a leading line        "];
			expected.extend(vec!["~"; 9]);
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_normal_lines() {
		view_module_test(Size::new(30, 10), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.push_line(ViewLine::from("This is a line"));
			view_data.set_view_size(30, 10);
			test_context.view.render(&view_data).unwrap();
			let mut expected = vec!["This is a line                "];
			expected.extend(vec!["~"; 9]);
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_tailing_lines() {
		view_module_test(Size::new(30, 10), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.push_trailing_line(ViewLine::from("This is a trailing line"));
			view_data.set_view_size(30, 10);
			test_context.view.render(&view_data).unwrap();
			let mut expected = vec!["~"; 9];
			expected.push("This is a trailing line       ");
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_all_lines() {
		view_module_test(Size::new(30, 10), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.push_leading_line(ViewLine::from("This is a leading line"));
			view_data.push_line(ViewLine::from("This is a line"));
			view_data.push_trailing_line(ViewLine::from("This is a trailing line"));
			view_data.set_view_size(30, 10);
			test_context.view.render(&view_data).unwrap();
			let mut expected = vec!["This is a leading line        ", "This is a line                "];
			expected.extend(vec!["~"; 7]);
			expected.push("This is a trailing line       ");
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_with_full_screen_data() {
		view_module_test(Size::new(30, 6), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.push_leading_line(ViewLine::from("This is a leading line"));
			view_data.push_line(ViewLine::from("This is line 1"));
			view_data.push_line(ViewLine::from("This is line 2"));
			view_data.push_line(ViewLine::from("This is line 3"));
			view_data.push_line(ViewLine::from("This is line 4"));
			view_data.push_trailing_line(ViewLine::from("This is a trailing line"));
			view_data.set_view_size(30, 6);
			test_context.view.render(&view_data).unwrap();
			let expected = vec![
				"This is a leading line        ",
				"This is line 1                ",
				"This is line 2                ",
				"This is line 3                ",
				"This is line 4                ",
				"This is a trailing line       ",
			];
			TestContext::assert_output(&expected);
		});
	}

	#[test]
	#[serial_test::serial]
	fn render_with_scroll_bar() {
		view_module_test(Size::new(30, 6), |mut test_context| {
			let mut view_data = ViewData::new();
			view_data.push_leading_line(ViewLine::from("This is a leading line"));
			view_data.push_line(ViewLine::from("This is line 1"));
			view_data.push_line(ViewLine::from("This is line 2"));
			view_data.push_line(ViewLine::from("This is line 3"));
			view_data.push_line(ViewLine::from("This is line 4"));
			view_data.push_line(ViewLine::from("This is line 5"));
			view_data.push_trailing_line(ViewLine::from("This is a trailing line"));
			view_data.set_view_size(30, 6);
			test_context.view.render(&view_data).unwrap();
			let expected = vec![
				"This is a leading line        ",
				"This is line 1               █",
				"This is line 2                ",
				"This is line 3                ",
				"This is line 4                ",
				"This is a trailing line       ",
			];
			TestContext::assert_output(&expected);
		});
	}
}
