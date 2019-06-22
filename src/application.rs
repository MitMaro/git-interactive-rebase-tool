use crate::action::Action;
use crate::git_interactive::GitInteractive;

use crate::config::Config;
use crate::confirm_abort::ConfirmAbort;
use crate::confirm_rebase::ConfirmRebase;
use crate::constants::{LIST_HELP_LINES, VISUAL_MODE_HELP_LINES};
use crate::edit::Edit;
use crate::input::{Input, InputHandler};
use crate::process::{
	ExitStatus,
	HandleInputResult,
	HandleInputResultBuilder,
	ProcessModule,
	ProcessResult,
	ProcessResultBuilder,
	State,
};
use crate::show_commit::ShowCommit;
use crate::view::View;
use crate::window::Window;
use core::borrow::Borrow;
use std::process::Command;
use std::process::ExitStatus as ProcessExitStatus;

pub struct Application<'a> {
	config: &'a Config,
	confirm_abort: ConfirmAbort,
	confirm_rebase: ConfirmRebase,
	edit: Edit,
	git_interactive: GitInteractive,
	input_handler: &'a InputHandler<'a>,
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
			config,
			confirm_abort: ConfirmAbort::new(),
			confirm_rebase: ConfirmRebase::new(),
			edit: Edit::new(),
			git_interactive,
			input_handler,
			show_commit: ShowCommit::new(),
			view,
		}
	}

	fn get_cursor_index(&self) -> usize {
		*self.git_interactive.get_selected_line_index() - 1
	}

	pub fn activate(&mut self, state: State) {
		match state {
			State::ConfirmAbort => self.confirm_abort.activate(state, &self.git_interactive),
			State::ConfirmRebase => self.confirm_rebase.activate(state, &self.git_interactive),
			State::Edit => self.edit.activate(state, &self.git_interactive),
			State::Error { .. } => {},
			State::Exiting => {},
			State::ExternalEditor(_) => {},
			State::ExternalEditorError => {},
			State::ExternalEditorFinish(_) => {},
			State::Help(_) => {},
			State::List => {},
			State::ShowCommit => self.show_commit.activate(state, &self.git_interactive),
			State::VisualMode => {},
			State::WindowSizeError(_) => {},
		}
	}

	pub fn deactivate(&mut self, state: State) {
		match state {
			State::ConfirmAbort => self.confirm_abort.deactivate(),
			State::ConfirmRebase => self.confirm_rebase.deactivate(),
			State::Edit => self.edit.deactivate(),
			State::Error { .. } => {},
			State::Exiting => {},
			State::ExternalEditor(_) => {},
			State::ExternalEditorError => {},
			State::ExternalEditorFinish(_) => {},
			State::Help(_) => {},
			State::List => {},
			State::ShowCommit => self.show_commit.deactivate(),
			State::VisualMode => {},
			State::WindowSizeError(_) => {},
		}
	}

	pub fn process(&mut self, state: State) -> ProcessResult {
		match state {
			State::ConfirmAbort => self.confirm_abort.process(&mut self.git_interactive),
			State::ConfirmRebase => self.confirm_rebase.process(&mut self.git_interactive),
			State::Edit => self.edit.process(&mut self.git_interactive),
			State::Error { .. } => ProcessResult::new(),
			State::Exiting => ProcessResult::new(),
			State::ExternalEditor(return_state) => self.process_external_editor(return_state.borrow()),
			State::ExternalEditorError => self.process_external_editor_error(),
			State::ExternalEditorFinish(_) => self.process_external_editor_finish(),
			State::Help(_) => ProcessResult::new(),
			State::List => self.process_list(),
			State::ShowCommit => self.show_commit.process(&mut self.git_interactive),
			State::VisualMode => self.process_list(),
			State::WindowSizeError(_) => ProcessResult::new(),
		}
	}

	pub fn process_external_editor(&mut self, return_state: &State) -> ProcessResult {
		let mut result = ProcessResultBuilder::new();

		result = if let Err(e) = self.run_editor() {
			result.error(e.as_str(), State::ExternalEditorError)
		}
		else {
			result.state(State::ExternalEditorFinish(Box::new(return_state.clone())))
		};

		result.build()
	}

	pub fn process_external_editor_finish(&mut self) -> ProcessResult {
		let mut result = ProcessResultBuilder::new();
		result = if let Err(e) = self.git_interactive.reload_file(self.config.comment_char.as_str()) {
			result.error(e.as_str(), State::List)
		}
		else if self.git_interactive.get_lines().is_empty() {
			result.error("Rebase empty", State::List)
		}
		else {
			result.state(State::List)
		};

		result.build()
	}

	pub fn process_external_editor_error(&mut self) -> ProcessResult {
		ProcessResultBuilder::new()
			.state(State::Exiting)
			.exit_status(
				if self.git_interactive.get_lines().is_empty() {
					ExitStatus::Good
				}
				else {
					ExitStatus::StateError
				},
			)
			.build()
	}

	pub fn process_list(&mut self) -> ProcessResult {
		let lines = self.git_interactive.get_lines();
		let selected_index = self.get_cursor_index();
		self.view.update_main_top(lines.len(), selected_index);
		ProcessResult::new()
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
			State::Error { message, .. } => self.draw_error(message.as_str()),
			State::Exiting => self.draw_exiting(),
			State::ExternalEditor(_) => {},
			State::ExternalEditorError => {},
			State::ExternalEditorFinish(_) => {},
			State::Help(help_state) => self.draw_help(help_state.borrow()),
			State::List => self.draw_main(false),
			State::VisualMode => self.draw_main(true),
			State::ShowCommit => self.show_commit.render(&self.view, &self.git_interactive),
			State::WindowSizeError(_) => self.draw_window_size_error(),
		}
		self.view.refresh()
	}

	fn draw_error(&self, error_message: &str) {
		self.view.draw_error(error_message);
	}

	fn draw_main(&self, visual_mode: bool) {
		self.view.draw_main(
			self.git_interactive.get_lines(),
			self.get_cursor_index(),
			if visual_mode {
				Some(self.git_interactive.get_visual_start_index() - 1)
			}
			else {
				None
			},
		);
	}

	fn draw_help(&self, help_state: &State) {
		self.view.draw_help(
			if *help_state == State::List {
				LIST_HELP_LINES
			}
			else {
				VISUAL_MODE_HELP_LINES
			},
		);
	}

	fn draw_exiting(&self) {
		self.view.draw_exiting();
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
			State::Error { return_state, .. } => self.handle_error_input(return_state.borrow()),
			State::Exiting => HandleInputResult::new(Input::Other),
			State::ExternalEditor(return_state) => self.handle_external_editor_input(return_state.borrow()),
			State::ExternalEditorError => HandleInputResult::new(Input::Other),
			State::ExternalEditorFinish(_) => HandleInputResult::new(Input::Other),
			State::Help(help_state) => self.handle_help_input(help_state.borrow()),
			State::List => self.handle_list_input(),
			State::VisualMode => self.handle_visual_mode_input(),
			State::ShowCommit => {
				self.show_commit
					.handle_input_with_view(&self.input_handler, &mut self.git_interactive, &self.view)
			},
			State::WindowSizeError(_) => self.handle_window_size_error_input(),
		}
	}

	fn handle_help_input(&mut self, help_state: &State) -> HandleInputResult {
		let help_lines = if *help_state == State::List {
			LIST_HELP_LINES
		}
		else {
			VISUAL_MODE_HELP_LINES
		};
		let input = self.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorDown => {
				self.view.update_help_top(false, false, help_lines);
			},
			Input::MoveCursorUp => {
				self.view.update_help_top(true, false, help_lines);
			},
			Input::Resize => {
				self.view.update_help_top(true, true, help_lines);
			},
			_ => {
				result = result.state(help_state.clone());
			},
		}
		result.build()
	}

	fn handle_visual_mode_input(&mut self) -> HandleInputResult {
		let input = self.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorDown => {
				self.git_interactive.move_cursor_down(1);
			},
			Input::MoveCursorUp => {
				self.git_interactive.move_cursor_up(1);
			},
			Input::MoveCursorPageDown => {
				self.git_interactive.move_cursor_down(5);
			},
			Input::MoveCursorPageUp => {
				self.git_interactive.move_cursor_up(5);
			},
			Input::ActionDrop => self.git_interactive.set_visual_range_action(Action::Drop),
			Input::ActionEdit => self.git_interactive.set_visual_range_action(Action::Edit),
			Input::ActionFixup => self.git_interactive.set_visual_range_action(Action::Fixup),
			Input::ActionPick => self.git_interactive.set_visual_range_action(Action::Pick),
			Input::ActionReword => self.git_interactive.set_visual_range_action(Action::Reword),
			Input::ActionSquash => self.git_interactive.set_visual_range_action(Action::Squash),
			Input::SwapSelectedDown => self.git_interactive.swap_visual_range_down(),
			Input::SwapSelectedUp => self.git_interactive.swap_visual_range_up(),
			Input::ToggleVisualMode => {
				result = result.state(State::List);
			},
			Input::Help => {
				self.view.update_help_top(false, true, VISUAL_MODE_HELP_LINES);
				result = result.help(State::VisualMode);
			},
			_ => {},
		}
		result.build()
	}

	pub fn handle_error_input(&mut self, return_state: &State) -> HandleInputResult {
		let input = self.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Resize => {},
			_ => {
				result = result.state(return_state.clone());
			},
		}
		result.build()
	}

	pub fn handle_external_editor_input(&mut self, return_state: &State) -> HandleInputResult {
		let input = self.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Resize => {},
			_ => {
				result = result.state(return_state.clone());
			},
		}
		result.build()
	}

	pub fn handle_list_input(&mut self) -> HandleInputResult {
		let input = self.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Help => {
				self.view.update_help_top(false, true, LIST_HELP_LINES);
				result = result.help(State::List);
			},
			Input::ShowCommit => {
				if !self.git_interactive.get_selected_line_hash().is_empty() {
					result = result.state(State::ShowCommit);
				}
			},
			Input::Abort => {
				result = result.state(State::ConfirmAbort);
			},
			Input::ForceAbort => {
				self.git_interactive.clear();
				result = result.exit_status(ExitStatus::Good).state(State::Exiting);
			},
			Input::Rebase => {
				result = result.state(State::ConfirmRebase);
			},
			Input::ForceRebase => {
				result = result.exit_status(ExitStatus::Good).state(State::Exiting);
			},
			Input::ActionBreak => self.git_interactive.toggle_break(),
			Input::ActionDrop => self.set_selected_line_action(Action::Drop),
			Input::ActionEdit => self.set_selected_line_action(Action::Edit),
			Input::ActionFixup => self.set_selected_line_action(Action::Fixup),
			Input::ActionPick => self.set_selected_line_action(Action::Pick),
			Input::ActionReword => self.set_selected_line_action(Action::Reword),
			Input::ActionSquash => self.set_selected_line_action(Action::Squash),
			Input::Edit => {
				if *self.git_interactive.get_selected_line_action() == Action::Exec {
					result = result.state(State::Edit);
				}
			},
			Input::SwapSelectedDown => self.git_interactive.swap_selected_down(),
			Input::SwapSelectedUp => self.git_interactive.swap_selected_up(),
			Input::MoveCursorDown => self.git_interactive.move_cursor_down(1),
			Input::MoveCursorUp => self.git_interactive.move_cursor_up(1),
			Input::MoveCursorPageDown => self.git_interactive.move_cursor_down(5),
			Input::MoveCursorPageUp => self.git_interactive.move_cursor_up(5),
			Input::ToggleVisualMode => {
				self.git_interactive.start_visual_mode();
				result = result.state(State::VisualMode);
			},
			Input::OpenInEditor => result = result.state(State::ExternalEditor(Box::new(State::List))),
			_ => {},
		}
		result.build()
	}

	pub fn handle_window_size_error_input(&mut self) -> HandleInputResult {
		HandleInputResult::new(self.get_input())
	}

	pub fn run_editor(&mut self) -> Result<(), String> {
		self.git_interactive.write_file()?;
		let filepath = self.git_interactive.get_filepath();
		let callback = || -> Result<ProcessExitStatus, String> {
			// TODO: This doesn't handle editor with arguments (e.g. EDITOR="edit --arg")
			Command::new(&self.config.editor)
				.arg(filepath.as_os_str())
				.status()
				.map_err(|e| {
					format!(
						"Unable to run editor ({}):\n{}",
						self.config.editor.to_string_lossy(),
						e.to_string()
					)
				})
		};
		let exit_status: ProcessExitStatus = Window::leave_temporarily(callback)?;

		if !exit_status.success() {
			return Err(String::from("Editor returned non-zero exit status."));
		}

		Ok(())
	}

	pub fn write_file(&self) -> Result<(), String> {
		self.git_interactive.write_file()
	}

	fn set_selected_line_action(&mut self, action: Action) {
		self.git_interactive.set_selected_line_action(action);
		if self.config.auto_select_next {
			self.git_interactive.move_cursor_down(1);
		}
	}
}
