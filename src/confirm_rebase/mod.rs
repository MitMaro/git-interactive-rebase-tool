use crate::{
	components::Confirm,
	input::input_handler::{InputHandler, InputMode},
	process::{exit_status::ExitStatus, process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::TodoFile,
	view::{render_context::RenderContext, view_data::ViewData, View},
};

pub struct ConfirmRebase {
	dialog: Confirm,
}

impl ProcessModule for ConfirmRebase {
	fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &mut ViewData {
		self.dialog.get_view_data()
	}

	fn handle_input(&mut self, input_handler: &InputHandler<'_>, _: &mut View<'_>, _: &mut TodoFile) -> ProcessResult {
		let input = input_handler.get_input(InputMode::Confirm);
		let mut result = ProcessResult::new().input(input);
		if let Some(confirmed) = self.dialog.handle_input(input) {
			if confirmed {
				result = result.exit_status(ExitStatus::Good);
			}
			else {
				result = result.state(State::List);
			}
		}
		result
	}
}

impl ConfirmRebase {
	pub(crate) fn new(confirm_yes: &[String], confirm_no: &[String]) -> Self {
		Self {
			dialog: Confirm::new("Are you sure you want to rebase", confirm_yes, confirm_no),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		assert_process_result,
		assert_rendered_output,
		input::Input,
		process::testutil::{process_module_test, TestContext, ViewState},
	};

	#[test]
	#[serial_test::serial]
	fn build_view_data() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|test_context: TestContext<'_>| {
				let mut module = ConfirmRebase::new(
					&test_context.config.key_bindings.confirm_yes,
					&test_context.config.key_bindings.confirm_no,
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}Are you sure you want to rebase (y/n)? "
				);
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
				let mut module = ConfirmRebase::new(
					&test_context.config.key_bindings.confirm_yes,
					&test_context.config.key_bindings.confirm_no,
				);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Yes,
					exit_status = ExitStatus::Good
				);
				assert!(!test_context.rebase_todo_file.is_empty());
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
				let mut module = ConfirmRebase::new(
					&test_context.config.key_bindings.confirm_yes,
					&test_context.config.key_bindings.confirm_no,
				);
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
	fn handle_input_no_match_key() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = ConfirmRebase::new(
					&test_context.config.key_bindings.confirm_yes,
					&test_context.config.key_bindings.confirm_no,
				);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Resize);
			},
		);
	}
}
