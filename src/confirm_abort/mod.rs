use crate::input::input_handler::InputMode;
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::todo_file::TodoFile;
use crate::view::view_data::ViewData;
use crate::view::View;

pub struct ConfirmAbort {
	view_data: ViewData,
}

impl ProcessModule for ConfirmAbort {
	fn build_view_data(&mut self, view: &View<'_>, _: &TodoFile) -> &ViewData {
		let view_width = view.get_view_size().width();
		let view_height = view.get_view_size().height();
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(&mut self, view: &mut View<'_>, rebase_todo: &mut TodoFile) -> ProcessResult {
		let input = view.get_input(InputMode::Confirm);
		let mut result = ProcessResult::new().input(input);
		match input {
			Input::Yes => {
				rebase_todo.set_lines(vec![]);
				result = result.exit_status(ExitStatus::Good);
			},
			Input::No => {
				result = result.state(State::List);
			},
			_ => {},
		}
		result
	}
}

impl ConfirmAbort {
	pub(crate) fn new() -> Self {
		Self {
			view_data: ViewData::new_confirm("Are you sure you want to abort"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_process_result;
	use crate::assert_rendered_output;
	use crate::process::testutil::{process_module_test, TestContext, ViewState};

	#[test]
	#[serial_test::serial]
	fn build_view_data() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|test_context: TestContext<'_>| {
				let mut module = ConfirmAbort::new();
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "{TITLE}", "{PROMPT}", "Are you sure you want to abort");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn handle_input_yes() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Yes],
			|mut test_context: TestContext<'_>| {
				let mut module = ConfirmAbort::new();
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Yes,
					exit_status = ExitStatus::Good
				);
				assert_eq!(test_context.rebase_todo_file.get_lines().len(), 0);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn handle_input_no() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::No],
			|mut test_context: TestContext<'_>| {
				let mut module = ConfirmAbort::new();
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::No,
					state = State::List
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn handle_input_any_key() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('x')],
			|mut test_context: TestContext<'_>| {
				let mut module = ConfirmAbort::new();
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::No,
					state = State::List
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn handle_input_resize() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = ConfirmAbort::new();
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Resize);
			},
		);
	}
}
