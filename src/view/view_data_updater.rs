use super::{view_line::ViewLine, ViewData};

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

	pub(crate) fn reset(&mut self) {
		self.modified = true;
		self.view_data.reset();
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

	pub(crate) fn scroll_right(&mut self) {
		self.view_data.scroll_right();
	}

	pub(crate) fn scroll_left(&mut self) {
		self.view_data.scroll_left();
	}

	pub(super) const fn is_modified(&self) -> bool {
		self.modified
	}
}
