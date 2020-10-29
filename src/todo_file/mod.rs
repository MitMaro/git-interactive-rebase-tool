use action::Action;
use anyhow::{anyhow, Result};
use line::Line;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

pub mod action;
pub mod line;

pub struct EditContext {
	action: Option<Action>,
	content: Option<String>,
}

impl EditContext {
	pub const fn new() -> Self {
		Self {
			action: None,
			content: None,
		}
	}

	pub const fn action(mut self, action: Action) -> Self {
		self.action = Some(action);
		self
	}

	pub fn content(mut self, content: &str) -> Self {
		self.content = Some(content.to_string());
		self
	}

	pub const fn get_action(&self) -> &Option<Action> {
		&self.action
	}

	pub const fn get_content(&self) -> &Option<String> {
		&self.content
	}
}

pub struct TodoFile {
	comment_char: String,
	filepath: String,
	is_noop: bool,
	lines: Vec<Line>,
	selected_line_index: usize,
}

impl TodoFile {
	pub(crate) fn new(path: &str, comment_char: &str) -> Self {
		Self {
			comment_char: comment_char.to_string(),
			filepath: path.to_string(),
			lines: vec![],
			is_noop: false,
			selected_line_index: 1,
		}
	}

	pub(crate) fn set_lines(&mut self, lines: Vec<Line>) {
		self.is_noop = !lines.is_empty() && lines[0].get_action() == &Action::Noop;
		self.lines = if self.is_noop {
			vec![]
		}
		else {
			lines.into_iter().filter(|l| l.get_action() != &Action::Noop).collect()
		};
	}

	pub(crate) fn load_file(&mut self) -> Result<()> {
		let lines = read_to_string(Path::new(&self.filepath))
			.map_err(|err| anyhow!("Error reading file: {}", self.filepath).context(err))?
			.lines()
			.filter_map(|l| {
				if l.starts_with(self.comment_char.as_str()) || l.is_empty() {
					None
				}
				else {
					Some(Line::new(l).map_err(|err| anyhow!("Error reading file: {}", self.filepath).context(err)))
				}
			})
			.collect::<Result<Vec<Line>>>()?;
		self.set_lines(lines);
		Ok(())
	}

	pub(crate) fn write_file(&self) -> Result<()> {
		let mut file = File::create(&self.filepath)
			.map_err(|err| anyhow!(err).context(anyhow!("Error opening file: {}", self.filepath)))?;
		let file_contents = if self.is_noop {
			String::from("noop")
		}
		else {
			self.lines.iter().map(Line::to_text).collect::<Vec<String>>().join("\n")
		};
		writeln!(file, "{}", file_contents)
			.map_err(|err| anyhow!(err).context(anyhow!("Error writing file: {}", self.filepath)))?;
		Ok(())
	}

	pub(crate) fn set_noop(&mut self) {
		self.is_noop = true;
		self.lines.clear();
	}

	pub(crate) fn set_selected_line_index(&mut self, selected_line_index: usize) {
		self.selected_line_index = selected_line_index;
	}

	pub(crate) fn swap_lines(&mut self, a: usize, b: usize) {
		self.lines.swap(a, b);
	}

	pub(crate) fn add_line(&mut self, line_number: usize, line: Line) {
		self.lines.insert(line_number - 1, line);
	}

	pub(crate) fn remove_line(&mut self, line_number: usize) {
		self.lines.remove(line_number - 1);
	}

	pub(crate) fn update_selected(&mut self, edit_context: &EditContext) {
		self.update_range(self.selected_line_index, self.selected_line_index, edit_context);
	}

	pub(crate) fn update_range(&mut self, start_index: usize, end_index: usize, edit_context: &EditContext) {
		let range = if end_index <= start_index {
			end_index..=start_index
		}
		else {
			start_index..=end_index
		};

		for index in range {
			let line = &mut self.lines[index - 1];
			if let Some(action) = edit_context.get_action().as_ref() {
				line.set_action(*action);
			}

			if let Some(content) = edit_context.get_content().as_ref() {
				line.edit_content(content);
			}
		}
	}

	pub(crate) fn get_selected_line(&self) -> &Line {
		&self.lines[self.selected_line_index - 1]
	}

	pub(crate) const fn get_selected_line_index(&self) -> usize {
		self.selected_line_index
	}

	pub(crate) fn get_filepath(&self) -> &str {
		self.filepath.as_str()
	}

	pub(crate) const fn get_lines(&self) -> &Vec<Line> {
		&self.lines
	}

	pub(crate) const fn is_noop(&self) -> bool {
		self.is_noop
	}
}
