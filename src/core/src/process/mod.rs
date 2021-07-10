#[cfg(test)]
mod tests;

use std::{process::Command, thread};

use anyhow::{anyhow, Result};
use display::Tui;
use input::{Event, EventHandler, MetaEvent};
use todo_file::TodoFile;
use view::{spawn_view_thread, RenderContext, View, ViewSender};

use crate::module::{ExitStatus, Modules, ProcessResult, State};

pub(crate) struct Process {
	event_handler: EventHandler,
	exit_status: Option<ExitStatus>,
	rebase_todo: TodoFile,
	render_context: RenderContext,
	state: State,
	threads: Vec<thread::JoinHandle<()>>,
	view_sender: ViewSender,
}

impl Process {
	pub(crate) fn new<C: Tui + Send + 'static>(
		rebase_todo: TodoFile,
		event_handler: EventHandler,
		view: View<C>,
	) -> Self {
		#[allow(deprecated)]
		let view_size = view.get_view_size();
		let mut threads = vec![];

		let (view_sender, view_thread) = spawn_view_thread(view);
		threads.push(view_thread);

		Self {
			event_handler,
			exit_status: None,
			rebase_todo,
			render_context: RenderContext::new(view_size.width() as u16, view_size.height() as u16),
			state: State::List,
			threads,
			view_sender,
		}
	}

	pub(crate) fn run(&mut self, mut modules: Modules<'_>) -> Result<ExitStatus> {
		if self.view_sender.start().is_err() {
			self.exit_status = Some(ExitStatus::StateError);
			return Ok(ExitStatus::StateError);
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
			if self.view_sender.render(view_data).is_err() {
				self.exit_status = Some(ExitStatus::StateError);
				continue;
			}
			loop {
				if self.view_sender.is_poisoned() {
					self.exit_status = Some(ExitStatus::StateError);
					break;
				}
				let result = modules.handle_event(
					self.state,
					&self.event_handler,
					&self.view_sender,
					&mut self.rebase_todo,
				);

				if let Some(event) = result.event {
					if event != Event::None {
						self.handle_process_result(&mut modules, &result);
						break;
					}
				}
			}
		}
		if self.view_sender.stop().is_err() {
			return Ok(ExitStatus::StateError);
		}
		if let Some(status) = self.exit_status {
			if status != ExitStatus::Kill {
				self.rebase_todo.write_file()?;
			}
		}

		if self.view_sender.end().is_err() {
			return Ok(ExitStatus::StateError);
		}

		while let Some(handle) = self.threads.pop() {
			if handle.join().is_err() {
				return Ok(ExitStatus::StateError);
			}
		}

		Ok(self.exit_status.unwrap_or(ExitStatus::Good))
	}

	fn handle_process_result(&mut self, modules: &mut Modules<'_>, result: &ProcessResult) {
		let previous_state = self.state;

		// render context and view_sender need a size update early
		if let Some(Event::Resize(width, height)) = result.event {
			self.view_sender.resize(width, height);
			self.render_context.update(width, height);
		}

		if let Some(exit_status) = result.exit_status {
			self.exit_status = Some(exit_status);
		}

		if let Some(ref error) = result.error {
			self.state = State::Error;
			modules.error(self.state, error);
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
			Some(Event::Resize(..)) => {
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
						&ProcessResult::new().state(State::List).error(err.context(format!(
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
		self.view_sender.stop()?;

		let mut cmd = Command::new(external_command.0.clone());
		let _ = cmd.args(external_command.1.clone());

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

		self.view_sender.start()?;

		result
	}

	fn activate(&mut self, modules: &mut Modules<'_>, previous_state: State) {
		let result = modules.activate(self.state, &self.rebase_todo, previous_state);
		// always trigger a resize on activate, for modules that track size
		self.event_handler.push_event(Event::Resize(
			self.render_context.width() as u16,
			self.render_context.height() as u16,
		));
		self.handle_process_result(modules, &result);
	}
}
