mod artifact;
mod results;
#[cfg(test)]
mod tests;
pub(crate) mod thread;

use std::{
	io::ErrorKind,
	process::Command,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};

use anyhow::{anyhow, Error, Result};
use display::Size;
use input::StandardEvent;
use parking_lot::Mutex;
use runtime::ThreadStatuses;
use todo_file::TodoFile;
use view::RenderContext;

pub(crate) use self::{artifact::Artifact, results::Results, thread::Thread};
use crate::{
	events,
	events::{Event, MetaEvent},
	module,
	module::{ExitStatus, ModuleHandler, State},
};

pub(crate) struct Process<ModuleProvider: module::ModuleProvider> {
	ended: Arc<AtomicBool>,
	exit_status: Arc<Mutex<ExitStatus>>,
	input_state: events::State,
	module_handler: Arc<Mutex<ModuleHandler<ModuleProvider>>>,
	paused: Arc<AtomicBool>,
	rebase_todo: Arc<Mutex<TodoFile>>,
	render_context: Arc<Mutex<RenderContext>>,
	state: Arc<Mutex<State>>,
	thread_statuses: ThreadStatuses,
	view_state: view::State,
}

impl<ModuleProvider: module::ModuleProvider> Clone for Process<ModuleProvider> {
	fn clone(&self) -> Self {
		Self {
			ended: Arc::clone(&self.ended),
			exit_status: Arc::clone(&self.exit_status),
			input_state: self.input_state.clone(),
			module_handler: Arc::clone(&self.module_handler),
			paused: Arc::clone(&self.paused),
			rebase_todo: Arc::clone(&self.rebase_todo),
			render_context: Arc::clone(&self.render_context),
			state: Arc::clone(&self.state),
			thread_statuses: self.thread_statuses.clone(),
			view_state: self.view_state.clone(),
		}
	}
}

impl<ModuleProvider: module::ModuleProvider> Process<ModuleProvider> {
	pub(crate) fn new(
		initial_display_size: Size,
		rebase_todo: TodoFile,
		module_handler: ModuleHandler<ModuleProvider>,
		input_state: events::State,
		view_state: view::State,
		thread_statuses: ThreadStatuses,
	) -> Self {
		Self {
			ended: Arc::new(AtomicBool::from(false)),
			exit_status: Arc::new(Mutex::new(ExitStatus::None)),
			input_state,
			module_handler: Arc::new(Mutex::new(module_handler)),
			paused: Arc::new(AtomicBool::from(false)),
			rebase_todo: Arc::new(Mutex::new(rebase_todo)),
			render_context: Arc::new(Mutex::new(RenderContext::new(
				initial_display_size.width(),
				initial_display_size.height(),
			))),
			state: Arc::new(Mutex::new(State::WindowSizeError)),
			thread_statuses,
			view_state,
		}
	}

	pub(crate) fn is_ended(&self) -> bool {
		self.ended.load(Ordering::Acquire)
	}

	/// Permanently End the event read thread.
	pub(crate) fn end(&self) {
		self.ended.store(true, Ordering::Release);
	}

	pub(crate) fn state(&self) -> State {
		*self.state.lock()
	}

	pub(crate) fn set_state(&self, state: State) {
		*self.state.lock() = state;
	}

	pub(crate) fn exit_status(&self) -> ExitStatus {
		*self.exit_status.lock()
	}

	pub(crate) fn set_exit_status(&self, exit_status: ExitStatus) {
		*self.exit_status.lock() = exit_status;
	}

	pub(crate) fn should_exit(&self) -> bool {
		self.exit_status() != ExitStatus::None || self.is_ended()
	}

	pub(crate) fn is_exit_status_kill(&self) -> bool {
		self.exit_status() == ExitStatus::Kill
	}

	fn activate(&self, previous_state: State) -> Results {
		let mut module_handler = self.module_handler.lock();
		let mut results = module_handler.activate(self.state(), &self.rebase_todo.lock(), previous_state);
		// always trigger a resize on activate, for modules that track size
		results.enqueue_resize();
		results
	}

