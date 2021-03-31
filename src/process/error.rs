use crate::{
	display::display_color::DisplayColor,
	input::{input_handler::InputMode, Input},
	process::{
		process_module::ProcessModule,
		process_result::ProcessResult,
		state::State,
		util::handle_view_data_scroll,
	},
	todo_file::TodoFile,
	view::{line_segment::LineSegment, view_data::ViewData, view_line::ViewLine, View},
};

pub struct Error {
	return_state: State,
	view_data: ViewData,
}

impl ProcessModule for Error {
	fn activate(&mut self, _: &TodoFile, previous_state: State) -> ProcessResult {
		self.return_state = previous_state;
		ProcessResult::new()
	}

	fn build_view_data(&mut self, _: &View<'_>, _: &TodoFile) -> &mut ViewData {
		&mut self.view_data
	}

	fn handle_input(&mut self, view: &mut View<'_>, _: &mut TodoFile) -> ProcessResult {
		let input = view.get_input(InputMode::Default);
		let mut result = ProcessResult::new().input(input);
		if handle_view_data_scroll(input, &mut self.view_data).is_none() && input != Input::Resize {
			result = result.state(self.return_state);
		}
		result
	}
}

impl Error {
	pub const fn new() -> Self {
		Self {
			return_state: State::List,
			view_data: ViewData::new(),
		}
	}

	pub fn set_error_message(&mut self, error: &anyhow::Error) {
		self.view_data.reset();
		self.view_data.set_show_title(true);
		for cause in error.chain() {
			let error_text = format!("{:#}", cause);
			for err in error_text.split('\n') {
				self.view_data.push_line(ViewLine::from(err));
			}
		}
		self.view_data
			.push_trailing_line(ViewLine::from(LineSegment::new_with_color(
				"Press any key to continue",
				DisplayColor::IndicatorColor,
			)));
	}
}

#[cfg(test)]
mod tests {
	use anyhow::anyhow;

	use super::*;
	use crate::{
		assert_process_result,
		assert_rendered_output,
		process::testutil::{process_module_test, TestContext, ViewState},
	};

	#[test]
	#[serial_test::serial]
	fn simple_error() {
		process_module_test(&[], ViewState::default(), &[], |test_context: TestContext<'_>| {
			let mut module = Error::new();
			module.set_error_message(&anyhow!("Test Error"));
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
	#[serial_test::serial]
	fn error_with_contest() {
		process_module_test(&[], ViewState::default(), &[], |test_context: TestContext<'_>| {
			let mut module = Error::new();
			module.set_error_message(&anyhow!("Test Error").context("Context"));
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
	#[serial_test::serial]
	fn error_with_newlines() {
		process_module_test(&[], ViewState::default(), &[], |test_context: TestContext<'_>| {
			let mut module = Error::new();
			module.set_error_message(&anyhow!("Test\nError").context("With\nContext"));
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
	#[serial_test::serial]
	fn return_state() {
		process_module_test(
			&[],
			ViewState::default(),
			&[Input::Character('a')],
			|mut test_context: TestContext<'_>| {
				let mut module = Error::new();
				test_context.activate(&mut module, State::ConfirmRebase);
				module.set_error_message(&anyhow!("Test Error"));
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Character('a'),
					state = State::ConfirmRebase
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn resize() {
		process_module_test(
			&[],
			ViewState::default(),
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = Error::new();
				test_context.activate(&mut module, State::ConfirmRebase);
				module.set_error_message(&anyhow!("Test Error"));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Resize);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn scroll_events() {
		process_module_test(
			&[],
			ViewState::default(),
			&[
				Input::ScrollLeft,
				Input::ScrollRight,
				Input::ScrollDown,
				Input::ScrollUp,
				Input::ScrollJumpDown,
				Input::ScrollJumpUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Error::new();
				test_context.activate(&mut module, State::ConfirmRebase);
				module.set_error_message(&anyhow!("Test Error"));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollLeft);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollRight);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollDown);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollUp);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollJumpDown);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ScrollJumpUp);
			},
		);
	}
}
