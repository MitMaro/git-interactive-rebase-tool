#[cfg(test)]
mod tests;
use std::collections::HashMap;

use crate::{
	display::display_color::DisplayColor,
	input::Input,
	process::util::handle_view_data_scroll,
	view::{line_segment::LineSegment, view_data::ViewData, view_line::ViewLine},
};

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

	pub fn get_view_data(&mut self, view_width: usize, view_height: usize) -> &ViewData {
		self.view_data.set_view_size(view_width, view_height);
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
		self.view_data.rebuild();
		&self.view_data
	}

	pub fn handle_input(&mut self, input: Input) -> Option<&T> {
		if handle_view_data_scroll(input, &mut self.view_data).is_none() {
			match input {
				Input::Resize => {},
				Input::Character(c) => {
					if let Some(v) = self.map.get(&c) {
						self.invalid_selection = false;
						return Some(v);
					}
					self.invalid_selection = true;
				},
				_ => {
					self.invalid_selection = true;
				},
			}
		}
		None
	}
}
