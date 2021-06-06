use crate::{
	components::confirm::{Confirm, Confirmed},
	input::EventHandler,
	process::{exit_status::ExitStatus, process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::TodoFile,
	view::{RenderContext, ViewData, ViewSender},
};

pub struct ConfirmAbort {
	dialog: Confirm,
}

impl ProcessModule for ConfirmAbort {
	fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &ViewData {
		self.dialog.get_view_data()
	}

	fn handle_events(
		&mut self,
		event_handler: &EventHandler,
		_: &ViewSender,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		let (confirmed, event) = self.dialog.handle_event(event_handler);
		let mut result = ProcessResult::from(event);
		match confirmed {
			Confirmed::Yes => {
				rebase_todo.set_lines(vec![]);
				result = result.exit_status(ExitStatus::Good);
			},
			Confirmed::No => {
				result = result.state(State::List);
			},
			Confirmed::Other => {},
		}
		result
	}
}

impl ConfirmAbort {
	pub(crate) fn new(confirm_yes: &[String], confirm_no: &[String]) -> Self {
		Self {
			dialog: Confirm::new("Are you sure you want to abort", confirm_yes, confirm_no),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		assert_process_result,
		assert_rendered_output,
		input::{Event, KeyCode, MetaEvent},
		process::testutil::{process_module_test, TestContext},
	};

	#[test]
	fn build_view_data() {
		process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
			let mut module = ConfirmAbort::new(
				&test_context.config.key_bindings.confirm_yes,
				&test_context.config.key_bindings.confirm_no,
			);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}Are you sure you want to abort (y/n)? "
			);
		});
	}

	#[test]
	fn handle_event_yes() {
		process_module_test(
			&["pick aaa comment"],
			&[Event::from(MetaEvent::Yes)],
			|mut test_context: TestContext<'_>| {
				let mut module = ConfirmAbort::new(
					&test_context.config.key_bindings.confirm_yes,
					&test_context.config.key_bindings.confirm_no,
				);
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::Yes),
					exit_status = ExitStatus::Good
				);
				assert!(test_context.rebase_todo_file.is_empty());
			},
		);
	}

	#[test]
	fn handle_event_no() {
		process_module_test(
			&["pick aaa comment"],
			&[Event::from(MetaEvent::No)],
			|mut test_context: TestContext<'_>| {
				let mut module = ConfirmAbort::new(
					&test_context.config.key_bindings.confirm_yes,
					&test_context.config.key_bindings.confirm_no,
				);
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::No),
					state = State::List
				);
			},
		);
	}

	#[test]
	fn handle_event_confirmed_other() {
		process_module_test(
			&["pick aaa comment"],
			&[Event::from(KeyCode::Null)],
			|mut test_context: TestContext<'_>| {
				let mut module = ConfirmAbort::new(
					&test_context.config.key_bindings.confirm_yes,
					&test_context.config.key_bindings.confirm_no,
				);
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(KeyCode::Null)
				);
			},
		);
	}
}
