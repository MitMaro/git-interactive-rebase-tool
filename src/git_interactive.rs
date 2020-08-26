use crate::list::action::Action;
use crate::list::line::Line;
use std::cmp;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

fn load_filepath(path: &PathBuf, comment_char: &str) -> Result<Vec<Line>, String> {
	read_to_string(&path)
		.map_err(|why| format!("Error reading file, {}\nReason: {}", path.display(), why))?
		.lines()
		.filter_map(|l| {
			if l.starts_with(comment_char) || l.is_empty() {
				None
			}
			else {
				Some(Line::new(l).map_err(|e| format!("Error reading file, {}", e)))
			}
		})
		.collect()
}

pub struct GitInteractive {
	filepath: PathBuf,
	lines: Vec<Line>,
	selected_line_index: usize,
	visual_index_start: usize,
	comment_char: String,
}

impl GitInteractive {
	pub(crate) fn new(lines: Vec<Line>, path: PathBuf, comment_char: &str) -> Result<Self, String> {
		Ok(Self {
			filepath: path,
			lines,
			selected_line_index: 1,
			visual_index_start: 1,
			comment_char: String::from(comment_char),
		})
	}

	pub(crate) fn new_from_filepath(filepath: &str, comment_char: &str) -> Result<Self, String> {
		let path = PathBuf::from(filepath);
		let lines = load_filepath(&path, comment_char)?;
		Self::new(lines, path, comment_char)
	}

	pub(crate) fn write_file(&self) -> Result<(), String> {
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
		for line in &self.lines {
			match writeln!(file, "{}", line.to_text()) {
				Ok(_) => {},
				Err(why) => {
					return Err(format!("Error writing to file, {}", why));
				},
			}
		}
		Ok(())
	}

	pub(crate) fn reload_file(&mut self) -> Result<(), String> {
		let lines = load_filepath(&self.filepath, self.comment_char.as_str())?;

		self.lines = lines;
		Ok(())
	}

	pub(crate) fn clear(&mut self) {
		self.lines.clear();
	}

	pub(crate) fn move_cursor_up(&mut self, amount: usize) {
		self.selected_line_index = match amount {
			a if a >= self.selected_line_index => 1,
			_ => self.selected_line_index - amount,
		};
	}

	pub(crate) fn move_cursor_down(&mut self, amount: usize) {
		self.selected_line_index = cmp::min(self.selected_line_index + amount, self.lines.len());
	}

	pub(crate) fn start_visual_mode(&mut self) {
		self.visual_index_start = self.selected_line_index;
	}

	#[allow(clippy::range_plus_one)]
	pub(crate) fn swap_visual_range_up(&mut self) {
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

	pub(crate) fn swap_selected_up(&mut self) {
		if self.selected_line_index == 1 {
			return;
		}
		self.lines
			.swap(self.selected_line_index - 1, self.selected_line_index - 2);
		self.move_cursor_up(1);
	}

	#[allow(clippy::range_plus_one)]
	pub(crate) fn swap_visual_range_down(&mut self) {
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

	pub(crate) fn swap_selected_down(&mut self) {
		if self.selected_line_index == self.lines.len() {
			return;
		}
		self.lines.swap(self.selected_line_index - 1, self.selected_line_index);
		self.move_cursor_down(1);
	}

	pub(crate) fn edit_selected_line(&mut self, content: &str) {
		self.lines[self.selected_line_index - 1].edit_content(content);
	}

	pub(crate) fn get_selected_line_edit_content(&self) -> &String {
		self.lines[self.selected_line_index - 1].get_edit_content()
	}

	#[allow(clippy::range_plus_one)]
	pub(crate) fn set_visual_range_action(&mut self, action: Action) {
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

	pub(crate) fn set_selected_line_action(&mut self, action: Action) {
		let selected_action = self.lines[self.selected_line_index - 1].get_action();
		if *selected_action != Action::Exec && *selected_action != Action::Break {
			self.lines[self.selected_line_index - 1].set_action(action);
		}
	}

	pub(crate) fn toggle_break(&mut self) {
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

	pub(crate) fn is_noop(&self) -> bool {
		!self.lines.is_empty() && *self.lines[0].get_action() == Action::Noop
	}

	pub(crate) fn get_selected_line_hash(&self) -> &String {
		self.lines[self.selected_line_index - 1].get_hash()
	}

	pub(crate) fn get_selected_line_action(&self) -> &Action {
		self.lines[self.selected_line_index - 1].get_action()
	}

	pub(crate) const fn get_selected_line_index(&self) -> &usize {
		&self.selected_line_index
	}

	pub(crate) const fn get_visual_start_index(&self) -> &usize {
		&self.visual_index_start
	}

	pub(crate) const fn get_filepath(&self) -> &PathBuf {
		&self.filepath
	}

	pub(crate) const fn get_lines(&self) -> &Vec<Line> {
		&self.lines
	}
}
