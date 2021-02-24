use std::{
	fs::{read_to_string, File},
	io::Write,
	path::Path,
	slice::Iter,
};

use action::Action;
use anyhow::{anyhow, Result};
use line::Line;

use crate::todo_file::edit_content::EditContext;

pub mod action;
pub mod edit_content;
pub mod line;

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
			comment_char: String::from(comment_char),
			filepath: path.to_owned(),
			lines: vec![],
			is_noop: false,
			selected_line_index: 0,
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

	pub(crate) fn set_selected_line_index(&mut self, selected_line_index: usize) {
		self.selected_line_index = if selected_line_index >= self.lines.len() {
			self.lines.len() - 1
		}
		else {
			selected_line_index
		}
	}

	pub(crate) fn swap_lines(&mut self, a: usize, b: usize) {
		self.lines.swap(a, b);
	}

	pub(crate) fn add_line(&mut self, index: usize, line: Line) {
		self.lines.insert(index, line);
	}

	pub(crate) fn remove_line(&mut self, index: usize) {
		self.lines.remove(index);
	}

	pub(crate) fn update_range(&mut self, start_index: usize, end_index: usize, edit_context: &EditContext) {
		let range = if end_index <= start_index {
			end_index..=start_index
		}
		else {
			start_index..=end_index
		};

		for index in range {
			let line = &mut self.lines[index];
			if let Some(action) = edit_context.get_action().as_ref() {
				line.set_action(*action);
			}

			if let Some(content) = edit_context.get_content().as_ref() {
				line.edit_content(content);
			}
		}
	}

	pub(crate) fn get_selected_line(&self) -> &Line {
		&self.lines[self.selected_line_index]
	}

	pub(crate) const fn get_selected_line_index(&self) -> usize {
		self.selected_line_index
	}

	pub(crate) fn get_filepath(&self) -> &str {
		self.filepath.as_str()
	}

	pub(crate) fn get_line(&self, index: usize) -> Option<&Line> {
		self.lines.get(index)
	}

	pub(crate) fn get_lines_owned(&self) -> Vec<Line> {
		self.lines.to_owned()
	}

	pub(crate) fn get_maximum_line_index(&self) -> usize {
		let len = self.lines.len();
		if len == 0 {
			0
		}
		else {
			len - 1
		}
	}

	pub(crate) const fn is_noop(&self) -> bool {
		self.is_noop
	}

	pub fn iter(&self) -> Iter<'_, Line> {
		self.lines.iter()
	}

	pub(crate) fn is_empty(&self) -> bool {
		self.lines.is_empty()
	}
}

#[cfg(test)]
mod tests {
	use tempfile::{Builder, NamedTempFile};

	use super::*;

	fn create_todo_file(file_contents: &[&str]) -> NamedTempFile {
		let todo_file = Builder::new()
			.prefix("git-rebase-todo-scratch")
			.suffix("")
			.tempfile()
			.unwrap();
		write!(todo_file.as_file(), "{}", file_contents.join("\n")).unwrap();
		todo_file
	}

	macro_rules! assert_todo_lines {
		($todo_file_path:expr, $($arg:expr),*) => {
			let actual_lines = $todo_file_path.get_lines_owned();

			let expected = vec![$( Line::new($arg).unwrap(), )*];
			assert_eq!(
				actual_lines.iter().map(Line::to_text).collect::<String>(),
				expected.iter().map(Line::to_text).collect::<String>()
			);
		};
	}

	macro_rules! assert_read_todo_file {
		($todo_file_path:expr, $($arg:expr),*) => {
			let expected = vec![$( $arg, )*];
			let content = read_to_string(Path::new($todo_file_path.path().as_os_str().to_str().unwrap())).unwrap();
			assert_eq!(content, format!("{}\n", expected.join("\n")));
		};
	}

