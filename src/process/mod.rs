mod error;
pub mod exit_status;
mod help;
pub mod process_module;
pub mod process_result;
pub mod state;
#[cfg(test)]
pub mod testutil;

use crate::config::Config;
use crate::confirm_abort::ConfirmAbort;
use crate::confirm_rebase::ConfirmRebase;
use crate::constants::{MINIMUM_COMPACT_WINDOW_WIDTH, MINIMUM_WINDOW_HEIGHT};
use crate::display::Display;
use crate::edit::Edit;
use crate::exiting::Exiting;
use crate::external_editor::ExternalEditor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::list::List;
use crate::process::error::Error;
use crate::process::exit_status::ExitStatus;
use crate::process::help::Help;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::show_commit::ShowCommit;
use crate::view::View;
use crate::window_size_error::WindowSizeError;

pub struct Process<'r> {
	confirm_abort: ConfirmAbort,
	confirm_rebase: ConfirmRebase,
	edit: Edit,
	error: Option<Error>,
	exit_status: Option<ExitStatus>,
	exiting: Exiting,
	external_editor: ExternalEditor<'r>,
	git_interactive: GitInteractive,
	help: Option<Help>,
	input_handler: &'r InputHandler<'r>,
	list: List<'r>,
	show_commit: ShowCommit<'r>,
	state: State,
	view: &'r View<'r>,
	window_size_error: WindowSizeError,
}

impl<'r> Process<'r> {
	pub(crate) fn new(
		git_interactive: GitInteractive,
		view: &'r View<'r>,
		display: &'r Display<'r>,
		input_handler: &'r InputHandler<'r>,
		config: &'r Config,
	) -> Self
	{
		Self {
			confirm_abort: ConfirmAbort::new(),
			confirm_rebase: ConfirmRebase::new(),
			edit: Edit::new(),
			error: None,
			exit_status: None,
			exiting: Exiting::new(),
			external_editor: ExternalEditor::new(display, config.git.editor.as_str()),
			git_interactive,
			help: None,
			input_handler,
			list: List::new(config),
			show_commit: ShowCommit::new(config),
			state: State::List,
			view,
			window_size_error: WindowSizeError::new(),
		}
	}

	pub(crate) fn run(&mut self) -> Result<Option<ExitStatus>, String> {
		self.check_window_size();
		while self.exit_status.is_none() {
			if self.help.is_none() && self.error.is_none() {
				self.process();
			}
			self.render();
			self.handle_input();
		}
		self.exit_end()?;
		Ok(self.exit_status)
	}

	fn activate(&mut self) {
		match self.state {
			State::ConfirmAbort => self.confirm_abort.activate(&self.state, &self.git_interactive),
			State::ConfirmRebase => self.confirm_rebase.activate(&self.state, &self.git_interactive),
			State::Edit => self.edit.activate(&self.state, &self.git_interactive),
			State::Exiting => self.exiting.activate(&self.state, &self.git_interactive),
			State::ExternalEditor => self.external_editor.activate(&self.state, &self.git_interactive),
			State::List => self.list.activate(&self.state, &self.git_interactive),
			State::ShowCommit => self.show_commit.activate(&self.state, &self.git_interactive),
			State::WindowSizeError(_) => self.window_size_error.activate(&self.state, &self.git_interactive),
		}
	}

	fn deactivate(&mut self) {
		match self.state {
			State::ConfirmAbort => self.confirm_abort.deactivate(),
			State::ConfirmRebase => self.confirm_rebase.deactivate(),
			State::Edit => self.edit.deactivate(),
			State::Exiting => self.exiting.deactivate(),
			State::ExternalEditor => self.external_editor.deactivate(),
			State::List => self.list.deactivate(),
			State::ShowCommit => self.show_commit.deactivate(),
			State::WindowSizeError(_) => self.window_size_error.deactivate(),
		}
	}

	fn process(&mut self) {
		let result = match self.state {
			State::ConfirmAbort => self.confirm_abort.process(&mut self.git_interactive, self.view),
			State::ConfirmRebase => self.confirm_rebase.process(&mut self.git_interactive, self.view),
			State::Edit => self.edit.process(&mut self.git_interactive, self.view),
			State::Exiting => self.exiting.process(&mut self.git_interactive, self.view),
			State::ExternalEditor => self.external_editor.process(&mut self.git_interactive, self.view),
			State::List => self.list.process(&mut self.git_interactive, self.view),
			State::ShowCommit => self.show_commit.process(&mut self.git_interactive, self.view),
			State::WindowSizeError(_) => self.window_size_error.process(&mut self.git_interactive, self.view),
		};

		self.handle_process_result(result);
	}

