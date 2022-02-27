mod exit_status;
mod modules;
mod process_result;
mod state;

use anyhow::Error;
use input::{Event, InputOptions, KeyBindings};
use lazy_static::lazy_static;
use todo_file::TodoFile;
use view::{RenderContext, ViewData, ViewSender};

pub(crate) use self::{exit_status::ExitStatus, modules::Modules, process_result::ProcessResult, state::State};

lazy_static! {
	static ref DEFAULT_INPUT_OPTIONS: InputOptions = InputOptions::RESIZE;
}

pub(crate) trait Module: Send {
	fn activate(&mut self, _rebase_todo: &TodoFile, _previous_state: State) -> ProcessResult {
		ProcessResult::new()
	}

	fn deactivate(&mut self) {}

	fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData;

	fn input_options(&self) -> &InputOptions {
		&DEFAULT_INPUT_OPTIONS
	}

	fn read_event(&self, event: Event, _key_bindings: &KeyBindings) -> Event {
		event
	}

	fn handle_event(&mut self, event: Event, _view_sender: &ViewSender, _rebase_todo: &mut TodoFile) -> ProcessResult;

	fn handle_error(&mut self, _error: &Error) {}
}
