pub mod line_segment;
pub mod scroll_position;
#[cfg(test)]
pub mod testutil;
pub mod view_data;
pub mod view_line;

use crate::constants::{TITLE, TITLE_HELP_INDICATOR_LENGTH, TITLE_LENGTH, TITLE_SHORT, TITLE_SHORT_LENGTH};
use crate::display::display_color::DisplayColor;
use crate::display::{Display, Size};
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::Config;

pub struct View<'v> {
	config: &'v Config,
	display: &'v Display<'v>,
}

impl<'v> View<'v> {
	pub(crate) const fn new(display: &'v Display<'_>, config: &'v Config) -> Self {
		Self { display, config }
	}

	pub(crate) fn get_view_size(&self) -> Size {
		self.display.get_window_size()
	}

	pub(crate) fn render(&self, view_data: &ViewData) {
		self.display.clear();
		let window_height = self.display.get_window_size().height();

		let mut line_index = 0;

		if view_data.show_title() {
			self.display.ensure_at_line_start(line_index);
			line_index += 1;
			self.draw_title(view_data.show_help());
		}

		if let Some(ref prompt) = *view_data.get_prompt() {
			self.display.set_style(false, false, false);
			self.display.draw_str("\n");
			self.display.draw_str(&format!(
				"{} ({}/{})?",
				prompt, self.config.key_bindings.confirm_yes, self.config.key_bindings.confirm_no
			));
			self.display.draw_str(" ");
			return;
		}

		let leading_lines = view_data.get_leading_lines();
		let lines = view_data.get_lines();
		let trailing_lines = view_data.get_trailing_lines();

		let view_height = window_height - leading_lines.len() - trailing_lines.len();

		let show_scroll_bar = view_data.should_show_scroll_bar();
		let scroll_indicator_index = view_data.get_scroll_index();

		for line in leading_lines {
			self.display.ensure_at_line_start(line_index);
			line_index += 1;
			self.draw_view_line(line);
		}

		for (index, line) in lines.iter().enumerate() {
			self.display.ensure_at_line_start(line_index);
			self.draw_view_line(line);
			if show_scroll_bar {
				self.display.ensure_at_line_start(line_index);
				self.display.move_from_end_of_line(1);
				self.display.color(DisplayColor::Normal, false);
				self.display.set_style(scroll_indicator_index != index, false, true);
				self.display.draw_str(" ");
			}
			self.display.color(DisplayColor::Normal, false);
			self.display.set_style(false, false, false);
			line_index += 1;
		}

		if view_height > lines.len() {
			self.display.color(DisplayColor::Normal, false);
			self.display.set_style(false, false, false);
			let draw_height = view_height - lines.len() - if view_data.show_title() { 1 } else { 0 };
			self.display.ensure_at_line_start(line_index);
			for _x in 0..draw_height {
				line_index += 1;
				self.display
					.draw_str(format!("{}\n", self.config.theme.character_vertical_spacing).as_str());
			}
		}

		for line in trailing_lines {
			self.display.ensure_at_line_start(line_index);
			line_index += 1;
			self.draw_view_line(line);
		}
		self.display.refresh();
	}

	fn draw_view_line(&self, line: &ViewLine) {
		for segment in line.get_segments() {
			self.display.color(segment.get_color(), line.get_selected());
			self.display
				.set_style(segment.is_dimmed(), segment.is_underlined(), segment.is_reversed());
			self.display.draw_str(segment.get_content());
		}

		// reset style
		self.display.color(DisplayColor::Normal, false);
		self.display.set_style(false, false, false);
		self.display.fill_end_of_line();
	}

