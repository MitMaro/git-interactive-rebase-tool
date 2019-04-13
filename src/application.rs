use crate::action::Action;
use crate::git_interactive::GitInteractive;

use crate::config::Config;
use crate::constants::{EXIT_CODE_GOOD, EXIT_CODE_STATE_ERROR, EXIT_CODE_WRITE_ERROR};
use crate::input::Input;
use crate::view::View;
use crate::window::Window;
use std::process::Command;
use std::process::ExitStatus;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Exiting,
	ExternalEditor,
	ExternalEditorFinish,
	ExternalEditorError,
	Error,
	Help,
	List,
	ShowCommit,
	WindowSizeError,
}

pub struct Application<'a> {
	config: &'a Config,
	error_message: Option<String>,
	exit_code: Option<i32>,
	git_interactive: GitInteractive,
	previous_state: Option<State>,
	state: State,
	view: View<'a>,
	window: &'a Window<'a>,
}

impl<'a> Application<'a> {
	pub fn new(git_interactive: GitInteractive, view: View<'a>, window: &'a Window<'a>, config: &'a Config) -> Self {
		Application {
			config,
			error_message: None,
			exit_code: None,
			git_interactive,
			previous_state: None,
			state: State::List,
			view,
			window,
		}
	}

	pub fn run(&mut self) -> Result<Option<i32>, String> {
		self.handle_resize();
		while self.exit_code == None {
			// process based on input, allowed to change state
			self.process();
			// draw output for state, including state change from process
			self.draw();
			// handle input for state
			self.handle_input();
		}
		self.exit_end()?;
		Ok(self.exit_code)
	}

	fn get_cursor_index(&self) -> usize {
		*self.git_interactive.get_selected_line_index() - 1
	}

	fn process(&mut self) {
		match self.state {
			State::ConfirmAbort => {},
			State::ConfirmRebase => {},
			State::Error => {},
			State::Exiting => {},
			State::ExternalEditor => self.process_external_editor(),
			State::ExternalEditorError => self.process_external_editor_error(),
			State::ExternalEditorFinish => self.process_external_editor_finish(),
			State::Help => {},
			State::List => self.process_list(),
			State::ShowCommit => self.process_show_commit(),
			State::WindowSizeError => {},
		}
	}

	fn process_external_editor(&mut self) {
		if let Err(e) = self.run_editor() {
			self.set_error(e, State::ExternalEditorFinish);
			return;
		}
		self.state = State::ExternalEditorFinish;
	}

	fn process_external_editor_finish(&mut self) {
		if let Err(e) = self.git_interactive.reload_file(self.config.comment_char.as_str()) {
			self.set_error(e, State::ExternalEditorError);
			return;
		}

		if self.git_interactive.get_lines().is_empty() {
			self.set_error(String::from("Rebase empty"), State::ExternalEditorError);
			// exit will occur in error
			return;
		}
		self.state = State::List;
	}

	fn process_external_editor_error(&mut self) {
		self.state = State::Exiting;
		if self.git_interactive.get_lines().is_empty() {
			self.exit_finish();
			return;
		}
		self.exit_error();
	}

	fn process_list(&mut self) {
		let lines = self.git_interactive.get_lines();
		let selected_index = self.get_cursor_index();
		self.view.update_main_top(lines.len(), selected_index);
	}

	fn process_show_commit(&mut self) {
		if let Err(e) = self.git_interactive.load_commit_stats() {
			self.set_error(e, State::List);
		};
	}

	fn draw(&self) {
		self.window.clear();
		match self.state {
			State::ConfirmAbort => self.view.draw_confirm("Are you sure you want to abort"),
			State::ConfirmRebase => self.view.draw_confirm("Are you sure you want to rebase"),
			State::Error => self.draw_error(),
			State::Exiting => self.view.draw_exiting(),
			State::ExternalEditor => {},
			State::ExternalEditorError => self.draw_error(),
			State::ExternalEditorFinish => {},
			State::Help => self.view.draw_help(),
			State::List => {
				self.view
					.draw_main(self.git_interactive.get_lines(), self.get_cursor_index())
			},
			State::ShowCommit => self.view.draw_show_commit(self.git_interactive.get_commit_stats()),
			State::WindowSizeError => self.view.draw_window_size_error(),
		}
		self.window.refresh();
	}

	fn draw_error(&self) {
		let message = match self.error_message {
			Some(ref msg) => msg.as_str(),
			None => "Error...",
		};
		self.view.draw_error(message);
	}

	fn handle_resize(&mut self) {
		let check = self.view.check_window_size();
		if !check && self.state != State::WindowSizeError {
			self.previous_state = Some(self.state);
			self.state = State::WindowSizeError;
		}
		else if check && self.state == State::WindowSizeError {
			self.state = self.previous_state.unwrap_or(State::List);
			self.previous_state = None;
		}
	}

	fn get_input(&mut self) -> Input {
		let input = self.window.get_input();
		if let Input::Resize = input {
			self.handle_resize();
		}
		input
	}

	fn get_confirm(&mut self) -> Option<bool> {
		let input = self.window.get_confirm();
		if input.is_none() {
			self.handle_resize();
		};
		input
	}

