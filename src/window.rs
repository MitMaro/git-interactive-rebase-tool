use std::cmp;
use std::path::PathBuf;
use std::process::{
	Command
};
use std::error::Error;
use pad::{PadStr, Alignment};

#[cfg(not(test))]
use pancurses as pancurses;

#[cfg(test)]
use mocks::mockcurses as pancurses;

pub use pancurses::Input as PancursesInput;

use action::{
	Action,
	action_to_str
};
use line::Line;

use commit::Commit;
use color::Color;
use config::Config;
use input::Input;

const COLOR_TABLE: [i16; 8] = [
	pancurses::COLOR_WHITE, // the default foreground color must be the first (see #77)
	pancurses::COLOR_BLACK,
	pancurses::COLOR_BLUE,
	pancurses::COLOR_CYAN,
	pancurses::COLOR_GREEN,
	pancurses::COLOR_MAGENTA,
	pancurses::COLOR_RED,
	pancurses::COLOR_YELLOW,
];

pub struct Window {
	config: Config,
	pub window: pancurses::Window,
	top: usize
}

impl Window {
	pub fn new(config: Config) -> Self {
		let window = pancurses::initscr();
		window.keypad(true);

		pancurses::curs_set(0);
		pancurses::noecho();

		if pancurses::has_colors() {
			pancurses::start_color();
		}
		pancurses::use_default_colors();

		for (i, color) in COLOR_TABLE.iter().enumerate() {
			pancurses::init_pair(i as i16, *color, -1);
		}
		

		Window {
			config,
			window,
			top: 0
		}
	}

	pub fn resize_term(&self) {
		pancurses::resize_term(0, 0);
	}
	
	pub fn draw(&self, lines: &[Line], selected_index: usize) {
		self.window.clear();
		self.draw_title();
		let window_height = self.get_window_height();
		
		if self.top > 0 {
			self.draw_more_indicator(self.top);
		}
		self.window.addstr("\n");

		let mut index: usize = self.top + 1;
		for line in lines
			.iter()
			.skip(self.top)
			.take(window_height)
		{
			self.draw_line(line, index == selected_index);
			index += 1;
		}
		if window_height < lines.len() - self.top {
			self.draw_more_indicator((lines.len() - window_height - self.top) as usize);
		}
		self.window.addstr("\n");
		self.draw_footer();
		self.window.refresh();
	}

	fn draw_more_indicator(&self, remaining: usize) {
		self.set_color(self.config.foreground_color);
		self.window.attron(pancurses::A_DIM);
		self.window.attron(pancurses::A_REVERSE);
		self.window.addstr(&format!("  -- {} --  ", remaining));
		self.window.attroff(pancurses::A_REVERSE);
		self.window.attroff(pancurses::A_DIM);
	}

	fn draw_title(&self) {
		self.set_color(self.config.foreground_color);
		self.set_dim(true);
		self.set_underline(true);
		self.window.addstr("Git Interactive Rebase                       ? for help\n");
		self.set_underline(false);
		self.set_dim(false);
	}

	fn draw_line(&self, line: &Line, selected: bool) {
		self.set_color(self.config.foreground_color);
		if selected {
			self.window.addstr(" > ");
		}
		else {
			self.window.addstr("   ");
		}
		match *line.get_action() {
			Action::Pick => self.set_color(self.config.pick_color),
			Action::Reword => self.set_color(self.config.reword_color),
			Action::Edit => self.set_color(self.config.edit_color),
			Action::Exec => self.set_color(self.config.exec_color),
			Action::Squash => self.set_color(self.config.squash_color),
			Action::Fixup => self.set_color(self.config.fixup_color),
			Action::Drop => self.set_color(self.config.drop_color)
		}
		self.window.addstr(&format!("{:6}", action_to_str(line.get_action())));
		self.set_color(self.config.foreground_color);
		self.window.addstr(&format!(" {} {}\n", line.get_hash_or_command(), line.get_comment()));
	}

	fn draw_footer(&self) {
		self.set_color(self.config.foreground_color);
		self.set_dim(true);
		self.window.mvaddstr(
			self.window.get_max_y() - 1,
			0,
			"Actions: [ up, down, q/Q, w/W, c, j, k, p, r, e, s, f, d, ? ]"
		);
		self.set_dim(false);
	}
	
