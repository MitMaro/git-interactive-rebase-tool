use action::Action;
use git_interactive::GitInteractive;

use window::{
	Window
};
use input::Input;
use config::Config;

const EXIT_CODE_GOOD: i32 = 0;
const EXIT_CODE_WRITE_ERROR: i32 = 8;

#[derive(PartialEq, Debug)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Help,
	List,
	ShowCommit,
}

pub struct Application {
	config: Config,
	pub exit_code: Option<i32>,
	git_interactive: GitInteractive,
	state: State,
	window: Window,
}

impl Application {
	pub fn new(git_interactive: GitInteractive, window: Window, config: Config) -> Self {
		Application {
			config,
			exit_code: None,
			git_interactive,
			state: State::List,
			window,
		}
	}
	
	pub fn process_input(&mut self) {
		match self.state {
			State::ConfirmAbort => self.process_confirm_abort(),
			State::ConfirmRebase => self.process_confirm_rebase(),
			State::Help => self.process_help_input(),
			State::List => self.process_list_input(),
			State::ShowCommit => self.process_show_commit_input(),
		}
	}
	
	pub fn draw(&self) {
		match self.state {
			State::ConfirmAbort => {
				self.window.draw_confirm("Are you sure you want to abort");
			},
			State::ConfirmRebase => {
				self.window.draw_confirm("Are you sure you want to rebase");
			}
			State::Help => {
				self.window.draw_help();
			},
			State::List => {
				self.window.draw(
					self.git_interactive.get_lines(),
					*self.git_interactive.get_selected_line_index()
				);
			},
			State::ShowCommit => {
				self.window.draw_show_commit(
					self.git_interactive.get_selected_line_hash(),
					self.git_interactive.get_git_root()
				);
			},
		}
	}

	fn abort(&mut self) {
		self.git_interactive.clear();
		self.finish();
	}
	
	fn finish(&mut self) {
		self.exit_code = Some(EXIT_CODE_GOOD);
	}

	pub fn end(&mut self) -> Result<(), String>{
		self.window.end();
		match self.git_interactive.write_file() {
			Ok(_) => {},
			Err(msg) => {
				self.exit_code = Some(EXIT_CODE_WRITE_ERROR);
				return Err(msg)
			}
		}
		Ok(())
	}
	
	fn process_help_input(&mut self) {
		self.window.get_input();
		self.state = State::List;
	}
	
	fn process_show_commit_input(&mut self) {
		self.window.get_input();
		self.state = State::List;
	}

	fn process_confirm_abort(&mut self) {
		if self.window.get_confirm() {
			self.abort();
		}
		else {
			self.state = State::List;
		}
	}

	fn process_confirm_rebase(&mut self) {
		if self.window.get_confirm() {
			self.finish();
		}
		else {
			self.state = State::List;
		}
	}

	fn process_list_input(&mut self) {
		match self.window.get_input() {
			Input::Help => self.state = State::Help,
			Input::ShowCommit => self.state = State::ShowCommit,
			Input::Abort => self.state = State::ConfirmAbort,
			Input::ForceAbort => self.abort(),
			Input::Rebase => self.state = State::ConfirmRebase,
			Input::ForceRebase => self.finish(),
			Input::Drop => self.set_selected_line_action(Action::Drop),
			Input::Edit => self.set_selected_line_action(Action::Edit),
			Input::Fixup => self.set_selected_line_action(Action::Fixup),
			Input::Pick => self.set_selected_line_action(Action::Pick),
			Input::Reword => self.set_selected_line_action(Action::Reword),
			Input::Squash => self.set_selected_line_action(Action::Squash),
			Input::SwapSelectedDown => {
				self.git_interactive.swap_selected_down();
				self.reset_top();
			},
			Input::SwapSelectedUp => {
				self.git_interactive.swap_selected_up();
				self.reset_top();
			},
			Input::MoveCursorDown => {
				self.git_interactive.move_cursor_down(1);
				self.reset_top();
			},
			Input::MoveCursorUp => {
				self.git_interactive.move_cursor_up(1);
				self.reset_top();
			},
			Input::MoveCursorPageDown => {
				self.git_interactive.move_cursor_down(5);
				self.reset_top();
			},
			Input::MoveCursorPageUp => {
				self.git_interactive.move_cursor_up(5);
				self.reset_top();
			},
			Input::Resize => {
				self.window.resize_term();
				self.reset_top()
			},
			Input::OpenInEditor => {
				match self.run_editor() {
					Ok(Some(new_gi)) => {
						self.git_interactive = new_gi
					},
					Ok(None) => {},
					Err(e) => {
						self.window.draw_prompt(format!("{}\n\nPress any key to continue.", e).as_str());
						self.window.window.getch();
					},
				}
				self.reset_top();
			},
			Input::Other => {}
		}
	}

	fn reset_top(&mut self) {
		self.window.set_top(
			self.git_interactive.get_lines().len(),
			*self.git_interactive.get_selected_line_index()
		)
	}

	fn set_selected_line_action(&mut self, action: Action) {
		self.git_interactive.set_selected_line_action(action);
		if self.config.auto_select_next {
			self.git_interactive.move_cursor_down(1);
		}
	}

