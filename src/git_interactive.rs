use std::cmp;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::action::Action;
use crate::commit::Commit;
use crate::line::Line;

fn load_filepath(path: &PathBuf, comment_char: &str) -> Result<Vec<Line>, String> {
	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(why) => {
			return Err(format!("Error opening file, {}\nReason: {}", path.display(), why));
		},
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Ok(_) => {},
		Err(why) => {
			return Err(format!("Error reading file, {}\nReason: {}", path.display(), why));
		},
	}

	// catch noop rebases
	s.lines()
		.filter(|l| !l.starts_with(comment_char) && !l.is_empty())
		.map(|l| {
			match Line::new(l) {
				Ok(line) => Ok(line),
				Err(e) => Err(format!("Error reading file, {}", e)),
			}
		})
		.collect()
}

pub struct GitInteractive {
	filepath: PathBuf,
	lines: Vec<Line>,
	selected_line_index: usize,
	visual_index_start: usize,
}

impl GitInteractive {
	pub fn new_from_filepath(filepath: &str, comment_char: &str) -> Result<Self, String> {
		let path = PathBuf::from(filepath);
		let lines = load_filepath(&path, comment_char)?;

		Ok(GitInteractive {
			filepath: path,
			lines,
			selected_line_index: 1,
			visual_index_start: 1,
		})
	}

	pub fn write_file(&self) -> Result<(), String> {
		let mut file = match File::create(&self.filepath) {
			Ok(file) => file,
			Err(why) => {
				return Err(format!(
					"Error opening file, {}\nReason: {}",
					self.filepath.display(),
					why
				));
			},
		};
		for line in self.lines.iter() {
			match writeln!(file, "{}", line.to_text()) {
				Ok(_) => {},
				Err(why) => {
					return Err(format!("Error writing to file, {}", why));
				},
			}
		}
		Ok(())
	}

	pub fn reload_file(&mut self, comment_char: &str) -> Result<(), String> {
		let lines = load_filepath(&self.filepath, comment_char)?;

		self.lines = lines;
		Ok(())
	}

	pub fn clear(&mut self) {
		self.lines.clear();
	}

	pub fn move_cursor_up(&mut self, amount: usize) {
		self.selected_line_index = match amount {
			a if a >= self.selected_line_index => 1,
			_ => self.selected_line_index - amount,
		};
	}

	pub fn move_cursor_down(&mut self, amount: usize) {
		self.selected_line_index = cmp::min(self.selected_line_index + amount, self.lines.len());
	}

	pub fn start_visual_mode(&mut self) {
		self.visual_index_start = self.selected_line_index;
	}

	#[allow(clippy::range_plus_one)]
	pub fn swap_visual_range_up(&mut self) {
		if self.selected_line_index == 1 || self.visual_index_start == 1 {
			return;
		}

		let range = if self.selected_line_index <= self.visual_index_start {
			self.selected_line_index..self.visual_index_start + 1
		}
		else {
			self.visual_index_start..self.selected_line_index + 1
		};

		for index in range {
			self.lines.swap(index - 1, index - 2);
		}
		self.visual_index_start -= 1;
		self.move_cursor_up(1);
	}

	pub fn swap_selected_up(&mut self) {
		if self.selected_line_index == 1 {
			return;
		}
		self.lines
			.swap(self.selected_line_index - 1, self.selected_line_index - 2);
		self.move_cursor_up(1);
	}

	#[allow(clippy::range_plus_one)]
	pub fn swap_visual_range_down(&mut self) {
		if self.selected_line_index == self.lines.len() || self.visual_index_start == self.lines.len() {
			return;
		}

		let range = if self.selected_line_index <= self.visual_index_start {
			self.selected_line_index..self.visual_index_start + 1
		}
		else {
			self.visual_index_start..self.selected_line_index + 1
		};

		for index in range.rev() {
			self.lines.swap(index - 1, index);
		}
		self.visual_index_start += 1;
		self.move_cursor_down(1);
	}

	pub fn swap_selected_down(&mut self) {
		if self.selected_line_index == self.lines.len() {
			return;
		}
		self.lines.swap(self.selected_line_index - 1, self.selected_line_index);
		self.move_cursor_down(1);
	}

	pub fn edit_selected_line(&mut self, content: &str) {
		self.lines[self.selected_line_index - 1].edit_content(content);
	}

	pub fn get_selected_line_edit_content(&self) -> &String {
		self.lines[self.selected_line_index - 1].get_edit_content()
	}

	#[allow(clippy::range_plus_one)]
	pub fn set_visual_range_action(&mut self, action: Action) {
		let range = if self.selected_line_index <= self.visual_index_start {
			self.selected_line_index..self.visual_index_start + 1
		}
		else {
			self.visual_index_start..self.selected_line_index + 1
		};

		for index in range {
			let selected_action = self.lines[index - 1].get_action();
			if *selected_action != Action::Exec && *selected_action != Action::Break {
				self.lines[index - 1].set_action(action);
			}
		}
	}

	pub fn set_selected_line_action(&mut self, action: Action) {
		let selected_action = self.lines[self.selected_line_index - 1].get_action();
		if *selected_action != Action::Exec && *selected_action != Action::Break {
			self.lines[self.selected_line_index - 1].set_action(action);
		}
	}

	pub fn toggle_break(&mut self) {
		let selected_action = self.lines[self.selected_line_index - 1].get_action();
		if *selected_action == Action::Break {
			self.lines.remove(self.selected_line_index - 1);
			if self.selected_line_index != 1 {
				self.selected_line_index -= 1;
			}
		}
		else {
			self.lines.insert(self.selected_line_index, Line::new_break());
			if self.selected_line_index != self.lines.len() {
				self.selected_line_index += 1;
			}
		}
	}

	pub fn load_commit_stats(&self) -> Result<Commit, String> {
		let selected_action = self.lines[self.selected_line_index - 1].get_action();
		if *selected_action != Action::Exec && *selected_action != Action::Break {
			return Ok(Commit::from_commit_hash(self.get_selected_line_hash().as_str())?);
		}
		Err(String::from("Cannot load commit for the selected action"))
	}

	pub fn is_noop(&self) -> bool {
		!self.lines.is_empty() && *self.lines[0].get_action() == Action::Noop
	}

	pub fn get_selected_line_hash(&self) -> &String {
		self.lines[self.selected_line_index - 1].get_hash()
	}

	pub fn get_selected_line_action(&self) -> &Action {
		self.lines[self.selected_line_index - 1].get_action()
	}

	pub fn get_selected_line_index(&self) -> &usize {
		&self.selected_line_index
	}

	pub fn get_visual_start_index(&self) -> &usize {
		&self.visual_index_start
	}

	pub fn get_filepath(&self) -> &PathBuf {
		&self.filepath
	}

	pub fn get_lines(&self) -> &Vec<Line> {
		&self.lines
	}
}
