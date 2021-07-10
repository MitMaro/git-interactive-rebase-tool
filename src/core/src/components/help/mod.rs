#[cfg(test)]
mod tests;

use display::DisplayColor;
use input::{Event, InputOptions};
use lazy_static::lazy_static;
use unicode_segmentation::UnicodeSegmentation;
use view::{handle_view_data_scroll, LineSegment, ViewData, ViewLine, ViewSender};

lazy_static! {
	pub static ref INPUT_OPTIONS: InputOptions = InputOptions::new().movement(true);
}

pub(crate) struct Help {
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

	pub(crate) fn new_from_keybindings(keybindings: &[(Vec<String>, String)]) -> Self {
		let max_key_length = Self::get_max_help_key_length(keybindings);
		let view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_retain_scroll_position(false);
			updater.push_leading_line(
				ViewLine::new_pinned(vec![LineSegment::new_with_color_and_style(
					format!(" {0:width$} Action", "Key", width = max_key_length).as_str(),
					DisplayColor::Normal,
					false,
					true,
					false,
				)])
				.set_padding_with_color_and_style(' ', DisplayColor::Normal, false, true, false),
			);

			for line in keybindings {
				updater.push_line(ViewLine::new_with_pinned_segments(
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

			updater.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
				"Press any key to close",
				DisplayColor::IndicatorColor,
			)]));
		});
		Self {
			active: false,
			view_data,
		}
	}

	pub(crate) fn get_view_data(&mut self) -> &ViewData {
		&self.view_data
	}

	pub(crate) fn handle_event(&mut self, event: Event, view_sender: &ViewSender) {
		if handle_view_data_scroll(event, view_sender).is_none() {
			if let Event::Key(_) = event {
				self.active = false;
			}
		}
	}

	pub(crate) fn set_active(&mut self) {
		self.active = true;
	}

	pub(crate) const fn is_active(&self) -> bool {
		self.active
	}
}
