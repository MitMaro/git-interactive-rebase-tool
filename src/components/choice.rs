#[cfg(test)]
mod tests;

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
	display::DisplayColor,
	events::Event,
	input::{InputOptions, KeyCode},
	util::handle_view_data_scroll,
	view::{LineSegment, ViewData, ViewLine},
};

lazy_static! {
	pub(crate) static ref INPUT_OPTIONS: InputOptions = InputOptions::RESIZE | InputOptions::MOVEMENT;
}

pub(crate) struct Choice<T> {
	map: HashMap<char, T>,
	view_data: ViewData,
	options: Vec<(T, char, String)>,
	invalid_selection: bool,
}

impl<T> Choice<T>
where T: Clone
{
	#[allow(clippy::pattern_type_mismatch)]
	pub(crate) fn new(options: Vec<(T, char, String)>) -> Self {
		let map = options
			.iter()
			.map(|(v, k, _)| {
				let c = *k;
				let t = v.clone();
				(c, t)
			})
			.collect::<HashMap<char, T>>();
		Self {
			map,
			options,
			view_data: ViewData::new(|updater| {
				updater.set_show_title(true);
				updater.set_retain_scroll_position(false);
			}),
			invalid_selection: false,
		}
	}

	pub(crate) fn set_prompt(&mut self, prompt_lines: Vec<ViewLine>) {
		self.view_data.update_view_data(|updater| {
			updater.clear();
			for line in prompt_lines {
				updater.push_leading_line(line);
			}
			updater.push_leading_line(ViewLine::new_empty_line());
		});
	}

	#[allow(clippy::pattern_type_mismatch)]
	pub(crate) fn get_view_data(&mut self) -> &ViewData {
		let options = &self.options;
		let invalid_selection = self.invalid_selection;
		self.view_data.update_view_data(|updater| {
			updater.clear_body();
			for (_, key, description) in options {
				updater.push_line(ViewLine::from(format!("{key}) {description}")));
			}
			updater.push_line(ViewLine::new_empty_line());
			if invalid_selection {
				updater.push_line(ViewLine::from(LineSegment::new_with_color(
					"Invalid option selected. Please choose an option.",
					DisplayColor::IndicatorColor,
				)));
			}
			else {
				updater.push_line(ViewLine::from(LineSegment::new_with_color(
					"Please choose an option.",
					DisplayColor::IndicatorColor,
				)));
			}
		});
		&self.view_data
	}

	pub(crate) fn handle_event(&mut self, event: Event, view_state: &crate::view::State) -> Option<&T> {
		if handle_view_data_scroll(event, view_state).is_none() {
			if let Event::Key(key_event) = event {
				if let KeyCode::Char(c) = key_event.code {
					if let Some(v) = self.map.get(&c) {
						self.invalid_selection = false;
						return Some(v);
					}
				}
				self.invalid_selection = true;
			}
		}
		None
	}
}
