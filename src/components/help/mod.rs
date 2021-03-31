#[cfg(test)]
mod tests;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
	display::display_color::DisplayColor,
	input::Input,
	process::util::handle_view_data_scroll,
	view::{line_segment::LineSegment, view_data::ViewData, view_line::ViewLine},
};

pub struct Help {
	active: bool,
	view_data: ViewData,
}

impl Help {
	fn get_max_help_key_length(lines: &[(Vec<String>, String)]) -> usize {
		let mut max_length = 0;
		for &(ref key, _) in lines {
			let combined_key = key.join(", ");
			let len = UnicodeSegmentation::graphemes(combined_key.as_str(), true).count();
			if len > max_length {
				max_length = len;
			}
		}
		max_length
	}

	pub fn new_from_keybindings(keybindings: &[(Vec<String>, String)]) -> Self {
		let max_key_length = Self::get_max_help_key_length(keybindings);
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		view_data.push_leading_line(
			ViewLine::new_pinned(vec![LineSegment::new_with_color_and_style(
				format!(" {0:width$} Action", "Key", width = max_key_length).as_str(),
				DisplayColor::Normal,
				false,
				true,
				false,
			)])
			.set_padding_color_and_style(DisplayColor::Normal, false, true, false),
		);

		for line in keybindings {
			view_data.push_line(ViewLine::new_with_pinned_segments(
				vec![
					LineSegment::new_with_color(
						format!(" {0:width$}", line.0.join(", "), width = max_key_length).as_str(),
						DisplayColor::IndicatorColor,
					),
					LineSegment::new_with_color_and_style("|", DisplayColor::Normal, true, false, false),
					LineSegment::new(line.1.as_str()),
				],
				2,
			));
		}

		view_data.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
			"Press any key to close",
			DisplayColor::IndicatorColor,
		)]));
		Self {
			active: false,
			view_data,
		}
	}

	pub fn get_view_data(&mut self) -> &mut ViewData {
		&mut self.view_data
	}

	pub fn handle_input(&mut self, input: Input) {
		if input != Input::Resize && handle_view_data_scroll(input, &mut self.view_data).is_none() {
			self.active = false;
		}
	}

	pub fn set_active(&mut self) {
		self.active = true;
	}

	pub const fn is_active(&self) -> bool {
		self.active
	}
}