	pub fn draw_show_commit(&self, commit: &str, git_root: &PathBuf) {
		let result = Command::new("git")
			.current_dir(git_root)
			.args(&[
				"diff-tree",
				"--numstat",
				"--format=%aN%x1E%aE%x1E%ad%x1E%s%x1E%b%x1E",
				commit
			])
			.output()
		;
		
		self.window.clear();
		self.draw_title();
		match result {
			Ok(output) => {
				self.set_color(self.config.foreground_color);
				match Commit::new(&String::from_utf8_lossy(&output.stdout)) {
					Ok(commit_data) => {
						self.set_color(self.config.indicator_color);
						self.window.addstr(&format!("\nCommit: {}\n", commit));
						self.set_color(self.config.foreground_color);
						self.window.addstr(&format!(
							"Author: {} <{}>\n", commit_data.get_author_name(), commit_data.get_author_email()
						));
						self.window.addstr(&format!(
							"Date: {}\n",
							commit_data.get_date()
						));
						
						self.window.addstr(&format!(
							"\n{}\n\n{}\n",
							commit_data.get_subject(),
							commit_data.get_body()
						));
						let max_add_change_length = commit_data
							.get_file_stats()
							.iter()
							.fold(0, |a, x| cmp::max(a, x.get_added().len()));
						
						let max_remove_change_length = commit_data
							.get_file_stats()
							.iter()
							.fold(0, |a, x| cmp::max(a, x.get_added().len()));
						
						for file_stat in commit_data.get_file_stats() {
							self.set_color(self.config.diff_add_color);
							self.window.addstr(
								&file_stat.get_added().pad_to_width_with_alignment(max_add_change_length, Alignment::Right)
							);
							self.set_color(self.config.foreground_color);
							self.window.addstr(" | ");
							self.set_color(self.config.diff_remove_color);
							self.window.addstr(
								&file_stat.get_removed().pad_to_width_with_alignment(max_remove_change_length, Alignment::Left)
							);
							self.set_color(self.config.foreground_color);
							self.window.addstr(&format!("  {}\n", &file_stat.get_name()));
						}
					},
					Err(msg) => {
						self.set_color(self.config.error_color);
						self.window.addstr(&msg);
					}
				}
			},
			Err(msg) => {
				self.set_color(self.config.error_color);
				self.window.addstr(msg.description());
			}
		}
		self.set_color(self.config.indicator_color);
		self.window.addstr("\n\nHit any key to close");
		self.window.refresh();
	}
	
	pub fn draw_help(&self) {
		self.window.clear();
		self.draw_title();
		self.set_color(self.config.foreground_color);
		self.window.addstr("\n Key        Action\n");
		self.window.addstr(" --------------------------------------------------\n");
		self.draw_help_command("Up", "Move selection up");
		self.draw_help_command("Down", "Move selection down");
		self.draw_help_command("Page Up", "Move selection up 5 lines");
		self.draw_help_command("Page Down", "Move selection down 5 lines");
		self.draw_help_command("q", "Abort interactive rebase");
		self.draw_help_command("Q", "Immediately abort interactive rebase");
		self.draw_help_command("w", "Write interactive rebase file");
		self.draw_help_command("W", "Immediately write interactive rebase file");
		self.draw_help_command("?", "Show help");
		self.draw_help_command("c", "Show commit information");
		self.draw_help_command("j", "Move selected commit down");
		self.draw_help_command("k", "Move selected commit up");
		self.draw_help_command("p", "Set selected commit to be picked");
		self.draw_help_command("r", "Set selected commit to be reworded");
		self.draw_help_command("e", "Set selected commit to be edited");
		self.draw_help_command("s", "Set selected commit to be squashed");
		self.draw_help_command("f", "Set selected commit to be fixed-up");
		self.draw_help_command("d", "Set selected commit to be dropped");
		self.set_color(self.config.indicator_color);
		self.window.addstr("\n\nHit any key to close help");
		self.window.refresh();
	}
	
	fn draw_help_command(&self, command: &str, help: &str) {
		self.set_color(self.config.indicator_color);
		self.window.addstr(&format!(" {:9}    ", command));
		self.set_color(self.config.foreground_color);
		self.window.addstr(&format!("{}\n", help));
	}

