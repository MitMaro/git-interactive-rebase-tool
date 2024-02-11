use crate::todo_file::{Line, Operation};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct HistoryItem {
	pub(crate) start_index: usize,
	pub(crate) end_index: usize,
	pub(crate) operation: Operation,
	pub(crate) lines: Vec<Line>,
}

impl HistoryItem {
	pub(crate) const fn new_load() -> Self {
		Self {
			operation: Operation::Load,
			start_index: 0,
			end_index: 0,
			lines: vec![],
		}
	}

	pub(crate) fn new_modify(start_index: usize, end_index: usize, lines: Vec<Line>) -> Self {
		Self {
			operation: Operation::Modify,
			start_index,
			end_index,
			lines,
		}
	}

	pub(crate) const fn new_add(start_index: usize, end_index: usize) -> Self {
		Self {
			operation: Operation::Add,
			start_index,
			end_index,
			lines: vec![],
		}
	}

	pub(crate) fn new_remove(start_index: usize, end_index: usize, lines: Vec<Line>) -> Self {
		Self {
			operation: Operation::Remove,
			start_index,
			end_index,
			lines,
		}
	}

	pub(crate) const fn new_swap_up(start_index: usize, end_index: usize) -> Self {
		Self {
			operation: Operation::SwapUp,
			start_index,
			end_index,
			lines: vec![],
		}
	}

	pub(crate) const fn new_swap_down(start_index: usize, end_index: usize) -> Self {
		Self {
			operation: Operation::SwapDown,
			start_index,
			end_index,
			lines: vec![],
		}
	}
}
