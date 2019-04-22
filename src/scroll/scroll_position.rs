use std::cell::RefCell;

pub struct ScrollPosition {
	big_scroll: usize,
	padding: usize,
	small_scroll: usize,
	value: RefCell<usize>,
}

impl ScrollPosition {
	pub fn new(padding: usize, big_scroll: usize, small_scroll: usize) -> Self {
		Self {
			big_scroll,
			padding,
			small_scroll,
			value: RefCell::new(0 as usize),
		}
	}

	pub fn reset(&self) {
		self.value.replace(0);
	}

	pub fn scroll_up(&self, window_height: usize, lines_length: usize) {
		self.update_top(true, window_height, lines_length);
	}

	pub fn scroll_down(&self, window_height: usize, lines_length: usize) {
		self.update_top(false, window_height, lines_length);
	}

	pub fn ensure_cursor_visible(&self, cursor: usize, window_height: usize, lines_length: usize) {
		let view_height = window_height as usize - self.padding;

		let current_value = *self.value.borrow();

		// TODO I think this can be simplified
		self.value.replace(match cursor {
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

	pub fn get_position(&self) -> usize {
		*self.value.borrow()
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

		let current_value = *self.value.borrow();

		if scroll_up {
			if current_value < amount {
				self.reset();
			}
			else {
				self.value.replace(current_value - amount);
			}
		}
		else if current_value + amount + view_height > lines_length {
			self.value.replace(lines_length - view_height);
		}
		else {
			self.value.replace(current_value + amount);
		}
	}
}
