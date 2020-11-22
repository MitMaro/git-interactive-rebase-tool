use crate::constants::{MINIMUM_COMPACT_WINDOW_WIDTH, MINIMUM_WINDOW_HEIGHT, MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH};
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::todo_file::TodoFile;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;

const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue";
const SHORT_ERROR_MESSAGE: &str = "Window too small";
const SIZE_ERROR_MESSAGE: &str = "Size!";
const BUG_WINDOW_SIZE_MESSAGE: &str = "Bug: window size is invalid!";

pub struct WindowSizeError {
	return_state: State,
	view_data: ViewData,
}

impl ProcessModule for WindowSizeError {
	fn activate(&mut self, _: &TodoFile, previous_state: State) -> ProcessResult {
		self.return_state = previous_state;
		ProcessResult::new()
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &TodoFile) -> &ViewData {
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
			else {
				// this message will always be safe here
				SHORT_ERROR_MESSAGE
			}
		}
		else {
			BUG_WINDOW_SIZE_MESSAGE
		};

		self.view_data.clear();
		self.view_data.push_line(ViewLine::from(message));
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(&mut self, input_handler: &InputHandler<'_>, _: &mut TodoFile, view: &View<'_>) -> ProcessResult {
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_process_result;
	use crate::assert_rendered_output;
	use crate::process::testutil::{process_module_test, TestContext, ViewState};
	use rstest::rstest;
	use std::convert::TryFrom;

	#[test]
	fn is_window_too_small_width_too_small() {
		assert!(WindowSizeError::is_window_too_small(
			MINIMUM_COMPACT_WINDOW_WIDTH,
			MINIMUM_WINDOW_HEIGHT + 1
		));
	}

	#[test]
	fn is_window_too_small_height_too_small() {
		assert!(WindowSizeError::is_window_too_small(
			MINIMUM_COMPACT_WINDOW_WIDTH + 1,
			MINIMUM_WINDOW_HEIGHT
		));
	}

	#[test]
	fn is_window_too_small_height_and_width_too_small() {
		assert!(WindowSizeError::is_window_too_small(
			MINIMUM_COMPACT_WINDOW_WIDTH,
			MINIMUM_WINDOW_HEIGHT
		));
	}

	#[test]
	fn is_window_too_small_width_and_height_large() {
		assert!(!WindowSizeError::is_window_too_small(
			MINIMUM_COMPACT_WINDOW_WIDTH + 1,
			MINIMUM_WINDOW_HEIGHT + 1
		));
	}

	#[rstest(
		width, height, expected,
		case::not_too_small(100, 100, "Bug: window size is invalid!"),
		case::width_too_small_long_message(SHORT_ERROR_MESSAGE.len(), MINIMUM_WINDOW_HEIGHT + 1, "Window too small"),
		case::width_too_small_short_message(SHORT_ERROR_MESSAGE.len() - 1, MINIMUM_WINDOW_HEIGHT + 1, "Size!"),
		case::height_too_small_long_message(
			MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH, MINIMUM_WINDOW_HEIGHT, "Window too small, increase height to continue"
		),
		case::height_too_small_short_message(
			MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH - 1, MINIMUM_WINDOW_HEIGHT, "Window too small"
		)
	)]
	#[allow(clippy::cast_possible_wrap)]
	#[serial_test::serial]
	fn build_view_data(width: usize, height: usize, expected: &str) {
		process_module_test(
			&[],
			ViewState {
				size: (i32::try_from(width).unwrap(), i32::try_from(height).unwrap()),
				..ViewState::default()
			},
			&[],
			|test_context: TestContext<'_>| {
				let mut module = WindowSizeError::new();
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "{BODY}", format!("{{Normal}}{}", expected));
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn input_resize_window_still_small() {
		process_module_test(
			&[],
			ViewState {
				size: (1, 1),
				..ViewState::default()
			},
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = WindowSizeError::new();
				test_context.activate(&mut module, State::ConfirmRebase);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Resize);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn input_resize_window_no_longer_too_small() {
		process_module_test(
			&[],
			ViewState {
				size: (100, 100),
				..ViewState::default()
			},
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = WindowSizeError::new();
				test_context.activate(&mut module, State::ConfirmRebase);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Resize,
					state = State::ConfirmRebase
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn input_other_character() {
		process_module_test(
			&[],
			ViewState {
				size: (100, 100),
				..ViewState::default()
			},
			&[Input::Character('a')],
			|mut test_context: TestContext<'_>| {
				let mut module = WindowSizeError::new();
				test_context.activate(&mut module, State::ConfirmRebase);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Character('a'));
			},
		);
	}
}
