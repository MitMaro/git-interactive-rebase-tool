use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::view::View;

pub(crate) struct Error {
	error_message: String,
	return_state: State,
}

impl ProcessModule for Error {
	fn activate(&mut self, state: State, _git_interactive: &GitInteractive) {
		if let State::Error { message, return_state } = state {
			self.error_message = message;
			self.return_state = *return_state;
		}
		else {
			panic!("Help module activated when not expected");
		}
	}

	fn deactivate(&mut self) {
		self.error_message.clear();
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_git_interactive: &mut GitInteractive,
		_view: &View<'_>,
	) -> HandleInputResult
	{
		let input = input_handler.get_input(InputMode::Default);
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Resize => {},
			_ => {
				result = result.state(self.return_state.clone());
			},
		}
		result.build()
	}

	fn render(&self, view: &View<'_>, _git_interactive: &GitInteractive) {
		view.draw_error(self.error_message.as_str());
	}
}

impl Error {
	pub(crate) fn new() -> Self {
		Self {
			error_message: String::from(""),
			return_state: State::List(false),
		}
	}
}
