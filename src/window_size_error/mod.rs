use lazy_static::lazy_static;

use crate::{
	input::{Event, EventHandler, InputOptions},
	module::{Module, ProcessResult, State},
	todo_file::TodoFile,
	view::{RenderContext, ViewData, ViewLine, ViewSender},
};

const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue";
const SHORT_ERROR_MESSAGE: &str = "Window too small";
const SIZE_ERROR_MESSAGE: &str = "Size!";

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::new().movement(true).resize(false);
}

pub struct WindowSizeError {
	return_state: State,
	view_data: ViewData,
}

impl Module for WindowSizeError {
	fn activate(&mut self, _: &TodoFile, previous_state: State) -> ProcessResult {
		self.return_state = previous_state;
		ProcessResult::new()
	}

	fn build_view_data(&mut self, context: &RenderContext, _: &TodoFile) -> &ViewData {
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

		self.view_data.update_view_data(|updater| {
			updater.clear();
			updater.push_line(ViewLine::from(message));
		});
		&mut self.view_data
	}

	fn handle_events(&mut self, event_handler: &EventHandler, _: &ViewSender, _: &mut TodoFile) -> ProcessResult {
		let event = event_handler.read_event(&INPUT_OPTIONS, |event, _| event);
		let mut result = ProcessResult::from(event);

		if let Event::Resize(width, height) = event {
			let render_context = RenderContext::new(width, height);
			if !render_context.is_window_too_small() {
				result = result.state(self.return_state);
			}
		}

		result
	}
}

impl WindowSizeError {
	pub fn new() -> Self {
		Self {
			return_state: State::List,
			view_data: ViewData::new(|_| {}),
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{assert_process_result, assert_rendered_output, process::testutil::process_module_test};

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
	fn build_view_data(width: usize, height: usize, expected: &str) {
		process_module_test(&[], &[], |mut test_context| {
			test_context.render_context.update(width as u16, height as u16);
			let mut module = WindowSizeError::new();
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(view_data, "{BODY}", format!("{{Normal}}{}", expected));
		});
	}

	#[test]
	fn event_resize_window_still_small() {
		process_module_test(&[], &[Event::Resize(1, 1)], |mut test_context| {
			let mut module = WindowSizeError::new();
			test_context.activate(&mut module, State::ConfirmRebase);
			assert_process_result!(test_context.handle_event(&mut module), event = Event::Resize(1, 1));
		});
	}

	#[test]
	fn event_resize_window_no_longer_too_small() {
		process_module_test(&[], &[Event::Resize(100, 100)], |mut test_context| {
			let mut module = WindowSizeError::new();
			test_context.activate(&mut module, State::ConfirmRebase);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::Resize(100, 100),
				state = State::ConfirmRebase
			);
		});
	}

	#[test]
	fn event_other_character() {
		process_module_test(&[], &[Event::from('a')], |mut test_context| {
			let mut module = WindowSizeError::new();
			test_context.activate(&mut module, State::ConfirmRebase);
			assert_process_result!(test_context.handle_event(&mut module), event = Event::from('a'));
		});
	}
}
