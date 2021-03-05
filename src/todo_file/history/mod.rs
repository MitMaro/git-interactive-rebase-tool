pub mod history_item;
mod operation;

#[cfg(test)]
mod tests;

use std::collections::VecDeque;

use crate::todo_file::{
	history::{history_item::HistoryItem, operation::Operation},
	line::Line,
	utils::{swap_range_down, swap_range_up},
};

pub struct History {
	redo_history: VecDeque<HistoryItem>,
	undo_history: VecDeque<HistoryItem>,
	limit: usize,
}

impl History {
	pub fn new(limit: u32) -> Self {
		Self {
			redo_history: VecDeque::new(),
			undo_history: VecDeque::new(),
			limit: limit as usize,
		}
	}

	pub fn apply_operation(lines: &mut Vec<Line>, operation: &HistoryItem) -> HistoryItem {
		match operation.operation {
			Operation::Modify => {
				let range = if operation.end_index <= operation.start_index {
					operation.end_index..=operation.start_index
				}
				else {
					operation.start_index..=operation.end_index
				};

				let mut changed_lines = vec![];
				for (i, index) in range.enumerate() {
					changed_lines.push(lines[index].clone());
					lines[index] = operation.lines[i].clone();
				}
				HistoryItem::new_modify(operation.start_index, operation.end_index, changed_lines)
			},
			Operation::Add => {
				let removed_line = lines.remove(operation.start_index);
				HistoryItem::new_remove(operation.start_index, removed_line)
			},
			Operation::Remove => {
				lines.insert(operation.start_index, operation.lines[0].clone());
				HistoryItem::new_add(operation.start_index)
			},
			Operation::SwapUp => {
				swap_range_down(lines, operation.start_index - 1, operation.end_index - 1);
				HistoryItem::new_swap_down(operation.start_index - 1, operation.end_index - 1)
			},
			Operation::SwapDown => {
				swap_range_up(lines, operation.start_index + 1, operation.end_index + 1);
				HistoryItem::new_swap_up(operation.start_index + 1, operation.end_index + 1)
			},
		}
	}

	pub fn record(&mut self, operations: HistoryItem) {
		self.redo_history.clear();
		// delete old entries on limit reached
		self.undo_history.push_back(operations);
		if self.undo_history.len() > self.limit {
			self.undo_history.pop_front();
		}
	}

	const fn get_last_index_range(history_item: &HistoryItem) -> (usize, usize) {
		match history_item.operation {
			Operation::Modify => (history_item.start_index, history_item.end_index),
			Operation::Add => (history_item.start_index, history_item.start_index),
			Operation::Remove | Operation::SwapUp => {
				let start_index = if history_item.start_index == 0 {
					0
				}
				else {
					history_item.start_index - 1
				};
				let end_index = if history_item.end_index == 0 {
					0
				}
				else {
					history_item.end_index - 1
				};
				(start_index, end_index)
			},
			Operation::SwapDown => (history_item.start_index + 1, history_item.end_index + 1),
		}
	}

	pub fn undo(&mut self, current: &mut Vec<Line>) -> Option<(usize, usize)> {
		self.undo_history.pop_back().map(|operation| {
			let history = Self::apply_operation(current, &operation);
			let update_range = Self::get_last_index_range(&history);
			self.redo_history.push_back(history);
			update_range
		})
	}

	pub fn redo(&mut self, current: &mut Vec<Line>) -> Option<(usize, usize)> {
		self.redo_history.pop_back().map(|operation| {
			let history = Self::apply_operation(current, &operation);
			let update_range = Self::get_last_index_range(&history);
			self.undo_history.push_back(history);
			update_range
		})
	}

	pub fn reset(&mut self) {
		self.undo_history.clear();
		self.redo_history.clear();
	}
}
