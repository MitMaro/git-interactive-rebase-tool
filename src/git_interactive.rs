use crate::list::action::Action;
use crate::list::line::Line;
use anyhow::{anyhow, Result};
use std::cmp;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

fn load_filepath(path: &PathBuf, comment_char: &str) -> Result<Vec<Line>> {
	read_to_string(&path)
		.map_err(|err| anyhow!("Error reading file: {}", path.display()).context(err))?
		.lines()
		.filter_map(|l| {
			if l.starts_with(comment_char) || l.is_empty() {
				None
			}
			else {
				Some(Line::new(l).map_err(|err| anyhow!("Error reading file: {}", path.display()).context(err)))
			}
		})
		.collect()
}

pub struct GitInteractive {
	filepath: PathBuf,
	lines: Vec<Line>,
	selected_line_index: usize,
	visual_index_start: Option<usize>,
	comment_char: String,
}

impl GitInteractive {
	pub(crate) fn new(lines: Vec<Line>, path: PathBuf, comment_char: &str) -> Result<Self> {
		Ok(Self {
			filepath: path,
			lines,
			selected_line_index: 1,
			visual_index_start: None,
			comment_char: String::from(comment_char),
		})
	}

	pub(crate) fn new_from_filepath(filepath: &str, comment_char: &str) -> Result<Self> {
		let path = PathBuf::from(filepath);
		let lines = load_filepath(&path, comment_char)?;
		Self::new(lines, path, comment_char)
	}

	pub(crate) fn write_file(&self) -> Result<()> {
		let mut file = File::create(&self.filepath)
			.map_err(|err| anyhow!(err).context(anyhow!("Error opening file: {}", self.filepath.display())))?;
		for line in &self.lines {
			writeln!(file, "{}", line.to_text())
				.map_err(|err| anyhow!(err).context(anyhow!("Error writing file: {}", self.filepath.display())))?;
		}
		Ok(())
	}

	pub(crate) fn set_lines(&mut self, lines: Vec<Line>) {
		self.lines = lines;
	}

	pub(crate) fn reload_file(&mut self) -> Result<()> {
		self.lines = load_filepath(&self.filepath, self.comment_char.as_str())?;
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
		self.visual_index_start = Some(self.selected_line_index);
	}

	pub(crate) fn end_visual_mode(&mut self) {
		self.visual_index_start = None;
	}

	pub(crate) fn swap_range_up(&mut self) {
		let end_index = self.visual_index_start.unwrap_or(self.selected_line_index);
		let start_index = self.selected_line_index;

		if end_index == 1 || start_index == 1 {
			return;
		}

		let range = if end_index <= start_index {
			end_index..=start_index
		}
		else {
			start_index..=end_index
		};

		for index in range {
			self.lines.swap(index - 1, index - 2);
		}

		if let Some(visual_index_start) = self.visual_index_start {
			self.visual_index_start = Some(visual_index_start - 1);
		}
		self.move_cursor_up(1);
	}

	pub(crate) fn swap_range_down(&mut self) {
		let end_index = self.visual_index_start.unwrap_or(self.selected_line_index);
		let start_index = self.selected_line_index;

		if end_index == self.lines.len() || start_index == self.lines.len() {
			return;
		}

		let range = if end_index <= start_index {
			end_index..=start_index
		}
		else {
			start_index..=end_index
		};

		for index in range.rev() {
			self.lines.swap(index - 1, index);
		}

		if let Some(visual_index_start) = self.visual_index_start {
			self.visual_index_start = Some(visual_index_start + 1);
		}

		self.move_cursor_down(1);
	}

	pub(crate) fn edit_selected_line(&mut self, content: &str) {
		self.lines[self.selected_line_index - 1].edit_content(content);
	}

	pub(crate) fn set_range_action(&mut self, action: Action) {
		let end_index = self.visual_index_start.unwrap_or(self.selected_line_index);
		let start_index = self.selected_line_index;

		let range = if start_index <= end_index {
			start_index..=end_index
		}
		else {
			end_index..=start_index
		};

		for index in range {
			let selected_action = self.lines[index - 1].get_action();
			if *selected_action != Action::Exec && *selected_action != Action::Break {
				self.lines[index - 1].set_action(action);
			}
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

	pub(crate) fn get_selected_line(&self) -> &Line {
		&self.lines[self.selected_line_index - 1]
	}

	pub(crate) const fn get_selected_line_index(&self) -> &usize {
		&self.selected_line_index
	}

	pub(crate) fn get_visual_start_index(&self) -> usize {
		self.visual_index_start.unwrap_or(self.selected_line_index)
	}

	pub(crate) const fn get_filepath(&self) -> &PathBuf {
		&self.filepath
	}

	pub(crate) const fn get_lines(&self) -> &Vec<Line> {
		&self.lines
	}
}
