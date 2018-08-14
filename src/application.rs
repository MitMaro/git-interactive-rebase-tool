use action::Action;
use git_interactive::GitInteractive;

use window::{
	Input,
	Window
};

const EXIT_CODE_GOOD: i32 = 0;
const EXIT_CODE_WRITE_ERROR: i32 = 8;

#[derive(PartialEq, Debug)]
pub enum State {
	List,
	ShowCommit,
	Help,
    ViewDiff
}

pub struct Application {
	pub exit_code: Option<i32>,
	window: Window,
	git_interactive: GitInteractive,
	state: State
}

impl Application {
	pub fn new(git_interactive: GitInteractive, window: Window) -> Self {
		Application {
			git_interactive,
			window,
			exit_code: None,
			state: State::List
		}
	}
	
	pub fn process_input(&mut self) {
		match self.state {
			State::List => self.process_list_input(),
			State::ShowCommit => self.process_show_commit_input(),
			State::Help => self.process_help_input(),
            State::ViewDiff => self.process_view_diff_input()
		}
	}
	
	pub fn draw(&self) {
		match self.state {
			State::List => {
				self.window.draw(
					self.git_interactive.get_lines(),
					self.git_interactive.get_selected_line_index()
				);
			},
			State::ShowCommit => {
				self.window.draw_show_commit(
					self.git_interactive.get_selected_line_hash(),
					self.git_interactive.get_git_root()
				);
			},
			State::Help => {
				self.window.draw_help();
			},
            State::ViewDiff => {
                self.window.draw_view_diff(
                    self.git_interactive.get_selected_line_hash(),
                    self.git_interactive.get_git_root()
                );
            }
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
		self.window.window.getch();
		self.state = State::List;
	}
	
	fn process_show_commit_input(&mut self) {
		self.window.window.getch();
		self.state = State::List;
	}

    fn process_view_diff_input(&mut self) {
        self.state = State::List
    }
	
    fn process_list_input(&mut self) {
		match self.window.window.getch() {
            Some(Input::Character(c)) if c == 'v' => {
                self.state = State::ViewDiff;
            },
			Some(Input::Character(c)) if c == '?' => {
				self.state = State::Help;
			},
			Some(Input::Character(c)) if c == 'c' => {
				self.state = State::ShowCommit;
			},
			Some(Input::Character(c))
				if (c == 'Q') || (c == 'q' && self.window.confirm("Are you sure you want to abort"))
					=> self.abort(),
			Some(Input::Character(c))
				if (c == 'W') || (c == 'w' && self.window.confirm("Are you sure you want to rebase"))
					=> self.finish(),
			Some(Input::Character(c))
				if c == 'p' => self.git_interactive.set_selected_line_action(Action::Pick),
			Some(Input::Character(c))
				if c == 'r' => self.git_interactive.set_selected_line_action(Action::Reword),
			Some(Input::Character(c))
				if c == 'e' => self.git_interactive.set_selected_line_action(Action::Edit),
			Some(Input::Character(c))
				if c == 's' => self.git_interactive.set_selected_line_action(Action::Squash),
			Some(Input::Character(c))
				if c == 'f' => self.git_interactive.set_selected_line_action(Action::Fixup),
			Some(Input::Character(c))
				if c == 'd' => self.git_interactive.set_selected_line_action(Action::Drop),
			Some(Input::Character(c)) if c == 'j' => {
				self.git_interactive.swap_selected_down();
				self.reset_top();
			},
			Some(Input::Character(c)) if c == 'k' => {
				self.git_interactive.swap_selected_up();
				self.reset_top();
			},
			Some(Input::KeyDown) => {
				self.git_interactive.move_cursor_down(1);
				self.reset_top();
			},
			Some(Input::KeyUp) => {
				self.git_interactive.move_cursor_up(1);
				self.reset_top();
			},
			Some(Input::KeyPPage) => {
				self.git_interactive.move_cursor_up(5);
				self.reset_top();
			},
			Some(Input::KeyNPage) => {
				self.git_interactive.move_cursor_down(5);
				self.reset_top();
			},
			Some(Input::KeyResize) => self.reset_top(),
			_ => {}
		}
		()
	}
	
	fn reset_top(&mut self) {
		self.window.set_top(
			self.git_interactive.get_lines().len(),
			*self.git_interactive.get_selected_line_index()
		)
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
		Input
	};
	use action::Action;
	
	#[test]
	fn application_read_all_actions() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let app = Application::new(gi, window);
		assert_eq!(app.git_interactive.get_lines().len(), 12);
	}
	
