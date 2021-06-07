mod exit_status;
mod modules;
mod process_result;
mod state;

#[cfg(test)]
pub mod testutil;

use anyhow::Error;
use input::EventHandler;
use todo_file::TodoFile;
use view::{RenderContext, ViewData, ViewSender};

pub use self::{exit_status::ExitStatus, modules::Modules, process_result::ProcessResult, state::State};

pub trait Module {
	fn activate(&mut self, _rebase_todo: &TodoFile, _previous_state: State) -> ProcessResult {
		ProcessResult::new()
	}

	fn deactivate(&mut self) {}

	fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData;

	fn handle_events(
		&mut self,
		_event_handler: &EventHandler,
		_view_sender: &ViewSender,
		_rebase_todo: &mut TodoFile,
	) -> ProcessResult;

	fn handle_error(&mut self, _error: &Error) {}
}
