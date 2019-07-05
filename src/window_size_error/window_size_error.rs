use crate::constants::{
	HEIGHT_ERROR_MESSAGE,
	MINIMUM_COMPACT_WINDOW_WIDTH,
	MINIMUM_WINDOW_HEIGHT,
	MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH,
	SHORT_ERROR_MESSAGE,
	SHORT_ERROR_MESSAGE_WIDTH,
};
use crate::git_interactive::GitInteractive;
use crate::input::InputHandler;
use crate::process::{HandleInputResult, ProcessModule};
use crate::view::View;
use crate::window::WindowColor;

pub struct WindowSizeError {}

impl ProcessModule for WindowSizeError {
	fn handle_input(
		&mut self,
		input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
		_view: &View,
	) -> HandleInputResult
	{
		HandleInputResult::new(input_handler.get_input())
	}

	fn render(&self, view: &View, _git_interactive: &GitInteractive) {
		let (window_width, window_height) = view.get_view_size();

		view.set_color(WindowColor::Foreground);
		if window_width <= MINIMUM_COMPACT_WINDOW_WIDTH {
			if window_width >= SHORT_ERROR_MESSAGE_WIDTH {
				view.draw_str(SHORT_ERROR_MESSAGE);
			}
			else {
				// not much to do if the window gets too narrow
				view.draw_str("Size!\n");
			}
			return;
		}

		if window_height <= MINIMUM_WINDOW_HEIGHT {
			if window_width >= MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH {
				view.draw_str(HEIGHT_ERROR_MESSAGE);
			}
			else if window_width >= SHORT_ERROR_MESSAGE_WIDTH {
				view.draw_str(SHORT_ERROR_MESSAGE);
			}
			else {
				// not much to do if the window gets too narrow
				view.draw_str("Size!\n");
			}
		}
	}
}

impl WindowSizeError {
	pub fn new() -> Self {
		Self {}
	}
}
