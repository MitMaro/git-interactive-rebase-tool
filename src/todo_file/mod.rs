use std::{
	fs::{read_to_string, File},
	io::Write,
	path::Path,
	slice::Iter,
};

use action::Action;
use anyhow::{anyhow, Result};
use line::Line;

use crate::todo_file::{
	edit_content::EditContext,
	history::{history_item::HistoryItem, History},
	utils::{swap_range_down, swap_range_up},
};

pub mod action;
pub mod edit_content;
mod history;
pub mod line;
mod utils;

pub struct TodoFile {
	comment_char: String,
	filepath: String,
	history: History,
	is_noop: bool,
	lines: Vec<Line>,
	selected_line_index: usize,
}

impl TodoFile {
	pub(crate) fn new(path: &str, undo_limit: u32, comment_char: &str) -> Self {
		Self {
			comment_char: String::from(comment_char),
			filepath: path.to_owned(),
			history: History::new(undo_limit),
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
		if self.selected_line_index >= self.lines.len() {
			self.selected_line_index = if self.lines.is_empty() { 0 } else { self.lines.len() - 1 };
		}
		self.history.reset();
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
		self.selected_line_index = if self.lines.is_empty() {
			0
		}
		else if selected_line_index >= self.lines.len() {
			self.lines.len() - 1
		}
		else {
			selected_line_index
		}
	}

	pub(crate) fn swap_range_up(&mut self, start_index: usize, end_index: usize) -> bool {
		if end_index == 0 || start_index == 0 || self.lines.is_empty() {
			return false;
		}

		let max_index = self.lines.len() - 1;
		let end = if end_index > max_index { max_index } else { end_index };
		let start = if start_index > max_index {
			max_index
		}
		else {
			start_index
		};

		swap_range_up(&mut self.lines, start, end);
		self.history.record(HistoryItem::new_swap_up(start, end));
		true
	}

	pub(crate) fn swap_range_down(&mut self, start_index: usize, end_index: usize) -> bool {
		let len = self.lines.len();
		let max_index = if len == 0 { 0 } else { len - 1 };

		if end_index == max_index || start_index == max_index {
			return false;
		}

		swap_range_down(&mut self.lines, start_index, end_index);
		self.history.record(HistoryItem::new_swap_down(start_index, end_index));
		true
	}

	pub(crate) fn add_line(&mut self, index: usize, line: Line) {
		let i = if index > self.lines.len() {
			self.lines.len()
		}
		else {
			index
		};
		self.lines.insert(i, line);
		self.history.record(HistoryItem::new_add(i));
	}

	pub(crate) fn remove_line(&mut self, index: usize) -> bool {
		if index >= self.lines.len() {
			false
		}
		else {
			let removed_line = self.lines.remove(index);
			self.history.record(HistoryItem::new_remove(index, removed_line));
			true
		}
	}

	pub(crate) fn update_range(&mut self, start_index: usize, end_index: usize, edit_context: &EditContext) {
		if self.lines.is_empty() {
			return;
		}

		let max_index = self.lines.len() - 1;
		let end = if end_index > max_index { max_index } else { end_index };
		let start = if start_index > max_index {
			max_index
		}
		else {
			start_index
		};

		let range = if end <= start { end..=start } else { start..=end };

		let mut lines = vec![];
		for index in range {
			let line = &mut self.lines[index];
			lines.push(line.clone());
			if let Some(action) = edit_context.get_action().as_ref() {
				line.set_action(*action);
			}

			if let Some(content) = edit_context.get_content().as_ref() {
				line.edit_content(content);
			}
		}
		self.history.record(HistoryItem::new_modify(start, end, lines));
	}

	pub(crate) fn undo(&mut self) -> Option<(usize, usize)> {
		self.history.undo(&mut self.lines)
	}

	pub(crate) fn redo(&mut self) -> Option<(usize, usize)> {
		self.history.redo(&mut self.lines)
	}

	pub(crate) fn get_selected_line(&self) -> Option<&Line> {
		self.lines.get(self.selected_line_index)
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

	fn create_and_load_todo_file(file_contents: &[&str]) -> (TodoFile, NamedTempFile) {
		let todo_file_path = Builder::new()
			.prefix("git-rebase-todo-scratch")
			.suffix("")
			.tempfile()
			.unwrap();
		write!(todo_file_path.as_file(), "{}", file_contents.join("\n")).unwrap();
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), 1, "#");
		todo_file.load_file().unwrap();
		(todo_file, todo_file_path)
	}

	macro_rules! assert_read_todo_file {
		($todo_file_path:expr, $($arg:expr),*) => {
			let expected = vec![$( $arg, )*];
			let content = read_to_string(Path::new($todo_file_path)).unwrap();
			assert_eq!(content, format!("{}\n", expected.join("\n")));
		};
	}

	macro_rules! assert_todo_lines {
		($todo_file_path:expr, $($arg:expr),*) => {
			let actual_lines = $todo_file_path.get_lines_owned();

			let expected = vec![$( Line::new($arg).unwrap(), )*];
			assert_eq!(
				actual_lines.iter().map(Line::to_text).collect::<Vec<String>>().join(", "),
				expected.iter().map(Line::to_text).collect::<Vec<String>>().join(", ")
			);
		};
	}

