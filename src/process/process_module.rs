use crate::input::input_handler::InputHandler;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::todo_file::TodoFile;
use crate::view::view_data::ViewData;
use crate::view::View;

pub trait ProcessModule {
	fn activate(&mut self, _rebase_todo: &TodoFile, _previous_state: State) -> ProcessResult {
		ProcessResult::new()
	}

	fn deactivate(&mut self) {}

	fn build_view_data(&mut self, _view: &View<'_>, _rebase_todo: &TodoFile) -> &ViewData;

	fn handle_input(
		&mut self,
		_input_handler: &InputHandler<'_>,
		_rebase_todo: &mut TodoFile,
		_view: &View<'_>,
	) -> ProcessResult;

	fn get_help_keybindings_descriptions(&self) -> Option<Vec<(String, String)>> {
		None
	}
}