	fn handle_input(&mut self) {
		match self.state {
			State::ConfirmAbort => self.handle_confirm_abort_input(),
			State::ConfirmRebase => self.handle_confirm_rebase_input(),
			State::Error => self.handle_error_input(),
			State::Exiting => {},
			State::ExternalEditor => self.handle_external_editor_input(),
			State::ExternalEditorError => self.handle_error_input(),
			State::ExternalEditorFinish => {},
			State::Help => self.handle_help_input(),
			State::List => self.handle_list_input(),
			State::ShowCommit => self.handle_show_commit_input(),
			State::WindowSizeError => self.handle_window_size_error_input(),
		};
	}

	fn handle_help_input(&mut self) {
		match self.get_input() {
			Input::MoveCursorDown => {
				self.view.update_help_top(false, false);
			},
			Input::MoveCursorUp => {
				self.view.update_help_top(true, false);
			},
			Input::Resize => {
				self.view.update_help_top(true, true);
			},
			_ => {
				self.state = State::List;
			},
		}
	}

	fn handle_show_commit_input(&mut self) {
		match self.get_input() {
			Input::MoveCursorDown => {
				self.view
					.update_commit_top(false, false, self.git_interactive.get_commit_stats_length());
			},
			Input::MoveCursorUp => {
				self.view
					.update_commit_top(true, false, self.git_interactive.get_commit_stats_length());
			},
			Input::Resize => {
				self.view
					.update_commit_top(true, false, self.git_interactive.get_commit_stats_length());
			},
			_ => {
				self.state = State::List;
			},
		}
	}

	fn handle_confirm_abort_input(&mut self) {
		match self.get_confirm() {
			Some(true) => {
				self.exit_abort();
				self.state = State::Exiting;
			},
			Some(false) => {
				self.state = State::List;
			},
			None => {},
		}
	}

	fn handle_confirm_rebase_input(&mut self) {
		match self.get_confirm() {
			Some(true) => {
				self.exit_finish();
				self.state = State::Exiting;
			},
			Some(false) => {
				self.state = State::List;
			},
			None => {},
		}
	}

	fn handle_error_input(&mut self) {
		match self.get_input() {
			Input::Resize => {},
			_ => {
				self.error_message = None;
				self.state = self.previous_state.unwrap_or(State::List);
				self.previous_state = None;
			},
		}
	}

	fn handle_external_editor_input(&mut self) {
		match self.get_input() {
			Input::Resize => {},
			_ => {
				self.error_message = None;
				self.state = self.previous_state.unwrap_or(State::List);
				self.previous_state = None;
			},
		}
	}

	fn handle_list_input(&mut self) {
		match self.get_input() {
			Input::Help => {
				self.view.update_help_top(false, true);
				self.state = State::Help
			},
			Input::ShowCommit => {
				if !self.git_interactive.get_selected_line_hash().is_empty() {
					self.view.update_commit_top(false, true, 0);
					self.state = State::ShowCommit
				}
			},
			Input::Abort => self.state = State::ConfirmAbort,
			Input::ForceAbort => {
				self.exit_abort();
				self.state = State::Exiting;
			},
			Input::Rebase => self.state = State::ConfirmRebase,
			Input::ForceRebase => {
				self.exit_finish();
				self.state = State::Exiting;
			},
			Input::Break => self.git_interactive.toggle_break(),
			Input::Drop => self.set_selected_line_action(Action::Drop),
			Input::Edit => self.set_selected_line_action(Action::Edit),
			Input::Fixup => self.set_selected_line_action(Action::Fixup),
			Input::Pick => self.set_selected_line_action(Action::Pick),
			Input::Reword => self.set_selected_line_action(Action::Reword),
			Input::Squash => self.set_selected_line_action(Action::Squash),
			Input::SwapSelectedDown => {
				self.git_interactive.swap_selected_down();
			},
			Input::SwapSelectedUp => {
				self.git_interactive.swap_selected_up();
			},
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
			Input::Resize => {},
			Input::OpenInEditor => {
				self.state = State::ExternalEditor;
			},
			Input::Other => {},
		}
	}

	fn handle_window_size_error_input(&mut self) {
		self.get_input();
	}

	fn run_editor(&mut self) -> Result<(), String> {
		self.git_interactive.write_file()?;
		let filepath = self.git_interactive.get_filepath();
		let callback = || -> Result<ExitStatus, String> {
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
		let exit_status: ExitStatus = Window::leave_temporarily(callback)?;

		if !exit_status.success() {
			return Err(String::from("Editor returned non-zero exit status."));
		}

		Ok(())
	}

	fn set_selected_line_action(&mut self, action: Action) {
		self.git_interactive.set_selected_line_action(action);
		if self.config.auto_select_next {
			self.git_interactive.move_cursor_down(1);
		}
	}

	fn set_error(&mut self, msg: String, next_state: State) {
		self.previous_state = Some(next_state);
		self.state = State::Error;
		self.error_message = Some(msg);
	}

	fn exit_abort(&mut self) {
		self.git_interactive.clear();
		self.exit_finish();
	}

	fn exit_finish(&mut self) {
		self.exit_code = Some(EXIT_CODE_GOOD);
	}

	fn exit_error(&mut self) {
		self.exit_code = Some(EXIT_CODE_STATE_ERROR);
	}

	fn exit_end(&mut self) -> Result<(), String> {
		self.window.end();
		match self.git_interactive.write_file() {
			Ok(_) => {},
			Err(msg) => {
				self.exit_code = Some(EXIT_CODE_WRITE_ERROR);
				return Err(msg);
			},
		}
		Ok(())
	}
}
