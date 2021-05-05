pub mod error;
pub mod exit_status;
pub mod modules;
pub mod process_module;
pub mod process_result;
pub mod state;
pub mod window_size_error;

#[cfg(test)]
mod tests;
#[cfg(test)]
pub mod testutil;

use std::process::Command;

use anyhow::{anyhow, Result};

use crate::{
	input::{Event, EventHandler, MetaEvent},
	process::{exit_status::ExitStatus, modules::Modules, process_result::ProcessResult, state::State},
	todo_file::TodoFile,
	view::{render_context::RenderContext, View},
};

pub struct Process<'r> {
	exit_status: Option<ExitStatus>,
	event_handler: EventHandler,
	rebase_todo: TodoFile,
	render_context: RenderContext,
	state: State,
	view: View<'r>,
}

impl<'r> Process<'r> {
	pub(crate) fn new(rebase_todo: TodoFile, event_handler: EventHandler, view: View<'r>) -> Self {
		let view_size = view.get_view_size();
		Self {
			render_context: RenderContext::new(view_size.width() as u16, view_size.height() as u16),
			exit_status: None,
			event_handler,
			rebase_todo,
			state: State::List,
			view,
		}
	}

	pub(crate) fn run(&'r mut self, mut modules: Modules<'r>) -> Result<Option<ExitStatus>> {
		if self.view.start().is_err() {
			return Ok(Some(ExitStatus::StateError));
		}
		self.handle_process_result(
			&mut modules,
			&ProcessResult::new().event(Event::Resize(
				self.render_context.width() as u16,
				self.render_context.height() as u16,
			)),
		);
		self.activate(&mut modules, State::List);
		while self.exit_status.is_none() {
			let view_data = modules.build_view_data(self.state, &self.render_context, &self.rebase_todo);
			if self.view.render(view_data).is_err() {
				self.exit_status = Some(ExitStatus::StateError);
				continue;
			}

			loop {
				let result = modules.handle_input(self.state, &self.event_handler, &mut self.rebase_todo);

				if let Some(event) = result.event {
					if event != Event::None {
						self.handle_process_result(&mut modules, &result);
						break;
					}
				}
			}
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

	fn handle_process_result(&mut self, modules: &mut Modules<'r>, result: &ProcessResult) {
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

		match result.event {
			Some(Event::Meta(MetaEvent::Exit)) => {
				self.exit_status = Some(ExitStatus::Abort);
			},
			Some(Event::Meta(MetaEvent::Kill)) => {
				self.exit_status = Some(ExitStatus::Kill);
			},
			Some(Event::Resize(width, height)) => {
				self.render_context.update(width, height);
				if self.state != State::WindowSizeError && self.render_context.is_window_too_small() {
					self.state = State::WindowSizeError;
					self.activate(modules, previous_state);
				}
			},
			_ => {},
		};

		if let Some(ref external_command) = result.external_command {
			match self.run_command(external_command) {
				Ok(meta_event) => self.event_handler.push_event(Event::from(meta_event)),
				Err(err) => {
					self.handle_process_result(
						modules,
						&ProcessResult::new().error(err.context(format!(
							"Unable to run {} {}",
							external_command.0,
							external_command.1.join(" ")
						))),
					);
				},
			}
		}
	}

	fn run_command(&mut self, external_command: &(String, Vec<String>)) -> Result<MetaEvent> {
		self.view.end()?;

		let mut cmd = Command::new(external_command.0.clone());
		cmd.args(external_command.1.clone());

		let result = cmd
			.status()
			.map(|status| {
				if status.success() {
					MetaEvent::ExternalCommandSuccess
				}
				else {
					MetaEvent::ExternalCommandError
				}
			})
			.map_err(|err| anyhow!(err));

		self.view.start()?;

		result
	}

	fn activate(&mut self, modules: &mut Modules<'r>, previous_state: State) {
		let result = modules.activate(self.state, &self.rebase_todo, previous_state);
		// always trigger a resize on activate, for modules that track size
		self.event_handler.push_event(Event::Resize(
			self.render_context.width() as u16,
			self.render_context.height() as u16,
		));
		self.handle_process_result(modules, &result);
	}
}
