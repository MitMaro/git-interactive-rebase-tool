use std::cmp;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::io::Write;

use action::Action;
use line::Line;

pub struct GitInteractive {
	git_root: PathBuf,
	filepath: PathBuf,
	lines: Vec<Line>,
	selected_line_index: usize
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
					selected_line_index: 1
				}
			),
			Err(e) => Err(format!(
				"Error reading file, {}\n\
				Reason: {}", path.display(), e
			))
		}
	}
	
	pub fn write_file(&self) -> Result<(), String> {
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
		Ok(())
	}
	
	pub fn clear(&mut self) {
		self.lines.clear();
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
	
	pub fn swap_selected_up(&mut self) {
		if self.selected_line_index == 1 {
			return
		}
		self.lines.swap(self.selected_line_index - 1, self.selected_line_index - 2);
		self.move_cursor_up(1);
	}
	
	pub fn swap_selected_down(&mut self) {
		if self.selected_line_index == self.lines.len() {
			return
		}
		self.lines.swap(self.selected_line_index - 1, self.selected_line_index);
		self.move_cursor_down(1);
	}
	
	pub fn set_selected_line_action(&mut self, action: Action) {
		if *self.lines[self.selected_line_index - 1].get_action() != Action::Exec {
			self.lines[self.selected_line_index - 1].set_action(action);
		}
	}
	
	pub fn get_selected_line_hash(&self) -> &String {
		self.lines[self.selected_line_index - 1].get_hash_or_command()
	}
	
	pub fn get_selected_line_index(&self) -> &usize {
		&self.selected_line_index
	}
	
	pub fn get_git_root(&self) -> &PathBuf {
		&self.git_root
	}
	
	pub fn get_lines(&self) -> &Vec<Line> {
		&self.lines
	}
}
