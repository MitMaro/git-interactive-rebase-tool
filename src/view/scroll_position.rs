use std::cell::RefCell;

#[derive(Debug, PartialEq)]
enum ScrollDirection {
	Up,
	Down,
}

pub(crate) struct ScrollPosition {
	left_value: RefCell<usize>,
	padding: usize,
	top_value: RefCell<usize>,
}

impl ScrollPosition {
	pub(crate) fn new(padding: usize) -> Self {
		Self {
			left_value: RefCell::new(0),
			padding,
			top_value: RefCell::new(0),
		}
	}

	pub(crate) fn reset(&self) {
		self.left_value.replace(0);
		self.top_value.replace(0);
	}

	pub(crate) fn scroll_up(&self, window_height: usize, lines_length: usize) {
		self.update_top(1, ScrollDirection::Up, window_height, lines_length);
	}

	pub(crate) fn scroll_down(&self, window_height: usize, lines_length: usize) {
		self.update_top(1, ScrollDirection::Down, window_height, lines_length);
	}

	pub(crate) fn page_up(&self, window_height: usize, lines_length: usize) {
		self.update_top(window_height / 2, ScrollDirection::Up, window_height, lines_length);
	}

	pub(crate) fn page_down(&self, window_height: usize, lines_length: usize) {
		self.update_top(window_height / 2, ScrollDirection::Down, window_height, lines_length);
	}

	pub(crate) fn scroll_left(&self, view_width: usize, max_line_width: usize) {
		let current_value = *self.left_value.borrow();
		if current_value != 0 {
			self.set_horizontal_scroll(current_value - 1, view_width, max_line_width);
		}
	}

	pub(crate) fn scroll_right(&self, view_width: usize, max_line_width: usize) {
		let current_value = *self.left_value.borrow();
		self.set_horizontal_scroll(current_value + 1, view_width, max_line_width);
	}

	fn set_horizontal_scroll(&self, new_value: usize, view_width: usize, max_line_width: usize) {
		// shrink view for a possible scroll bar
		let view_width = view_width - 1;
		if (new_value + view_width) > max_line_width {
			if view_width > max_line_width {
				self.left_value.replace(0);
			}
			else {
				self.left_value.replace(max_line_width - view_width);
			}
		}
		else {
			self.left_value.replace(new_value);
		}
	}

	pub(crate) fn ensure_cursor_visible(&self, cursor_position: usize, window_height: usize, lines_length: usize) {
		let view_height = window_height - self.padding;

		let current_value = *self.top_value.borrow();

		self.top_value.replace(match cursor_position {
			// show all if list is view height is long enough
			_ if lines_length <= view_height => 0,
			// last item selected, set top to show bottom of lines
			p if p >= lines_length - 1 => lines_length - view_height,
			// if on top two of list set top to top of list
			p if p < 1 => 0,
			// if selected item is hidden above top, shift top up
			p if p < current_value => p,
			// if selected item is hidden below, shift top down
			p if p >= current_value + view_height => p - view_height + 1,
			_ => current_value,
		});
	}

	pub(crate) fn get_top_position(&self) -> usize {
		*self.top_value.borrow()
	}

	pub(crate) fn get_left_position(&self) -> usize {
		*self.left_value.borrow()
	}

	fn update_top(&self, amount: usize, direction: ScrollDirection, window_height: usize, lines_length: usize) {
		let view_height = window_height - self.padding;

		if view_height >= lines_length {
			self.reset();
			return;
		}

		let current_value = *self.top_value.borrow();

		if direction == ScrollDirection::Up {
			if current_value < amount {
				self.reset();
			}
			else {
				self.top_value.replace(current_value - amount);
			}
		}
		else if current_value + amount + view_height > lines_length {
			self.top_value.replace(lines_length - view_height);
		}
		else {
			self.top_value.replace(current_value + amount);
		}
	}
}
