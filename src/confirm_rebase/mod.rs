use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::view::view_data::ViewData;
use crate::view::View;

pub struct ConfirmRebase {
	view_data: ViewData,
}

impl ProcessModule for ConfirmRebase {
	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (window_width, window_height) = view.get_view_size();
		self.view_data.set_view_size(window_width, window_height);
		self.view_data.rebuild();
		&self.view_data
	}

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
				result = result.state(State::List);
			},
			_ => {},
		}
		result.build()
	}
}

impl ConfirmRebase {
	pub(crate) fn new() -> Self {
		Self {
			view_data: ViewData::new_confirm("Are you sure you want to rebase"),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::assert_handle_input_result;
	use crate::build_render_output;
	use crate::config::Config;
	use crate::confirm_rebase::ConfirmRebase;
	use crate::display::curses::Input as CursesInput;
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
		confirm_rebase_build_view_data,
		vec!["pick aaa comment"],
		build_render_output!("{TITLE}", "{PROMPT}", "Are you sure you want to rebase"),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(ConfirmRebase::new()) }
	);

	process_module_handle_input_test!(
		confirm_rebase_handle_input_yes,
		vec!["pick aaa comment"],
		CursesInput::Character('y'),
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut confirm_rebase = ConfirmRebase::new();
			let result = confirm_rebase.handle_input(input_handler, git_interactive, view);

			assert_handle_input_result!(
				result,
				input = Input::Yes,
				state = State::Exiting,
				exit_status = ExitStatus::Good
			);
			assert_eq!(git_interactive.get_lines().len(), 1);
		}
	);

	process_module_handle_input_test!(
		confirm_rebase_handle_input_no,
		vec!["pick aaa comment"],
		CursesInput::Character('n'),
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut confirm_rebase = ConfirmRebase::new();
			let result = confirm_rebase.handle_input(input_handler, git_interactive, view);
			assert_handle_input_result!(result, input = Input::No, state = State::List);
			assert_eq!(git_interactive.get_lines().len(), 1);
		}
	);

	process_module_handle_input_test!(
		confirm_rebase_handle_input_any_key,
		vec!["pick aaa comment"],
		CursesInput::Character('x'),
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut confirm_rebase = ConfirmRebase::new();
			let result = confirm_rebase.handle_input(input_handler, git_interactive, view);
			assert_handle_input_result!(result, input = Input::No, state = State::List);
		}
	);

	process_module_handle_input_test!(
		confirm_rebase_handle_input_resize,
		vec!["pick aaa comment"],
		CursesInput::KeyResize,
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut confirm_rebase = ConfirmRebase::new();
			let result = confirm_rebase.handle_input(input_handler, git_interactive, view);
			assert_handle_input_result!(result, input = Input::Resize);
		}
	);
}
