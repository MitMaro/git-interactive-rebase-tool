use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
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
	) -> ProcessResult
	{
		let input = input_handler.get_input(InputMode::Confirm);
		let mut result = ProcessResult::new().input(input);
		match input {
			Input::Yes => {
				git_interactive.clear();
				result = result.exit_status(ExitStatus::Good);
			},
			Input::No => {
				result = result.state(State::List);
			},
			_ => {},
		}
		result
	}
}

impl ConfirmAbort {
	pub(crate) fn new() -> Self {
		Self {
			view_data: ViewData::new_confirm("Are you sure you want to abort"),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::assert_process_result;
	use crate::build_render_output;
	use crate::config::Config;
	use crate::confirm_abort::ConfirmAbort;
	use crate::display::Display;
	use crate::git_interactive::GitInteractive;
	use crate::input::input_handler::InputHandler;
	use crate::input::Input;
	use crate::process::exit_status::ExitStatus;
	use crate::process::process_module::ProcessModule;
	use crate::process::state::State;
	use crate::process_module_handle_input_test;
	use crate::process_module_test;
	use crate::view::View;

	process_module_test!(
		confirm_abort_build_view_data,
		["pick aaa comment"],
		build_render_output!("{TITLE}", "{PROMPT}", "Are you sure you want to abort"),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(ConfirmAbort::new()) }
	);

	process_module_handle_input_test!(
		confirm_abort_handle_input_yes,
		["pick aaa comment"],
		[Input::Yes],
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut confirm_abort = ConfirmAbort::new();
			let result = confirm_abort.handle_input(input_handler, git_interactive, view);
			assert_process_result!(result, input = Input::Yes, exit_status = ExitStatus::Good);
			assert_eq!(git_interactive.get_lines().len(), 0);
		}
	);

	process_module_handle_input_test!(
		confirm_abort_handle_input_no,
		["pick aaa comment"],
		[Input::No],
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut confirm_abort = ConfirmAbort::new();
			let result = confirm_abort.handle_input(input_handler, git_interactive, view);
			assert_process_result!(result, input = Input::No, state = State::List);
		}
	);

	process_module_handle_input_test!(
		confirm_abort_handle_input_any_key,
		["pick aaa comment"],
		[Input::Character('x')],
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut confirm_abort = ConfirmAbort::new();
			let result = confirm_abort.handle_input(input_handler, git_interactive, view);
			assert_process_result!(result, input = Input::No, state = State::List);
		}
	);

	process_module_handle_input_test!(
		confirm_abort_handle_input_resize,
		["pick aaa comment"],
		[Input::Resize],
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut confirm_abort = ConfirmAbort::new();
			let result = confirm_abort.handle_input(input_handler, git_interactive, view);
			assert_process_result!(result, input = Input::Resize);
		}
	);
}
