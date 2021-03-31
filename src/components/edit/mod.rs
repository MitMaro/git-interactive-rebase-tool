#[cfg(test)]
mod tests;

use unicode_segmentation::UnicodeSegmentation;

use crate::{
	display::display_color::DisplayColor,
	input::Input,
	view::{line_segment::LineSegment, view_data::ViewData, view_line::ViewLine},
};

pub struct Edit {
	content: String,
	cursor_position: usize,
	description: Option<String>,
	label: Option<String>,
}

impl Edit {
	pub(crate) fn new() -> Self {
		Self {
			content: String::from(""),
			cursor_position: 0,
			description: None,
			label: None,
		}
	}

	pub fn update_view_data(&mut self, view_data: &mut ViewData) {
		let line = self.content.as_str();
		let pointer = self.cursor_position;

		let graphemes = UnicodeSegmentation::graphemes(line, true);

		let start = graphemes.clone().take(pointer).collect::<String>();
		let indicator = graphemes.clone().skip(pointer).take(1).collect::<String>();
		let end = graphemes.skip(pointer + 1).collect::<String>();

		if let Some(description) = self.description.as_ref() {
			view_data.push_leading_line(ViewLine::from(vec![LineSegment::new_with_color(
				description.as_str(),
				DisplayColor::IndicatorColor,
			)]));
			view_data.push_leading_line(ViewLine::new_empty_line());
		}
		let mut segments = vec![];
		if let Some(label) = self.label.as_ref() {
			segments.push(LineSegment::new_with_color_and_style(
				label.as_str(),
				DisplayColor::Normal,
				true,
				false,
				false,
			));
		}
		segments.push(LineSegment::new(start.as_str()));
		segments.push(LineSegment::new_with_color_and_style(
			indicator.as_str(),
			DisplayColor::Normal,
			false,
			true,
			false,
		));
		segments.push(LineSegment::new(end.as_str()));
		if indicator.is_empty() {
			segments.push(LineSegment::new_with_color_and_style(
				" ",
				DisplayColor::Normal,
				false,
				true,
				false,
			));
		}
		view_data.push_line(ViewLine::from(segments));
		view_data.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
			"Enter to finish",
			DisplayColor::IndicatorColor,
		)]));
		view_data.ensure_column_visible(pointer);
		view_data.ensure_line_visible(0);
	}

	pub fn handle_input(&mut self, input: Input) -> bool {
		match input {
			Input::Character(c) => {
				let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
					.take(self.cursor_position)
					.collect::<String>();
				let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
					.skip(self.cursor_position)
					.collect::<String>();
				self.content = format!("{}{}{}", start, c, end);
				self.cursor_position += 1;
			},
			Input::Backspace => {
				if self.cursor_position != 0 {
					let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.take(self.cursor_position - 1)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.skip(self.cursor_position)
						.collect::<String>();
					self.content = format!("{}{}", start, end);
					self.cursor_position -= 1;
				}
			},
			Input::Delete => {
				let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
				if self.cursor_position != length {
					let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.take(self.cursor_position)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.skip(self.cursor_position + 1)
						.collect::<String>();
					self.content = format!("{}{}", start, end);
				}
			},
			Input::Home => self.cursor_position = 0,
			Input::End => self.cursor_position = UnicodeSegmentation::graphemes(self.content.as_str(), true).count(),
			Input::Right => {
				let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
				if self.cursor_position < length {
					self.cursor_position += 1;
				}
			},
			Input::Left => {
				if self.cursor_position != 0 {
					self.cursor_position -= 1;
				}
			},
			_ => return false,
		}
		true
	}

	pub fn set_description(&mut self, description: &str) {
		self.description = Some(String::from(description));
	}

	pub fn set_label(&mut self, label: &str) {
		self.label = Some(String::from(label));
	}

	pub fn set_content(&mut self, content: &str) {
		self.content = String::from(content);
		self.cursor_position = UnicodeSegmentation::graphemes(content, true).count();
	}

	pub fn clear(&mut self) {
		self.content.clear();
		self.cursor_position = 0;
	}

	pub fn get_content(&self) -> String {
		self.content.clone()
	}
}
