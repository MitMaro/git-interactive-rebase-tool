use input::EventHandler;
use todo_file::TodoFile;
use view::{RenderContext, ViewData, ViewSender};

use crate::{
	components::confirm::{Confirm, Confirmed},
	module::{ExitStatus, Module, ProcessResult, State},
};

pub(crate) struct ConfirmRebase {
	dialog: Confirm,
}

impl Module for ConfirmRebase {
	fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &ViewData {
		self.dialog.get_view_data()
	}

	fn handle_events(&mut self, event_handler: &EventHandler, _: &ViewSender, _: &mut TodoFile) -> ProcessResult {
		let (confirmed, event) = self.dialog.handle_event(event_handler);
		let mut result = ProcessResult::from(event);
		match confirmed {
			Confirmed::Yes => {
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

impl ConfirmRebase {
	pub(crate) fn new(confirm_yes: &[String], confirm_no: &[String]) -> Self {
		Self {
			dialog: Confirm::new("Are you sure you want to rebase", confirm_yes, confirm_no),
		}
	}
}

#[cfg(test)]
mod tests {
	use input::{Event, KeyCode, MetaEvent};
	use view::assert_rendered_output;

	use super::*;
	use crate::{assert_process_result, testutil::module_test};

	fn create_confirm_rebase() -> ConfirmRebase {
		ConfirmRebase::new(&[String::from("y")], &[String::from("n")])
	}

	#[test]
	fn build_view_data() {
		module_test(&["pick aaa comment"], &[], |test_context| {
			let mut module = create_confirm_rebase();
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}Are you sure you want to rebase (y/n)? "
			);
		});
	}

	#[test]
	fn handle_event_yes() {
		module_test(
			&["pick aaa comment"],
			&[Event::from(MetaEvent::Yes)],
			|mut test_context| {
				let mut module = create_confirm_rebase();
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::Yes),
					exit_status = ExitStatus::Good
				);
				assert!(!test_context.rebase_todo_file.is_empty());
			},
		);
	}

	#[test]
	fn handle_event_no() {
		module_test(
			&["pick aaa comment"],
			&[Event::from(MetaEvent::No)],
			|mut test_context| {
				let mut module = create_confirm_rebase();
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(MetaEvent::No),
					state = State::List
				);
			},
		);
	}

	#[test]
	fn handle_event_no_match_key() {
		module_test(
			&["pick aaa comment"],
			&[Event::from(KeyCode::Null)],
			|mut test_context| {
				let mut module = create_confirm_rebase();
				assert_process_result!(
					test_context.handle_event(&mut module),
					event = Event::from(KeyCode::Null)
				);
			},
		);
	}
}
