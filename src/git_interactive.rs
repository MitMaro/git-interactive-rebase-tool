use std::cmp;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::io::Write;
use std::ops::RangeInclusive;

use action::Action;
use line::Line;

pub struct GitInteractive {
	git_root: PathBuf,
	filepath: PathBuf,
	lines: Vec<Line>,
	selected_line_index: usize,
	anchor_line_index: Option<usize>,
}

impl GitInteractive {
	pub fn new_from_filepath(filepath: &str, comment_char: &str) -> Result<Self, String> {
		let path = PathBuf::from(filepath);

		let mut file = match File::open(&path) {
			Ok(file) => file,
			Err(why) => {
				return Err(format!(
					"Error opening file, {}\n\
					Reason: {}", path.display(), why
				));
			}
		};

		let mut s = String::new();
		match file.read_to_string(&mut s) {
			Ok(_) => {},
			Err(why) => {
				return Err(format!(
					"Error reading file, {}\n\
					Reason: {}", path.display(), why
				));
			}
		}

		let mut git_root = PathBuf::from(filepath);
		git_root.pop();

		// catch noop rebases
		let parsed_result = match s.lines().nth(0) {
			Some("noop") => Ok(Vec::new()),
			_ => {
				s.lines()
					.filter(|l| !l.starts_with(comment_char) && !l.is_empty())
					.map(|l| Line::new(l))
					.collect()
			}
		};
		
		match parsed_result {
			Ok(lines) => Ok(
				GitInteractive {
					git_root,
					filepath: path,
					lines,
					selected_line_index: 1,
					anchor_line_index: None,
				}
			),
			Err(e) => Err(format!(
				"Error reading file, {}\n\
				Reason: {}", path.display(), e
			))
		}
	}
	
	pub fn write_file(&self) -> Result<String, String> {
		let mut file = match File::create(&self.filepath) {
			Ok(file) => file,
			Err(why) => {
				return Err(format!(
					"Error opening file, {}\n\
					Reason: {}", self.filepath.display(), why
				));
			}
		};
		
		for line in &self.lines {
			match writeln!(file, "{}", line.to_text()) {
				Ok(_) => {},
				Err(why) => {
					return Err(format!(
						"Error writing to file, {}", why
					));
				}
			}
		}
		// Return the path to the todo file. Since it was constructed from str originally,
		// converting back should be safe...
		Ok(self.filepath.to_str().map(String::from).unwrap())
	}
	
	pub fn clear(&mut self) {
		self.lines.clear();
	}
	
	pub fn toggle_selection(&mut self) {
		self.anchor_line_index = match self.anchor_line_index {
			None => Some(self.selected_line_index),
			Some(_) => None
		}
	}

	pub fn move_cursor_up(&mut self, amount: usize) {
		self.selected_line_index = match amount {
			a if a >= self.selected_line_index => 1,
			_ => self.selected_line_index - amount
		}
	}
	
	pub fn move_cursor_down(&mut self, amount: usize) {
		self.selected_line_index = cmp::min(self.selected_line_index + amount, self.lines.len());
	}
	
	fn get_selected_lines(&self) -> RangeInclusive<usize> { // 1-based
		let sel = self.selected_line_index;
		let anc = self.anchor_line_index.unwrap_or(sel);
		RangeInclusive::new(cmp::min(anc, sel), cmp::max(anc, sel))
	}

	pub fn swap_selected_up(&mut self) {
		let sel_range = self.get_selected_lines();
		if *sel_range.start() == 1usize {
			return
		}
		for line_index in sel_range {
			// move each line in the selection one line up
			self.lines.swap(line_index - 1, line_index - 2);
		}
		self.selected_line_index -= 1;
		if let Some(a) = self.anchor_line_index {
			self.anchor_line_index = Some(a - 1);
		}
	}
	
	pub fn swap_selected_down(&mut self) {
		let sel_range = self.get_selected_lines();
		if *sel_range.end() == self.lines.len() {
			return
		}
		for line_index in sel_range.rev() {
			self.lines.swap(line_index - 1, line_index);
		}
		self.selected_line_index += 1;
		if let Some(a) = self.anchor_line_index {
			self.anchor_line_index = Some(a + 1);
		}
	}
	
	pub fn set_selected_line_action(&mut self, action: Action) {
		for line_index in self.get_selected_lines() {
			if *self.lines[line_index - 1].get_action() != Action::Exec {
				self.lines[line_index - 1].set_action(action);
			}
		}
	}
	
	pub fn get_selected_line_hash(&self) -> &String {
		self.lines[self.selected_line_index - 1].get_hash_or_command()
	}
	
	pub fn get_selected_line_index(&self) -> &usize {
		&self.selected_line_index
	}

	pub fn get_anchor_line_index(&self) -> Option<usize> {
		self.anchor_line_index
	}
	
	pub fn get_git_root(&self) -> &PathBuf {
		&self.git_root
	}
	
	pub fn get_lines(&self) -> &Vec<Line> {
		&self.lines
	}
}
