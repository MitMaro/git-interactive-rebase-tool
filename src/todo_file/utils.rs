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

pub fn remove_range(lines: &mut Vec<Line>, start_index: usize, end_index: usize) -> Vec<Line> {
	let mut removed_lines = vec![];
	if end_index <= start_index {
		for _ in end_index..=start_index {
			removed_lines.push(lines.remove(end_index));
		}
	}
	else {
		for _ in start_index..=end_index {
			removed_lines.push(lines.remove(start_index));
		}
	};

	removed_lines
}

pub fn add_range(lines: &mut Vec<Line>, new_lines: &[Line], start_index: usize, end_index: usize) {
	let range = if end_index <= start_index {
		end_index..=start_index
	}
	else {
		start_index..=end_index
	};

	for (add_index, index) in range.enumerate() {
		lines.insert(index, new_lines[add_index].clone());
	}
}
