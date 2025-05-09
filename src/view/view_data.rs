use uuid::Uuid;

use crate::view::{ViewDataUpdater, ViewLine, ViewLines};

/// Represents the content to be rendered to the `View`.
#[derive(Debug)]
pub(crate) struct ViewData {
	lines: ViewLines,
	lines_leading: ViewLines,
	lines_trailing: ViewLines,
	name: String,
	retain_scroll_position: bool,
	scroll_version: u32,
	show_help: bool,
	show_title: bool,
	version: u32,
	visible_column: Option<usize>,
	visible_rows: Vec<usize>,
}

impl ViewData {
	/// Create a new instance using a `ViewDataUpdater`.
	pub(crate) fn new<C>(callback: C) -> Self
	where C: FnOnce(&mut ViewDataUpdater<'_>) {
		let mut view_data = Self {
			lines: ViewLines::new(),
			lines_leading: ViewLines::new(),
			lines_trailing: ViewLines::new(),
			name: Uuid::new_v4().hyphenated().to_string(),
			retain_scroll_position: true,
			scroll_version: 0,
			show_help: false,
			show_title: false,
			version: 0,
			visible_column: None,
			visible_rows: vec![],
		};
		let mut view_data_updater = ViewDataUpdater::new(&mut view_data);
		callback(&mut view_data_updater);
		view_data
	}

	/// Update the view data using a `ViewDataUpdater`. This allows for batch updating of the `ViewData`.
	pub(crate) fn update_view_data<C>(&mut self, callback: C)
	where C: FnOnce(&mut ViewDataUpdater<'_>) {
		let modified = {
			let mut view_data_updater = ViewDataUpdater::new(self);
			callback(&mut view_data_updater);
			view_data_updater.is_modified()
		};
		if modified {
			self.version = self.version.wrapping_add(1);
		}
	}

	pub(crate) fn clear(&mut self) {
		self.lines_leading.clear();
		self.lines.clear();
		self.lines_trailing.clear();
	}

	pub(crate) fn clear_body(&mut self) {
		self.lines.clear();
	}

	pub(crate) fn clear_visible_lines(&mut self) {
		self.visible_rows.clear();
	}

	pub(crate) fn ensure_line_visible(&mut self, row_index: usize) {
		self.visible_rows.push(row_index);
	}

	pub(crate) fn ensure_column_visible(&mut self, column_index: usize) {
		self.visible_column = Some(column_index);
	}

	pub(crate) fn set_show_title(&mut self, show: bool) {
		self.show_title = show;
	}

	pub(crate) fn set_show_help(&mut self, show: bool) {
		self.show_help = show;
	}

	pub(crate) fn push_leading_line(&mut self, view_line: ViewLine) {
		self.lines_leading.push(view_line);
	}

	pub(crate) fn push_line(&mut self, view_line: ViewLine) {
		self.lines.push(view_line);
	}

	pub(crate) fn push_trailing_line(&mut self, view_line: ViewLine) {
		self.lines_trailing.push(view_line);
	}

	pub(crate) fn set_retain_scroll_position(&mut self, value: bool) {
		self.retain_scroll_position = value;
	}

	pub(crate) fn reset_scroll_position(&mut self) {
		self.scroll_version = self.scroll_version.wrapping_add(1);
	}

	pub(crate) const fn show_title(&self) -> bool {
		self.show_title
	}

	pub(crate) const fn show_help(&self) -> bool {
		self.show_help
	}

	pub(crate) const fn leading_lines(&self) -> &ViewLines {
		&self.lines_leading
	}

	pub(crate) const fn lines(&self) -> &ViewLines {
		&self.lines
	}

	pub(crate) const fn trailing_lines(&self) -> &ViewLines {
		&self.lines_trailing
	}

	pub(crate) const fn get_visible_column(&self) -> Option<usize> {
		self.visible_column
	}

	pub(crate) const fn visible_rows(&self) -> &Vec<usize> {
		&self.visible_rows
	}

	pub(crate) fn get_name(&self) -> &str {
		self.name.as_str()
	}

	pub(crate) const fn get_version(&self) -> u32 {
		self.version
	}

	pub(crate) const fn retain_scroll_position(&self) -> bool {
		self.retain_scroll_position
	}

