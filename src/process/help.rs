use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::process::util::handle_view_data_scroll;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use anyhow::Result;
use unicode_segmentation::UnicodeSegmentation;

fn get_max_help_key_length(lines: &[(String, String)]) -> usize {
	let mut max_length = 0;
	for &(ref key, _) in lines {
		let len = UnicodeSegmentation::graphemes(key.as_str(), true).count();
		if len > max_length {
			max_length = len;
		}
	}
	max_length
}

pub struct Help {
	return_state: Option<State>,
	no_help_view_data: ViewData,
	view_data: Option<ViewData>,
}

impl ProcessModule for Help {
	fn activate(&mut self, _: &GitInteractive, return_state: State) -> Result<()> {
		if self.return_state.is_none() {
			self.return_state = Some(return_state);
		}
		Ok(())
	}

	fn deactivate(&mut self) {
		self.return_state = None;
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();
		let view_data = self.view_data.as_mut().unwrap_or(&mut self.no_help_view_data);
		view_data.set_view_size(view_width, view_height);
		view_data.rebuild();
		view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_: &mut GitInteractive,
		_: &View<'_>,
	) -> ProcessResult
	{
		let input = input_handler.get_input(InputMode::Default);
		let mut result = ProcessResult::new().input(input);
		let mut view_data = self.view_data.as_mut().unwrap_or(&mut self.no_help_view_data);
		if handle_view_data_scroll(input, &mut view_data).is_none() && input != Input::Resize {
			result = result.state(self.return_state.unwrap_or(State::List));
		}
		result
	}
}

impl Help {
	pub fn new() -> Self {
		let mut no_help_view_data = ViewData::new();
		no_help_view_data.set_content(ViewLine::new(vec![LineSegment::new("Help not available")]));

		Self {
			return_state: None,
			view_data: None,
			no_help_view_data,
		}
	}

	pub fn clear_help(&mut self) {
		self.return_state = None;
		self.view_data = None;
	}

	pub fn update_from_keybindings_descriptions(&mut self, keybindings: &[(String, String)]) {
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
					LineSegment::new(line.1.as_str()),
				],
				2,
			));
		}

		view_data.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color(
			"Any key to close",
			DisplayColor::IndicatorColor,
		)]));

		self.view_data = Some(view_data);
	}

	pub fn update_from_view_data(&mut self, view_data: ViewData) {
		self.view_data = Some(view_data);
	}
}
