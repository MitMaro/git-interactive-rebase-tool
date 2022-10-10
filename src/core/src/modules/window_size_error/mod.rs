use input::InputOptions;
use lazy_static::lazy_static;
use todo_file::TodoFile;
use view::{RenderContext, ViewData, ViewLine};

use crate::{
	events::Event,
	module::{Module, State},
	process::Results,
};

const HEIGHT_ERROR_MESSAGE: &str = "Window too small, increase height to continue";
const SHORT_ERROR_MESSAGE: &str = "Window too small";
const SIZE_ERROR_MESSAGE: &str = "Size!";

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::MOVEMENT;
}

pub(crate) struct WindowSizeError {
	return_state: State,
	view_data: ViewData,
}

impl Module for WindowSizeError {
	fn activate(&mut self, _: &TodoFile, previous_state: State) -> Results {
		self.return_state = previous_state;
		Results::new()
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

	fn input_options(&self) -> &InputOptions {
		&INPUT_OPTIONS
	}

	fn handle_event(&mut self, event: Event, _: &view::State, _: &mut TodoFile) -> Results {
		let mut results = Results::new();

		if let Event::Resize(width, height) = event {
			let render_context = RenderContext::new(width as usize, height as usize);
			if !render_context.is_window_too_small() {
				results.state(self.return_state);
			}
		}

		results
	}
}

impl WindowSizeError {
	pub(crate) fn new() -> Self {
		Self {
			return_state: State::List,
			view_data: ViewData::new(|_| {}),
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;
	use view::assert_rendered_output;

	use super::*;
	use crate::{assert_results, process::Artifact, testutil::module_test};

	const MINIMUM_WINDOW_HEIGHT: usize = 5;
	const MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH: usize = 45;

	#[rstest]
	#[case::width_too_small_long_message(
		SHORT_ERROR_MESSAGE.len(),
		MINIMUM_WINDOW_HEIGHT + 1,
		"Window too small"
	)]
	#[case::width_too_small_short_message(SHORT_ERROR_MESSAGE.len() - 1, MINIMUM_WINDOW_HEIGHT + 1, "Size!")]
	#[case::height_too_small_long_message(
		MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH,
		MINIMUM_WINDOW_HEIGHT,
		"Window too small, increase height to continue"
	)]
	#[case::height_too_small_short_message(
		MINIMUM_WINDOW_HEIGHT_ERROR_WIDTH - 1,
		MINIMUM_WINDOW_HEIGHT,
		"Window too small"
	)]
	fn build_view_data(#[case] width: usize, #[case] height: usize, #[case] expected: &str) {
		module_test(&[], &[], |mut test_context| {
			test_context.render_context.update(width as u16, height as u16);
			let mut module = WindowSizeError::new();
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(view_data, "{BODY}", format!("{{Normal}}{expected}"));
		});
	}

	#[test]
	fn event_resize_window_still_small() {
		module_test(&[], &[Event::Resize(1, 1)], |mut test_context| {
			let mut module = WindowSizeError::new();
			let _ = test_context.activate(&mut module, State::ConfirmRebase);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::Resize(1, 1))
			);
		});
	}

	#[test]
	fn event_resize_window_no_longer_too_small() {
		module_test(&[], &[Event::Resize(100, 100)], |mut test_context| {
			let mut module = WindowSizeError::new();
			let _ = test_context.activate(&mut module, State::ConfirmRebase);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::Resize(100, 100)),
				Artifact::ChangeState(State::ConfirmRebase)
			);
		});
	}

	#[test]
	fn event_other_character() {
		module_test(&[], &[Event::from('a')], |mut test_context| {
			let mut module = WindowSizeError::new();
			let _ = test_context.activate(&mut module, State::ConfirmRebase);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('a'))
			);
		});
	}
}
