use std::cell::RefCell;

pub struct ScrollPosition {
	big_scroll: usize,
	left_value: RefCell<usize>,
	padding: usize,
	small_scroll: usize,
	top_value: RefCell<usize>,
}

impl ScrollPosition {
	pub fn new(padding: usize, big_scroll: usize, small_scroll: usize) -> Self {
		Self {
			big_scroll,
			left_value: RefCell::new(0 as usize),
			padding,
			small_scroll,
			top_value: RefCell::new(0 as usize),
		}
	}

	pub fn reset(&self) {
		self.left_value.replace(0);
		self.top_value.replace(0);
	}

	pub fn scroll_up(&self, window_height: usize, lines_length: usize) {
		self.update_top(true, window_height, lines_length);
	}

	pub fn scroll_down(&self, window_height: usize, lines_length: usize) {
		self.update_top(false, window_height, lines_length);
	}

	pub fn scroll_left(&self, view_width: usize, max_line_width: usize) {
		let current_value = *self.left_value.borrow();
		if current_value != 0 {
			self.set_horizontal_scroll(current_value - 1, view_width, max_line_width);
		}
	}

	pub fn scroll_right(&self, view_width: usize, max_line_width: usize) {
		let current_value = *self.left_value.borrow();
		self.set_horizontal_scroll(current_value + 1, view_width, max_line_width);
	}

	fn set_horizontal_scroll(&self, new_value: usize, view_width: usize, max_line_width: usize) {
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

	pub fn ensure_cursor_visible(&self, cursor: usize, window_height: usize, lines_length: usize) {
		let view_height = window_height as usize - self.padding;

		let current_value = *self.top_value.borrow();

		// TODO I think this can be simplified
		self.top_value.replace(match cursor {
			// show all if list is view height is long enough
			_ if lines_length <= view_height => 0,
			// last item selected, set top to show bottom of lines
			s if s >= lines_length - 1 => lines_length - view_height,
			// if on top two of list set top to top of list
			s if s < 1 => 0,
			// if selected item is hidden above top, shift top up
			s if s < current_value => s,
			// if starting scrolling, hide top two
			s if current_value == 0 && s >= view_height => 1,
			// if selected item is hidden below, shift top down
			s if s >= current_value + view_height => s - view_height + 1,
			_ => current_value,
		});
	}

	pub fn get_top_position(&self) -> usize {
		*self.top_value.borrow()
	}

	pub fn get_left_position(&self) -> usize {
		*self.left_value.borrow()
	}

	fn update_top(&self, scroll_up: bool, window_height: usize, lines_length: usize) {
		let view_height = window_height as usize - self.padding;

		if view_height >= lines_length {
			self.reset();
			return;
		}

		let amount = match view_height {
			h if h > 20 => self.big_scroll,
			h if h > 10 => self.small_scroll,
			_ => 1,
		};

		let current_value = *self.top_value.borrow();

		if scroll_up {
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
