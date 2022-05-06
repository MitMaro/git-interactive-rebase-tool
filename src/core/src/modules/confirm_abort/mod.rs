use input::InputOptions;
use todo_file::TodoFile;
use view::{RenderContext, ViewData, ViewSender};

use crate::{
	components::confirm::{Confirm, Confirmed, INPUT_OPTIONS},
	events::{Event, KeyBindings},
	module::{ExitStatus, Module, State},
	process::Results,
};

pub(crate) struct ConfirmAbort {
	dialog: Confirm,
}

impl Module for ConfirmAbort {
	fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &ViewData {
		self.dialog.get_view_data()
	}

	fn input_options(&self) -> &InputOptions {
		&INPUT_OPTIONS
	}

	fn read_event(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		Confirm::read_event(event, key_bindings)
	}

	fn handle_event(&mut self, event: Event, _: &ViewSender, rebase_todo: &mut TodoFile) -> Results {
		let confirmed = self.dialog.handle_event(event);
		let mut results = Results::new();
		match confirmed {
			Confirmed::Yes => {
				rebase_todo.set_lines(vec![]);
				results.exit_status(ExitStatus::Good);
			},
			Confirmed::No => {
				results.state(State::List);
			},
			Confirmed::Other => {},
		}
		results
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
	use input::KeyCode;
	use view::assert_rendered_output;

	use super::*;
	use crate::{assert_results, events::MetaEvent, process::Artifact, testutil::module_test};

	fn create_confirm_abort() -> ConfirmAbort {
		ConfirmAbort::new(&[String::from("y")], &[String::from("n")])
	}

	#[test]
	fn build_view_data() {
		module_test(&["pick aaa comment"], &[], |test_context| {
			let mut module = create_confirm_abort();
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE,
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}Are you sure you want to abort (y/n)? "
			);
		});
	}

	#[test]
	fn handle_event_yes() {
		module_test(
			&["pick aaa comment"],
			&[Event::from(MetaEvent::Yes)],
			|mut test_context| {
				let mut module = create_confirm_abort();
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from(MetaEvent::Yes)),
					Artifact::ExitStatus(ExitStatus::Good)
				);
				assert!(test_context.rebase_todo_file.is_empty());
			},
		);
	}

	#[test]
	fn handle_event_no() {
		module_test(
			&["pick aaa comment"],
			&[Event::from(MetaEvent::No)],
			|mut test_context| {
				let mut module = create_confirm_abort();
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from(MetaEvent::No)),
					Artifact::ChangeState(State::List)
				);
			},
		);
	}

	#[test]
	fn handle_event_confirmed_other() {
		module_test(
			&["pick aaa comment"],
			&[Event::from(KeyCode::Null)],
			|mut test_context| {
				let mut module = create_confirm_abort();
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from(KeyCode::Null))
				);
			},
		);
	}
}
