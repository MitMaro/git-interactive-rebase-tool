pub mod error;
pub mod exit_status;
pub mod help;
pub mod modules;
pub mod process_module;
pub mod process_result;
pub mod state;
#[cfg(test)]
pub mod testutil;
pub mod util;
pub mod window_size_error;

use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::modules::Modules;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::process::window_size_error::WindowSizeError;
use crate::view::View;
use anyhow::Result;

pub struct Process<'r> {
	exit_status: Option<ExitStatus>,
	git_interactive: GitInteractive,
	input_handler: &'r InputHandler<'r>,
	state: State,
	view: &'r View<'r>,
}

impl<'r> Process<'r> {
	pub(crate) const fn new(
		git_interactive: GitInteractive,
		view: &'r View<'r>,
		input_handler: &'r InputHandler<'r>,
	) -> Self
	{
		Self {
			state: State::List,
			exit_status: None,
			git_interactive,
			input_handler,
			view,
		}
	}

	pub(crate) fn run(&mut self, mut modules: Modules<'_>) -> Result<Option<ExitStatus>> {
		let (view_width, view_height) = self.view.get_view_size();
		if WindowSizeError::is_window_too_small(view_width, view_height) {
			self.handle_process_result(&mut modules, &ProcessResult::new().state(State::WindowSizeError));
		}
		while self.exit_status.is_none() {
			let result = modules.process(self.state, &mut self.git_interactive);
			if self.handle_process_result(&mut modules, &result) {
				continue;
			}
			self.view
				.render(modules.build_view_data(self.state, self.view, &self.git_interactive));
			let result = modules.handle_input(self.state, self.input_handler, &mut self.git_interactive, self.view);
			self.handle_process_result(&mut modules, &result);
		}
		self.exit_end()?;
		Ok(self.exit_status)
	}

	fn handle_process_result(&mut self, modules: &mut Modules<'_>, result: &ProcessResult) -> bool {
		let previous_state = self.state;

		if let Some(exit_status) = result.exit_status {
			self.exit_status = Some(exit_status);
		}

		if let Some(ref error) = result.error {
			modules.set_error_message(error);
			self.state = State::Error;
			self.activate(modules, result.state.unwrap_or(previous_state));
		}
		else if let Some(new_state) = result.state {
			if new_state != self.state {
				modules.deactivate(self.state);
				self.state = new_state;
				self.activate(modules, previous_state);
			}
		}

		match result.input {
			Some(Input::Resize) => {
				let (view_width, view_height) = self.view.get_view_size();
				if self.state != State::WindowSizeError && WindowSizeError::is_window_too_small(view_width, view_height)
				{
					self.state = State::WindowSizeError;
					self.activate(modules, previous_state);
				}
			},
			Some(Input::Help) => {
				if previous_state != State::Help {
					self.state = State::Help;
					modules.update_help_data(previous_state);
					self.activate(modules, previous_state);
				}
			},
			_ => {},
		};

		previous_state != self.state
	}

	fn activate(&mut self, modules: &mut Modules<'_>, previous_state: State) {
		let result = modules.activate(self.state, &self.git_interactive, previous_state);
		self.handle_process_result(modules, &result);
	}

	fn exit_end(&mut self) -> Result<()> {
		let result = self.git_interactive.write_file();
		if result.is_err() {
			self.exit_status = Some(ExitStatus::FileWriteError);
		}
		result
	}
}