	fn run_editor(&self) -> Result<Option<GitInteractive>, String> {
		let filepath = self.git_interactive.write_file()?;
		let exit_status = Window::leave_temporarily(|| -> Result<std::process::ExitStatus, String> {
			use std::process::Command;
			// TODO: This doesn't handle editor with arguments (e.g. EDITOR="edit --arg")
			Command::new(&self.config.editor)
				.arg(&filepath)
				.status()
				.map_err(|e| format!("Unable to run editor ({}):\n{}",
					self.config.editor.to_string_lossy(), e.to_string()))
		})?;
		if !exit_status.success() {
			self.window.draw_confirm("Editor returned non-zero exit status.\n\
				Reload the todo file anyway");
			if !self.window.get_confirm() {
				return Ok(None)
			}
		}
		Ok(Some(
			GitInteractive::new_from_filepath(filepath.as_str(),
				&(self.config.comment_char))?
		))
	}
}

#[cfg(test)]
mod tests {
	use super::{
		Application,
		State
	};
	use git_interactive::GitInteractive;
	use window::{
		Window,
		PancursesInput
	};
	use action::Action;
	use config::Config;

	fn test_setup(todo_file: &str, comment_char: &str) -> Application {
		let gi = GitInteractive::new_from_filepath(todo_file, comment_char).unwrap();
		let config = Config::new().unwrap();
		let window = Window::new(config.clone());
		Application::new(gi, window, config)
	}

	#[test]
	fn application_read_all_actions() {
		let app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		assert_eq!(app.git_interactive.get_lines().len(), 14);
	}
	
	#[test]
	fn application_show_help() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::Character('?');
		app.process_input();
		assert_eq!(app.state, State::Help);
	}
	
	#[test]
	fn application_show_commit() {
		// first commit in
		let mut app = test_setup("test/git-rebase-todo-show-commit.in", "#");
		app.window.window.next_char = PancursesInput::Character('c');
		app.process_input();
		assert_eq!(app.state, State::ShowCommit);
	}
	
	#[test]
	fn application_scroll_basic() {
		let mut app = test_setup("test/git-rebase-todo-long.in", "#");
		app.window.window.next_char = PancursesInput::KeyDown;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 2);
		app.window.window.next_char = PancursesInput::KeyUp;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 1);
		app.window.window.next_char = PancursesInput::KeyNPage;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 6);
		app.window.window.next_char = PancursesInput::KeyPPage;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 1);
	}

	#[test]
	fn application_scroll_limits() {
		let mut app = test_setup("test/git-rebase-todo-short.in", "#");
		app.window.window.next_char = PancursesInput::KeyUp;
		app.process_input();
		app.process_input();
		app.window.window.next_char = PancursesInput::KeyPPage;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 1);
		app.window.window.next_char = PancursesInput::KeyDown;
		app.process_input();
		app.process_input();
		app.process_input();
		app.process_input();
		app.process_input();
		app.window.window.next_char = PancursesInput::KeyNPage;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 3);
	}
	
	#[test]
	fn application_set_pick() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		// first item is already pick
		app.window.window.next_char = PancursesInput::KeyDown;
		app.process_input();
		app.window.window.next_char = PancursesInput::Character('p');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[1].get_action(), Action::Pick);
	}
	
	#[test]
	fn application_set_reword() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::Character('r');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Reword);
	}
	
	#[test]
	fn application_set_edit() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::Character('e');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Edit);
	}

	#[test]
	fn application_not_set_exec_action() {
		let mut app = test_setup("test/git-rebase-todo-exec.in", "#");
		app.window.window.next_char = PancursesInput::Character('p');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Exec);
		app.window.window.next_char = PancursesInput::Character('r');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Exec);
		app.window.window.next_char = PancursesInput::Character('e');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Exec);
		app.window.window.next_char = PancursesInput::Character('s');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Exec);
		app.window.window.next_char = PancursesInput::Character('f');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Exec);
		app.window.window.next_char = PancursesInput::Character('d');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Exec);
	}

	#[test]
	fn application_set_squash() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::Character('s');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Squash);
	}
	
	#[test]
	fn application_set_drop() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::Character('d');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Drop);
	}
	
	#[test]
	fn application_swap_down() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::Character('j');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_hash_or_command(), "bbb");
		assert_eq!(*app.git_interactive.get_lines()[1].get_hash_or_command(), "aaa");
		assert_eq!(*app.git_interactive.get_selected_line_index(), 2);
	}
	
	#[test]
	fn application_swap_up() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::KeyDown;
		app.process_input();
		app.window.window.next_char = PancursesInput::Character('k');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_hash_or_command(), "bbb");
		assert_eq!(*app.git_interactive.get_lines()[1].get_hash_or_command(), "aaa");
		assert_eq!(*app.git_interactive.get_selected_line_index(), 1);
	}
	
	#[test]
	fn application_quit() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::Character('Q');
		app.process_input();
		assert_eq!(app.exit_code.unwrap(), 0);
		assert!(app.git_interactive.get_lines().is_empty());
	}
	
	#[test]
	fn application_finish() {
		let mut app = test_setup("test/git-rebase-todo-all-actions.in", "#");
		app.window.window.next_char = PancursesInput::Character('W');
		app.process_input();
		assert_eq!(app.exit_code.unwrap(), 0);
		assert!(!app.git_interactive.get_lines().is_empty());
	}

	#[test]
	fn application_alternative_comment_character() {
		let mut app = test_setup("test/git-rebase-alternative-comment-character.in", "%");
		app.window.window.next_char = PancursesInput::Character('W');
		app.process_input();
		assert_eq!(app.exit_code.unwrap(), 0);
		assert!(!app.git_interactive.get_lines().is_empty());
	}
}
