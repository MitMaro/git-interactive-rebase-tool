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

use anyhow::Result;

use crate::{
	input::Input,
	process::{
		exit_status::ExitStatus,
		modules::Modules,
		process_result::ProcessResult,
		state::State,
		window_size_error::WindowSizeError,
	},
	todo_file::TodoFile,
	view::View,
};

pub struct Process<'r> {
	exit_status: Option<ExitStatus>,
	rebase_todo: TodoFile,
	state: State,
	view: View<'r>,
}

impl<'r> Process<'r> {
	pub(crate) const fn new(rebase_todo: TodoFile, view: View<'r>) -> Self {
		Self {
			exit_status: None,
			rebase_todo,
			state: State::List,
			view,
		}
	}

	pub(crate) fn run(&mut self, mut modules: Modules<'_>) -> Result<Option<ExitStatus>> {
		if self.view.start().is_err() {
			return Ok(Some(ExitStatus::StateError));
		}
		let view_width = self.view.get_view_size().width();
		let view_height = self.view.get_view_size().height();
		if WindowSizeError::is_window_too_small(view_width, view_height) {
			self.handle_process_result(&mut modules, &ProcessResult::new().state(State::WindowSizeError));
		}
		self.activate(&mut modules, State::List);
		while self.exit_status.is_none() {
			if self
				.view
				.render(modules.build_view_data(self.state, &self.view, &self.rebase_todo))
				.is_err()
			{
				self.exit_status = Some(ExitStatus::StateError);
				continue;
			}
			let result = modules.handle_input(self.state, &mut self.view, &mut self.rebase_todo);
			self.handle_process_result(&mut modules, &result);
		}
		if self.view.end().is_err() {
			return Ok(Some(ExitStatus::StateError));
		}
		if let Some(status) = self.exit_status {
			if status != ExitStatus::Kill {
				self.rebase_todo.write_file()?;
			}
		}
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
			Some(Input::Kill) => {
				self.exit_status = Some(ExitStatus::Kill);
			},
			Some(Input::Resize) => {
				let view_width = self.view.get_view_size().width();
				let view_height = self.view.get_view_size().height();
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
