use crate::{
	input::{
		input_handler::{InputHandler, InputMode},
		Input,
	},
	process::{process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::TodoFile,
	view::{render_context::RenderContext, view_data::ViewData, view_line::ViewLine, View},
};

const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue";
const SHORT_ERROR_MESSAGE: &str = "Window too small";
const SIZE_ERROR_MESSAGE: &str = "Size!";

pub struct WindowSizeError {
	return_state: State,
	view_data: ViewData,
}

impl ProcessModule for WindowSizeError {
	fn activate(&mut self, _: &TodoFile, previous_state: State) -> ProcessResult {
		self.return_state = previous_state;
		ProcessResult::new()
	}

	fn build_view_data(&mut self, context: &RenderContext, _: &TodoFile) -> &mut ViewData {
		let view_width = context.width();
		let message = if !context.is_minimum_view_width() {
			if view_width >= SHORT_ERROR_MESSAGE.len() {
				SHORT_ERROR_MESSAGE
			}
			else {
				// not much to do if the window gets too narrow
				SIZE_ERROR_MESSAGE
			}
		}
		else if view_width >= HEIGHT_ERROR_MESSAGE.len() {
			HEIGHT_ERROR_MESSAGE
		}
		else {
			// this message will always be safe here
			SHORT_ERROR_MESSAGE
		};

		self.view_data.clear();
		self.view_data.push_line(ViewLine::from(message));
		&mut self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		view: &mut View<'_>,
		_: &mut TodoFile,
	) -> ProcessResult {
		let input = input_handler.get_input(InputMode::Default);
		let mut result = ProcessResult::new().input(input);

		if input == Input::Resize && !view.get_render_context().is_window_too_small() {
			result = result.state(self.return_state);
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
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{
		assert_process_result,
		assert_rendered_output,
		display::size::Size,
		process::testutil::{process_module_test, TestContext, ViewState},
	};

	const MINIMUM_WINDOW_HEIGHT: usize = 5;
	const MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH: usize = 45;

	#[rstest(
		width, height, expected,
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
				size: Size::new(width, height),
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
			ViewState { size: Size::new(1, 1) },
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
				size: Size::new(100, 100),
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
			ViewState::default(),
			&[Input::Character('a')],
			|mut test_context: TestContext<'_>| {
				let mut module = WindowSizeError::new();
				test_context.activate(&mut module, State::ConfirmRebase);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Character('a'));
			},
		);
	}
}
