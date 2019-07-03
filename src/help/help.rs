use crate::constants::{LIST_HELP_LINES, VISUAL_MODE_HELP_LINES};
use crate::git_interactive::GitInteractive;
use crate::input::{Input, InputHandler};
use crate::process::{HandleInputResult, HandleInputResultBuilder, ProcessModule, State};
use crate::scroll::ScrollPosition;
use crate::view::{LineSegment, View, ViewLine};
use crate::window::WindowColor;

fn get_help_lines(return_state: &State) -> &[(&str, &str)] {
	if let State::List(visual_mode) = *return_state {
		if visual_mode {
			VISUAL_MODE_HELP_LINES
		}
		else {
			LIST_HELP_LINES
		}
	}
	else {
		&[]
	}
}

pub struct Help {
	scroll_position: ScrollPosition,
	return_state: State,
}

impl ProcessModule for Help {
	fn activate(&mut self, state: State, _git_interactive: &GitInteractive) {
		self.scroll_position.reset();
		if let State::Help(return_state) = state {
			self.return_state = *return_state;
		}
		else {
			panic!("Help module activated when not expected");
		}
	}

	fn render(&self, view: &View, _git_interactive: &GitInteractive) {
		let (view_width, view_height) = view.get_view_size();

		let mut view_lines: Vec<ViewLine> = vec![];

		for line in get_help_lines(&self.return_state) {
			view_lines.push(ViewLine::new(vec![
				LineSegment::new_with_color(format!(" {:4} ", line.0).as_str(), WindowColor::IndicatorColor),
				LineSegment::new(line.1),
			]));
		}

		view.draw_title(false);

		view.set_color(WindowColor::Foreground);
		view.set_style(false, true, false);
		view.draw_str(" Key   Action");
		if view_width > 13 {
			let padding = " ".repeat(view_width - 13);
			view.draw_str(padding.as_str());
		}

		view.draw_view_lines(view_lines, self.scroll_position.get_position(), view_height - 3);

		view.set_color(WindowColor::IndicatorColor);
		view.draw_str("Any key to close");
	}
}

impl Help {
	pub fn new() -> Self {
		Self {
			return_state: State::List(false),
			scroll_position: ScrollPosition::new(3, 6, 3),
		}
	}

	// TODO refactor to remove need for view
	pub fn handle_input_with_view(
		&mut self,
		input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let (_, window_height) = view.get_view_size();
		let input = input_handler.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorDown => {
				self.scroll_position
					.scroll_down(window_height, get_help_lines(&self.return_state).len());
			},
			Input::MoveCursorUp => {
				self.scroll_position
					.scroll_up(window_height, get_help_lines(&self.return_state).len());
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
}
