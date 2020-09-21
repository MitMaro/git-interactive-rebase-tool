use crate::constants::{MINIMUM_COMPACT_WINDOW_WIDTH, MINIMUM_WINDOW_HEIGHT, MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH};
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use anyhow::Result;

const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue";
const SHORT_ERROR_MESSAGE: &str = "Window too small";
const SIZE_ERROR_MESSAGE: &str = "Size!";
const BUG_WINDOW_SIZE_MESSAGE: &str = "Bug: window size is invalid!";

pub struct WindowSizeError {
	return_state: State,
	view_data: ViewData,
}

impl ProcessModule for WindowSizeError {
	fn activate(&mut self, _: &GitInteractive, previous_state: State) -> Result<()> {
		self.return_state = previous_state;
		Ok(())
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();
		let message = if view_width <= MINIMUM_COMPACT_WINDOW_WIDTH {
			if view_width >= SHORT_ERROR_MESSAGE.len() {
				SHORT_ERROR_MESSAGE
			}
			else {
				// not much to do if the window gets too narrow
				SIZE_ERROR_MESSAGE
			}
		}
		else if view_height <= MINIMUM_WINDOW_HEIGHT {
			if view_width >= MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH {
				HEIGHT_ERROR_MESSAGE
			}
			else if view_width >= SHORT_ERROR_MESSAGE.len() {
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
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_: &mut GitInteractive,
		view: &View<'_>,
	) -> ProcessResult
	{
		let input = input_handler.get_input(InputMode::Default);
		let mut result = ProcessResult::new().input(input);

		if input == Input::Resize {
			let (view_width, view_height) = view.get_view_size();
			if !Self::is_window_too_small(view_width, view_height) {
				result = result.state(self.return_state);
			}
		}

		result
	}
}

impl WindowSizeError {
	pub const fn new() -> Self {
		Self {
			return_state: State::List,
			view_data: ViewData::new(),
		}
	}

	pub const fn is_window_too_small(window_width: usize, window_height: usize) -> bool {
		window_width <= MINIMUM_COMPACT_WINDOW_WIDTH || window_height <= MINIMUM_WINDOW_HEIGHT
	}
}
