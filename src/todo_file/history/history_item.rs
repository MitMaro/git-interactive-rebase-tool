use crate::todo_file::{history::operation::Operation, line::Line};

#[derive(Debug, PartialEq)]
pub struct HistoryItem {
	pub start_index: usize,
	pub end_index: usize,
	pub operation: Operation,
	pub lines: Vec<Line>,
}

impl HistoryItem {
	pub fn new_modify(start_index: usize, end_index: usize, lines: Vec<Line>) -> Self {
		Self {
			operation: Operation::Modify,
			start_index,
			end_index,
			lines,
		}
	}

	pub const fn new_add(start_index: usize, end_index: usize) -> Self {
		Self {
			operation: Operation::Add,
			start_index,
			end_index,
			lines: vec![],
		}
	}

	pub fn new_remove(start_index: usize, end_index: usize, lines: Vec<Line>) -> Self {
		Self {
			operation: Operation::Remove,
			start_index,
			end_index,
			lines,
		}
	}

	pub const fn new_swap_up(start_index: usize, end_index: usize) -> Self {
		Self {
			operation: Operation::SwapUp,
			start_index,
			end_index,
			lines: vec![],
		}
	}

	pub const fn new_swap_down(start_index: usize, end_index: usize) -> Self {
		Self {
			operation: Operation::SwapDown,
			start_index,
			end_index,
			lines: vec![],
		}
	}
}
