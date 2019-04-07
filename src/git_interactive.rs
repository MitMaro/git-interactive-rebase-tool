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
	match s.lines().nth(0) {
		Some("noop") => Ok(Vec::new()),
		_ => {
			s.lines()
				.filter(|l| !l.starts_with(comment_char) && !l.is_empty())
				.map(|l| {
					match Line::new(l) {
						Ok(line) => Ok(line),
						Err(e) => Err(format!("Error reading file, {}", e)),
					}
				})
				.collect()
		},
	}
}

pub struct GitInteractive {
	filepath: PathBuf,
	lines: Vec<Line>,
	selected_commit_stats: Option<Commit>,
	selected_line_index: usize,
}

impl GitInteractive {
	pub fn new_from_filepath(filepath: &str, comment_char: &str) -> Result<Self, String> {
		let path = PathBuf::from(filepath);
		let lines = load_filepath(&path, comment_char)?;

		Ok(GitInteractive {
			filepath: path,
			lines,
			selected_commit_stats: None,
			selected_line_index: 1,
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

	pub fn swap_selected_up(&mut self) {
		if self.selected_line_index == 1 {
			return;
		}
		self.lines
			.swap(self.selected_line_index - 1, self.selected_line_index - 2);
		self.move_cursor_up(1);
	}

	pub fn swap_selected_down(&mut self) {
		if self.selected_line_index == self.lines.len() {
			return;
		}
		self.lines.swap(self.selected_line_index - 1, self.selected_line_index);
		self.move_cursor_down(1);
	}

	pub fn set_selected_line_action(&mut self, action: Action) {
		if *self.lines[self.selected_line_index - 1].get_action() != Action::Exec {
			self.lines[self.selected_line_index - 1].set_action(action);
		}
	}

	// TODO this is kind of clunky and might be replaceable with a RefCell
	pub fn load_commit_stats(&mut self) -> Result<(), String> {
		self.selected_commit_stats = Some(Commit::from_commit_hash(self.get_selected_line_hash())?);
		Ok(())
	}

	pub fn get_commit_stats(&self) -> &Option<Commit> {
		&self.selected_commit_stats
	}

	pub fn get_commit_stats_length(&self) -> usize {
		match &self.selected_commit_stats {
			Some(s) => {
				let mut len = s.get_file_stats_length();

				match s.get_body() {
					Some(b) => {
						len += b.lines().count();
					},
					None => {},
				}
				len + 3 // author + date + commit hash
			},
			None => 0,
		}
	}

	pub fn get_selected_line_hash(&self) -> &String {
		self.lines[self.selected_line_index - 1].get_hash_or_command()
	}

	pub fn get_selected_line_index(&self) -> &usize {
		&self.selected_line_index
	}

	pub fn get_filepath(&self) -> &PathBuf {
		&self.filepath
	}

	pub fn get_lines(&self) -> &Vec<Line> {
		&self.lines
	}
}
