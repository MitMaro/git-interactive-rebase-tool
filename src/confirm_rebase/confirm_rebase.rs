use crate::git_interactive::GitInteractive;
use crate::input::{Input, InputHandler};
use crate::process::{ExitStatus, HandleInputResult, HandleInputResultBuilder, ProcessModule, State};
use crate::view::View;

pub struct ConfirmRebase {}

impl ProcessModule for ConfirmRebase {
	fn handle_input(
		&mut self,
		input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
	) -> HandleInputResult
	{
		let input = input_handler.get_confirm();
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

	fn render(&self, view: &View, _git_interactive: &GitInteractive) {
		view.draw_confirm("Are you sure you want to rebase");
	}
}

impl ConfirmRebase {
	pub fn new() -> Self {
		Self {}
	}
}
