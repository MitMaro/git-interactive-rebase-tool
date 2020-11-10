#[derive(Copy, Clone, Debug, PartialEq)]
enum ScrollDirection {
	Up,
	Down,
	Left,
	Right,
}

pub struct ScrollPosition {
	left_value: usize,
	lines_length: usize,
	max_line_width: usize,
	top_value: usize,
	view_height: usize,
	view_width: usize,
}

impl ScrollPosition {
	pub(crate) const fn new() -> Self {
		Self {
			left_value: 0,
			lines_length: 0,
			max_line_width: 0,
			top_value: 0,
			view_height: 0,
			view_width: 0,
		}
	}

	pub(crate) fn reset(&mut self) {
		self.left_value = 0;
		self.lines_length = 0;
		self.max_line_width = 0;
		self.top_value = 0;
	}

	pub(crate) fn scroll_up(&mut self) {
		self.update_top(1, ScrollDirection::Up);
	}

	pub(crate) fn scroll_down(&mut self) {
		self.update_top(1, ScrollDirection::Down);
	}

	pub(crate) fn page_up(&mut self) {
		self.update_top(self.view_height / 2, ScrollDirection::Up);
	}

	pub(crate) fn page_down(&mut self) {
		self.update_top(self.view_height / 2, ScrollDirection::Down);
	}

	pub(crate) fn scroll_left(&mut self) {
		self.update_left(1, ScrollDirection::Left);
	}

	pub(crate) fn scroll_right(&mut self) {
		self.update_left(1, ScrollDirection::Right);
	}

	pub(crate) fn ensure_line_visible(&mut self, line_index: usize) {
		let current_value = self.top_value;

		self.top_value = match line_index {
			// show all of list if view height is long enough
			_ if self.lines_length <= self.view_height => 0,
			// last item selected, set top to show bottom of lines
			p if p >= self.lines_length - 1 => self.lines_length - self.view_height,
			// if on top two of list set top to top of list
			p if p < 1 => 0,
			// if selected item is hidden above top, shift top up
			p if p < current_value => p,
			// if selected item is hidden below, shift top down
			p if p >= current_value + self.view_height => p - self.view_height + 1,
			_ => current_value,
		};
	}

	pub(crate) fn ensure_column_visible(&mut self, column_index: usize) {
		let current_value = self.left_value;

		self.left_value = match column_index {
			// show all of max column length if view width is wide enough
			_ if self.max_line_width <= self.view_width => 0,
			// last column selected, set left to show as much left as possible
			p if p >= self.max_line_width - 1 => self.max_line_width - self.view_width,
			// if on last two of column set left to zero
			p if p < 1 => 0,
			// if selected column is hidden to the left, shift left
			p if p < current_value => p,
			// if selected column is hidden to the right, shift right
			p if p >= current_value + self.view_width => p - self.view_width + 1,
			_ => current_value,
		};
	}

	pub(crate) const fn get_top_position(&self) -> usize {
		self.top_value
	}

	pub(crate) const fn get_left_position(&self) -> usize {
		self.left_value
	}

	pub(super) fn view_resize(&mut self, view_height: usize, view_width: usize) {
		if self.view_height != view_height || self.view_width != view_width {
			self.view_height = view_height;
			self.view_width = view_width;
			self.recalulate();
		}
	}

	pub(super) fn set_line_maximums(&mut self, max_line_width: usize, lines_length: usize) {
		if self.lines_length != lines_length || self.max_line_width != max_line_width {
			self.lines_length = lines_length;
			self.max_line_width = max_line_width;
			self.recalulate();
		}
	}

	fn recalulate(&mut self) {
		if self.view_height >= self.lines_length {
			self.top_value = 0;
		}
		// recalculate top to remove any padding space below the set of lines
		else if self.lines_length > self.view_height && (self.lines_length - self.top_value) < self.view_height {
			self.update_top(
				self.view_height + self.top_value - self.lines_length,
				ScrollDirection::Up,
			);
		}

		if self.view_width >= self.max_line_width {
			self.left_value = 0;
		}
		// recalculate left to remove any padding space to the right
		else if self.max_line_width > self.view_width && (self.max_line_width - self.left_value) < self.view_width {
			self.update_left(
				self.view_width + self.left_value - self.max_line_width,
				ScrollDirection::Left,
			);
		}
	}

