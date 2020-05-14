#[derive(Debug, PartialEq)]
enum ScrollDirection {
	Up,
	Down,
	Left,
	Right,
}

pub(crate) struct ScrollPosition {
	left_value: usize,
	padding: usize,
	top_value: usize,
}

impl ScrollPosition {
	pub(crate) fn new(padding: usize) -> Self {
		Self {
			left_value: 0,
			padding,
			top_value: 0,
		}
	}

	pub(crate) fn reset(&mut self) {
		self.left_value = 0;
		self.top_value = 0;
	}

	pub(crate) fn scroll_up(&mut self, window_height: usize, lines_length: usize) {
		self.update_top(1, ScrollDirection::Up, window_height, lines_length);
	}

	pub(crate) fn scroll_down(&mut self, window_height: usize, lines_length: usize) {
		self.update_top(1, ScrollDirection::Down, window_height, lines_length);
	}

	pub(crate) fn page_up(&mut self, window_height: usize, lines_length: usize) {
		self.update_top(window_height / 2, ScrollDirection::Up, window_height, lines_length);
	}

	pub(crate) fn page_down(&mut self, window_height: usize, lines_length: usize) {
		self.update_top(window_height / 2, ScrollDirection::Down, window_height, lines_length);
	}

	pub(crate) fn scroll_left(&mut self, window_width: usize, max_line_width: usize) {
		self.update_left(1, ScrollDirection::Left, window_width, max_line_width);
	}

	pub(crate) fn scroll_right(&mut self, window_width: usize, max_line_width: usize) {
		self.update_left(1, ScrollDirection::Right, window_width, max_line_width);
	}

	pub(crate) fn view_resize(
		&mut self,
		window_height: usize,
		window_width: usize,
		lines_length: usize,
		max_line_length: usize,
	)
	{
		let top_value = self.top_value;
		let left_value = self.left_value;

		if window_height <= self.padding {
			self.top_value = 0;
		}
		else {
			let view_height = window_height - self.padding;

			if view_height >= lines_length {
				self.top_value = 0;
			}
			// recalculate top to remove any padding space below the set of lines
			else if lines_length > view_height && (lines_length - top_value) < view_height {
				self.update_top(
					view_height + top_value - lines_length,
					ScrollDirection::Up,
					window_height,
					lines_length,
				);
			}
		}

		if window_width <= 1 {
			self.left_value = 0;
		}
		else {
			let view_width = window_width - 1; // reduce by 1 for possible scroll bar

			if view_width >= max_line_length {
				self.left_value = 0;
			}
			// recalculate left to remove any padding space to the right
			else if max_line_length > view_width && (max_line_length - left_value) < view_width {
				self.update_left(
					view_width + left_value - max_line_length,
					ScrollDirection::Left,
					window_width,
					max_line_length,
				);
			}
		}
	}

	// TODO: reevaluate and add tests
	pub(crate) fn ensure_cursor_visible(
		&mut self,
		new_cursor_position: usize,
		window_height: usize,
		lines_length: usize,
	)
	{
		let view_height = window_height - self.padding;

		let current_value = self.top_value;

		self.top_value = match new_cursor_position {
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
		};
	}

	pub(crate) fn get_top_position(&self) -> usize {
		self.top_value
	}

	pub(crate) fn get_left_position(&self) -> usize {
		self.left_value
	}