	#[test]
	fn application_show_help() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('?');
		app.process_input();
		assert_eq!(app.state, State::Help);
	}
	
	#[test]
	fn application_show_commit() {
		// first commit in
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-show-commit.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('c');
		app.process_input();
		assert_eq!(app.state, State::ShowCommit);
	}
	
	#[test]
	fn application_scroll_basic() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-long.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::KeyDown;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 2);
		app.window.window.next_char = Input::KeyUp;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 1);
		app.window.window.next_char = Input::KeyNPage;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 6);
		app.window.window.next_char = Input::KeyPPage;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 1);
	}

	#[test]
	fn application_scroll_limits() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-short.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::KeyUp;
		app.process_input();
		app.process_input();
		app.window.window.next_char = Input::KeyPPage;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 1);
		app.window.window.next_char = Input::KeyDown;
		app.process_input();
		app.process_input();
		app.process_input();
		app.process_input();
		app.process_input();
		app.window.window.next_char = Input::KeyNPage;
		app.process_input();
		assert_eq!(*app.git_interactive.get_selected_line_index(), 3);
	}
	
	#[test]
	fn application_set_pick() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		// first item is already pick
		app.window.window.next_char = Input::KeyDown;
		app.process_input();
		app.window.window.next_char = Input::Character('p');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[1].get_action(), Action::Pick);
	}
	
	#[test]
	fn application_set_reword() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('r');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Reword);
	}
	
	#[test]
	fn application_set_edit() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('e');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Edit);
	}
	
	#[test]
	fn application_set_squash() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('s');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Squash);
	}
	
	#[test]
	fn application_set_drop() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('d');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_action(), Action::Drop);
	}
	
	#[test]
	fn application_swap_down() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('j');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_hash(), "bbb");
		assert_eq!(*app.git_interactive.get_lines()[1].get_hash(), "aaa");
		assert_eq!(*app.git_interactive.get_selected_line_index(), 2);
	}
	
	#[test]
	fn application_swap_up() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::KeyDown;
		app.process_input();
		app.window.window.next_char = Input::Character('k');
		app.process_input();
		assert_eq!(*app.git_interactive.get_lines()[0].get_hash(), "bbb");
		assert_eq!(*app.git_interactive.get_lines()[1].get_hash(), "aaa");
		assert_eq!(*app.git_interactive.get_selected_line_index(), 1);
	}
	
	#[test]
	fn application_quit() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('Q');
		app.process_input();
		assert_eq!(app.exit_code.unwrap(), 0);
		assert!(app.git_interactive.get_lines().is_empty());
	}
	
	#[test]
	fn application_finish() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-todo-all-actions.in", "#").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('W');
		app.process_input();
		assert_eq!(app.exit_code.unwrap(), 0);
		assert!(!app.git_interactive.get_lines().is_empty());
	}

	#[test]
	fn application_alternative_comment_character() {
		let gi = GitInteractive::new_from_filepath("test/git-rebase-alternative-comment-character.in", "%").unwrap();
		let window = Window::new();
		let mut app = Application::new(gi, window);
		app.window.window.next_char = Input::Character('W');
		app.process_input();
		assert_eq!(app.exit_code.unwrap(), 0);
		assert!(!app.git_interactive.get_lines().is_empty());
	}
}
