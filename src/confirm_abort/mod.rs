use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::view::view_data::ViewData;
use crate::view::View;

pub struct ConfirmAbort {
	view_data: ViewData,
}

impl ProcessModule for ConfirmAbort {
	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (window_width, window_height) = view.get_view_size();
		self.view_data.set_view_size(window_width, window_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		git_interactive: &mut GitInteractive,
		_view: &View<'_>,
	) -> HandleInputResult
	{
		let input = input_handler.get_input(InputMode::Confirm);
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Yes => {
				git_interactive.clear();
				result = result.exit_status(ExitStatus::Good).state(State::Exiting);
			},
			Input::No => {
				result = result.state(State::List(false));
			},
			_ => {},
		}
		result.build()
	}
}

impl ConfirmAbort {
	pub(crate) fn new() -> Self {
		Self {
			view_data: ViewData::new_confirm("Are you sure you want to abort"),
		}
	}
}
