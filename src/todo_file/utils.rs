use crate::todo_file::line::Line;

pub fn swap_range_up(lines: &mut Vec<Line>, start_index: usize, end_index: usize) {
	let range = if end_index <= start_index {
		(end_index - 1)..start_index
	}
	else {
		(start_index - 1)..end_index
	};
	for index in range {
		lines.swap(index, index + 1);
	}
}

pub fn swap_range_down(lines: &mut Vec<Line>, start_index: usize, end_index: usize) {
	let range = if end_index <= start_index {
		end_index..=start_index
	}
	else {
		start_index..=end_index
	};

	for index in range.rev() {
		lines.swap(index, index + 1);
	}
}