	fn draw_title(&self, show_help: bool) {
		self.display.color(DisplayColor::Normal, false);
		self.display.set_style(false, true, false);
		let window_width = self.display.get_window_size().width();

		let title_help_indicator_total_length = TITLE_HELP_INDICATOR_LENGTH + self.config.key_bindings.help.len();

		if window_width >= TITLE_LENGTH {
			self.display.draw_str(TITLE);
			// only draw help if there is room
			if window_width > TITLE_LENGTH + title_help_indicator_total_length {
				if (window_width - TITLE_LENGTH - title_help_indicator_total_length) > 0 {
					let padding = " ".repeat(window_width - TITLE_LENGTH - title_help_indicator_total_length);
					self.display.draw_str(padding.as_str());
				}
				if show_help {
					self.display
						.draw_str(format!("Help: {}", self.config.key_bindings.help).as_str());
				}
				else {
					let padding = " ".repeat(title_help_indicator_total_length);
					self.display.draw_str(padding.as_str());
				}
			}
			else if (window_width - TITLE_LENGTH) > 0 {
				let padding = " ".repeat(window_width - TITLE_LENGTH);
				self.display.draw_str(padding.as_str());
			}
		}
		else {
			self.display.draw_str(TITLE_SHORT);
			if (window_width - TITLE_SHORT_LENGTH) > 0 {
				let padding = " ".repeat(window_width - TITLE_SHORT_LENGTH);
				self.display.draw_str(padding.as_str());
			}
		}

		// reset style
		self.display.color(DisplayColor::Normal, false);
		self.display.set_style(false, false, false);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::config::Config;
	use crate::display::curses::Curses;
	use std::env::set_var;
	use std::path::Path;

	pub struct TestContext {
		pub config: Config,
		pub curses: Curses,
	}

	pub fn view_module_test<F>(callback: F)
	where F: FnOnce(TestContext) {
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
		let curses = Curses::new();
		curses.resize_term(10, 20);
		callback(TestContext { config, curses });
	}

	fn assert_output(curses: &Curses, expected: &[&str]) {
		assert_eq!(curses.get_output().join(""), expected.join(""));
	}

	#[test]
	fn get_view_size() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 20);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			assert_eq!(view.get_view_size(), Size::new(20, 10));
		});
	}

	#[test]
	fn render_empty() {
		view_module_test(|mut test_context| {
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let view_data = ViewData::new();
			view.render(&view_data);
			assert_output(&test_context.curses, &["~\n"; 10]);
		});
	}

	#[test]
	fn render_title_full_width() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 35);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.set_show_title(true);
			view.render(&view_data);
			let mut expected = vec!["Git Interactive Rebase Tool        "];
			expected.extend(vec!["~\n"; 9]);
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_title_short_title() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 26);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.set_show_title(true);
			view.render(&view_data);
			let mut expected = vec!["Git Rebase                "];
			expected.extend(vec!["~\n"; 9]);
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_title_full_width_with_help() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 35);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.set_show_title(true);
			view_data.set_show_help(true);
			view.render(&view_data);
			let mut expected = vec!["Git Interactive Rebase Tool Help: ?"];
			expected.extend(vec!["~\n"; 9]);
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_title_full_width_with_help_enabled_but_not_enough_length() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 34);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.set_show_title(true);
			view_data.set_show_help(true);
			view.render(&view_data);
			let mut expected = vec!["Git Interactive Rebase Tool       "];
			expected.extend(vec!["~\n"; 9]);
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_prompt() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 35);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let view_data = ViewData::new_confirm("This is a prompt");
			view.render(&view_data);
			let expected = vec!["Git Interactive Rebase Tool        ", "\nThis is a prompt (y/n)? "];
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_leading_lines() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 30);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.push_leading_line(ViewLine::from("This is a leading line"));
			view_data.set_view_size(30, 10);
			view.render(&view_data);
			let mut expected = vec!["This is a leading line        {HLINE| |30}"];
			expected.extend(vec!["~\n"; 9]);
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_normal_lines() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 30);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.push_line(ViewLine::from("This is a line"));
			view_data.set_view_size(30, 10);
			view.render(&view_data);
			let mut expected = vec!["This is a line                {HLINE| |30}"];
			expected.extend(vec!["~\n"; 9]);
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_tailing_lines() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 30);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.push_trailing_line(ViewLine::from("This is a trailing line"));
			view_data.set_view_size(30, 10);
			view.render(&view_data);
			let mut expected = vec!["~\n"; 9];
			expected.push("This is a trailing line       {HLINE| |30}");
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_all_lines() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(10, 30);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.push_leading_line(ViewLine::from("This is a leading line"));
			view_data.push_line(ViewLine::from("This is a line"));
			view_data.push_trailing_line(ViewLine::from("This is a trailing line"));
			view_data.set_view_size(30, 10);
			view.render(&view_data);
			let mut expected = vec![
				"This is a leading line        {HLINE| |30}",
				"This is a line                {HLINE| |30}",
			];
			expected.extend(vec!["~\n"; 7]);
			expected.push("This is a trailing line       {HLINE| |30}");
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_with_full_screen_data() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(6, 30);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.push_leading_line(ViewLine::from("This is a leading line"));
			view_data.push_line(ViewLine::from("This is line 1"));
			view_data.push_line(ViewLine::from("This is line 2"));
			view_data.push_line(ViewLine::from("This is line 3"));
			view_data.push_line(ViewLine::from("This is line 4"));
			view_data.push_trailing_line(ViewLine::from("This is a trailing line"));
			view_data.set_view_size(30, 6);
			view.render(&view_data);
			let expected = vec![
				"This is a leading line        {HLINE| |30}",
				"This is line 1                {HLINE| |30}",
				"This is line 2                {HLINE| |30}",
				"This is line 3                {HLINE| |30}",
				"This is line 4                {HLINE| |30}",
				"This is a trailing line       {HLINE| |30}",
			];
			assert_output(&test_context.curses, &expected);
		});
	}

	#[test]
	fn render_with_scroll_bar() {
		view_module_test(|mut test_context| {
			test_context.curses.resize_term(6, 30);
			let display = Display::new(&mut test_context.curses, &test_context.config.theme);
			let view = View::new(&display, &test_context.config);
			let mut view_data = ViewData::new();
			view_data.push_leading_line(ViewLine::from("This is a leading line"));
			view_data.push_line(ViewLine::from("This is line 1"));
			view_data.push_line(ViewLine::from("This is line 2"));
			view_data.push_line(ViewLine::from("This is line 3"));
			view_data.push_line(ViewLine::from("This is line 4"));
			view_data.push_line(ViewLine::from("This is line 5"));
			view_data.push_trailing_line(ViewLine::from("This is a trailing line"));
			view_data.set_view_size(30, 6);
			view.render(&view_data);
			let expected = vec![
				"This is a leading line        {HLINE| |30}",
				"This is line 1               {HLINE| |30} ",
				"This is line 2               {HLINE| |30} ",
				"This is line 3               {HLINE| |30} ",
				"This is line 4               {HLINE| |30} ",
				"This is a trailing line       {HLINE| |30}",
			];
			assert_output(&test_context.curses, &expected);
		});
	}
}
