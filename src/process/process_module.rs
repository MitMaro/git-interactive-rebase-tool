use crate::{
	process::{process_result::ProcessResult, state::State},
	todo_file::TodoFile,
	view::{view_data::ViewData, View},
};

pub trait ProcessModule {
	fn activate(&mut self, _rebase_todo: &TodoFile, _previous_state: State) -> ProcessResult {
		ProcessResult::new()
	}

	fn deactivate(&mut self) {}

	fn build_view_data(&mut self, _view: &View<'_>, _rebase_todo: &TodoFile) -> &ViewData;

	fn handle_input(&mut self, _view: &mut View<'_>, _rebase_todo: &mut TodoFile) -> ProcessResult;

	fn get_help_keybindings_descriptions(&self) -> Option<Vec<(Vec<String>, String)>> {
		None
	}
}
