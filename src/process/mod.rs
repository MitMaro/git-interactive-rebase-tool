pub mod exit_status;
pub mod handle_input_result;
pub mod process_module;
pub mod process_result;
pub mod state;

use crate::config::Config;
use crate::confirm_abort::ConfirmAbort;
use crate::confirm_rebase::ConfirmRebase;
use crate::display::Display;
use crate::edit::Edit;
use crate::error::Error;
use crate::exiting::Exiting;
use crate::external_editor::ExternalEditor;
use crate::git_interactive::GitInteractive;
use crate::help::Help;
use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::list::List;
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::show_commit::ShowCommit;
use crate::view::View;
use crate::window_size_error::WindowSizeError;
use std::cell::RefCell;

pub struct Process<'r> {
	confirm_abort: ConfirmAbort,
	confirm_rebase: ConfirmRebase,
	edit: Edit,
	error: Error,
	exit_status: Option<ExitStatus>,
	exiting: Exiting,
	external_editor: ExternalEditor<'r>,
	git_interactive: GitInteractive,
	help: Help<'r>,
	input_handler: &'r InputHandler<'r>,
	list: List<'r>,
	show_commit: ShowCommit<'r>,
	state: RefCell<State>,
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
			error: Error::new(),
			exit_status: None,
			exiting: Exiting::new(),
			external_editor: ExternalEditor::new(display, config.git.editor.as_str()),
			git_interactive,
			help: Help::new(&config.key_bindings),
			input_handler,
			list: List::new(config),
			show_commit: ShowCommit::new(config),
			state: RefCell::new(State::List(false)),
			view,
			window_size_error: WindowSizeError::new(),
		}
	}

	pub(crate) fn run(&mut self) -> Result<Option<ExitStatus>, String> {
		self.check_window_size();
		while self.exit_status.is_none() {
			self.process();
			self.render();
			self.handle_input();
		}
		self.exit_end()?;
		Ok(self.exit_status)
	}

	fn activate(&mut self) {
		let state = self.get_state();
		match state {
			State::ConfirmAbort => self.confirm_abort.activate(state, &self.git_interactive),
			State::ConfirmRebase => self.confirm_rebase.activate(state, &self.git_interactive),
			State::Edit => self.edit.activate(state, &self.git_interactive),
			State::Error { .. } => self.error.activate(state, &self.git_interactive),
			State::Exiting => self.exiting.activate(state, &self.git_interactive),
			State::ExternalEditor => self.external_editor.activate(state, &self.git_interactive),
			State::Help(_) => self.help.activate(state, &self.git_interactive),
			State::List(_) => self.list.activate(state, &self.git_interactive),
			State::ShowCommit => self.show_commit.activate(state, &self.git_interactive),
			State::WindowSizeError(_) => self.window_size_error.activate(state, &self.git_interactive),
		}
	}

	fn deactivate(&mut self) {
		match self.get_state() {
			State::ConfirmAbort => self.confirm_abort.deactivate(),
			State::ConfirmRebase => self.confirm_rebase.deactivate(),
			State::Edit => self.edit.deactivate(),
			State::Error { .. } => self.error.deactivate(),
			State::Exiting => self.exiting.deactivate(),
			State::ExternalEditor => self.external_editor.deactivate(),
			State::Help(_) => self.help.deactivate(),
			State::List(_) => self.list.deactivate(),
			State::ShowCommit => self.show_commit.deactivate(),
			State::WindowSizeError(_) => self.window_size_error.deactivate(),
		}
	}

	fn process(&mut self) {
		let result = match self.get_state() {
			State::ConfirmAbort => self.confirm_abort.process(&mut self.git_interactive, self.view),
			State::ConfirmRebase => self.confirm_rebase.process(&mut self.git_interactive, self.view),
			State::Edit => self.edit.process(&mut self.git_interactive, self.view),
			State::Error { .. } => self.error.process(&mut self.git_interactive, self.view),
			State::Exiting => self.exiting.process(&mut self.git_interactive, self.view),
			State::ExternalEditor => self.external_editor.process(&mut self.git_interactive, self.view),
			State::Help(_) => self.help.process(&mut self.git_interactive, self.view),
			State::List(_) => self.list.process(&mut self.git_interactive, self.view),
			State::ShowCommit => self.show_commit.process(&mut self.git_interactive, self.view),
			State::WindowSizeError(_) => self.window_size_error.process(&mut self.git_interactive, self.view),
		};

		if let Some(exit_status) = result.exit_status {
			self.exit_status = Some(exit_status);
		}

		if let Some(new_state) = result.state {
			if new_state != self.get_state() {
				self.deactivate();
				self.set_state(new_state);
				self.activate();
			}
		}
	}

	fn render(&mut self) {
		self.view.clear();
		match self.get_state() {
			State::ConfirmAbort => {
				let view_data = self.confirm_abort.build_view_data(self.view, &self.git_interactive);
				self.view.draw_view_data(view_data);
			},
			State::ConfirmRebase => {
				let view_data = self.confirm_rebase.build_view_data(self.view, &self.git_interactive);
				self.view.draw_view_data(view_data);
			},
			State::Edit => {
				self.view
					.draw_view_data(self.edit.build_view_data(self.view, &self.git_interactive));
			},
			State::Error { .. } => {
				let view_data = self.error.build_view_data(self.view, &self.git_interactive);
				self.view.draw_view_data(view_data);
			},
			State::Exiting => {
				self.view
					.draw_view_data(self.exiting.build_view_data(self.view, &self.git_interactive))
			},
			State::ExternalEditor => {
				let view_data = self.external_editor.build_view_data(self.view, &self.git_interactive);
				self.view.draw_view_data(view_data);
			},
			State::Help(_) => {
				let view_data = self.help.build_view_data(self.view, &self.git_interactive);
				self.view.draw_view_data(view_data);
			},
			State::List(_) => {
				self.view
					.draw_view_data(self.list.build_view_data(self.view, &self.git_interactive))
			},
			State::ShowCommit => {
				self.view
					.draw_view_data(self.show_commit.build_view_data(self.view, &self.git_interactive))
			},
			State::WindowSizeError(_) => {
				self.view
					.draw_view_data(self.window_size_error.build_view_data(self.view, &self.git_interactive))
			},
		};
		self.view.refresh()
	}

	fn handle_input(&mut self) {
		let result = match self.get_state() {
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
			State::Error { .. } => {
				self.error
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
			State::Help(_) => {
				self.help
					.handle_input(self.input_handler, &mut self.git_interactive, self.view)
			},
			State::List(_) => {
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
		};

		if let Some(exit_status) = result.exit_status {
			self.exit_status = Some(exit_status);
		}

		if let Some(new_state) = result.state {
			if new_state != self.get_state() {
				self.deactivate();
				self.set_state(new_state);
				self.activate();
			}
		}

		if let Input::Resize = result.input {
			self.check_window_size();
		}
	}

	fn check_window_size(&self) {
		let check = self.view.check_window_size();
		let state = self.get_state();
		if let State::WindowSizeError(return_state) = state {
			if check {
				self.set_state(*return_state);
			}
		}
		else if !check {
			self.set_state(State::WindowSizeError(Box::new(self.get_state())));
		}
	}

	fn set_state(&self, new_state: State) {
		self.state.replace(new_state);
	}

	fn get_state(&self) -> State {
		self.state.borrow().clone()
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
