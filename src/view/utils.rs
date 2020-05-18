pub(crate) fn get_scroll_position_index(position: usize, item_count: usize, height: usize) -> usize {
	if position == 0 || item_count < height {
		return 0;
	}

	if position >= item_count - height {
		return height - 1;
	}

	let max_position = item_count - height - 1;

	match max_position {
		// special case for when input range is low, center scrollbar
		1 => (0.5 * height as f64) as usize,
		_ => {
			// TODO this can be improved to provide a smoother scroll
			let slope = (height as f64 - 2.0 - 1.0) / (max_position as f64 - 1.0);
			let output = 1.0 + (slope * (position as f64 - 1.0));
			output.round() as usize
		},
	}
}

#[cfg(test)]
mod tests {
	use crate::view::utils::get_scroll_position_index;

	#[test]
	fn get_scroll_position_case_top_position() {
		assert_eq!(get_scroll_position_index(0, 100, 10), 0);
	}

	#[test]
	fn get_scroll_position_case_end_position() {
		assert_eq!(get_scroll_position_index(100, 100, 10), 9);
	}

	#[test]
	fn get_scroll_position_case_position_one_down() {
		assert_eq!(get_scroll_position_index(1, 100, 10), 1);
	}

	#[test]
	fn get_scroll_position_case_position_low_input_range_1() {
		assert_eq!(get_scroll_position_index(1, 10, 8), 4);
	}

	#[test]
	fn get_scroll_position_case_item_count_smaller_than_height() {
		assert_eq!(get_scroll_position_index(1, 10, 11), 0);
	}

	#[test]
	fn get_scroll_position_case_position_outside_item_count() {
		assert_eq!(get_scroll_position_index(100, 100, 100), 99);
	}

	#[test]
	fn get_scroll_position_case_position_extreme_lows() {
		assert_eq!(get_scroll_position_index(0, 0, 0), 0);
		assert_eq!(get_scroll_position_index(1, 2, 1), 0);
		assert_eq!(get_scroll_position_index(1, 2, 0), 0);
	}
}