	pub(crate) const fn get_scroll_version(&self) -> u32 {
		self.scroll_version
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_updater_function() {
		let view_data = ViewData::new(|updater| updater.set_show_title(true));
		assert!(view_data.show_title());
	}

	#[test]
	fn update_view_data_without_modifications() {
		let mut view_data = ViewData::new(|_| {});
		let current_version = view_data.get_version();
		view_data.update_view_data(|_| {});
		assert_eq!(view_data.get_version(), current_version);
	}

	#[test]
	fn update_view_data_with_modifications() {
		let mut view_data = ViewData::new(|_| {});
		let current_version = view_data.get_version();
		view_data.update_view_data(|updater| updater.set_show_title(true));
		assert_ne!(view_data.get_version(), current_version);
	}

	#[test]
	fn clear() {
		let mut view_data = ViewData::new(|_| {});
		view_data.push_line(ViewLine::new_empty_line());
		view_data.push_leading_line(ViewLine::new_empty_line());
		view_data.push_trailing_line(ViewLine::new_empty_line());
		view_data.clear();
		assert_eq!(view_data.leading_lines().count(), 0);
		assert_eq!(view_data.lines().count(), 0);
		assert_eq!(view_data.trailing_lines().count(), 0);
	}

	#[test]
	fn clear_body() {
		let mut view_data = ViewData::new(|_| {});
		view_data.push_line(ViewLine::new_empty_line());
		view_data.push_leading_line(ViewLine::new_empty_line());
		view_data.push_trailing_line(ViewLine::new_empty_line());
		view_data.clear_body();
		assert_ne!(view_data.leading_lines().count(), 0);
		assert_eq!(view_data.lines().count(), 0);
		assert_ne!(view_data.trailing_lines().count(), 0);
	}

	#[test]
	fn ensure_line_visible() {
		let mut view_data = ViewData::new(|_| {});
		view_data.ensure_line_visible(10);
		view_data.ensure_line_visible(11);
		assert_eq!(view_data.visible_rows(), &vec![10, 11]);
	}

	#[test]
	fn clear_visible_lines() {
		let mut view_data = ViewData::new(|_| {});
		view_data.ensure_line_visible(10);
		view_data.clear_visible_lines();
		assert_eq!(view_data.visible_rows().len(), 0);
	}

	#[test]
	fn ensure_column_visible() {
		let mut view_data = ViewData::new(|_| {});
		view_data.ensure_column_visible(10);
		assert_eq!(view_data.get_visible_column().unwrap(), 10);
	}

	#[test]
	fn set_show_title() {
		let mut view_data = ViewData::new(|_| {});
		view_data.set_show_title(false);
		assert!(!view_data.show_title());
	}

	#[test]
	fn set_show_help() {
		let mut view_data = ViewData::new(|_| {});
		view_data.set_show_help(false);
		assert!(!view_data.show_help());
	}

	#[test]
	fn push_leading_line() {
		let mut view_data = ViewData::new(|_| {});
		view_data.push_leading_line(ViewLine::new_empty_line());
		assert_eq!(view_data.leading_lines().count(), 1);
	}

	#[test]
	fn push_line() {
		let mut view_data = ViewData::new(|_| {});
		view_data.push_line(ViewLine::new_empty_line());
		assert_eq!(view_data.lines().count(), 1);
	}

	#[test]
	fn push_trailing_line() {
		let mut view_data = ViewData::new(|_| {});
		view_data.push_trailing_line(ViewLine::new_empty_line());
		assert_eq!(view_data.trailing_lines().count(), 1);
	}

	#[test]
	fn set_retain_scroll_position() {
		let mut view_data = ViewData::new(|_| {});
		view_data.set_retain_scroll_position(false);
		assert!(!view_data.retain_scroll_position());
	}

	#[test]
	fn reset_scroll_position() {
		let mut view_data = ViewData::new(|_| {});
		let current_version = view_data.get_scroll_version();
		view_data.reset_scroll_position();
		assert_ne!(view_data.get_scroll_version(), current_version);
	}

	#[test]
	fn get_name() {
		let view_data_1 = ViewData::new(|_| {});
		let view_data_2 = ViewData::new(|_| {});
		assert!(!view_data_1.get_name().is_empty());
		assert_ne!(view_data_1.get_name(), view_data_2.get_name());
	}
}
