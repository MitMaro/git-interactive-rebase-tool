// LINT-REPLACE-START
// This section is autogenerated, do not modify directly
// enable all rustc's built-in lints
#![cfg_attr(allow_unknown_lints, allow(unknown_lints))]
#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_compatibility,
	rust_2018_idioms,
	unused,
	warnings
)]
// rustc's additional allowed by default lints
#![deny(
	absolute_paths_not_starting_with_crate,
	deprecated_in_future,
	disjoint_capture_drop_reorder,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	or_patterns_back_compat,
	pointer_structural_match,
	semicolon_in_expressions_from_macros,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_code,
	unsafe_op_in_unsafe_fn,
	unstable_features,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_qualifications,
	unused_results,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
	clippy::blanket_clippy_restriction_lints,
	clippy::implicit_return,
	clippy::missing_docs_in_private_items,
	clippy::redundant_pub_crate,
	clippy::tabs_in_doc_comments
)]
#![deny(
	rustdoc::bare_urls,
	rustdoc::broken_intra_doc_links,
	rustdoc::invalid_codeblock_attributes,
	rustdoc::invalid_html_tags,
	rustdoc::missing_crate_level_docs,
	rustdoc::private_doc_tests,
	rustdoc::private_intra_doc_links
)]
// LINT-REPLACE-END
#![allow(
	clippy::as_conversions,
	clippy::indexing_slicing,
	clippy::implicit_return,
	clippy::integer_arithmetic,
	clippy::missing_docs_in_private_items,
	clippy::missing_errors_doc,
	clippy::missing_inline_in_public_items,
	clippy::module_name_repetitions
)]

//! Git Interactive Rebase Tool - Todo File Module
//!
//! # Description
//! This module is used to handle working with the rebase todo file.

mod action;
mod edit_content;
mod history;
mod line;
mod utils;

use std::{
	fs::{read_to_string, File},
	io::Write,
	path::Path,
	slice::Iter,
};

use anyhow::{anyhow, Result};

pub use self::{action::Action, edit_content::EditContext, line::Line};
use self::{
	history::{History, HistoryItem},
	utils::{remove_range, swap_range_down, swap_range_up},
};

/// Represents a rebase file.
#[derive(Debug)]
pub struct TodoFile {
	comment_char: String,
	filepath: String,
	history: History,
	is_noop: bool,
	lines: Vec<Line>,
	selected_line_index: usize,
}

impl TodoFile {
	/// Create a new instance.
	#[must_use]
	pub fn new(path: &str, undo_limit: u32, comment_char: &str) -> Self {
		Self {
			comment_char: String::from(comment_char),
			filepath: path.to_owned(),
			history: History::new(undo_limit),
			lines: vec![],
			is_noop: false,
			selected_line_index: 0,
		}
	}

	/// Set the rebase lines.
	pub fn set_lines(&mut self, lines: Vec<Line>) {
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

	/// Load the rebase file from disk.
	pub fn load_file(&mut self) -> Result<()> {
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

	/// Write the rebase file to disk.
	pub fn write_file(&self) -> Result<()> {
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

	/// Set the selected line index.
	pub fn set_selected_line_index(&mut self, selected_line_index: usize) {
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

	/// Swap a range of lines up.
	pub fn swap_range_up(&mut self, start_index: usize, end_index: usize) -> bool {
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

	/// Swap a range of lines down.
	pub fn swap_range_down(&mut self, start_index: usize, end_index: usize) -> bool {
		let len = self.lines.len();
		let max_index = if len == 0 { 0 } else { len - 1 };

		if end_index == max_index || start_index == max_index {
			return false;
		}

		swap_range_down(&mut self.lines, start_index, end_index);
		self.history.record(HistoryItem::new_swap_down(start_index, end_index));
		true
	}

	/// Add a new line.
	pub fn add_line(&mut self, index: usize, line: Line) {
		let i = if index > self.lines.len() {
			self.lines.len()
		}
		else {
			index
		};
		self.lines.insert(i, line);
		self.history.record(HistoryItem::new_add(i, i));
	}

	/// Remove a range of lines.
	pub fn remove_lines(&mut self, start_index: usize, end_index: usize) {
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
		self.history.record(HistoryItem::new_remove(start, end, removed_lines));
	}

	/// Update a range of lines.
	pub fn update_range(&mut self, start_index: usize, end_index: usize, edit_context: &EditContext) {
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

	/// Undo the last modification.
	pub fn undo(&mut self) -> Option<(usize, usize)> {
		self.history.undo(&mut self.lines)
	}

	/// Redo the last undone modification.
	pub fn redo(&mut self) -> Option<(usize, usize)> {
		self.history.redo(&mut self.lines)
	}

	/// Get the selected line.
	#[must_use]
	pub fn get_selected_line(&self) -> Option<&Line> {
		self.lines.get(self.selected_line_index)
	}

	/// Get the index of the last line that can be selected.
	#[must_use]
	pub fn get_max_selected_line_index(&self) -> usize {
		let len = self.lines.len();
		if len == 0 {
			0
		}
		else {
			len - 1
		}
	}

	/// Get the selected line index
	#[must_use]
	pub const fn get_selected_line_index(&self) -> usize {
		self.selected_line_index
	}

	/// Get the file path to the rebase file.
	#[must_use]
	pub fn get_filepath(&self) -> &str {
		self.filepath.as_str()
	}

	/// Get a line by index.
	#[must_use]
	pub fn get_line(&self, index: usize) -> Option<&Line> {
		self.lines.get(index)
	}

	/// Get an owned copy of the lines.
	#[must_use]
	pub fn get_lines_owned(&self) -> Vec<Line> {
		self.lines.clone()
	}

	/// Is the rebase file a noop.
	#[must_use]
	pub const fn is_noop(&self) -> bool {
		self.is_noop
	}

	/// Get an iterator over the lines.
	#[must_use]
	pub fn iter(&self) -> Iter<'_, Line> {
		self.lines.iter()
	}

	/// Does the rebase file contain no lines.
	#[must_use]
	pub fn is_empty(&self) -> bool {
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
		todo_file.history.record(HistoryItem::new_add(1, 1));
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
		todo_file.remove_lines(1, 1);
		assert_todo_lines!(todo_file, "pick aaa comment", "edit ccc comment");
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
	fn history_undo_redo() {
		let (mut todo_file, _) =
			create_and_load_todo_file(&["pick aaa comment", "drop bbb comment", "edit ccc comment"]);
		todo_file.update_range(0, 0, &EditContext::new().action(Action::Drop));
		let _undo_result = todo_file.undo();
		assert_todo_lines!(todo_file, "pick aaa comment", "drop bbb comment", "edit ccc comment");
		let _ = todo_file.redo();
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
		let _ = todo_file.swap_range_up(1, 2);
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
		assert!(todo_file.swap_range_down(0, 1));
		assert_todo_lines!(todo_file, "pick ccc comment", "pick aaa comment", "pick bbb comment");
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