	#[test]
	fn load_file() {
		let (todo_file, _) = create_and_load_todo_file(&["pick aaa foobar"]);
		assert_todo_lines!(todo_file, "pick aaa foobar");
	}

	#[test]
	fn load_noop_file() {
		let (todo_file, _) = create_and_load_todo_file(&["noop"]);
		assert!(todo_file.is_empty());
		assert!(todo_file.is_noop());
	}

	#[test]
	fn load_ignore_comments() {
		let (todo_file, _) = create_and_load_todo_file(&["# pick aaa comment", "pick aaa foo", "# pick aaa comment"]);
		assert_todo_lines!(todo_file, "pick aaa foo");
	}

	#[test]
	fn load_ignore_newlines() {
		let (todo_file, _) = create_and_load_todo_file(&["", "pick aaa foobar", ""]);
		assert_todo_lines!(todo_file, "pick aaa foobar");
	}

	#[test]
	fn set_lines() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.set_lines(vec![Line::new("pick bbb comment").unwrap()]);
		assert_todo_lines!(todo_file, "pick bbb comment");
	}

	#[test]
	fn set_lines_reset_history() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.history.record(HistoryItem::new_add(1));
		todo_file.set_lines(vec![Line::new("pick bbb comment").unwrap()]);
		assert!(todo_file.undo().is_none());
	}

	#[test]
	fn set_lines_reset_selected_index() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick a a", "pick b b", "pick c c"]);
		todo_file.selected_line_index = 2;
		todo_file.set_lines(vec![Line::new("pick a a").unwrap(), Line::new("pick b b").unwrap()]);
		assert_eq!(todo_file.selected_line_index, 1);
	}

	#[test]
	fn set_lines_reset_selected_index_empty_lis() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick a a", "pick b b", "pick c c"]);
		todo_file.selected_line_index = 2;
		todo_file.set_lines(vec![]);
		assert_eq!(todo_file.selected_line_index, 0);
	}

	#[test]
	fn write_file() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.set_lines(vec![Line::new("pick bbb comment").unwrap()]);
		todo_file.write_file().unwrap();
		assert_todo_lines!(todo_file, "pick bbb comment");
	}

	#[test]
	fn write_file_noop() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.set_lines(vec![Line::new("noop").unwrap()]);
		todo_file.write_file().unwrap();
		assert_read_todo_file!(todo_file.get_filepath(), "noop");
	}

	#[test]
	fn add_line_index_miss() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.add_line(100, Line::new("fixup ddd comment").unwrap());
		assert_todo_lines!(
			todo_file,
			"pick aaa comment",
			"drop bbb comment",
			"edit ccc comment",
			"fixup ddd comment"
		);
	}

	#[test]
	fn add_line() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
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
	fn add_line_record_history() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick aaa comment"]);
		todo_file.add_line(1, Line::new("fixup ddd comment").unwrap());
		todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment");
	}

	#[test]
	fn remove_line_index_miss() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		assert!(!todo_file.remove_line(100));
	}

	#[test]
	fn remove_line() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		assert!(todo_file.remove_line(1));
		assert_todo_lines!(todo_file, "pick aaa comment", "edit ccc comment");
	}

	#[test]
	fn remove_line_record_history() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick aaa comment", "edit ccc comment"]);
		todo_file.remove_line(1);
		todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment", "edit ccc comment");
	}

	#[test]
	fn update_range_full_set_action() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
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
		let (mut todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		todo_file.update_range(0, 2, &EditContext::new().content("echo"));
		assert_todo_lines!(todo_file, "exec echo", "exec echo", "exec echo");
	}

	#[test]
	fn update_range_edit_action() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.update_range(2, 0, &EditContext::new().action(Action::Reword));
		assert_todo_lines!(
			todo_file,
			"reword aaa comment",
			"reword bbb comment",
			"reword ccc comment"
		);
	}

	#[test]
	fn update_range_record_history() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick aaa comment"]);
		todo_file.update_range(0, 0, &EditContext::new().action(Action::Reword));
		todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment");
	}

	#[test]
	fn update_range_empty_list() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.update_range(0, 0, &EditContext::new().action(Action::Reword));
	}

	#[test]
	fn update_range_start_index_overflow() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick aaa comment", "pick bbb comment"]);
		todo_file.update_range(2, 0, &EditContext::new().action(Action::Reword));
		assert_todo_lines!(todo_file, "reword aaa comment", "reword bbb comment");
	}

	#[test]
	fn update_range_end_index_overflow() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick aaa comment", "pick bbb comment"]);
		todo_file.update_range(0, 2, &EditContext::new().action(Action::Reword));
		assert_todo_lines!(todo_file, "reword aaa comment", "reword bbb comment");
	}

	#[test]
	fn history_undo_redo() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.update_range(0, 0, &EditContext::new().action(Action::Drop));
		todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment", "drop bbb comment", "edit ccc comment");
		todo_file.redo();
		assert_todo_lines!(todo_file, "drop aaa comment", "drop bbb comment", "edit ccc comment");
	}

	#[test]
	fn swap_up() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(todo_file.swap_range_up(1, 2));
		assert_todo_lines!(todo_file, "pick bbb comment", "pick ccc comment", "pick aaa comment");
	}

	#[test]
	fn swap_up_records_history() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		todo_file.swap_range_up(1, 2);
		todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment", "pick bbb comment", "pick ccc comment");
	}

	#[test]
	fn swap_up_reverse_index() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(todo_file.swap_range_up(2, 1));
		assert_todo_lines!(todo_file, "pick bbb comment", "pick ccc comment", "pick aaa comment");
	}

	#[test]
	fn swap_up_single_line() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(todo_file.swap_range_up(1, 1));
		assert_todo_lines!(todo_file, "pick bbb comment", "pick aaa comment", "pick ccc comment");
	}

	#[test]
	fn swap_up_at_top_start_index() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(!todo_file.swap_range_up(0, 1));
		assert_todo_lines!(todo_file, "pick aaa comment", "pick bbb comment", "pick ccc comment");
	}

	#[test]
	fn swap_up_at_top_end_index() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(!todo_file.swap_range_up(1, 0));
		assert_todo_lines!(todo_file, "pick aaa comment", "pick bbb comment", "pick ccc comment");
	}

	#[test]
	fn swap_up_start_index_overflow() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(todo_file.swap_range_up(3, 1));
		assert_todo_lines!(todo_file, "pick bbb comment", "pick ccc comment", "pick aaa comment");
	}

	#[test]
	fn swap_up_end_index_overflow() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(todo_file.swap_range_up(3, 1));
		assert_todo_lines!(todo_file, "pick bbb comment", "pick ccc comment", "pick aaa comment");
	}

	#[test]
	fn swap_up_empty_list_index_out_of_bounds() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		assert!(!todo_file.swap_range_up(1, 1));
	}

	#[test]
	fn swap_down() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(todo_file.swap_range_down(0, 1));
		assert_todo_lines!(todo_file, "pick ccc comment", "pick aaa comment", "pick bbb comment");
	}

	#[test]
	fn swap_down_records_history() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		todo_file.swap_range_down(0, 1);
		todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment", "pick bbb comment", "pick ccc comment");
	}

	#[test]
	fn swap_down_reverse_index() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(todo_file.swap_range_down(1, 0));
		assert_todo_lines!(todo_file, "pick ccc comment", "pick aaa comment", "pick bbb comment");
	}

	#[test]
	fn swap_down_single_line() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(todo_file.swap_range_down(0, 0));
		assert_todo_lines!(todo_file, "pick bbb comment", "pick aaa comment", "pick ccc comment");
	}

	#[test]
	fn swap_down_at_bottom_end_index() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(!todo_file.swap_range_down(1, 2));
		assert_todo_lines!(todo_file, "pick aaa comment", "pick bbb comment", "pick ccc comment");
	}

	#[test]
	fn swap_down_at_bottom_start_index() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		assert!(!todo_file.swap_range_down(2, 1));
		assert_todo_lines!(todo_file, "pick aaa comment", "pick bbb comment", "pick ccc comment");
	}

	#[test]
	fn selected_line_index() {
		let (mut todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		todo_file.set_selected_line_index(1);
		assert_eq!(todo_file.get_selected_line_index(), 1);
	}

	#[test]
	fn selected_line_index_overflow() {
		let (mut todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		todo_file.set_selected_line_index(3);
		assert_eq!(todo_file.get_selected_line_index(), 2);
	}

	#[test]
	fn selected_line() {
		let (mut todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		todo_file.set_selected_line_index(0);
		assert_eq!(todo_file.get_selected_line().unwrap(), &Line::new("exec foo").unwrap());
	}

	#[test]
	fn selected_line_empty_list() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.set_selected_line_index(0);
		assert!(todo_file.get_selected_line().is_none());
	}

	#[test]
	fn get_line_miss_high() {
		let (todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		assert!(todo_file.get_line(4).is_none());
	}

	#[test]
	fn get_line_hit() {
		let (todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		assert_eq!(todo_file.get_line(1).unwrap(), &Line::new("exec bar").unwrap());
	}

	#[test]
	fn get_file_path() {
		let (todo_file, filepath) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		assert_eq!(todo_file.get_filepath(), filepath.path().to_str().unwrap());
	}

	#[test]
	fn iter() {
		let (todo_file, _) = create_and_load_todo_file(&["pick aaa comment"]);
		assert_eq!(
			todo_file.iter().next().unwrap(),
			&Line::new("pick aaa comment").unwrap()
		);
	}

	#[test]
	fn is_empty_true() {
		let (todo_file, _) = create_and_load_todo_file(&[]);
		assert!(todo_file.is_empty());
	}

	#[test]
	fn is_empty_false() {
		let (todo_file, _) = create_and_load_todo_file(&["pick aaa comment"]);
		assert!(!todo_file.is_empty());
	}
}
