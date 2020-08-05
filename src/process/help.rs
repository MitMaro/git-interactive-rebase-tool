use crate::display::display_color::DisplayColor;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::HandleInputResult;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use unicode_segmentation::UnicodeSegmentation;

pub struct Help {
	view_data: ViewData,
}

fn get_max_help_key_length(lines: &[(&str, &str)]) -> usize {
	let mut max_length = 0;
	for (key, _) in lines {
		let len = UnicodeSegmentation::graphemes(*key, true).count();
		if len > max_length {
			max_length = len;
		}
	}
	max_length
}

impl Help {
	pub fn new_from_keybindings_descriptions(keybindings: &[(&str, &str)]) -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);

		let max_key_length = get_max_help_key_length(keybindings);

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
						format!(" {0:width$}", line.0, width = max_key_length).as_str(),
						DisplayColor::IndicatorColor,
					),
					LineSegment::new_with_color_and_style("|", DisplayColor::Normal, true, false, false),
					LineSegment::new(line.1),
				],
				2,
			));
		}

		view_data.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
			"Any key to close",
			DisplayColor::IndicatorColor,
		)]));

		Self { view_data }
	}

	pub fn new_from_view_data(keybindings: Option<&[(&str, &str)]>, view_data: Option<ViewData>) -> Self {
		if let Some(k) = keybindings {
			Self::new_from_keybindings_descriptions(k)
		}
		else if let Some(view_data) = view_data {
			Self { view_data }
		}
		else {
			let mut view_data = ViewData::new();
			view_data.set_content(ViewLine::new(vec![LineSegment::new("Help not available")]));
			Self { view_data }
		}
	}

	pub fn get_view_data(&mut self, view: &View<'_>) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	pub fn handle_input(&mut self, input_handler: &InputHandler<'_>, view: &View<'_>) -> HandleInputResult {
		let input = input_handler.get_input(InputMode::Default);
		match input {
			Input::MoveCursorLeft => self.view_data.scroll_left(),
			Input::MoveCursorRight => self.view_data.scroll_right(),
			Input::MoveCursorDown => self.view_data.scroll_down(),
			Input::MoveCursorUp => self.view_data.scroll_up(),
			Input::MoveCursorPageDown => self.view_data.page_down(),
			Input::MoveCursorPageUp => self.view_data.page_up(),
			Input::Resize => {
				let (view_width, view_height) = view.get_view_size();
				self.view_data.set_view_size(view_width, view_height);
			},
			_ => return HandleInputResult::new(Input::Help),
		}
		HandleInputResult::new(input)
	}
}
