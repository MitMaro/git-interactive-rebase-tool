pub mod error;
pub mod exit_status;
pub mod help;
pub mod modules;
pub mod process_module;
pub mod process_result;
pub mod state;
pub mod util;
pub mod window_size_error;

#[cfg(test)]
mod tests;
#[cfg(test)]
pub mod testutil;

use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::todo_file::TodoFile;
use crate::view::View;
use anyhow::Result;
use exit_status::ExitStatus;
use modules::Modules;
use process_result::ProcessResult;
use state::State;
use window_size_error::WindowSizeError;

pub struct Process<'r> {
	exit_status: Option<ExitStatus>,
	rebase_todo: TodoFile,
	input_handler: &'r InputHandler<'r>,
	state: State,
	view: &'r View<'r>,
}

impl<'r> Process<'r> {
	pub(crate) const fn new(rebase_todo: TodoFile, view: &'r View<'r>, input_handler: &'r InputHandler<'r>) -> Self {
		Self {
			state: State::List,
			exit_status: None,
			rebase_todo,
			input_handler,
			view,
		}
	}

	pub(crate) fn run(&mut self, mut modules: Modules<'_>) -> Result<Option<ExitStatus>> {
		let (view_width, view_height) = self.view.get_view_size();
		if WindowSizeError::is_window_too_small(view_width, view_height) {
			self.handle_process_result(&mut modules, &ProcessResult::new().state(State::WindowSizeError));
		}
		self.activate(&mut modules, State::List);
		while self.exit_status.is_none() {
			self.view
				.render(modules.build_view_data(self.state, self.view, &self.rebase_todo));
			let result = modules.handle_input(self.state, self.input_handler, &mut self.rebase_todo, self.view);
			self.handle_process_result(&mut modules, &result);
		}
		self.rebase_todo.write_file()?;
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
			Some(Input::Exit) => {
				self.exit_status = Some(ExitStatus::Abort);
			},
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
		let result = modules.activate(self.state, &self.rebase_todo, previous_state);
		self.handle_process_result(modules, &result);
	}
}
