use crate::action::Action;
use crate::git_interactive::GitInteractive;

use crate::config::Config;
use crate::constants::{
	EXIT_CODE_GOOD,
	EXIT_CODE_STATE_ERROR,
	EXIT_CODE_WRITE_ERROR,
	LIST_HELP_LINES,
	VISUAL_MODE_HELP_LINES,
};
use crate::input::Input;
use crate::view::View;
use crate::window::Window;
use std::process::Command;
use std::process::ExitStatus;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Edit,
	EditFinish,
	Error,
	Exiting,
	ExternalEditor,
	ExternalEditorError,
	ExternalEditorFinish,
	Help,
	List,
	ShowCommit,
	VisualMode,
	WindowSizeError,
}

pub struct Application<'a> {
	config: &'a Config,
	edit_content: String,
	edit_content_cursor: usize,
	error_message: Option<String>,
	exit_code: Option<i32>,
	git_interactive: GitInteractive,
	help_state: State,
	previous_state: Option<State>,
	state: State,
	view: View<'a>,
	window: &'a Window<'a>,
}

impl<'a> Application<'a> {
	pub fn new(git_interactive: GitInteractive, view: View<'a>, window: &'a Window<'a>, config: &'a Config) -> Self {
		Application {
			config,
			edit_content: String::from(""),
			edit_content_cursor: 0,
			error_message: None,
			exit_code: None,
			git_interactive,
			help_state: State::List,
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
			State::Edit => {},
			State::EditFinish => self.process_edit_finish(),
			State::Error => {},
			State::Exiting => {},
			State::ExternalEditor => self.process_external_editor(),
			State::ExternalEditorError => self.process_external_editor_error(),
			State::ExternalEditorFinish => self.process_external_editor_finish(),
			State::Help => {},
			State::List => self.process_list(),
			State::ShowCommit => self.process_show_commit(),
			State::VisualMode => self.process_list(),
			State::WindowSizeError => {},
		}
	}

	fn process_edit_finish(&mut self) {
		self.git_interactive.edit_selected_line(self.edit_content.as_str());
		self.state = State::List;
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
			State::Edit => {
				self.view
					.draw_edit(self.edit_content.as_str(), self.edit_content_cursor)
			},
			State::EditFinish => {},
			State::Error => self.draw_error(),
			State::Exiting => self.view.draw_exiting(),
			State::ExternalEditor => {},
			State::ExternalEditorError => self.draw_error(),
			State::ExternalEditorFinish => {},
			State::Help => self.draw_help(),
			State::List => {
				self.view
					.draw_main(self.git_interactive.get_lines(), self.get_cursor_index(), None)
			},
			State::VisualMode => {
				self.view.draw_main(
					self.git_interactive.get_lines(),
					self.get_cursor_index(),
					Some(self.git_interactive.get_visual_start_index() - 1),
				)
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

	fn draw_help(&self) {
		self.view.draw_help(
			if self.help_state == State::List {
				LIST_HELP_LINES
			}
			else {
				VISUAL_MODE_HELP_LINES
			},
		);
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
			State::Edit => self.handle_edit(),
			State::EditFinish => {},
			State::Error => self.handle_error_input(),
			State::Exiting => {},
			State::ExternalEditor => self.handle_external_editor_input(),
			State::ExternalEditorError => self.handle_error_input(),
			State::ExternalEditorFinish => {},
			State::Help => self.handle_help_input(),
			State::List => self.handle_list_input(),
			State::VisualMode => self.handle_visual_mode_input(),
			State::ShowCommit => self.handle_show_commit_input(),
			State::WindowSizeError => self.handle_window_size_error_input(),
		};
	}

	fn handle_help_input(&mut self) {
		let help_lines = if self.help_state == State::List {
			LIST_HELP_LINES
		}
		else {
			VISUAL_MODE_HELP_LINES
		};
		match self.get_input() {
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
				self.state = self.help_state;
			},
		}
	}

	fn handle_visual_mode_input(&mut self) {
		match self.get_input() {
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
				self.state = State::List;
			},
			Input::Help => {
				self.view.update_help_top(false, true, VISUAL_MODE_HELP_LINES);
				self.help_state = self.state;
				self.state = State::Help;
			},
			_ => {},
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

	fn handle_edit(&mut self) {
		loop {
			match self.window.get_character() {
				Input::Character(c) => {
					let start = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true)
						.take(self.edit_content_cursor)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true)
						.skip(self.edit_content_cursor)
						.collect::<String>();
					self.edit_content = format!("{}{}{}", start, c, end);
					self.edit_content_cursor += 1;
				},
				Input::Backspace => {
					if self.edit_content_cursor == 0 {
						break;
					}
					let start = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true)
						.take(self.edit_content_cursor - 1)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true)
						.skip(self.edit_content_cursor)
						.collect::<String>();
					self.edit_content = format!("{}{}", start, end);
					self.edit_content_cursor -= 1;
				},
				Input::Delete => {
					let length = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true).count();
					if self.edit_content_cursor == length {
						break;
					}
					let start = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true)
						.take(self.edit_content_cursor)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true)
						.skip(self.edit_content_cursor + 1)
						.collect::<String>();
					self.edit_content = format!("{}{}", start, end);
				},
				Input::MoveCursorRight => {
					let length = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true).count();
					if self.edit_content_cursor < length {
						self.edit_content_cursor += 1;
					}
				},
				Input::MoveCursorLeft => {
					if self.edit_content_cursor != 0 {
						self.edit_content_cursor -= 1;
					}
				},
				Input::Enter => self.state = State::EditFinish,
				Input::Resize => self.handle_resize(),
				_ => {
					continue;
				},
			}
			break;
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
				self.view.update_help_top(false, true, LIST_HELP_LINES);
				self.help_state = self.state;
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
			Input::ActionBreak => self.git_interactive.toggle_break(),
			Input::ActionDrop => self.set_selected_line_action(Action::Drop),
			Input::ActionEdit => self.set_selected_line_action(Action::Edit),
			Input::ActionFixup => self.set_selected_line_action(Action::Fixup),
			Input::ActionPick => self.set_selected_line_action(Action::Pick),
			Input::ActionReword => self.set_selected_line_action(Action::Reword),
			Input::ActionSquash => self.set_selected_line_action(Action::Squash),
			Input::Edit => {
				if *self.git_interactive.get_selected_line_action() == Action::Exec {
					self.edit_content = self.git_interactive.get_selected_line_edit_content().clone();
					self.edit_content_cursor = UnicodeSegmentation::graphemes(self.edit_content.as_str(), true).count();
					self.state = State::Edit;
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
				self.state = State::VisualMode;
			},
			Input::OpenInEditor => self.state = State::ExternalEditor,
			_ => {},
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