	fn set_color(&self, color: Color) {
		match color {
			Color::White => self.window.attrset(pancurses::COLOR_PAIR(0)),
			Color::Black => self.window.attrset(pancurses::COLOR_PAIR(1)),
			Color::Blue => self.window.attrset(pancurses::COLOR_PAIR(2)),
			Color::Cyan => self.window.attrset(pancurses::COLOR_PAIR(3)),
			Color::Green => self.window.attrset(pancurses::COLOR_PAIR(4)),
			Color::Magenta => self.window.attrset(pancurses::COLOR_PAIR(5)),
			Color::Red => self.window.attrset(pancurses::COLOR_PAIR(6)),
			Color::Yellow => self.window.attrset(pancurses::COLOR_PAIR(7))
		};
	}

	fn set_dim(&self, on: bool) {
		if on {
			self.window.attron(pancurses::A_DIM);
		}
		else {
			self.window.attroff(pancurses::A_DIM);
		}
	}

	fn set_underline(&self, on: bool) {
		if on {
			self.window.attron(pancurses::A_UNDERLINE);
		}
		else {
			self.window.attroff(pancurses::A_UNDERLINE);
		}
	}

	pub fn get_input(&self) -> Input {
		match self.window.getch() {
			Some(PancursesInput::Character(c)) if c == '?' => Input::Help,
			Some(PancursesInput::Character(c)) if c == 'c' => Input::ShowCommit,
			Some(PancursesInput::Character(c)) if c == 'q' => Input::Abort,
			Some(PancursesInput::Character(c)) if c == 'Q' => Input::ForceAbort,
			Some(PancursesInput::Character(c)) if c == 'w' => Input::Rebase,
			Some(PancursesInput::Character(c)) if c == 'W' => Input::ForceRebase,
			Some(PancursesInput::Character(c)) if c == 'p' => Input::Pick,
			Some(PancursesInput::Character(c)) if c == 'r' => Input::Reword,
			Some(PancursesInput::Character(c)) if c == 'e' => Input::Edit,
			Some(PancursesInput::Character(c)) if c == 's' => Input::Squash,
			Some(PancursesInput::Character(c)) if c == 'f' => Input::Fixup,
			Some(PancursesInput::Character(c)) if c == 'd' => Input::Drop,
			Some(PancursesInput::Character(c)) if c == 'j' => Input::SwapSelectedDown,
			Some(PancursesInput::Character(c)) if c == 'k' => Input::SwapSelectedUp,
			Some(PancursesInput::KeyDown) => Input::MoveCursorDown,
			Some(PancursesInput::KeyUp) => Input::MoveCursorUp,
			Some(PancursesInput::KeyPPage) => Input::MoveCursorPageUp,
			Some(PancursesInput::KeyNPage) => Input::MoveCursorPageDown,
			Some(PancursesInput::KeyResize) => Input::Resize,
			_ => Input::Other,
		}
	}

	pub fn draw_prompt(&self, message: &str) {
		self.window.clear();
		self.draw_title();
		self.window.addstr(&format!("\n{} ", message));
	}

	pub fn draw_confirm(&self, message: &str) {
		self.draw_prompt(&format!("{}(y/n)? ", message));
	}

	pub fn get_confirm(&self) -> bool {
		match self.window.getch() {
			Some(PancursesInput::Character(c)) if c == 'y' || c == 'Y' => true,
			_ => false
		}
	}

	pub fn set_top(&mut self, line_length: usize, selected_index: usize) {
		let window_height = self.get_window_height();
		self.top = match selected_index {
			_ if line_length <= window_height => 0,
			s if s == line_length => line_length - window_height,
			s if self.top + 1 > s => s - 1,
			s if self.top + window_height <= s => s - window_height + 1,
			_ => self.top
		};
	}

	fn get_window_height(&self) -> usize {
		match self.window.get_max_y() {
			// 4 removed for other UI lines
			x if x >= 4 => (x - 4) as usize,
			_ => 4
		}
	}

	pub fn end(&self) {
		self.window.clear();
		self.window.refresh();
		pancurses::curs_set(1);
		pancurses::endwin();
	}
}
