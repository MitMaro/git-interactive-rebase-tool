pub(crate) fn get_scroll_position(position: usize, item_count: usize, height: usize) -> usize {
	if position == 0 {
		return 0;
	}

	let max_position = item_count - height - 1;

	if position == max_position + 1 {
		return height - 1;
	}

	match max_position {
		// special case for when input range is 0
		1 => (0.5 * height as f64) as usize,
		_ => {
			let slope = (height as f64 - 2.0 - 1.0) / (max_position as f64 - 1.0);
			let output = 1.0 + (slope * (position as f64 - 1.0));
			output.round() as usize
		},
	}
}
