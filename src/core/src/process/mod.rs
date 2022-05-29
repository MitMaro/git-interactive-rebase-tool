mod artifact;
mod results;
#[cfg(test)]
mod tests;

use std::{process::Command, thread};

use anyhow::{anyhow, Error, Result};
pub(crate) use artifact::Artifact;
use display::{CrossTerm, Tui};
use input::{spawn_event_thread, Sender as EventSender, StandardEvent};
pub(crate) use results::Results;
use todo_file::TodoFile;
use view::{spawn_view_thread, RenderContext, View, ViewSender};

use crate::{
	events::{Event, MetaEvent},
	module::{ExitStatus, ModuleHandler, State},
};

pub(crate) struct Process {
	exit_status: ExitStatus,
	rebase_todo: TodoFile,
	render_context: RenderContext,
	state: State,
	threads: Vec<thread::JoinHandle<()>>,
	view_sender: ViewSender,
	event_sender: EventSender<MetaEvent>,
}

impl Process {
	pub(crate) fn new<C: Tui + Send + 'static>(rebase_todo: TodoFile, view: View<C>) -> Self {
		#[allow(deprecated)]
		let view_size = view.get_view_size();
		let mut threads = vec![];

		let (view_sender, view_thread) = spawn_view_thread(view);
		threads.push(view_thread);
		let (event_sender, event_thread) = spawn_event_thread(CrossTerm::read_event);
		threads.push(event_thread);

		Self {
			exit_status: ExitStatus::None,
			rebase_todo,
			render_context: RenderContext::new(view_size.width() as u16, view_size.height() as u16),
			state: State::List,
			threads,
			view_sender,
			event_sender,
		}
	}

	pub(crate) fn run<ModuleProvider: crate::module::ModuleProvider>(
		&mut self,
		mut module_handler: ModuleHandler<ModuleProvider>,
	) -> Result<ExitStatus> {
		if self.view_sender.start().is_err() {
			self.exit_status = ExitStatus::StateError;
			return Ok(ExitStatus::StateError);
		}

		let activate_results = self.activate(&mut module_handler, State::List);
		self.handle_results(&mut module_handler, activate_results);
		while self.exit_status == ExitStatus::None {
			let view_data = module_handler.build_view_data(self.state, &self.render_context, &self.rebase_todo);
			if self.view_sender.render(view_data).is_err() {
				self.exit_status = ExitStatus::StateError;
				continue;
			}
			loop {
				if self.view_sender.is_poisoned() {
					self.exit_status = ExitStatus::StateError;
					break;
				}
				let results_maybe = module_handler.handle_event(
					self.state,
					&mut self.event_sender,
					&self.view_sender,
					&mut self.rebase_todo,
				);

				if self.exit_status != ExitStatus::None {
					break;
				}

				if let Some(results) = results_maybe {
					self.handle_results(&mut module_handler, results);
					break;
				}
			}
		}
		if self.view_sender.stop().is_err() {
			return Ok(ExitStatus::StateError);
		}
		if self.exit_status != ExitStatus::Kill {
			self.rebase_todo.write_file()?;
		}

		let (view_end_result, event_end_result) = (self.view_sender.end(), self.event_sender.end());

		if view_end_result.is_err() || event_end_result.is_err() {
			return Ok(ExitStatus::StateError);
		}

		while let Some(handle) = self.threads.pop() {
			if handle.join().is_err() {
				return Ok(ExitStatus::StateError);
			}
		}

		Ok(self.exit_status)
	}

	fn handle_results<ModuleProvider: crate::module::ModuleProvider>(
		&mut self,
		modules: &mut ModuleHandler<ModuleProvider>,
		mut results: Results,
	) {
		while let Some(artifact) = results.artifact() {
			results.append(match artifact {
				Artifact::Event(event) => self.handle_event_artifact(event),
				Artifact::ChangeState(state) => self.handle_state(state, modules),
				Artifact::Error(err, previous_state) => self.handle_error(&err, previous_state, modules),
				Artifact::ExitStatus(exit_status) => self.handle_exit_status(exit_status),
				Artifact::ExternalCommand(command) => self.handle_external_command(&command),
				Artifact::EnqueueResize => self.handle_enqueue_resize(),
			});
		}
	}

	fn handle_event_artifact(&mut self, event: Event) -> Results {
		let mut results = Results::new();
		match event {
			Event::Standard(StandardEvent::Exit) => {
				results.exit_status(ExitStatus::Abort);
			},
			Event::Standard(StandardEvent::Kill) => {
				results.exit_status(ExitStatus::Kill);
			},
			Event::Resize(width, height) => {
				self.view_sender.resize(width, height);
				self.render_context.update(width, height);

				if self.state != State::WindowSizeError && self.render_context.is_window_too_small() {
					results.state(State::WindowSizeError);
				}
			},
			_ => {},
		};
		results
	}

	fn handle_state<ModuleProvider: crate::module::ModuleProvider>(
		&mut self,
		state: State,
		modules: &mut ModuleHandler<ModuleProvider>,
	) -> Results {
		let mut results = Results::new();
		if self.state != state {
			let previous_state = self.state;
			self.state = state;
			results.append(modules.deactivate(previous_state));
			results.append(self.activate(modules, previous_state));
		}
		results
	}

	fn handle_error<ModuleProvider: crate::module::ModuleProvider>(
		&mut self,
		error: &Error,
		previous_state: Option<State>,
		modules: &mut ModuleHandler<ModuleProvider>,
	) -> Results {
		let mut results = Results::new();
		let return_state = previous_state.unwrap_or(self.state);
		self.state = State::Error;
		results.append(modules.error(self.state, error));
		results.append(self.activate(modules, return_state));
		results
	}

	fn handle_exit_status(&mut self, exit_status: ExitStatus) -> Results {
		self.exit_status = exit_status;
		Results::new()
	}

	fn handle_enqueue_resize(&mut self) -> Results {
		self.event_sender
			.enqueue_event(Event::Resize(
				self.render_context.width() as u16,
				self.render_context.height() as u16,
			))
			.expect("Enqueue Resize event failed");
		Results::new()
	}

	fn handle_external_command(&mut self, external_command: &(String, Vec<String>)) -> Results {
		let mut results = Results::new();
		match self.run_command(external_command) {
			Ok(meta_event) => {
				self.event_sender
					.enqueue_event(Event::from(meta_event))
					.expect("Enqueue event failed");
			},
			Err(err) => {
				results.error_with_return(
					err.context(format!(
						"Unable to run {} {}",
						external_command.0,
						external_command.1.join(" ")
					)),
					State::List,
				);
			},
		}
		results
	}

	fn run_command(&mut self, external_command: &(String, Vec<String>)) -> Result<MetaEvent> {
		self.view_sender.stop()?;
		self.event_sender.pause();

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

		self.event_sender.resume();
		self.view_sender.start()?;

		result
	}

	fn activate<ModuleProvider: crate::module::ModuleProvider>(
		&mut self,
		modules: &mut ModuleHandler<ModuleProvider>,
		previous_state: State,
	) -> Results {
		let mut results = modules.activate(self.state, &self.rebase_todo, previous_state);
		// always trigger a resize on activate, for modules that track size
		results.enqueue_resize();
		results
	}
}
