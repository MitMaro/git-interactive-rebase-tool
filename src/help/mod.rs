mod utils;

use crate::config::key_bindings::KeyBindings;
use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::help::utils::{get_list_normal_mode_help_lines, get_list_visual_mode_help_lines, get_max_help_key_length};
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;

pub(crate) struct Help<'h> {
	normal_mode_help_lines: [(&'h str, &'h str); 22],
	return_state: State,
	visual_mode_help_lines: [(&'h str, &'h str); 14],
	view_data: Option<ViewData>,
}

impl<'h> ProcessModule for Help<'h> {
	fn activate(&mut self, state: State, _git_interactive: &GitInteractive) {
		if let State::Help(return_state) = state {
			self.return_state = *return_state;
		}
		else {
			panic!("Help module activated when not expected");
		}
	}

	fn deactivate(&mut self) {
		self.view_data = None;
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let input = input_handler.get_input(InputMode::Default);
		let mut result = HandleInputResultBuilder::new(input);
		let view_data = self.view_data.as_mut().unwrap();
		match input {
			Input::MoveCursorLeft => view_data.scroll_left(),
			Input::MoveCursorRight => view_data.scroll_right(),
			Input::MoveCursorDown => view_data.scroll_down(),
			Input::MoveCursorUp => view_data.scroll_up(),
			Input::MoveCursorPageDown => view_data.page_down(),
			Input::MoveCursorPageUp => view_data.page_up(),
			Input::Resize => {
				let (view_width, view_height) = view.get_view_size();
				view_data.set_view_size(view_width, view_height);
			},
			_ => {
				result = result.state(self.return_state.clone());
			},
		}
		result.build()
	}

	fn render(&self, _: &View, _: &GitInteractive) {}
}

impl<'h> Help<'h> {
	pub(crate) fn new(key_bindings: &'h KeyBindings) -> Self {
		Self {
			normal_mode_help_lines: get_list_normal_mode_help_lines(key_bindings),
			return_state: State::List(false),
			visual_mode_help_lines: get_list_visual_mode_help_lines(key_bindings),
			view_data: None,
		}
	}

	pub(crate) fn build_view_data(&mut self, view: &View, _: &GitInteractive) -> &ViewData {
		match self.view_data {
			Some(ref v) => v,
			None => {
				let (view_width, view_height) = view.get_view_size();
				let mut view_data = ViewData::new();
				view_data.set_view_size(view_width, view_height);
				view_data.set_show_title(true);

				let lines: &[(&str, &str)] = if let State::List(visual_mode) = self.return_state {
					if visual_mode {
						&self.visual_mode_help_lines
					}
					else {
						&self.normal_mode_help_lines
					}
				}
				else {
					&[]
				};

				let max_key_length = get_max_help_key_length(lines);

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

				for line in lines {
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
				view_data.rebuild();

				self.view_data = Some(view_data);
				self.view_data.as_ref().unwrap()
			},
		}
	}
}
