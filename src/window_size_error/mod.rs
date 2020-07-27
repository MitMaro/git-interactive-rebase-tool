use crate::constants::{
	HEIGHT_ERROR_MESSAGE,
	MINIMUM_COMPACT_WINDOW_WIDTH,
	MINIMUM_WINDOW_HEIGHT,
	MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH,
	SHORT_ERROR_MESSAGE,
	SHORT_ERROR_MESSAGE_WIDTH,
};
use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::process::handle_input_result::HandleInputResult;
use crate::process::process_module::ProcessModule;
use crate::view::View;

pub(crate) struct WindowSizeError {}

impl ProcessModule for WindowSizeError {
	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_git_interactive: &mut GitInteractive,
		_view: &View<'_>,
	) -> HandleInputResult
	{
		HandleInputResult::new(input_handler.get_input(InputMode::Default))
	}

	fn render(&self, view: &View<'_>, _git_interactive: &GitInteractive) {
		let (window_width, window_height) = view.get_view_size();

		view.set_color(DisplayColor::Normal, false);
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
	pub(crate) fn new() -> Self {
		Self {}
	}
}
