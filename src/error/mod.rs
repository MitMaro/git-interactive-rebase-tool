use lazy_static::lazy_static;

use crate::{
	display::DisplayColor,
	input::{Event, EventHandler, InputOptions},
	module::{Module, ProcessResult, State},
	todo_file::TodoFile,
	view::{handle_view_data_scroll, LineSegment, RenderContext, ViewData, ViewLine, ViewSender},
};

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::new().movement(true);
}

pub struct Error {
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

	fn handle_events(
		&mut self,
		event_handler: &EventHandler,
		view_sender: &ViewSender,
		_: &mut TodoFile,
	) -> ProcessResult {
		let event = event_handler.read_event(&INPUT_OPTIONS, |event, _| event);
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
	pub fn new() -> Self {
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

	use super::*;
	use crate::{
		assert_process_result,
		assert_rendered_output,
		input::{Event, MetaEvent},
		process::testutil::process_module_test,
	};

	#[test]
	fn simple_error() {
		process_module_test(&[], &[], |test_context| {
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
		process_module_test(&[], &[], |test_context| {
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
		process_module_test(&[], &[], |test_context| {
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
		process_module_test(&[], &[Event::from('a')], |mut test_context| {
			let mut module = Error::new();
			test_context.activate(&mut module, State::ConfirmRebase);
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
		process_module_test(&[], &[Event::Resize(100, 100)], |mut test_context| {
			let mut module = Error::new();
			test_context.activate(&mut module, State::ConfirmRebase);
			module.handle_error(&anyhow!("Test Error"));
			assert_process_result!(test_context.handle_event(&mut module), event = Event::Resize(100, 100));
		});
	}

	#[test]
	fn scroll_events() {
		process_module_test(
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
				test_context.activate(&mut module, State::ConfirmRebase);
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
