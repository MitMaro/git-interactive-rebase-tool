use crate::constants::{MINIMUM_COMPACT_WINDOW_WIDTH, MINIMUM_WINDOW_HEIGHT, MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH};
use crate::input::input_handler::{InputHandler, InputMode};
use crate::process::process_result::ProcessResult;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;

pub struct WindowSizeError {
	view_data: ViewData,
}

const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue";
const SHORT_ERROR_MESSAGE: &str = "Window too small";
const SIZE_ERROR_MESSAGE: &str = "Size!";
const BUG_WINDOW_SIZE_MESSAGE: &str = "Bug: window size is not invalid!";

impl WindowSizeError {
	pub fn new(window_width: usize, window_height: usize) -> Self {
		let mut view_data = ViewData::new();
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

		view_data.push_line(ViewLine::new(vec![LineSegment::new(message)]));
		view_data.set_view_size(window_width, window_height);
		view_data.rebuild();
		Self { view_data }
	}

	pub const fn get_view_data(&self) -> &ViewData {
		&self.view_data
	}

	#[allow(clippy::unused_self)]
	pub fn handle_input(&self, input_handler: &InputHandler<'_>) -> ProcessResult {
		ProcessResult::new().input(input_handler.get_input(InputMode::Default))
	}
}
