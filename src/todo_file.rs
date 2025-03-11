//! Git Interactive Rebase Tool - Todo File Module
//!
//! # Description
//! This module is used to handle working with the rebase todo file.

mod action;
mod edit_content;
mod errors;
mod history;
mod line;
mod line_parser;
mod todo_file_options;
mod utils;

use std::{
	fs::{File, read_to_string},
	io::Write as _,
	path::{Path, PathBuf},
	slice::Iter,
};

use version_track::Version;

pub(crate) use self::{
	action::Action,
	edit_content::EditContext,
	errors::ParseError,
	line::Line,
	line_parser::LineParser,
	todo_file_options::TodoFileOptions,
};
use self::{
	history::{History, HistoryItem},
	utils::{remove_range, swap_range_down, swap_range_up},
};
use crate::todo_file::{
	errors::{FileReadErrorCause, IoError},
	history::Operation,
};

/// Represents a rebase file.
#[derive(Debug)]
pub(crate) struct TodoFile {
	filepath: PathBuf,
	history: History,
	is_noop: bool,
	lines: Vec<Line>,
	options: TodoFileOptions,
	selected_line_index: usize,
	version: Version,
}

impl TodoFile {
	/// Create a new instance.
	#[must_use]
	pub(crate) fn new<Path: AsRef<std::path::Path>>(path: Path, options: TodoFileOptions) -> Self {
		let history = History::new(options.undo_limit);

		Self {
			filepath: PathBuf::from(path.as_ref()),
			history,
			is_noop: false,
			lines: vec![],
			options,
			selected_line_index: 0,
			version: Version::new(),
		}
	}

