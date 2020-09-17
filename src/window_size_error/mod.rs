use crate::constants::{MINIMUM_COMPACT_WINDOW_WIDTH, MINIMUM_WINDOW_HEIGHT, MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH};
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;

pub struct WindowSizeError {
	view_data: ViewData,
}

const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue";
const SHORT_ERROR_MESSAGE: &str = "Window too small";
const SIZE_ERROR_MESSAGE: &str = "Size!";
const BUG_WINDOW_SIZE_MESSAGE: &str = "Bug: window size is not invalid!";

impl ProcessModule for WindowSizeError {
	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (window_width, window_height) = view.get_view_size();

		let message = if window_width <= MINIMUM_COMPACT_WINDOW_WIDTH {
			if window_width >= SHORT_ERROR_MESSAGE.len() {
				SHORT_ERROR_MESSAGE
			}
			else {
				// not much to do if the window gets too narrow
				SIZE_ERROR_MESSAGE
			}
		}
		else if window_height <= MINIMUM_WINDOW_HEIGHT {
			if window_width >= MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH {
				HEIGHT_ERROR_MESSAGE
			}
			else if window_width >= SHORT_ERROR_MESSAGE.len() {
				SHORT_ERROR_MESSAGE
			}
			else {
				// not much to do if the window gets too narrow
				SIZE_ERROR_MESSAGE
			}
		}
		else {
			BUG_WINDOW_SIZE_MESSAGE
		};

		self.view_data.clear();
		self.view_data.push_line(ViewLine::new(vec![LineSegment::new(message)]));
		self.view_data.set_view_size(window_width, window_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_git_interactive: &mut GitInteractive,
		_view: &View<'_>,
	) -> ProcessResult
	{
		ProcessResult::new().input(input_handler.get_input(InputMode::Default))
	}
}

impl WindowSizeError {
	pub const fn new() -> Self {
		Self {
			view_data: ViewData::new(),
		}
	}
}
