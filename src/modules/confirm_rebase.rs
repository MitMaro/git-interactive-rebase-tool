use crate::{
	components::confirm::{Confirm, Confirmed, INPUT_OPTIONS},
	input::{Event, InputOptions, KeyBindings},
	module::{ExitStatus, Module, State},
	process::Results,
	view::{RenderContext, ViewData},
};

pub(crate) struct ConfirmRebase {
	dialog: Confirm,
}

impl Module for ConfirmRebase {
	fn build_view_data(&mut self, _: &RenderContext) -> &ViewData {
		self.dialog.get_view_data()
	}

	fn input_options(&self) -> &InputOptions {
		&INPUT_OPTIONS
	}

	fn read_event(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		Confirm::read_event(event, key_bindings)
	}

	fn handle_event(&mut self, event: Event, _: &crate::view::State) -> Results {
		let confirmed = self.dialog.handle_event(event);
		let mut results = Results::new();
		match confirmed {
			Confirmed::Yes => {
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
		assert_rendered_output,
		assert_results,
		input::{KeyCode, StandardEvent},
		process::Artifact,
		test_helpers::{assertions::assert_rendered_output::AssertRenderOptions, testers},
	};

	fn create_confirm_rebase() -> ConfirmRebase {
		ConfirmRebase::new(&[String::from("y")], &[String::from("n")])
	}

	#[test]
	fn build_view_data() {
		testers::module(&["pick aaa comment"], &[], |test_context| {
			let mut module = create_confirm_rebase();
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE | AssertRenderOptions::INCLUDE_STYLE,
				view_data,
				"{TITLE}",
				"{BODY}",
				"{Normal}Are you sure you want to rebase (y/n)? "
			);
		});
	}

	#[test]
	fn handle_event_yes() {
		testers::module(
			&["pick aaa comment"],
			&[Event::from(StandardEvent::Yes)],
			|mut test_context| {
				let mut module = create_confirm_rebase();
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from(StandardEvent::Yes)),
					Artifact::ExitStatus(ExitStatus::Good)
				);
			},
		);
	}

	#[test]
	fn handle_event_no() {
		testers::module(
			&["pick aaa comment"],
			&[Event::from(StandardEvent::No)],
			|mut test_context| {
				let mut module = create_confirm_rebase();
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from(StandardEvent::No)),
					Artifact::ChangeState(State::List)
				);
			},
		);
	}

	#[test]
	fn handle_event_no_match_key() {
		testers::module(
			&["pick aaa comment"],
			&[Event::from(KeyCode::Null)],
			|mut test_context| {
				let mut module = create_confirm_rebase();
				assert_results!(
					test_context.handle_event(&mut module),
					Artifact::Event(Event::from(KeyCode::Null))
				);
			},
		);
	}
}
