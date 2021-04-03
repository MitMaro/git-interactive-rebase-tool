#[cfg(test)]
mod tests;
use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
	display::display_color::DisplayColor,
	input::{Event, EventHandler, InputOptions, KeyCode},
	view::{handle_view_data_scroll, line_segment::LineSegment, view_data::ViewData, view_line::ViewLine},
};

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::new().movement(true);
}

pub struct Choice<T> {
	map: HashMap<char, T>,
	view_data: ViewData,
	options: Vec<(T, char, String)>,
	invalid_selection: bool,
}

impl<T> Choice<T>
where T: Clone
{
	pub fn new(options: Vec<(T, char, String)>) -> Self {
		let map = options
			.iter()
			.map(|&(ref v, ref k, _)| {
				let c = *k;
				let t = v.clone();
				(c, t)
			})
			.collect::<HashMap<char, T>>();
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		Self {
			map,
			options,
			view_data,
			invalid_selection: false,
		}
	}

	pub fn set_prompt(&mut self, prompt_lines: Vec<ViewLine>) {
		self.view_data.clear();
		for line in prompt_lines {
			self.view_data.push_leading_line(line);
		}
		self.view_data.push_leading_line(ViewLine::new_empty_line());
	}

	pub fn get_view_data(&mut self) -> &mut ViewData {
		self.view_data.clear_body();
		for &(_, ref key, ref description) in &self.options {
			self.view_data
				.push_line(ViewLine::from(format!("{}) {}", key, description)));
		}
		self.view_data.push_line(ViewLine::new_empty_line());
		if self.invalid_selection {
			self.view_data.push_line(ViewLine::from(LineSegment::new_with_color(
				"Invalid option selected. Please choose an option.",
				DisplayColor::IndicatorColor,
			)));
		}
		else {
			self.view_data.push_line(ViewLine::from(LineSegment::new_with_color(
				"Please choose an option.",
				DisplayColor::IndicatorColor,
			)));
		}
		&mut self.view_data
	}

	pub fn handle_event(&mut self, event_handler: &EventHandler) -> (Option<&T>, Event) {
		let event = event_handler.read_event(&INPUT_OPTIONS, |event, _| event);

		if handle_view_data_scroll(event, &mut self.view_data).is_none() {
			if let Event::Key(key_event) = event {
				if let KeyCode::Char(c) = key_event.code {
					if let Some(v) = self.map.get(&c) {
						self.invalid_selection = false;
						return (Some(v), event);
					}
				}
				self.invalid_selection = true;
			}
		}
		(None, event)
	}
}