	fn render(&mut self) {
		self.view.render(
			if let Some(ref mut help) = self.help {
				help.get_view_data(self.view)
			}
			else if let Some(ref mut error) = self.error {
				error.get_view_data(self.view)
			}
			else {
				match self.state {
					State::ConfirmAbort => self.confirm_abort.build_view_data(self.view, &self.git_interactive),
					State::ConfirmRebase => self.confirm_rebase.build_view_data(self.view, &self.git_interactive),
					State::Edit => self.edit.build_view_data(self.view, &self.git_interactive),
					State::Exiting => self.exiting.build_view_data(self.view, &self.git_interactive),
					State::ExternalEditor => self.external_editor.build_view_data(self.view, &self.git_interactive),
					State::List => self.list.build_view_data(self.view, &self.git_interactive),
					State::ShowCommit => self.show_commit.build_view_data(self.view, &self.git_interactive),
					State::WindowSizeError(_) => {
						self.window_size_error.build_view_data(self.view, &self.git_interactive)
					},
				}
			},
		);
	}

	fn handle_input(&mut self) {
		let result = if let Some(ref mut help) = self.help {
			help.handle_input(self.input_handler, self.view)
		}
		else if let Some(ref mut error) = self.error {
			error.handle_input(self.input_handler)
		}
		else {
			match self.state {
				State::ConfirmAbort => {
					self.confirm_abort
						.handle_input(self.input_handler, &mut self.git_interactive, self.view)
				},
				State::ConfirmRebase => {
					self.confirm_rebase
						.handle_input(self.input_handler, &mut self.git_interactive, self.view)
				},
				State::Edit => {
					self.edit
						.handle_input(self.input_handler, &mut self.git_interactive, self.view)
				},
				State::Exiting => {
					self.exiting
						.handle_input(self.input_handler, &mut self.git_interactive, self.view)
				},
				State::ExternalEditor => {
					self.external_editor
						.handle_input(self.input_handler, &mut self.git_interactive, self.view)
				},
				State::List => {
					self.list
						.handle_input(self.input_handler, &mut self.git_interactive, self.view)
				},
				State::ShowCommit => {
					self.show_commit
						.handle_input(self.input_handler, &mut self.git_interactive, self.view)
				},
				State::WindowSizeError(_) => {
					self.window_size_error
						.handle_input(self.input_handler, &mut self.git_interactive, self.view)
				},
			}
		};
		self.handle_process_result(result);
	}

	fn handle_process_result(&mut self, result: ProcessResult) {
		if let Some(exit_status) = result.exit_status {
			self.exit_status = Some(exit_status);
		}

		if let Some(error_message) = result.error_message {
			self.error = Some(Error::new(error_message.as_str()));
		}

		match result.input {
			Some(Input::Help) => self.toggle_help(),
			Some(Input::Resize) => self.check_window_size(),
			Some(_) => {
				if self.error.is_some() {
					self.error = None;
				}
			},
			None => {},
		};

		if let Some(new_state) = result.state {
			if new_state != self.state {
				self.deactivate();
				self.state = new_state;
				self.activate();
			}
		}
	}

	fn toggle_help(&mut self) {
		if self.help.is_some() {
			self.help = None;
		}
		else {
			self.help = match self.state {
				State::List => {
					Some(Help::new_from_view_data(
						self.list.get_help_keybindings_descriptions(),
						self.list.get_help_view(),
					))
				},
				State::ShowCommit => {
					Some(Help::new_from_view_data(
						self.show_commit.get_help_keybindings_descriptions(),
						self.show_commit.get_help_view(),
					))
				},
				State::ConfirmAbort
				| State::ConfirmRebase
				| State::Edit
				| State::Exiting
				| State::ExternalEditor
				| State::WindowSizeError(_) => None,
			};
		}
	}

	fn check_window_size(&mut self) {
		let (window_width, window_height) = self.view.get_view_size();
		let check = !(window_width <= MINIMUM_COMPACT_WINDOW_WIDTH || window_height <= MINIMUM_WINDOW_HEIGHT);
		if let State::WindowSizeError(ref return_state) = self.state {
			if check {
				self.state = *return_state.clone();
			}
		}
		else if !check {
			self.state = State::WindowSizeError(Box::new(self.state.clone()));
		}
	}

	fn exit_end(&mut self) -> Result<(), String> {
		match self.git_interactive.write_file() {
			Ok(_) => {},
			Err(msg) => {
				self.exit_status = Some(ExitStatus::FileWriteError);
				return Err(msg);
			},
		}
		Ok(())
	}
}
