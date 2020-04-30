mod utils;

use crate::config::Config;
use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::help::utils::{
	get_list_normal_mode_help_lines,
	get_list_visual_mode_help_lines,
	get_max_help_description_length,
	get_max_help_key_length,
};
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::scroll::scroll_position::ScrollPosition;
use crate::view::line_segment::LineSegment;
use crate::view::view_line::ViewLine;
use crate::view::View;

pub(crate) struct Help<'h> {
	normal_mode_help_lines: [(&'h str, &'h str); 22],
	normal_mode_max_help_line_desc_length: usize,
	normal_mode_max_help_line_key_length: usize,
	return_state: State,
	scroll_position: ScrollPosition,
	visual_mode_help_lines: [(&'h str, &'h str); 14],
	visual_mode_max_help_line_desc_length: usize,
	visual_mode_max_help_line_key_length: usize,
}

impl<'h> ProcessModule for Help<'h> {
	fn activate(&mut self, state: State, _git_interactive: &GitInteractive) {
		self.scroll_position.reset();
		if let State::Help(return_state) = state {
			self.return_state = *return_state;
		}
		else {
			panic!("Help module activated when not expected");
		}
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let (view_width, view_height) = view.get_view_size();
		let input = input_handler.get_input(InputMode::Default);
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorLeft => {
				self.scroll_position
					.scroll_left(view_width, self.get_max_help_line_length())
			},
			Input::MoveCursorRight => {
				self.scroll_position
					.scroll_right(view_width, self.get_max_help_line_length())
			},
			Input::MoveCursorDown => {
				self.scroll_position
					.scroll_down(view_height, self.get_help_lines().len());
			},
			Input::MoveCursorUp => {
				self.scroll_position.scroll_up(view_height, self.get_help_lines().len());
			},
			Input::MoveCursorPageDown => {
				self.scroll_position.page_down(view_height, self.get_help_lines().len());
			},
			Input::MoveCursorPageUp => {
				self.scroll_position.page_up(view_height, self.get_help_lines().len());
			},
			Input::Resize => {
				self.scroll_position.reset();
			},
			_ => {
				result = result.state(self.return_state.clone());
			},
		}
		result.build()
	}

	fn render(&self, view: &View, _git_interactive: &GitInteractive) {
		let (view_width, view_height) = view.get_view_size();

		let mut view_lines: Vec<ViewLine> = vec![];

		for line in self.get_help_lines() {
			view_lines.push(ViewLine::new_with_pinned_segments(
				vec![
					LineSegment::new_with_color(
						format!(" {0:width$} ", line.0, width = self.get_max_help_key_length()).as_str(),
						DisplayColor::IndicatorColor,
					),
					LineSegment::new(line.1),
				],
				1,
			));
		}

		view.draw_title(false);

		view.set_color(DisplayColor::Normal, false);
		view.set_style(false, true, false);
		view.draw_str(" Key   Action");
		if view_width > 13 {
			let padding = " ".repeat(view_width - 13);
			view.draw_str(padding.as_str());
		}

		view.draw_view_lines(
			&view_lines,
			self.scroll_position.get_top_position(),
			self.scroll_position.get_left_position(),
			view_height - 3,
		);

		view.set_color(DisplayColor::IndicatorColor, false);
		view.draw_str("Any key to close");
	}
}

impl<'h> Help<'h> {
	pub(crate) fn new(config: &'h Config) -> Self {
		let normal_mode_help_lines = get_list_normal_mode_help_lines(config);
		let normal_mode_max_help_line_desc_length = get_max_help_description_length(&normal_mode_help_lines);
		let normal_mode_max_help_line_key_length = get_max_help_key_length(&normal_mode_help_lines);
		let visual_mode_help_lines = get_list_visual_mode_help_lines(config);
		let visual_mode_max_help_line_desc_length = get_max_help_description_length(&visual_mode_help_lines);
		let visual_mode_max_help_line_key_length = get_max_help_key_length(&visual_mode_help_lines);
		Self {
			normal_mode_help_lines,
			normal_mode_max_help_line_desc_length,
			normal_mode_max_help_line_key_length,
			return_state: State::List(false),
			scroll_position: ScrollPosition::new(3),
			visual_mode_help_lines,
			visual_mode_max_help_line_desc_length,
			visual_mode_max_help_line_key_length,
		}
	}

	fn get_help_lines(&self) -> &[(&str, &str)] {
		if let State::List(visual_mode) = self.return_state {
			if visual_mode {
				&self.visual_mode_help_lines
			}
			else {
				&self.normal_mode_help_lines
			}
		}
		else {
			&[]
		}
	}

	fn get_max_help_key_length(&self) -> usize {
		if let State::List(visual_mode) = self.return_state {
			if visual_mode {
				self.visual_mode_max_help_line_key_length
			}
			else {
				self.normal_mode_max_help_line_key_length
			}
		}
		else {
			4
		}
	}

	fn get_max_help_line_length(&self) -> usize {
		if let State::List(visual_mode) = self.return_state {
			if visual_mode {
				self.visual_mode_max_help_line_desc_length + self.visual_mode_max_help_line_key_length + 1
			}
			else {
				self.normal_mode_max_help_line_desc_length + self.normal_mode_max_help_line_key_length + 1
			}
		}
		else {
			4
		}
	}
}
