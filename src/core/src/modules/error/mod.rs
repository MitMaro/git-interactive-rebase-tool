use captur::capture;
use display::DisplayColor;
use input::{Event, InputOptions};
use lazy_static::lazy_static;
use todo_file::TodoFile;
use view::{handle_view_data_scroll, LineSegment, RenderContext, ViewData, ViewLine, ViewSender};

use crate::module::{Module, ProcessResult, State};

lazy_static! {
	pub static ref INPUT_OPTIONS: InputOptions = InputOptions::RESIZE | InputOptions::MOVEMENT;
}

pub(crate) struct Error {
	return_state: State,
	view_data: ViewData,
}

impl Module for Error {
	fn activate(&mut self, _: &TodoFile, previous_state: State) -> ProcessResult {
		self.return_state = previous_state;
		ProcessResult::new()
	}

	fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &ViewData {
		&self.view_data
	}

	fn input_options(&self) -> &InputOptions {
		&INPUT_OPTIONS
	}

	fn handle_event(&mut self, event: Event, view_sender: &ViewSender, _: &mut TodoFile) -> ProcessResult {
		let mut result = ProcessResult::from(event);
		if handle_view_data_scroll(event, view_sender).is_none() {
			if let Event::Key(_) = event {
				result = result.state(self.return_state);
			}
		}
		result
	}

	fn handle_error(&mut self, error: &anyhow::Error) {
		self.view_data.update_view_data(|updater| {
			capture!(error);
			updater.clear();
			for cause in error.chain() {
				let error_text = format!("{:#}", cause);
				for err in error_text.split('\n') {
					updater.push_line(ViewLine::from(err));
				}
			}
			updater.push_trailing_line(ViewLine::from(LineSegment::new_with_color(
				"Press any key to continue",
				DisplayColor::IndicatorColor,
			)));
		});
	}
}

impl Error {
	pub(crate) fn new() -> Self {
		Self {
			return_state: State::List,
			view_data: ViewData::new(|updater| {
				updater.set_show_title(true);
				updater.set_retain_scroll_position(false);
			}),
		}
	}
}

#[cfg(test)]
mod tests {
	use anyhow::anyhow;
	use input::{Event, MetaEvent};
	use view::assert_rendered_output;

	use super::*;
	use crate::{assert_process_result, testutil::module_test};

	#[test]
	fn simple_error() {
		module_test(&[], &[], |test_context| {
			let mut module = Error::new();
			module.handle_error(&anyhow!("Test Error"));
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}Test Error",
				"{TRAILING}",
				"{IndicatorColor}Press any key to continue"
			);
		});
	}

	#[test]
	fn error_with_contest() {
		module_test(&[], &[], |test_context| {
			let mut module = Error::new();
			module.handle_error(&anyhow!("Test Error").context("Context"));
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}Context",
				"{Normal}Test Error",
				"{TRAILING}",
				"{IndicatorColor}Press any key to continue"
			);
		});
	}

	#[test]
	fn error_with_newlines() {
		module_test(&[], &[], |test_context| {
			let mut module = Error::new();
			module.handle_error(&anyhow!("Test\nError").context("With\nContext"));
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}With",
				"{Normal}Context",
				"{Normal}Test",
				"{Normal}Error",
				"{TRAILING}",
				"{IndicatorColor}Press any key to continue"
			);
		});
	}

	#[test]
	fn return_state() {
		module_test(&[], &[Event::from('a')], |mut test_context| {
			let mut module = Error::new();
			let _ = test_context.activate(&mut module, State::ConfirmRebase);
			module.handle_error(&anyhow!("Test Error"));
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from('a'),
				state = State::ConfirmRebase
			);
		});
	}

	#[test]
	fn resize() {
		module_test(&[], &[Event::Resize(100, 100)], |mut test_context| {
			let mut module = Error::new();
			let _ = test_context.activate(&mut module, State::ConfirmRebase);
			module.handle_error(&anyhow!("Test Error"));
			assert_process_result!(test_context.handle_event(&mut module), event = Event::Resize(100, 100));
		});
	}

	#[test]
	fn scroll_events() {
		module_test(
			&[],
			&[
				Event::from(MetaEvent::ScrollLeft),
				Event::from(MetaEvent::ScrollRight),
				Event::from(MetaEvent::ScrollDown),
				Event::from(MetaEvent::ScrollUp),
				Event::from(MetaEvent::ScrollJumpDown),
				Event::from(MetaEvent::ScrollJumpUp),
			],
			|mut test_context| {
				let mut module = Error::new();
				let _ = test_context.activate(&mut module, State::ConfirmRebase);
				module.handle_error(&anyhow!("Test Error"));
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::ScrollLeft)
				);
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::ScrollRight)
				);
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::ScrollDown)
				);
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::ScrollUp)
				);
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::ScrollJumpDown)
				);
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::ScrollJumpUp)
				);
			},
		);
	}
}
