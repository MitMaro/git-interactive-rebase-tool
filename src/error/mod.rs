use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::view::view_data::ViewData;
use crate::view::View;

pub struct Error {
	return_state: State,
	view_data: Option<ViewData>,
	view_data_no_error: ViewData,
}

impl ProcessModule for Error {
	fn activate(&mut self, state: &State, _git_interactive: &GitInteractive) {
		if let State::Error { message, return_state } = state {
			self.view_data = Some(ViewData::new_error(message.as_str()));
			self.return_state = *return_state.clone();
		}
		else {
			self.view_data = None;
		}
	}

	fn deactivate(&mut self) {
		self.view_data = None;
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();
		if let Some(ref mut view_data) = self.view_data {
			view_data.set_view_size(view_width, view_height);
			view_data.rebuild();
			view_data
		}
		else {
			self.view_data_no_error.set_view_size(view_width, view_height);
			self.view_data_no_error.rebuild();
			&self.view_data_no_error
		}
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
		if let Input::Resize = input {
		}
		else {
			result = result.state(self.return_state.clone());
		}
		result.build()
	}
}

impl Error {
	pub(crate) fn new() -> Self {
		Self {
			return_state: State::List(false),
			view_data: None,
			view_data_no_error: ViewData::new_error("Help module activated without error message"),
		}
	}
}
