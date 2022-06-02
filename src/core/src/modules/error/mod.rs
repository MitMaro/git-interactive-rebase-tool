use captur::capture;
use display::DisplayColor;
use input::InputOptions;
use lazy_static::lazy_static;
use todo_file::TodoFile;
use view::{LineSegment, RenderContext, ViewData, ViewLine};

use crate::{
	events::Event,
	module::{Module, State},
	process::Results,
	util::handle_view_data_scroll,
};

lazy_static! {
	pub static ref INPUT_OPTIONS: InputOptions = InputOptions::RESIZE | InputOptions::MOVEMENT;
}

pub(crate) struct Error {
	return_state: State,
	view_data: ViewData,
}

impl Module for Error {
	fn activate(&mut self, _: &TodoFile, previous_state: State) -> Results {
		self.return_state = previous_state;
		Results::new()
	}

	fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &ViewData {
		&self.view_data
	}

	fn input_options(&self) -> &InputOptions {
		&INPUT_OPTIONS
	}

	fn handle_event(&mut self, event: Event, view_state: &view::State, _: &mut TodoFile) -> Results {
		let mut results = Results::new();
		if handle_view_data_scroll(event, view_state).is_none() {
			if let Event::Key(_) = event {
				results.state(self.return_state);
			}
		}
		results
	}

	fn handle_error(&mut self, error: &anyhow::Error) -> Results {
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
		Results::new()
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
	use view::assert_rendered_output;

	use super::*;
	use crate::{assert_results, process::Artifact, testutil::module_test};

	#[test]
	fn simple_error() {
		module_test(&[], &[], |test_context| {
			let mut module = Error::new();
			let _ = module.handle_error(&anyhow!("Test Error"));
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
			let _ = module.handle_error(&anyhow!("Test Error").context("Context"));
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
			let _ = module.handle_error(&anyhow!("Test\nError").context("With\nContext"));
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
			let _ = module.handle_error(&anyhow!("Test Error"));
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from('a')),
				Artifact::ChangeState(State::ConfirmRebase)
			);
		});
	}

	#[test]
	fn resize() {
		module_test(&[], &[Event::Resize(100, 100)], |mut test_context| {
			let mut module = Error::new();
			let _ = test_context.activate(&mut module, State::ConfirmRebase);
			let _ = module.handle_error(&anyhow!("Test Error"));
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::Resize(100, 100))
			);
		});
	}
}