	pub(crate) fn render(&self) {
		let rebase_todo = self.rebase_todo.lock();
		let render_context = *self.render_context.lock();
		let mut module_handler = self.module_handler.lock();
		let view_data = module_handler.build_view_data(self.state(), &render_context, &rebase_todo);
		// TODO It is not possible for this to fail. view::State should be updated to not return an error
		self.view_state.render(view_data).expect("Render failed");
	}

	pub(crate) fn write_todo_file(&self) -> Result<()> {
		self.rebase_todo.lock().write_file()
	}

	fn deactivate(&self, state: State) -> Results {
		let mut module_handler = self.module_handler.lock();
		module_handler.deactivate(state)
	}

	pub(crate) fn handle_event(&self) -> Option<Results> {
		self.module_handler.lock().handle_event(
			self.state(),
			&self.input_state.clone(),
			&self.view_state.clone(),
			&mut self.rebase_todo.lock(),
		)
	}

	fn handle_event_artifact(&self, event: Event) -> Results {
		let mut results = Results::new();
		match event {
			Event::Standard(StandardEvent::Exit) => {
				results.exit_status(ExitStatus::Abort);
			},
			Event::Standard(StandardEvent::Kill) => {
				results.exit_status(ExitStatus::Kill);
			},
			Event::Resize(width, height) => {
				self.view_state.resize(width, height);

				let mut render_context = self.render_context.lock();
				render_context.update(width, height);
				if self.state() != State::WindowSizeError && render_context.is_window_too_small() {
					results.state(State::WindowSizeError);
				}
			},
			_ => {},
		};
		results
	}

	fn handle_state(&self, state: State) -> Results {
		let mut results = Results::new();
		let previous_state = self.state();
		if previous_state != state {
			self.set_state(state);
			results.append(self.deactivate(previous_state));
			results.append(self.activate(previous_state));
		}
		results
	}

	fn handle_error(&self, error: &Error, previous_state: Option<State>) -> Results {
		let mut results = Results::new();
		let return_state = previous_state.unwrap_or_else(|| self.state());
		self.set_state(State::Error);
		results.append(self.activate(return_state));
		let mut module_handler = self.module_handler.lock();
		results.append(module_handler.error(State::Error, error));
		results
	}

	fn handle_exit_status(&self, exit_status: ExitStatus) -> Results {
		self.set_exit_status(exit_status);
		Results::new()
	}

	fn handle_enqueue_resize(&self) -> Results {
		let render_context = self.render_context.lock();
		self.input_state.enqueue_event(Event::Resize(
			render_context.width() as u16,
			render_context.height() as u16,
		));
		Results::new()
	}

	fn handle_external_command(&self, external_command: &(String, Vec<String>)) -> Results {
		let mut results = Results::new();

		match self.run_command(external_command) {
			Ok(meta_event) => {
				self.input_state.enqueue_event(Event::from(meta_event));
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

	fn run_command(&self, external_command: &(String, Vec<String>)) -> Result<MetaEvent> {
		self.view_state.stop()?;
		self.input_state.pause();

		self.thread_statuses
			.wait_for_status(view::REFRESH_THREAD_NAME, &runtime::Status::Waiting)?;
		self.thread_statuses
			.wait_for_status(input::THREAD_NAME, &runtime::Status::Waiting)?;

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
			.map_err(|err| {
				match err.kind() {
					ErrorKind::NotFound => {
						anyhow!("File does not exist: {}", external_command.0)
					},
					ErrorKind::PermissionDenied => {
						anyhow!("File not executable: {}", external_command.0)
					},
					_ => Error::from(err),
				}
			});

		self.input_state.resume();
		self.view_state.start()?;
		result
	}

	fn handle_results(&self, mut results: Results) {
		while let Some(artifact) = results.artifact() {
			results.append(match artifact {
				Artifact::ChangeState(state) => self.handle_state(state),
				Artifact::EnqueueResize => self.handle_enqueue_resize(),
				Artifact::Error(err, previous_state) => self.handle_error(&err, previous_state),
				Artifact::Event(event) => self.handle_event_artifact(event),
				Artifact::ExitStatus(exit_status) => self.handle_exit_status(exit_status),
				Artifact::ExternalCommand(command) => self.handle_external_command(&command),
			});
		}
	}
}
