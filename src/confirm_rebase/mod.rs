use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::view::View;

pub struct ConfirmRebase {}

impl ProcessModule for ConfirmRebase {
	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_git_interactive: &mut GitInteractive,
		_view: &View<'_>,
	) -> HandleInputResult
	{
		let input = input_handler.get_input(InputMode::Confirm);
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Yes => {
				result = result.exit_status(ExitStatus::Good).state(State::Exiting);
			},
			Input::No => {
				result = result.state(State::List(false));
			},
			_ => {},
		}
		result.build()
	}

	fn render(&self, view: &View<'_>, _git_interactive: &GitInteractive) {
		view.draw_confirm("Are you sure you want to rebase");
	}
}

impl ConfirmRebase {
	pub(crate) const fn new() -> Self {
		Self {}
	}
}