	fn update_top(&mut self, amount: usize, direction: ScrollDirection) {
		if self.view_height >= self.lines_length {
			self.top_value = 0;
			return;
		}

		let current_value = self.top_value;

		if direction == ScrollDirection::Up {
			if current_value < amount {
				self.top_value = 0;
			}
			else {
				self.top_value = current_value - amount;
			}
		}
		else if current_value + amount + self.view_height > self.lines_length {
			self.top_value = self.lines_length - self.view_height;
		}
		else {
			self.top_value = current_value + amount;
		}
	}

	fn update_left(&mut self, amount: usize, direction: ScrollDirection) {
		if self.view_width >= self.max_line_width {
			self.left_value = 0;
			return;
		}

		if direction == ScrollDirection::Left {
			if self.left_value < amount {
				self.left_value = 0;
			}
			else {
				self.left_value -= amount;
			}
		}
		else if self.left_value + amount + self.view_width > self.max_line_width {
			self.left_value = self.max_line_width - self.view_width;
		}
		else {
			self.left_value += amount;
		}
	}
}

#[cfg(test)]
mod tests {
	// Note: Some of these tests are duplicates logically, but are described differently
	use super::*;

	#[test]
	fn scroll_position_new() {
		let scroll_position = ScrollPosition::new();

		assert_eq!(scroll_position.get_top_position(), 0);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_reset() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.top_value = 1;
		scroll_position.left_value = 2;

		scroll_position.reset();

		assert_eq!(scroll_position.get_top_position(), 0);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_from_zero() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.scroll_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_from_bottom() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.top_value = 90; // 100-10
		scroll_position.scroll_up();
		assert_eq!(scroll_position.get_top_position(), 89);
	}