	/// Set the rebase lines.
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
		self.version.reset();
		self.history.reset();
	}

	/// Load the rebase file from disk.
	///
	/// # Errors
	///
	/// Returns error if the file cannot be read.
	pub(crate) fn load_file(&mut self) -> Result<(), IoError> {
		let lines: Result<Vec<Line>, IoError> = read_to_string(self.filepath.as_path())
			.map_err(|err| {
				IoError::FileRead {
					file: self.filepath.clone(),
					cause: FileReadErrorCause::from(err),
				}
			})?
			.lines()
			.filter_map(|l| {
				if l.starts_with(self.options.comment_prefix.as_str()) || l.is_empty() {
					None
				}
				else {
					Some(Line::parse(l).map_err(|err| {
						IoError::FileRead {
							file: self.filepath.clone(),
							cause: FileReadErrorCause::from(err),
						}
					}))
				}
			})
			.collect();
		self.set_lines(lines?);
		Ok(())
	}

	/// Write the rebase file to disk.
	/// # Errors
	///
	/// Returns error if the file cannot be written.
	pub(crate) fn write_file(&self) -> Result<(), IoError> {
		let mut file = File::create(&self.filepath).map_err(|err| {
			IoError::FileRead {
				file: self.filepath.clone(),
				cause: FileReadErrorCause::from(err),
			}
		})?;
		let file_contents = if self.is_noop {
			String::from("noop")
		}
		else {
			self.lines
				.iter()
				.flat_map(|l| {
					let mut lines = vec![Line::to_text(l)];
					if let Some(command) = self.options.line_changed_command.as_deref() {
						if l.is_modified() {
							let action = l.get_action();

							match *action {
								Action::Break | Action::Noop => {},
								Action::Drop
								| Action::Fixup
								| Action::Edit
								| Action::Pick
								| Action::Reword
								| Action::Squash => {
									lines.push(format!("exec {command} \"{}\" \"{}\"", action, l.get_hash()));
								},
								Action::Exec | Action::Label | Action::Reset | Action::Merge | Action::UpdateRef => {
									let original_label =
										l.original().map_or_else(|| l.get_content(), Line::get_content);
									lines.push(format!(
										"exec {command} \"{}\" \"{}\" \"{}\"",
										action,
										original_label,
										l.get_content()
									));
								},
							}
						}
					}
					lines
				})
				.collect::<Vec<String>>()
				.join("\n")
		};
		writeln!(file, "{file_contents}").map_err(|err| {
			IoError::FileRead {
				file: self.filepath.clone(),
				cause: FileReadErrorCause::from(err),
			}
		})?;
		Ok(())
	}

	/// Set the selected line index returning the new index based after ensuring within range.
	pub(crate) fn set_selected_line_index(&mut self, selected_line_index: usize) -> usize {
		self.selected_line_index = if self.lines.is_empty() {
			0
		}
		else if selected_line_index >= self.lines.len() {
			self.lines.len() - 1
		}
		else {
			selected_line_index
		};
		self.selected_line_index
	}

	/// Swap a range of lines up.
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
		self.version.increment();
		self.history.record(HistoryItem::new_swap_up(start, end));
		true
	}

	/// Swap a range of lines down.
	pub(crate) fn swap_range_down(&mut self, start_index: usize, end_index: usize) -> bool {
		let len = self.lines.len();
		let max_index = if len == 0 { 0 } else { len - 1 };

		if end_index == max_index || start_index == max_index {
			return false;
		}

		swap_range_down(&mut self.lines, start_index, end_index);
		self.version.increment();
		self.history.record(HistoryItem::new_swap_down(start_index, end_index));
		true
	}

	/// Add a new line.
	pub(crate) fn add_line(&mut self, index: usize, line: Line) {
		let i = if index > self.lines.len() {
			self.lines.len()
		}
		else {
			index
		};
		self.lines.insert(i, line);
		self.version.increment();
		self.history.record(HistoryItem::new_add(i, i));
	}

	/// Remove a range of lines.
	pub(crate) fn remove_lines(&mut self, start_index: usize, end_index: usize) {
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

		let removed_lines = remove_range(&mut self.lines, start, end);
		self.version.increment();
		self.history.record(HistoryItem::new_remove(start, end, removed_lines));
	}

	/// Update a range of lines.
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
			if let Some(action) = edit_context.get_action() {
				line.set_action(action);
			}

			if let Some(content) = edit_context.get_content() {
				line.edit_content(content);
			}

			if let Some(option) = edit_context.get_option() {
				line.toggle_option(option);
			}
		}
		self.version.increment();
		self.history.record(HistoryItem::new_modify(start, end, lines));
	}

	/// Undo the last modification.
	pub(crate) fn undo(&mut self) -> Option<(usize, usize)> {
		self.version.increment();
		if let Some((operation, start, end)) = self.history.undo(&mut self.lines) {
			return if operation == Operation::Load {
				None
			}
			else {
				Some((start, end))
			};
		}
		None
	}

	/// Redo the last undone modification.
	pub(crate) fn redo(&mut self) -> Option<(usize, usize)> {
		self.version.increment();
		self.history.redo(&mut self.lines).map(|(_, start, end)| (start, end))
	}

	/// Get the current version
	#[must_use]
	pub(crate) const fn version(&self) -> &Version {
		&self.version
	}

	/// Get the selected line.
	#[must_use]
	pub(crate) fn get_selected_line(&self) -> Option<&Line> {
		self.lines.get(self.selected_line_index)
	}

	/// Get the index of the last line that can be selected.
	#[must_use]
	pub(crate) fn get_max_selected_line_index(&self) -> usize {
		let len = self.lines.len();
		if len == 0 { 0 } else { len - 1 }
	}

	/// Get the selected line index
	#[must_use]
	pub(crate) const fn get_selected_line_index(&self) -> usize {
		self.selected_line_index
	}

	/// Get the file path to the rebase file.
	#[must_use]
	pub(crate) fn get_filepath(&self) -> &Path {
		self.filepath.as_path()
	}

	/// Get a line by index.
	#[must_use]
	pub(crate) fn get_line(&self, index: usize) -> Option<&Line> {
		self.lines.get(index)
	}

	/// Get an owned copy of the lines.
	#[must_use]
	pub(crate) fn get_lines_owned(&self) -> Vec<Line> {
		self.lines.clone()
	}

	/// Is the rebase file a noop.
	#[must_use]
	pub(crate) const fn is_noop(&self) -> bool {
		self.is_noop
	}

	/// Get an iterator over the lines.
	pub(crate) fn lines_iter(&self) -> Iter<'_, Line> {
		self.lines.iter()
	}

	/// Does the rebase file contain no lines.
	#[must_use]
	pub(crate) fn is_empty(&self) -> bool {
		self.lines.is_empty()
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_none, assert_some_eq};
	use tempfile::{Builder, NamedTempFile};

	use super::*;
	use crate::{assert_empty, assert_not_empty};

	fn create_line(line: &str) -> Line {
		Line::parse(line).unwrap()
	}

	fn create_and_load_todo_file_with_options(
		file_contents: &[&str],
		todo_file_options: TodoFileOptions,
	) -> (TodoFile, NamedTempFile) {
		let todo_file_path = Builder::new()
			.prefix("git-rebase-todo-scratch")
			.suffix("")
			.tempfile()
			.unwrap();
		write!(todo_file_path.as_file(), "{}", file_contents.join("\n")).unwrap();
		let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), todo_file_options);
		todo_file.load_file().unwrap();
		(todo_file, todo_file_path)
	}

	fn create_and_load_todo_file(file_contents: &[&str]) -> (TodoFile, NamedTempFile) {
		create_and_load_todo_file_with_options(file_contents, TodoFileOptions::new(1, "#"))
	}

	macro_rules! assert_read_todo_file {
		($todo_file_path:expr, $($arg:expr),*) => {
			let expected = [$( $arg, )*];
			let content = read_to_string(Path::new($todo_file_path)).unwrap();
			pretty_assertions::assert_str_eq!(content, format!("{}\n", expected.join("\n")));
		};
	}

	macro_rules! assert_todo_lines {
		($todo_file_path:expr, $($arg:expr),*) => {
			let actual_lines = $todo_file_path.get_lines_owned();

			let expected = vec![$( create_line($arg), )*];
			pretty_assertions::assert_str_eq!(
				actual_lines.iter().map(Line::to_text).collect::<Vec<String>>().join("\n"),
				expected.iter().map(Line::to_text).collect::<Vec<String>>().join("\n")
			);
		};
	}

	#[test]
	fn load_file() {
		let (todo_file, _) = create_and_load_todo_file(&["pick aaa foobar"]);
		assert_todo_lines!(todo_file, "pick aaa foobar");
		assert_ne!(todo_file.version(), &Version::new());
	}

	#[test]
	fn load_noop_file() {
		let (todo_file, _) = create_and_load_todo_file(&["noop"]);
		assert_empty!(todo_file);
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
		let old_version = todo_file.version;
		todo_file.set_lines(vec![create_line("pick bbb comment")]);
		assert_todo_lines!(todo_file, "pick bbb comment");
		assert_ne!(todo_file.version(), &old_version);
	}

	#[test]
	fn set_lines_reset_history() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.history.record(HistoryItem::new_add(1, 1));
		todo_file.set_lines(vec![create_line("pick bbb comment")]);
		assert_none!(todo_file.undo());
	}

	#[test]
	fn set_lines_reset_selected_index() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick a a", "pick b b", "pick c c"]);
		todo_file.selected_line_index = 2;
		todo_file.set_lines(vec![create_line("pick a a"), create_line("pick b b")]);
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
		todo_file.set_lines(vec![create_line("pick bbb comment")]);
		todo_file.write_file().unwrap();
		assert_todo_lines!(todo_file, "pick bbb comment");
	}

	#[test]
	fn write_file_with_exec_command_modified_line_with_reference() {
		fn create_modified_line(action: &str) -> Line {
			let mut parsed = create_line(format!("{action} label").as_str());
			parsed.edit_content("new-label");
			parsed
		}
		let mut options = TodoFileOptions::new(10, "#");
		options.line_changed_command("command");
		let (mut todo_file, _) = create_and_load_todo_file_with_options(&[], options);
		todo_file.set_lines(vec![
			create_modified_line("label"),
			create_modified_line("reset"),
			create_modified_line("merge"),
			create_modified_line("update-ref"),
		]);
		todo_file.write_file().unwrap();
		assert_read_todo_file!(
			todo_file.get_filepath(),
			"label new-label",
			"exec command \"label\" \"label\" \"new-label\"",
			"reset new-label",
			"exec command \"reset\" \"label\" \"new-label\"",
			"merge new-label",
			"exec command \"merge\" \"label\" \"new-label\"",
			"update-ref new-label",
			"exec command \"update-ref\" \"label\" \"new-label\""
		);
	}

	#[test]
	fn write_file_with_exec_command_modified_line_with_hash() {
		fn create_modified_line(action: &str) -> Line {
			let mut parsed = create_line(format!("{action} bbb comment").as_str());
			parsed.set_action(
				if parsed.get_action() == &Action::Fixup {
					Action::Pick
				}
				else {
					Action::Fixup
				},
			);
			parsed
		}
		let mut options = TodoFileOptions::new(10, "#");
		options.line_changed_command("command");
		let (mut todo_file, _) = create_and_load_todo_file_with_options(&[], options);
		let mut line = create_line("pick bbb comment");
		line.set_action(Action::Fixup);
		todo_file.set_lines(vec![
			create_modified_line("drop"),
			create_modified_line("fixup"),
			create_modified_line("edit"),
			create_modified_line("pick"),
			create_modified_line("reword"),
			create_modified_line("squash"),
		]);
		todo_file.write_file().unwrap();
		assert_read_todo_file!(
			todo_file.get_filepath(),
			"fixup bbb comment",
			"exec command \"fixup\" \"bbb\"",
			"pick bbb comment",
			"exec command \"pick\" \"bbb\"",
			"fixup bbb comment",
			"exec command \"fixup\" \"bbb\"",
			"fixup bbb comment",
			"exec command \"fixup\" \"bbb\"",
			"fixup bbb comment",
			"exec command \"fixup\" \"bbb\"",
			"fixup bbb comment",
			"exec command \"fixup\" \"bbb\""
		);
	}

	#[test]
	fn write_file_with_exec_command_modified_line_with_exec() {
		let mut options = TodoFileOptions::new(10, "#");
		options.line_changed_command("command");
		let (mut todo_file, _) = create_and_load_todo_file_with_options(&[], options);
		let mut line = create_line("exec command");
		line.edit_content("new-command");
		todo_file.set_lines(vec![line]);
		todo_file.write_file().unwrap();
		assert_read_todo_file!(
			todo_file.get_filepath(),
			"exec new-command",
			"exec command \"exec\" \"command\" \"new-command\""
		);
	}

	#[test]
	fn write_file_with_exec_command_modified_line_with_break() {
		let mut options = TodoFileOptions::new(10, "#");
		options.line_changed_command("command");
		let (mut todo_file, _) = create_and_load_todo_file_with_options(&[], options);
		todo_file.set_lines(vec![create_line("break")]);
		todo_file.write_file().unwrap();
		assert_read_todo_file!(todo_file.get_filepath(), "break");
	}

	#[test]
	fn write_file_noop() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.set_lines(vec![create_line("noop")]);
		todo_file.write_file().unwrap();
		assert_read_todo_file!(todo_file.get_filepath(), "noop");
	}

	#[test]
	fn add_line_index_miss() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.add_line(100, create_line("fixup ddd comment"));
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
		let old_version = *todo_file.version();
		todo_file.add_line(1, create_line("fixup ddd comment"));
		assert_todo_lines!(
			todo_file,
			"pick aaa comment",
			"fixup ddd comment",
			"drop bbb comment",
			"edit ccc comment"
		);
		assert_ne!(todo_file.version(), &old_version);
	}

	#[test]
	fn add_line_record_history() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick aaa comment"]);
		todo_file.add_line(1, create_line("fixup ddd comment"));
		let _undo_result = todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment");
	}

	#[test]
	fn remove_lines_index_miss_start() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.remove_lines(100, 1);
		assert_todo_lines!(todo_file, "pick aaa comment");
	}

	#[test]
	fn remove_lines_index_miss_end() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.remove_lines(1, 100);
		assert_todo_lines!(todo_file, "pick aaa comment");
	}

	#[test]
	fn remove_lines_index_miss_start_and_end() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.remove_lines(100, 100);
		assert_todo_lines!(todo_file, "pick aaa comment", "drop bbb comment");
	}

	#[test]
	fn remove_lines() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let old_version = *todo_file.version();
		todo_file.remove_lines(1, 1);
		assert_todo_lines!(todo_file, "pick aaa comment", "edit ccc comment");
		assert_ne!(todo_file.version(), &old_version);
	}

	#[test]
	fn remove_lines_empty_list() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		todo_file.remove_lines(1, 1);
	}

	#[test]
	fn remove_lines_record_history() {
		let (mut todo_file, _) = create_and_load_todo_file(&["pick aaa comment", "edit ccc comment"]);
		todo_file.remove_lines(1, 1);
		let _undo_result = todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment", "edit ccc comment");
	}

	#[test]
	fn update_range_full_set_action() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		let old_version = *todo_file.version();
		todo_file.update_range(0, 2, &EditContext::new().action(Action::Reword));
		assert_todo_lines!(
			todo_file,
			"reword aaa comment",
			"reword bbb comment",
			"reword ccc comment"
		);
		assert_ne!(todo_file.version(), &old_version);
	}

	#[test]
	fn update_range_full_set_content() {
		let (mut todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		todo_file.update_range(0, 2, &EditContext::new().content("echo"));
		assert_todo_lines!(todo_file, "exec echo", "exec echo", "exec echo");
	}

	#[test]
	fn update_range_set_option() {
		let (mut todo_file, _) = create_and_load_todo_file(&["fixup aaa comment"]);
		let old_version = *todo_file.version();
		todo_file.update_range(0, 2, &EditContext::new().option("-c"));
		assert_todo_lines!(todo_file, "fixup -c aaa comment");
		assert_ne!(todo_file.version(), &old_version);
	}

	#[test]
	fn update_range_reverse_indexes() {
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
		let _undo_result = todo_file.undo();
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
	fn undo_load_operation() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		assert_none!(todo_file.undo());
	}

	#[test]
	fn undo_empty_history() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		// set short history, to remove load entry
		todo_file.history = History::new(1);
		todo_file.update_range(0, 0, &EditContext::new().action(Action::Drop));
		_ = todo_file.undo(); // remove Drop operation
		assert_none!(todo_file.undo());
	}

	#[test]
	fn undo_operation() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.update_range(0, 1, &EditContext::new().action(Action::Drop));
		assert_some_eq!(todo_file.undo(), (0, 1));
	}

	#[test]
	fn history_undo_redo() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.update_range(0, 0, &EditContext::new().action(Action::Drop));
		let old_version = *todo_file.version();
		let _undo_result = todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment", "drop bbb comment", "edit ccc comment");
		assert_ne!(todo_file.version(), &old_version);
		let old_version = *todo_file.version();
		_ = todo_file.redo();
		assert_todo_lines!(todo_file, "drop aaa comment", "drop bbb comment", "edit ccc comment");
		assert_ne!(todo_file.version(), &old_version);
	}

	#[test]
	fn redo_empty_history() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		assert_none!(todo_file.redo());
	}

	#[test]
	fn redo_operation() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.update_range(0, 1, &EditContext::new().action(Action::Drop));
		_ = todo_file.undo();
		assert_some_eq!(todo_file.redo(), (0, 1));
	}

	#[test]
	fn swap_up() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		let old_version = *todo_file.version();
		assert!(todo_file.swap_range_up(1, 2));
		assert_todo_lines!(todo_file, "pick bbb comment", "pick ccc comment", "pick aaa comment");
		assert_ne!(todo_file.version(), &old_version);
	}

	#[test]
	fn swap_up_records_history() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		_ = todo_file.swap_range_up(1, 2);
		let _undo_result = todo_file.undo();
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
		let old_version = *todo_file.version();
		assert!(todo_file.swap_range_down(0, 1));
		assert_todo_lines!(todo_file, "pick ccc comment", "pick aaa comment", "pick bbb comment");
		assert_ne!(todo_file.version(), &old_version);
	}

	#[test]
	fn swap_down_records_history() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "pick bbb comment", "pick ccc comment"]);
		let _swap_result = todo_file.swap_range_down(0, 1);
		let _undo_result = todo_file.undo();
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
		let selected_line_index = todo_file.set_selected_line_index(1);
		assert_eq!(selected_line_index, 1);
		assert_eq!(todo_file.get_selected_line_index(), 1);
	}

	#[test]
	fn selected_line_index_overflow() {
		let (mut todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		let selected_line_index = todo_file.set_selected_line_index(3);
		assert_eq!(selected_line_index, 2);
		assert_eq!(todo_file.get_selected_line_index(), 2);
	}

	#[test]
	fn selected_line() {
		let (mut todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		_ = todo_file.set_selected_line_index(0);
		assert_some_eq!(todo_file.get_selected_line(), &create_line("exec foo"));
	}

	#[test]
	fn selected_line_empty_list() {
		let (mut todo_file, _) = create_and_load_todo_file(&[]);
		_ = todo_file.set_selected_line_index(0);
		assert_none!(todo_file.get_selected_line());
	}

	#[test]
	fn get_max_selected_line() {
		let (todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		assert_eq!(todo_file.get_max_selected_line_index(), 2);
	}

	#[test]
	fn get_max_selected_line_empty_list() {
		let (todo_file, _) = create_and_load_todo_file(&[]);
		assert_eq!(todo_file.get_max_selected_line_index(), 0);
	}

	#[test]
	fn get_line_miss_high() {
		let (todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		assert_none!(todo_file.get_line(4));
	}

	#[test]
	fn get_line_hit() {
		let (todo_file, _) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		assert_some_eq!(todo_file.get_line(1), &create_line("exec bar"));
	}

	#[test]
	fn get_file_path() {
		let (todo_file, filepath) = create_and_load_todo_file(&["exec foo", "exec bar", "exec foobar"]);
		assert_eq!(todo_file.get_filepath(), filepath.path());
	}

	#[test]
	fn iter() {
		let (todo_file, _) = create_and_load_todo_file(&["pick aaa comment"]);
		assert_some_eq!(todo_file.lines_iter().next(), &create_line("pick aaa comment"));
	}

	#[test]
	fn is_empty_true() {
		let (todo_file, _) = create_and_load_todo_file(&[]);
		assert_empty!(todo_file);
	}

	#[test]
	fn is_empty_false() {
		let (todo_file, _) = create_and_load_todo_file(&["pick aaa comment"]);
		assert_not_empty!(todo_file);
	}
}
