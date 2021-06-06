use super::{ViewData, ViewLine};

pub struct ViewDataUpdater<'v> {
	modified: bool,
	view_data: &'v mut ViewData,
}

impl<'v> ViewDataUpdater<'v> {
	pub(super) fn new(view_data: &'v mut ViewData) -> Self {
		Self {
			view_data,
			modified: false,
		}
	}

	pub(crate) fn clear(&mut self) {
		self.modified = true;
		self.view_data.clear();
	}

	pub(crate) fn clear_body(&mut self) {
		self.modified = true;
		self.view_data.clear_body();
	}

	pub(crate) fn ensure_line_visible(&mut self, row_index: usize) {
		self.modified = true;
		self.view_data.ensure_line_visible(row_index);
	}

	pub(crate) fn ensure_column_visible(&mut self, column_index: usize) {
		self.modified = true;
		self.view_data.ensure_column_visible(column_index);
	}

	pub(crate) fn set_show_title(&mut self, show: bool) {
		self.modified = true;
		self.view_data.set_show_title(show);
	}

	pub(crate) fn set_show_help(&mut self, show: bool) {
		self.modified = true;
		self.view_data.set_show_help(show);
	}

	pub(crate) fn push_leading_line(&mut self, view_line: ViewLine) {
		self.modified = true;
		self.view_data.push_leading_line(view_line);
	}

	pub(crate) fn push_line(&mut self, view_line: ViewLine) {
		self.modified = true;
		self.view_data.push_line(view_line);
	}

	pub(crate) fn push_trailing_line(&mut self, view_line: ViewLine) {
		self.modified = true;
		self.view_data.push_trailing_line(view_line);
	}

	pub(crate) fn set_retain_scroll_position(&mut self, value: bool) {
		self.modified = true;
		self.view_data.set_retain_scroll_position(value);
	}

	pub(crate) fn reset_scroll_position(&mut self) {
		self.modified = true;
		self.view_data.reset_scroll_position();
	}

	pub(super) const fn is_modified(&self) -> bool {
		self.modified
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn clear() {
		let mut view_data = ViewData::new(|_| {});
		view_data.push_line(ViewLine::new_empty_line());
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.clear();
		assert!(updater.is_modified());
		assert!(view_data.is_empty());
	}

	#[test]
	fn clear_body() {
		let mut view_data = ViewData::new(|_| {});
		view_data.push_line(ViewLine::new_empty_line());
		view_data.push_leading_line(ViewLine::new_empty_line());
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.clear_body();
		assert!(updater.is_modified());
		assert!(view_data.get_lines().is_empty());
		assert_eq!(view_data.get_leading_lines().len(), 1);
	}

	#[test]
	fn ensure_line_visible() {
		let mut view_data = ViewData::new(|_| {});
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.ensure_line_visible(10);
		assert!(updater.is_modified());
		assert_eq!(view_data.get_visible_row().unwrap(), 10);
	}

	#[test]
	fn ensure_column_visible() {
		let mut view_data = ViewData::new(|_| {});
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.ensure_column_visible(10);
		assert!(updater.is_modified());
		assert_eq!(view_data.get_visible_column().unwrap(), 10);
	}

	#[test]
	fn set_show_title() {
		let mut view_data = ViewData::new(|_| {});
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.set_show_title(true);
		assert!(updater.is_modified());
		assert!(view_data.show_title());
	}

	#[test]
	fn set_show_help() {
		let mut view_data = ViewData::new(|_| {});
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.set_show_help(true);
		assert!(updater.is_modified());
		assert!(view_data.show_help());
	}

	#[test]
	fn push_leading_line() {
		let mut view_data = ViewData::new(|_| {});
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.push_leading_line(ViewLine::new_empty_line());
		assert!(updater.is_modified());
		assert_eq!(view_data.get_leading_lines().len(), 1);
	}

	#[test]
	fn push_line() {
		let mut view_data = ViewData::new(|_| {});
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.push_line(ViewLine::new_empty_line());
		assert!(updater.is_modified());
		assert_eq!(view_data.get_lines().len(), 1);
	}

	#[test]
	fn push_trailing_line() {
		let mut view_data = ViewData::new(|_| {});
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.push_trailing_line(ViewLine::new_empty_line());
		assert!(updater.is_modified());
		assert_eq!(view_data.get_trailing_lines().len(), 1);
	}

	#[test]
	fn set_retain_scroll_position() {
		let mut view_data = ViewData::new(|_| {});
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.set_retain_scroll_position(false);
		assert!(updater.is_modified());
		assert!(!view_data.retain_scroll_position());
	}

	#[test]
	fn reset_scroll_position() {
		let mut view_data = ViewData::new(|_| {});
		let previous_scroll_version = view_data.get_scroll_version();
		let mut updater = ViewDataUpdater::new(&mut view_data);
		updater.reset_scroll_position();
		assert!(updater.is_modified());
		assert_ne!(view_data.get_scroll_version(), previous_scroll_version);
	}
}