	#[test]
	fn load_file() {
		let todo_file_path = create_todo_file(&["pick aaa foobar"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		assert_todo_lines!(todo_file, "pick aaa foobar");
	}

	#[test]
	fn load_noop_file() {
		let todo_file_path = create_todo_file(&["noop"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		assert!(todo_file.is_empty());
		assert!(todo_file.is_noop());
	}

	#[test]
	fn load_ignore_comments() {
		let todo_file_path = create_todo_file(&["# pick aaa comment", "pick aaa foo", "# pick aaa comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		assert_todo_lines!(todo_file, "pick aaa foo");
	}

	#[test]
	fn load_ignore_newlines() {
		let todo_file_path = create_todo_file(&["", "pick aaa foobar", ""]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		assert_todo_lines!(todo_file, "pick aaa foobar");
	}

	#[test]
	fn write_file() {
		let todo_file_path = create_todo_file(&[]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.set_lines(vec![Line::new("pick bbb comment").unwrap()]);
		todo_file.write_file().unwrap();
		assert_todo_lines!(todo_file, "pick bbb comment");
	}

	#[test]
	fn write_file_noop() {
		let todo_file_path = create_todo_file(&[]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.set_lines(vec![Line::new("noop").unwrap()]);
		todo_file.write_file().unwrap();
		assert_read_todo_file!(&todo_file_path, "noop");
	}

	#[test]
	#[should_panic]
	fn swap_lines_index_miss() {
		let todo_file_path = create_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.swap_lines(100, 101);
	}

	#[test]
	fn swap_lines() {
		let todo_file_path = create_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.swap_lines(0, 1);
		assert_todo_lines!(todo_file, "drop bbb comment", "pick aaa comment", "edit ccc comment");
	}

	#[test]
	#[should_panic]
	fn add_line_index_miss() {
		let todo_file_path = create_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.add_line(100, Line::new("fixup ddd comment").unwrap());
	}

	#[test]
	fn add_line() {
		let todo_file_path = create_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.add_line(1, Line::new("fixup ddd comment").unwrap());
		assert_todo_lines!(
			todo_file,
			"pick aaa comment",
			"fixup ddd comment",
			"drop bbb comment",
			"edit ccc comment"
		);
	}

	#[test]
	#[should_panic]
	fn remove_line_index_miss() {
		let todo_file_path = create_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.remove_line(100);
	}

	#[test]
	fn remove_line() {
		let todo_file_path = create_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.remove_line(1);
		assert_todo_lines!(todo_file, "pick aaa comment", "edit ccc comment");
	}

	#[test]
	fn update_range_full_set_action() {
		let todo_file_path = create_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.update_range(0, 2, &EditContext::new().action(Action::Reword));
		assert_todo_lines!(
			todo_file,
			"reword aaa comment",
			"reword bbb comment",
			"reword ccc comment"
		);
	}

	#[test]
	fn update_range_full_set_content() {
		let todo_file_path = create_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.update_range(0, 2, &EditContext::new().content("echo"));
		assert_todo_lines!(todo_file, "exec echo", "exec echo", "exec echo");
	}

	#[test]
	fn update_range_swap_range() {
		let todo_file_path = create_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), "#");
		todo_file.load_file().unwrap();
		todo_file.update_range(2, 0, &EditContext::new().action(Action::Reword));
		assert_todo_lines!(
			todo_file,
			"reword aaa comment",
			"reword bbb comment",
			"reword ccc comment"
		);
	}

	#[test]
	fn selected_line_index() {
		let todo_file_path = create_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		todo_file.set_selected_line_index(1);
		assert_eq!(todo_file.get_selected_line_index(), 1);
	}

	#[test]
	fn selected_line_index_overflow() {
		let todo_file_path = create_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		todo_file.set_selected_line_index(3);
		assert_eq!(todo_file.get_selected_line_index(), 2);
	}

	#[test]
	fn selected_line() {
		let todo_file_path = create_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		todo_file.set_selected_line_index(0);
		assert_eq!(todo_file.get_selected_line(), &Line::new("exec foo").unwrap());
	}

	#[test]
	fn get_line_miss_high() {
		let todo_file_path = create_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		assert!(todo_file.get_line(4).is_none());
	}

	#[test]
	fn get_line_hit() {
		let todo_file_path = create_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		assert_eq!(todo_file.get_line(1).unwrap(), &Line::new("exec bar").unwrap());
	}

	#[test]
	fn get_file_path() {
		let todo_file_path = create_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let todo_file = TodoFile::new(filepath, "#");
		assert_eq!(todo_file.get_filepath(), filepath);
	}

	#[test]
	fn maximum_line_index() {
		let todo_file_path = create_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		assert_eq!(todo_file.get_maximum_line_index(), 2);
	}

	#[test]
	fn maximum_line_index_empty_todo() {
		let todo_file_path = create_todo_file(&[]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		assert_eq!(todo_file.get_maximum_line_index(), 0);
	}

	#[test]
	fn iter() {
		let todo_file_path = create_todo_file(&["pick aaa comment"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		assert_eq!(
			todo_file.iter().next().unwrap(),
			&Line::new("pick aaa comment").unwrap()
		);
	}

	#[test]
	fn is_empty_true() {
		let todo_file_path = create_todo_file(&[]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let todo_file = TodoFile::new(filepath, "#");
		assert!(todo_file.is_empty());
	}

	#[test]
	fn is_empty_false() {
		let todo_file_path = create_todo_file(&["pick aaa comment"]);
		let filepath = todo_file_path.path().to_str().unwrap();
		let mut todo_file = TodoFile::new(filepath, "#");
		todo_file.load_file().unwrap();
		assert!(!todo_file.is_empty());
	}
}