	fn update_top(&mut self, amount: usize, direction: ScrollDirection, window_height: usize, lines_length: usize) {
		let view_height = window_height - self.padding;

		if view_height >= lines_length {
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
		else if current_value + amount + view_height > lines_length {
			self.top_value = lines_length - view_height;
		}
		else {
			self.top_value = current_value + amount;
		}
	}

	fn update_left(&mut self, amount: usize, direction: ScrollDirection, window_width: usize, max_line_width: usize) {
		// shrink view for a possible scroll bar
		let view_width = window_width - 1;

		if view_width >= max_line_width {
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
		else if self.left_value + amount + view_width > max_line_width {
			self.left_value = max_line_width - view_width;
		}
		else {
			self.left_value += amount;
		}
	}
}

#[cfg(test)]
mod tests {
	// Note: Some of these tests are duplicates logically, but are described differently
	use crate::view::scroll_position::ScrollPosition;

	#[test]
	fn scroll_position_new() {
		let scroll_position = ScrollPosition::new(0);

		assert_eq!(scroll_position.get_top_position(), 0);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_reset() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 1;
		scroll_position.left_value = 2;

		scroll_position.reset();

		assert_eq!(scroll_position.get_top_position(), 0);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_from_zero() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_up(10, 100);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_from_bottom() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 90; // 100-10
		scroll_position.scroll_up(10, 100);
		assert_eq!(scroll_position.get_top_position(), 89);
	}

	#[test]
	fn scroll_position_scroll_up_from_one_down() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 1; // 100-10
		scroll_position.scroll_up(10, 100);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_equals_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_up(10, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_equals_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_down(10, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_one_less_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 1;
		scroll_position.scroll_up(9, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_one_less_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_down(9, 10);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_one_greater_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 1;
		scroll_position.scroll_up(11, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_one_greater_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_down(11, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_equals_list_with_padding() {
		let mut scroll_position = ScrollPosition::new(2);
		scroll_position.scroll_up(8, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_equals_list_with_padding() {
		let mut scroll_position = ScrollPosition::new(2);
		scroll_position.scroll_down(12, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_one_less_list_with_padding() {
		let mut scroll_position = ScrollPosition::new(2);
		scroll_position.top_value = 1;
		scroll_position.scroll_up(11, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_one_less_list_with_padding() {
		let mut scroll_position = ScrollPosition::new(2);
		scroll_position.scroll_down(11, 10);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_scroll_up_when_view_size_one_greater_list_with_padding() {
		let mut scroll_position = ScrollPosition::new(2);
		scroll_position.top_value = 1;
		scroll_position.scroll_up(13, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_when_view_size_one_greater_list_with_padding() {
		let mut scroll_position = ScrollPosition::new(2);
		scroll_position.scroll_down(13, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_down_from_zero_with_room() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_down(10, 100);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_scroll_down_from_second_last_of_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 89; // 100-10-1
		scroll_position.scroll_down(10, 100);
		assert_eq!(scroll_position.get_top_position(), 90);
	}

	#[test]
	fn scroll_position_scroll_down_from_bottom_of_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 90; // 100-10
		scroll_position.scroll_down(10, 100);
		assert_eq!(scroll_position.get_top_position(), 90);
	}

	#[test]
	fn scroll_position_page_up_from_zero() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.page_up(10, 100);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_up_from_bottom() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 90; // 100-10
		scroll_position.page_up(10, 100);
		assert_eq!(scroll_position.get_top_position(), 85);
	}

	#[test]
	fn scroll_position_page_up_from_page_down() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 5;
		scroll_position.page_up(10, 100);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_up_when_view_size_equals_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.page_up(10, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_down_when_view_size_equals_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.page_down(10, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_up_when_view_size_less_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 4;
		scroll_position.page_up(9, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_down_when_view_size_one_less_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.page_down(9, 10);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_page_up_when_view_size_greater_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 5;
		scroll_position.page_up(11, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_down_when_view_size_one_greater_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.page_down(11, 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_page_down_from_zero_with_room() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.page_down(10, 100);
		assert_eq!(scroll_position.get_top_position(), 5);
	}

	#[test]
	fn scroll_position_page_down_from_bottom_of_list() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 85; // 100-10
		scroll_position.page_down(10, 100);
		assert_eq!(scroll_position.get_top_position(), 90);
	}

	#[test]
	fn scroll_position_scroll_left_from_zero() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_left(10, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_left_from_one() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 1;
		scroll_position.scroll_left(10, 100);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_left_from_middle() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 50;
		scroll_position.scroll_left(10, 100);
		assert_eq!(scroll_position.get_left_position(), 49);
	}

	#[test]
	fn scroll_position_scroll_left_near_right() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 90;
		scroll_position.scroll_left(10, 100);
		assert_eq!(scroll_position.get_left_position(), 89);
	}

	#[test]
	fn scroll_position_scroll_left_window_size_same_as_max_line_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_left(10, 10);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_left_window_size_greater_than_max_line_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_left(100, 10);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_right_from_zero() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_right(10, 100);
		assert_eq!(scroll_position.get_left_position(), 1);
	}

	#[test]
	fn scroll_position_scroll_right_from_one() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 1;
		scroll_position.scroll_right(10, 100);
		assert_eq!(scroll_position.get_left_position(), 2);
	}

	#[test]
	fn scroll_position_scroll_right_from_middle() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 50;
		scroll_position.scroll_right(10, 100);
		assert_eq!(scroll_position.get_left_position(), 51);
	}

	#[test]
	fn scroll_position_scroll_right_near_right() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 91;
		scroll_position.scroll_right(10, 100);
		assert_eq!(scroll_position.get_left_position(), 91);
	}

	#[test]
	fn scroll_position_scroll_right_window_size_same_as_max_line_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_right(10, 10);
		assert_eq!(scroll_position.get_left_position(), 1); // 1 for extra width padding
	}

	#[test]
	fn scroll_position_scroll_right_window_size_one_more_as_max_line_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_right(11, 10);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_scroll_right_window_size_greater_than_max_line_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.scroll_right(100, 10);
		assert_eq!(scroll_position.get_left_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_and_width_greater_than_number_of_lines_max_line_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 25;
		scroll_position.top_value = 25;
		scroll_position.view_resize(100, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_and_width_zero_no_padding() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 25;
		scroll_position.top_value = 25;
		scroll_position.view_resize(0, 0, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_and_width_zero_with_padding() {
		let mut scroll_position = ScrollPosition::new(5);
		scroll_position.left_value = 25;
		scroll_position.top_value = 25;
		scroll_position.view_resize(0, 0, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_one_greater_than_lines_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 25;
		scroll_position.view_resize(51, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_one_greater_than_lines_length_with_padding() {
		let mut scroll_position = ScrollPosition::new(3);
		scroll_position.top_value = 25;
		scroll_position.view_resize(54, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_exactly_lines_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 25;
		scroll_position.view_resize(50, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_exactly_lines_length_with_padding() {
		let mut scroll_position = ScrollPosition::new(3);
		scroll_position.top_value = 25;
		scroll_position.view_resize(53, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_height_one_less_than_lines_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 25;
		scroll_position.view_resize(49, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_view_resize_view_height_one_less_than_lines_length_with_padding() {
		let mut scroll_position = ScrollPosition::new(3);
		scroll_position.top_value = 25;
		scroll_position.view_resize(52, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_view_resize_view_height_large_resize_greater_lines_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 10;
		scroll_position.view_resize(20, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 10);
	}

	#[test]
	fn scroll_position_view_resize_view_height_large_resize_greater_at_limit() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 10;
		scroll_position.view_resize(40, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 10);
	}

	#[test]
	fn scroll_position_view_resize_view_height_large_resize_greater_one_pass_limit() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 10;
		scroll_position.view_resize(41, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 9);
	}

	#[test]
	fn scroll_position_view_resize_view_height_large_resize_greater_one_remain_limit() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.top_value = 10;
		scroll_position.view_resize(49, 100, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 1);
	}

	#[test]
	fn scroll_position_view_resize_view_width_one_greater_than_max_line_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 25;
		scroll_position.view_resize(100, 52, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_exactly_lines_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 25;
		scroll_position.view_resize(100, 51, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 0);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_one_less_than_lines_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 25;
		scroll_position.view_resize(100, 50, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 1);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_large_resize_greater_lines_length() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 10;
		scroll_position.view_resize(100, 21, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_large_resize_greater_at_limit() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 10;
		scroll_position.view_resize(100, 41, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 10);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_large_resize_greater_one_pass_limit() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 10;
		scroll_position.view_resize(100, 42, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 9);
		assert_eq!(scroll_position.get_top_position(), 0);
	}

	#[test]
	fn scroll_position_view_resize_view_width_large_resize_greater_one_remain_limit() {
		let mut scroll_position = ScrollPosition::new(0);
		scroll_position.left_value = 10;
		scroll_position.view_resize(100, 50, 50, 50);
		assert_eq!(scroll_position.get_left_position(), 1);
		assert_eq!(scroll_position.get_top_position(), 0);
	}
}
