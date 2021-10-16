#[cfg(test)]
mod tests;

use display::DisplayColor;
use input::{Event, EventHandler, InputOptions, KeyCode, KeyEvent, KeyModifiers};
use lazy_static::lazy_static;
use unicode_segmentation::UnicodeSegmentation;
use view::{LineSegment, ViewData, ViewLine};

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::new();
}

pub(crate) struct Edit {
	content: String,
	cursor_position: usize,
	description: Option<String>,
	finished: bool,
	label: Option<String>,
	view_data: ViewData,
}

impl Edit {
	pub(crate) fn new() -> Self {
		let view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
		});
		Self {
			content: String::from(""),
			cursor_position: 0,
			description: None,
			finished: false,
			label: None,
			view_data,
		}
	}

	pub(crate) fn get_view_data(&mut self) -> &ViewData {
		let line = self.content.as_str();
		let pointer = self.cursor_position;

		let graphemes = UnicodeSegmentation::graphemes(line, true);

		let start = graphemes.clone().take(pointer).collect::<String>();
		let indicator = graphemes.clone().skip(pointer).take(1).collect::<String>();
		let end = graphemes.skip(pointer + 1).collect::<String>();

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
		if !start.is_empty() {
			segments.push(LineSegment::new(start.as_str()));
		}
		segments.push(
			if indicator.is_empty() {
				LineSegment::new_with_color_and_style(" ", DisplayColor::Normal, false, true, false)
			}
			else {
				LineSegment::new_with_color_and_style(indicator.as_str(), DisplayColor::Normal, false, true, false)
			},
		);
		if !end.is_empty() {
			segments.push(LineSegment::new(end.as_str()));
		}
		let description = self.description.as_ref();
		self.view_data.update_view_data(|updater| {
			updater.clear();
			if let Some(description) = description {
				updater.push_leading_line(ViewLine::from(vec![LineSegment::new_with_color(
					description.as_str(),
					DisplayColor::IndicatorColor,
				)]));
				updater.push_leading_line(ViewLine::new_empty_line());
			}
			updater.push_line(ViewLine::from(segments));
			updater.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
				"Enter to finish",
				DisplayColor::IndicatorColor,
			)]));
			updater.ensure_column_visible(pointer);
			updater.ensure_line_visible(0);
		});
		&self.view_data
	}

	pub(crate) fn handle_event(&mut self, event_handler: &EventHandler) -> Event {
		let event = event_handler.read_event(&INPUT_OPTIONS, |event, _| event);

		match event {
			Event::Key(KeyEvent {
				code: KeyCode::Backspace,
				modifiers: KeyModifiers::NONE,
			}) => {
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
			Event::Key(KeyEvent {
				code: KeyCode::Delete,
				modifiers: KeyModifiers::NONE,
			}) => {
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
			Event::Key(KeyEvent {
				code: KeyCode::Home,
				modifiers: KeyModifiers::NONE,
			}) => self.cursor_position = 0,
			Event::Key(KeyEvent {
				code: KeyCode::End,
				modifiers: KeyModifiers::NONE,
			}) => self.cursor_position = UnicodeSegmentation::graphemes(self.content.as_str(), true).count(),
			Event::Key(KeyEvent {
				code: KeyCode::Right,
				modifiers: KeyModifiers::NONE,
			}) => {
				let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
				if self.cursor_position < length {
					self.cursor_position += 1;
				}
			},
			Event::Key(KeyEvent {
				code: KeyCode::Left,
				modifiers: KeyModifiers::NONE,
			}) => {
				if self.cursor_position != 0 {
					self.cursor_position -= 1;
				}
			},
			Event::Key(KeyEvent {
				code: KeyCode::Enter,
				modifiers: KeyModifiers::NONE,
			}) => self.finished = true,
			Event::Key(KeyEvent {
				code: KeyCode::Char(c),
				modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
			}) => {
				let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
					.take(self.cursor_position)
					.collect::<String>();
				let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
					.skip(self.cursor_position)
					.collect::<String>();
				self.content = format!("{}{}{}", start, c, end);
				self.cursor_position += 1;
			},
			_ => {},
		}

		event
	}

	pub(crate) fn set_description(&mut self, description: &str) {
		self.description = Some(String::from(description));
	}

	pub(crate) fn set_label(&mut self, label: &str) {
		self.label = Some(String::from(label));
	}

	pub(crate) fn set_content(&mut self, content: &str) {
		self.content = String::from(content);
		self.cursor_position = UnicodeSegmentation::graphemes(content, true).count();
	}

	pub(crate) fn clear(&mut self) {
		self.content.clear();
		self.cursor_position = 0;
		self.finished = false;
	}

	pub(crate) const fn is_finished(&self) -> bool {
		self.finished
	}

	pub(crate) fn get_content(&self) -> String {
		self.content.clone()
	}
}