	#[test]
	fn scroll_position_scroll_up_from_one_down() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.top_value = 1; // 100-10
		scroll_position.scroll_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_equals_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 10;
		scroll_position.scroll_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_equals_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 10;
		scroll_position.scroll_down();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_one_less_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 9;
		scroll_position.top_value = 1;
		scroll_position.scroll_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_one_less_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 9;
		scroll_position.scroll_down();
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_one_greater_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 11;
		scroll_position.top_value = 1;
		scroll_position.scroll_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_one_greater_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 11;
		scroll_position.scroll_down();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_from_zero_with_room() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.scroll_down();
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_scroll_down_from_second_last_of_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.top_value = 89; // 100-10-1
		scroll_position.scroll_down();
		assert_eq!(scroll_position.get_top_position(), 90);
	}

	#[test]
	fn scroll_position_scroll_down_from_bottom_of_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.top_value = 90; // 100-10
		scroll_position.scroll_down();
		assert_eq!(scroll_position.get_top_position(), 90);
	}

	#[test]
	fn scroll_position_page_up_from_zero() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.page_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_up_from_bottom() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.top_value = 90; // 100-10
		scroll_position.page_up();
		assert_eq!(scroll_position.get_top_position(), 85);
	}

	#[test]
	fn scroll_position_page_up_from_page_down() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.top_value = 5;
		scroll_position.page_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_up_when_view_size_equals_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 10;
		scroll_position.page_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_down_when_view_size_equals_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 10;
		scroll_position.page_down();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_up_when_view_size_less_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 9;
		scroll_position.top_value = 4;
		scroll_position.page_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_down_when_view_size_one_less_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 9;
		scroll_position.page_down();
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_page_up_when_view_size_greater_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 11;
		scroll_position.top_value = 5;
		scroll_position.page_up();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_down_when_view_size_one_greater_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_height = 11;
		scroll_position.page_down();
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_down_from_zero_with_room() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 11;
		scroll_position.page_down();
		assert_eq!(scroll_position.get_top_position(), 5);
	}

	#[test]
	fn scroll_position_page_down_from_bottom_of_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 100;
		scroll_position.view_height = 10;
		scroll_position.top_value = 85; // 100-10
		scroll_position.page_down();
		assert_eq!(scroll_position.get_top_position(), 90);
	}

	#[test]
	fn scroll_position_scroll_left_from_zero() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 100;
		scroll_position.view_width = 10;
		scroll_position.scroll_left();
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_left_from_one() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 100;
		scroll_position.view_width = 10;
		scroll_position.left_value = 1;
		scroll_position.scroll_left();
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_left_from_middle() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 100;
		scroll_position.view_width = 10;
		scroll_position.left_value = 50;
		scroll_position.scroll_left();
		assert_eq!(scroll_position.get_left_position(), 49);
	}

	#[test]
	fn scroll_position_scroll_left_near_right() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 100;
		scroll_position.view_width = 10;
		scroll_position.left_value = 90;
		scroll_position.scroll_left();
		assert_eq!(scroll_position.get_left_position(), 89);
	}

	#[test]
	fn scroll_position_scroll_left_window_size_same_as_max_line_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_width = 10;
		scroll_position.scroll_left();
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_left_window_size_greater_than_max_line_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_width = 100;
		scroll_position.scroll_left();
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_right_from_zero() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 100;
		scroll_position.view_width = 10;
		scroll_position.scroll_right();
		assert_eq!(scroll_position.get_left_position(), 1);
	}

	#[test]
	fn scroll_position_scroll_right_from_one() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 100;
		scroll_position.view_width = 10;
		scroll_position.left_value = 1;
		scroll_position.scroll_right();
		assert_eq!(scroll_position.get_left_position(), 2);
	}

	#[test]
	fn scroll_position_scroll_right_from_middle() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 100;
		scroll_position.view_width = 10;
		scroll_position.left_value = 50;
		scroll_position.scroll_right();
		assert_eq!(scroll_position.get_left_position(), 51);
	}

	#[test]
	fn scroll_position_scroll_right_near_right() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 100;
		scroll_position.view_width = 10;
		scroll_position.left_value = 90;
		scroll_position.scroll_right();
		assert_eq!(scroll_position.get_left_position(), 90);
	}

	#[test]
	fn scroll_position_scroll_right_window_size_same_as_max_line_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_width = 10;
		scroll_position.scroll_right();
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_right_window_size_one_more_as_max_line_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_width = 11;
		scroll_position.scroll_right();
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_right_window_size_greater_than_max_line_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_width = 100;
		scroll_position.scroll_right();
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_set_height_width() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.view_resize(111, 222);
		assert_eq!(scroll_position.view_height, 111);
		assert_eq!(scroll_position.view_width, 222);
	}

	#[test]
	fn scroll_position_view_resize_view_height_and_width_greater_than_number_of_lines_max_line_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 25;
		scroll_position.top_value = 25;
		scroll_position.lines_length = 50;
		scroll_position.view_resize(100, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_and_width_zero() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 25;
		scroll_position.top_value = 26;
		scroll_position.lines_length = 50;
		scroll_position.max_line_width = 50;
		scroll_position.view_resize(0, 0);
		assert_eq!(scroll_position.get_left_position(), 25);
		assert_eq!(scroll_position.get_top_position(), 26);
	}

	#[test]
	fn scroll_position_view_resize_view_height_one_greater_than_lines_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.top_value = 25;
		scroll_position.lines_length = 50;
		scroll_position.view_resize(51, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_exactly_lines_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.top_value = 25;
		scroll_position.lines_length = 50;
		scroll_position.view_resize(50, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_one_less_than_lines_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.top_value = 25;
		scroll_position.lines_length = 50;
		scroll_position.view_resize(49, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_view_resize_view_height_large_resize_greater_lines_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.top_value = 10;
		scroll_position.lines_length = 50;
		scroll_position.view_resize(20, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 10);
	}

	#[test]
	fn scroll_position_view_resize_view_height_large_resize_greater_at_limit() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.top_value = 10;
		scroll_position.lines_length = 50;
		scroll_position.view_resize(40, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 10);
	}

	#[test]
	fn scroll_position_view_resize_view_height_large_resize_greater_one_pass_limit() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.top_value = 10;
		scroll_position.lines_length = 50;
		scroll_position.view_resize(41, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 9);
	}

	#[test]
	fn scroll_position_view_resize_view_height_large_resize_greater_one_remain_limit() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.top_value = 10;
		scroll_position.lines_length = 50;
		scroll_position.view_resize(49, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_view_resize_view_width_one_greater_than_max_line_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 25;
		scroll_position.max_line_width = 50;
		scroll_position.view_resize(100, 52);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_exactly_lines_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 25;
		scroll_position.max_line_width = 50;
		scroll_position.view_resize(100, 51);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_one_less_than_lines_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 25;
		scroll_position.max_line_width = 50;
		scroll_position.view_resize(100, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_large_resize_greater_lines_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 10;
		scroll_position.max_line_width = 50;
		scroll_position.view_resize(100, 21);
		assert_eq!(scroll_position.get_left_position(), 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_large_resize_greater_at_limit() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 10;
		scroll_position.max_line_width = 50;
		scroll_position.view_resize(100, 40);
		assert_eq!(scroll_position.get_left_position(), 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_large_resize_greater_one_pass_limit() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 10;
		scroll_position.max_line_width = 50;
		scroll_position.view_resize(100, 41);
		assert_eq!(scroll_position.get_left_position(), 9);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_large_resize_greater_one_remain_limit() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.left_value = 10;
		scroll_position.max_line_width = 50;
		scroll_position.view_resize(100, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_ensure_line_visible_move_index_down_to_scroll_boundary() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_resize(5, 0);
		scroll_position.ensure_line_visible(0);
		assert_eq!(scroll_position.get_top_position(), 0);
		scroll_position.ensure_line_visible(1);
		assert_eq!(scroll_position.get_top_position(), 0);
		scroll_position.ensure_line_visible(2);
		assert_eq!(scroll_position.get_top_position(), 0);
		scroll_position.ensure_line_visible(3);
		assert_eq!(scroll_position.get_top_position(), 0);
		scroll_position.ensure_line_visible(4);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_ensure_line_visible_move_index_down_from_scroll_boundary_to_bottom_of_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_resize(5, 0);
		scroll_position.ensure_line_visible(5);
		assert_eq!(scroll_position.get_top_position(), 1);
		scroll_position.ensure_line_visible(6);
		assert_eq!(scroll_position.get_top_position(), 2);
		scroll_position.ensure_line_visible(7);
		assert_eq!(scroll_position.get_top_position(), 3);
		scroll_position.ensure_line_visible(8);
		assert_eq!(scroll_position.get_top_position(), 4);
		scroll_position.ensure_line_visible(9);
		assert_eq!(scroll_position.get_top_position(), 5);
	}

	#[test]
	fn scroll_position_ensure_line_visible_move_index_down_past_list_length() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_resize(5, 0);
		scroll_position.ensure_line_visible(100);
		assert_eq!(scroll_position.get_top_position(), 5);
	}

	#[test]
	fn scroll_position_ensure_line_visible_move_index_jump_to_bottom() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.view_resize(5, 0);
		scroll_position.ensure_line_visible(9);
		assert_eq!(scroll_position.get_top_position(), 5);
	}

	#[test]
	fn scroll_position_ensure_line_visible_move_index_up_to_scroll_boundary() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.top_value = 5;
		scroll_position.view_resize(5, 0);
		scroll_position.ensure_line_visible(9);
		assert_eq!(scroll_position.get_top_position(), 5);
		scroll_position.ensure_line_visible(8);
		assert_eq!(scroll_position.get_top_position(), 5);
		scroll_position.ensure_line_visible(7);
		assert_eq!(scroll_position.get_top_position(), 5);
		scroll_position.ensure_line_visible(6);
		assert_eq!(scroll_position.get_top_position(), 5);
		scroll_position.ensure_line_visible(5);
		assert_eq!(scroll_position.get_top_position(), 5);
	}

	#[test]
	fn scroll_position_ensure_line_visible_move_index_up_from_scroll_boundary_to_top_of_list() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.top_value = 5;
		scroll_position.view_resize(5, 0);
		scroll_position.ensure_line_visible(4);
		assert_eq!(scroll_position.get_top_position(), 4);
		scroll_position.ensure_line_visible(3);
		assert_eq!(scroll_position.get_top_position(), 3);
		scroll_position.ensure_line_visible(2);
		assert_eq!(scroll_position.get_top_position(), 2);
		scroll_position.ensure_line_visible(1);
		assert_eq!(scroll_position.get_top_position(), 1);
		scroll_position.ensure_line_visible(0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_ensure_line_visible_move_index_jump_to_top() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.lines_length = 10;
		scroll_position.top_value = 5;
		scroll_position.view_resize(5, 0);
		scroll_position.ensure_line_visible(0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn ensure_column_visible_move_index_right_to_boundary() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_resize(1, 5);
		scroll_position.ensure_column_visible(0);
		assert_eq!(scroll_position.get_left_position(), 0);
		scroll_position.ensure_column_visible(1);
		assert_eq!(scroll_position.get_left_position(), 0);
		scroll_position.ensure_column_visible(2);
		assert_eq!(scroll_position.get_left_position(), 0);
		scroll_position.ensure_column_visible(3);
		assert_eq!(scroll_position.get_left_position(), 0);
		scroll_position.ensure_column_visible(4);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn ensure_column_visible_move_index_right_to_end_of_line() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_resize(1, 5);
		scroll_position.ensure_column_visible(5);
		assert_eq!(scroll_position.get_left_position(), 1);
		scroll_position.ensure_column_visible(6);
		assert_eq!(scroll_position.get_left_position(), 2);
		scroll_position.ensure_column_visible(7);
		assert_eq!(scroll_position.get_left_position(), 3);
		scroll_position.ensure_column_visible(8);
		assert_eq!(scroll_position.get_left_position(), 4);
		scroll_position.ensure_column_visible(9);
		assert_eq!(scroll_position.get_left_position(), 5);
	}

	#[test]
	fn ensure_column_visible_move_index_right_past_end_of_line() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_resize(1, 5);
		scroll_position.ensure_column_visible(100);
		assert_eq!(scroll_position.get_left_position(), 5);
	}

	#[test]
	fn ensure_column_visible_move_index_jump_right_to_end() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.view_resize(1, 5);
		scroll_position.ensure_column_visible(9);
		assert_eq!(scroll_position.get_left_position(), 5);
	}

	#[test]
	fn ensure_column_visible_move_index_to_start_of_line() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.left_value = 5;
		scroll_position.view_resize(1, 5);
		scroll_position.ensure_column_visible(9);
		assert_eq!(scroll_position.get_left_position(), 5);
		scroll_position.ensure_column_visible(8);
		assert_eq!(scroll_position.get_left_position(), 5);
		scroll_position.ensure_column_visible(7);
		assert_eq!(scroll_position.get_left_position(), 5);
		scroll_position.ensure_column_visible(6);
		assert_eq!(scroll_position.get_left_position(), 5);
		scroll_position.ensure_column_visible(5);
		assert_eq!(scroll_position.get_left_position(), 5);
	}

	#[test]
	fn ensure_column_visible_move_index_from_scroll_boundary_to_start_of_line() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.left_value = 5;
		scroll_position.view_resize(1, 5);
		scroll_position.ensure_column_visible(4);
		assert_eq!(scroll_position.get_left_position(), 4);
		scroll_position.ensure_column_visible(3);
		assert_eq!(scroll_position.get_left_position(), 3);
		scroll_position.ensure_column_visible(2);
		assert_eq!(scroll_position.get_left_position(), 2);
		scroll_position.ensure_column_visible(1);
		assert_eq!(scroll_position.get_left_position(), 1);
		scroll_position.ensure_column_visible(0);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn ensure_column_visible_move_index_jump_right_to_start() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.left_value = 5;
		scroll_position.view_resize(1, 5);
		scroll_position.ensure_column_visible(0);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn set_line_maximums() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 1;
		scroll_position.lines_length = 1;
		scroll_position.set_line_maximums(10, 20);
		assert_eq!(scroll_position.max_line_width, 10);
		assert_eq!(scroll_position.lines_length, 20);
	}

	#[test]
	fn set_line_maximums_no_change() {
		let mut scroll_position = ScrollPosition::new();
		scroll_position.max_line_width = 10;
		scroll_position.lines_length = 20;
		scroll_position.set_line_maximums(10, 20);
		assert_eq!(scroll_position.max_line_width, 10);
		assert_eq!(scroll_position.lines_length, 20);
	}
}
