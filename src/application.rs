use crate::config::Config;
use crate::confirm_abort::ConfirmAbort;
use crate::confirm_rebase::ConfirmRebase;
use crate::constants::{LIST_HELP_LINES, VISUAL_MODE_HELP_LINES};
use crate::edit::Edit;
use crate::error::Error;
use crate::exiting::Exiting;
use crate::external_editor::ExternalEditor;
use crate::git_interactive::GitInteractive;
use crate::input::{Input, InputHandler};
use crate::list::List;
use crate::process::{HandleInputResult, HandleInputResultBuilder, ProcessModule, ProcessResult, State};
use crate::show_commit::ShowCommit;
use crate::view::View;
use core::borrow::Borrow;

fn get_help_lines(help_state: &State) -> &[(&str, &str)] {
	if let State::List(visual_mode) = *help_state {
		if visual_mode {
			LIST_HELP_LINES
		}
		else {
			VISUAL_MODE_HELP_LINES
		}
	}
	else {
		&[]
	}
}

pub struct Application<'a> {
	confirm_abort: ConfirmAbort,
	confirm_rebase: ConfirmRebase,
	edit: Edit,
	error: Error,
	exiting: Exiting,
	external_editor: ExternalEditor<'a>,
	git_interactive: GitInteractive,
	input_handler: &'a InputHandler<'a>,
	list: List<'a>,
	show_commit: ShowCommit,
	view: View<'a>,
}

impl<'a> Application<'a> {
	pub fn new(
		git_interactive: GitInteractive,
		view: View<'a>,
		input_handler: &'a InputHandler<'a>,
		config: &'a Config,
	) -> Self
	{
		Self {
			confirm_abort: ConfirmAbort::new(),
			confirm_rebase: ConfirmRebase::new(),
			edit: Edit::new(),
			error: Error::new(),
			exiting: Exiting::new(),
			external_editor: ExternalEditor::new(config),
			git_interactive,
			input_handler,
			list: List::new(config),
			show_commit: ShowCommit::new(),
			view,
		}
	}

	pub fn activate(&mut self, state: State) {
		match state {
			State::ConfirmAbort => self.confirm_abort.activate(state, &self.git_interactive),
			State::ConfirmRebase => self.confirm_rebase.activate(state, &self.git_interactive),
			State::Edit => self.edit.activate(state, &self.git_interactive),
			State::Error { .. } => self.error.activate(state, &self.git_interactive),
			State::Exiting => self.exiting.activate(state, &self.git_interactive),
			State::ExternalEditor => self.external_editor.activate(state, &self.git_interactive),
			State::Help(_) => {},
			State::List(_) => self.list.activate(state, &self.git_interactive),
			State::ShowCommit => self.show_commit.activate(state, &self.git_interactive),
			State::WindowSizeError(_) => {},
		}
	}

	pub fn deactivate(&mut self, state: State) {
		match state {
			State::ConfirmAbort => self.confirm_abort.deactivate(),
			State::ConfirmRebase => self.confirm_rebase.deactivate(),
			State::Edit => self.edit.deactivate(),
			State::Error { .. } => self.error.deactivate(),
			State::Exiting => self.exiting.deactivate(),
			State::ExternalEditor => self.external_editor.deactivate(),
			State::Help(_) => {},
			State::List(_) => self.list.deactivate(),
			State::ShowCommit => self.show_commit.deactivate(),
			State::WindowSizeError(_) => {},
		}
	}

	pub fn process(&mut self, state: State) -> ProcessResult {
		match state {
			State::ConfirmAbort => self.confirm_abort.process(&mut self.git_interactive),
			State::ConfirmRebase => self.confirm_rebase.process(&mut self.git_interactive),
			State::Edit => self.edit.process(&mut self.git_interactive),
			State::Error { .. } => self.error.process(&mut self.git_interactive),
			State::Exiting => self.exiting.process(&mut self.git_interactive),
			State::ExternalEditor => self.external_editor.process(&mut self.git_interactive),
			State::Help(_) => ProcessResult::new(),
			State::List(_) => self.list.process_with_view(&mut self.git_interactive, &self.view),
			State::ShowCommit => self.show_commit.process(&mut self.git_interactive),
			State::WindowSizeError(_) => ProcessResult::new(),
		}
	}

	pub fn check_window_size(&self) -> bool {
		self.view.check_window_size()
	}

	pub fn render(&self, state: State) {
		self.view.clear();
		match state {
			State::ConfirmAbort => self.confirm_abort.render(&self.view, &self.git_interactive),
			State::ConfirmRebase => self.confirm_rebase.render(&self.view, &self.git_interactive),
			State::Edit => self.edit.render(&self.view, &self.git_interactive),
			State::Error { .. } => self.error.render(&self.view, &self.git_interactive),
			State::Exiting => self.exiting.render(&self.view, &self.git_interactive),
			State::ExternalEditor => self.external_editor.render(&self.view, &self.git_interactive),
			State::Help(help_state) => self.draw_help(help_state.borrow()),
			State::List(_) => self.list.render(&self.view, &self.git_interactive),
			State::ShowCommit => self.show_commit.render(&self.view, &self.git_interactive),
			State::WindowSizeError(_) => self.draw_window_size_error(),
		}
		self.view.refresh()
	}

	fn draw_help(&self, help_state: &State) {
		self.view.draw_help(get_help_lines(help_state))
	}

	fn draw_window_size_error(&self) {
		self.view.draw_window_size_error();
	}

	pub fn get_input(&self) -> Input {
		self.input_handler.get_input()
	}

	pub fn handle_input(&mut self, state: State) -> HandleInputResult {
		match state {
			State::ConfirmAbort => {
				self.confirm_abort
					.handle_input(self.input_handler, &mut self.git_interactive)
			},
			State::ConfirmRebase => {
				self.confirm_rebase
					.handle_input(self.input_handler, &mut self.git_interactive)
			},
			State::Edit => self.edit.handle_input(&self.input_handler, &mut self.git_interactive),
			State::Error { .. } => self.error.handle_input(&self.input_handler, &mut self.git_interactive),
			State::Exiting => {
				self.exiting
					.handle_input(&self.input_handler, &mut self.git_interactive)
			},
			State::ExternalEditor => {
				self.external_editor
					.handle_input(&self.input_handler, &mut self.git_interactive)
			},
			State::Help(help_state) => self.handle_help_input(help_state.borrow()),
			State::List(_) => {
				self.list
					.handle_input_with_view(&self.input_handler, &mut self.git_interactive, &self.view)
			},
			State::ShowCommit => {
				self.show_commit
					.handle_input_with_view(&self.input_handler, &mut self.git_interactive, &self.view)
			},
			State::WindowSizeError(_) => self.handle_window_size_error_input(),
		}
	}

	fn handle_help_input(&mut self, help_state: &State) -> HandleInputResult {
		let input = self.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorDown => {
				self.view.update_help_top(false, false, get_help_lines(help_state));
			},
			Input::MoveCursorUp => {
				self.view.update_help_top(true, false, get_help_lines(help_state));
			},
			Input::Resize => {
				self.view.update_help_top(true, true, get_help_lines(help_state));
			},
			_ => {
				result = result.state(help_state.clone());
			},
		}
		result.build()
	}

	pub fn handle_window_size_error_input(&mut self) -> HandleInputResult {
		HandleInputResult::new(self.get_input())
	}

	pub fn write_file(&self) -> Result<(), String> {
		self.git_interactive.write_file()
	}
}
