use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::process::handle_input_result::HandleInputResult;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::view::view_data::ViewData;
use crate::view::View;

pub trait ProcessModule {
	fn activate(&mut self, _state: State, _git_interactive: &GitInteractive) {}

	fn deactivate(&mut self) {}

	fn build_view_data(&mut self, _view: &View<'_>, _git_interactive: &GitInteractive) -> &ViewData;

	fn process(&mut self, _git_interactive: &mut GitInteractive, _view: &View<'_>) -> ProcessResult {
		ProcessResult::new()
	}

	fn handle_input(
		&mut self,
		_input_handler: &InputHandler<'_>,
		_git_interactive: &mut GitInteractive,
		_view: &View<'_>,
	) -> HandleInputResult
	{
		HandleInputResult::new(Input::Other)
	}
}
