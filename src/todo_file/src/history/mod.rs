mod history_item;
mod operation;

#[cfg(test)]
mod tests;

use std::{cmp::min, collections::VecDeque};

pub(crate) use super::history::{history_item::HistoryItem, operation::Operation};
use super::{
	line::Line,
	utils::{add_range, remove_range, swap_range_down, swap_range_up},
};

#[derive(Debug)]
pub(crate) struct History {
	redo_history: VecDeque<HistoryItem>,
	undo_history: VecDeque<HistoryItem>,
	limit: usize,
}

impl History {
	pub(crate) fn new(limit: u32) -> Self {
		Self {
			redo_history: VecDeque::new(),
			undo_history: VecDeque::new(),
			limit: limit.try_into().expect("History limit is too large"),
		}
	}

	pub(crate) fn apply_operation(lines: &mut Vec<Line>, operation: &HistoryItem) -> HistoryItem {
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
				let removed_lines = remove_range(lines, operation.start_index, operation.end_index);
				HistoryItem::new_remove(operation.start_index, operation.end_index, removed_lines)
			},
			Operation::Remove => {
				add_range(lines, &operation.lines, operation.start_index, operation.end_index);
				HistoryItem::new_add(operation.start_index, operation.end_index)
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

	pub(crate) fn record(&mut self, operations: HistoryItem) {
		self.redo_history.clear();
		// delete old entries on limit reached
		self.undo_history.push_back(operations);
		if self.undo_history.len() > self.limit {
			let _pop_result = self.undo_history.pop_front();
		}
	}

	pub(crate) fn undo(&mut self, current: &mut Vec<Line>) -> Option<(usize, usize)> {
		self.undo_history.pop_back().map(|operation| {
			let history = Self::apply_operation(current, &operation);
			let update_range = Self::get_last_index_range(&history, current.len());
			self.redo_history.push_back(history);
			update_range
		})
	}

	pub(crate) fn redo(&mut self, current: &mut Vec<Line>) -> Option<(usize, usize)> {
		self.redo_history.pop_back().map(|operation| {
			let history = Self::apply_operation(current, &operation);
			let update_range = Self::get_last_index_range(&history, current.len());
			self.undo_history.push_back(history);
			update_range
		})
	}

	pub(crate) fn reset(&mut self) {
		self.undo_history.clear();
		self.redo_history.clear();
	}

	fn get_last_index_range(history_item: &HistoryItem, list_length: usize) -> (usize, usize) {
		match history_item.operation {
			Operation::Add | Operation::Modify => (history_item.start_index, history_item.end_index),
			Operation::Remove => {
				let index = min(history_item.start_index, history_item.end_index);
				if index == 0 || list_length == 0 {
					(0, 0)
				}
				else if index >= list_length {
					(list_length - 1, list_length - 1)
				}
				else {
					(index, index)
				}
			},
			Operation::SwapUp => {
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
}
