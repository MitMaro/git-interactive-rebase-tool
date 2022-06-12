use claim::assert_some_eq;
use testutils::assert_empty;

use super::*;

fn history_item_to_string(item: &HistoryItem) -> String {
	let range = if item.start_index == item.end_index {
		item.start_index.to_string()
	}
	else {
		format!("{}-{}", item.start_index, item.end_index)
	};
	format!(
		"{:?}[{}] {}",
		item.operation,
		range,
		item.lines.iter().map(Line::to_text).collect::<Vec<String>>().join(", ")
	)
}

fn _assert_history_items(actual: &[HistoryItem], expected: &[HistoryItem]) {
	let actual_strings: Vec<String> = actual.iter().map(history_item_to_string).collect();
	let expected_strings: Vec<String> = expected.iter().map(history_item_to_string).collect();
	pretty_assertions::assert_str_eq!(actual_strings.join("\n"), expected_strings.join("\n"));
}

macro_rules! assert_history_items {
	($history_items:expr, $($arg:expr),*) => {
		let expected = &vec![$( $arg, )*];
		_assert_history_items(&Vec::from($history_items), &expected);
	};
}

fn create_lines() -> Vec<Line> {
	vec![
		Line::new("pick aaa c1").unwrap(),
		Line::new("pick bbb c2").unwrap(),
		Line::new("pick ccc c3").unwrap(),
		Line::new("pick ddd c4").unwrap(),
		Line::new("pick eee c5").unwrap(),
	]
}

macro_rules! assert_todo_lines {
	($lines:expr, $($arg:expr),*) => {
		let expected = vec![$( Line::new($arg).unwrap(), )*];
		pretty_assertions::assert_str_eq!(
			$lines.iter().map(Line::to_text).collect::<Vec<String>>().join("\n"),
			expected.iter().map(Line::to_text).collect::<Vec<String>>().join("\n")
		);
	};
}

#[test]
fn new() {
	let history = History::new(100);
	assert_eq!(history.limit, 100);
	assert_empty!(history.undo_history);
	assert_empty!(history.redo_history);
}

#[test]
fn record_history() {
	let mut history = History::new(5);
	history.redo_history.push_front(HistoryItem::new_add(1, 1));
	history.record(HistoryItem::new_add(1, 1));
	assert_history_items!(history.undo_history, HistoryItem::new_add(1, 1));
	assert_empty!(history.redo_history);
}

#[test]
fn record_history_overflow_limit() {
	let mut history = History::new(3);
	history.record(HistoryItem::new_add(1, 1));
	history.record(HistoryItem::new_add(2, 2));
	history.record(HistoryItem::new_add(3, 3));
	history.record(HistoryItem::new_add(4, 4));
	assert_history_items!(
		history.undo_history,
		HistoryItem::new_add(2, 2),
		HistoryItem::new_add(3, 3),
		HistoryItem::new_add(4, 4)
	);
	assert_empty!(history.redo_history);
}

#[test]
fn undo_redo_add_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_add(0, 0));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (0, 0));
	assert_todo_lines!(lines, "pick bbb c2", "pick ccc c3", "pick ddd c4", "pick eee c5");
	assert_some_eq!(history.redo(&mut lines), (0, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_add_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_add(4, 4));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (3, 3));
	assert_todo_lines!(lines, "pick aaa c1", "pick bbb c2", "pick ccc c3", "pick ddd c4");
	assert_some_eq!(history.redo(&mut lines), (4, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_add_middle() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_add(2, 2));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 2));
	assert_todo_lines!(lines, "pick aaa c1", "pick bbb c2", "pick ddd c4", "pick eee c5");
	assert_some_eq!(history.redo(&mut lines), (2, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_add_range_start_index_at_top() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_add(0, 1));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (0, 0));
	assert_todo_lines!(lines, "pick ccc c3", "pick ddd c4", "pick eee c5");
	assert_some_eq!(history.redo(&mut lines), (0, 1));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_add_range_end_index_at_top() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_add(1, 0));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (0, 0));
	assert_todo_lines!(lines, "pick ccc c3", "pick ddd c4", "pick eee c5");
	assert_some_eq!(history.redo(&mut lines), (1, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_add_range_start_index_at_bottom() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_add(4, 3));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 2));
	assert_todo_lines!(lines, "pick aaa c1", "pick bbb c2", "pick ccc c3");
	assert_some_eq!(history.redo(&mut lines), (4, 3));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_add_range_end_index_at_bottom() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_add(3, 4));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 2));
	assert_todo_lines!(lines, "pick aaa c1", "pick bbb c2", "pick ccc c3");
	assert_some_eq!(history.redo(&mut lines), (3, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_remove_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_remove(0, 0, vec![Line::new("drop xxx cx").unwrap()]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (0, 0));
	assert_todo_lines!(
		lines,
		"drop xxx cx",
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (0, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_remove_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_remove(5, 5, vec![Line::new("drop xxx cx").unwrap()]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (5, 5));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5",
		"drop xxx cx"
	);
	assert_some_eq!(history.redo(&mut lines), (4, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_remove_middle() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_remove(2, 2, vec![Line::new("drop xxx cx").unwrap()]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"drop xxx cx",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (2, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_remove_range_start_index_top() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_remove(0, 1, vec![
		Line::new("drop xxx cx").unwrap(),
		Line::new("drop yyy cy").unwrap(),
	]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (0, 1));
	assert_todo_lines!(
		lines,
		"drop xxx cx",
		"drop yyy cy",
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (0, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_remove_range_start_index_bottom() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_remove(6, 5, vec![
		Line::new("drop xxx cx").unwrap(),
		Line::new("drop yyy cy").unwrap(),
	]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (6, 5));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5",
		"drop xxx cx",
		"drop yyy cy"
	);
	assert_some_eq!(history.redo(&mut lines), (4, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_remove_range_end_index_top() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_remove(1, 0, vec![
		Line::new("drop xxx cx").unwrap(),
		Line::new("drop yyy cy").unwrap(),
	]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (1, 0));
	assert_todo_lines!(
		lines,
		"drop xxx cx",
		"drop yyy cy",
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (0, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_remove_range_end_index_bottom() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_remove(5, 6, vec![
		Line::new("drop xxx cx").unwrap(),
		Line::new("drop yyy cy").unwrap(),
	]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (5, 6));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5",
		"drop xxx cx",
		"drop yyy cy"
	);
	assert_some_eq!(history.redo(&mut lines), (4, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_up_single_index_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_up(1, 1));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (1, 1));
	assert_todo_lines!(
		lines,
		"pick bbb c2",
		"pick aaa c1",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (0, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_up_single_index_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_up(4, 4));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (4, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick eee c5",
		"pick ddd c4"
	);
	assert_some_eq!(history.redo(&mut lines), (3, 3));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_up_single_index_middle() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_up(2, 2));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick ccc c3",
		"pick bbb c2",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (1, 1));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_up_range_down_index_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_up(1, 2));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (1, 2));
	assert_todo_lines!(
		lines,
		"pick ccc c3",
		"pick aaa c1",
		"pick bbb c2",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (0, 1));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_up_range_down_index_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_up(3, 4));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (3, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick eee c5",
		"pick ccc c3",
		"pick ddd c4"
	);
	assert_some_eq!(history.redo(&mut lines), (2, 3));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_up_range_up_index_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_up(2, 1));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 1));
	assert_todo_lines!(
		lines,
		"pick ccc c3",
		"pick aaa c1",
		"pick bbb c2",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (1, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_up_range_up_index_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_up(4, 3));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (4, 3));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick eee c5",
		"pick ccc c3",
		"pick ddd c4"
	);
	assert_some_eq!(history.redo(&mut lines), (3, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_down_range_down_index_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_down(0, 1));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (0, 1));
	assert_todo_lines!(
		lines,
		"pick bbb c2",
		"pick ccc c3",
		"pick aaa c1",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (1, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_down_range_down_index_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_down(2, 3));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 3));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ddd c4",
		"pick eee c5",
		"pick ccc c3"
	);
	assert_some_eq!(history.redo(&mut lines), (3, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_down_range_up_index_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_down(1, 0));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (1, 0));
	assert_todo_lines!(
		lines,
		"pick bbb c2",
		"pick ccc c3",
		"pick aaa c1",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (2, 1));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_swap_down_range_up_index_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_swap_down(3, 2));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (3, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ddd c4",
		"pick eee c5",
		"pick ccc c3"
	);
	assert_some_eq!(history.redo(&mut lines), (4, 3));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_modify_single_index_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_modify(0, 0, vec![Line::new("drop xxx cx").unwrap()]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (0, 0));
	assert_todo_lines!(
		lines,
		"drop xxx cx",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (0, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_modify_single_index_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_modify(4, 4, vec![Line::new("drop xxx cx").unwrap()]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (4, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"drop xxx cx"
	);
	assert_some_eq!(history.redo(&mut lines), (4, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_modify_single_index_middle() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_modify(2, 2, vec![Line::new("drop xxx cx").unwrap()]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"drop xxx cx",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (2, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_modify_range_down_index_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_modify(0, 2, vec![
		Line::new("drop xx1 c1").unwrap(),
		Line::new("drop xx2 c2").unwrap(),
		Line::new("drop xx3 c3").unwrap(),
	]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (0, 2));
	assert_todo_lines!(
		lines,
		"drop xx1 c1",
		"drop xx2 c2",
		"drop xx3 c3",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (0, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_modify_range_down_index_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_modify(2, 4, vec![
		Line::new("drop xx1 c1").unwrap(),
		Line::new("drop xx2 c2").unwrap(),
		Line::new("drop xx3 c3").unwrap(),
	]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"drop xx1 c1",
		"drop xx2 c2",
		"drop xx3 c3"
	);
	assert_some_eq!(history.redo(&mut lines), (2, 4));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_modify_range_up_index_start() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_modify(2, 0, vec![
		Line::new("drop xx1 c1").unwrap(),
		Line::new("drop xx2 c2").unwrap(),
		Line::new("drop xx3 c3").unwrap(),
	]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (2, 0));
	assert_todo_lines!(
		lines,
		"drop xx1 c1",
		"drop xx2 c2",
		"drop xx3 c3",
		"pick ddd c4",
		"pick eee c5"
	);
	assert_some_eq!(history.redo(&mut lines), (2, 0));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn undo_redo_modify_range_up_index_end() {
	let mut history = History::new(10);
	history.record(HistoryItem::new_modify(4, 2, vec![
		Line::new("drop xx1 c1").unwrap(),
		Line::new("drop xx2 c2").unwrap(),
		Line::new("drop xx3 c3").unwrap(),
	]));
	let mut lines = create_lines();
	assert_some_eq!(history.undo(&mut lines), (4, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"drop xx1 c1",
		"drop xx2 c2",
		"drop xx3 c3"
	);
	assert_some_eq!(history.redo(&mut lines), (4, 2));
	assert_todo_lines!(
		lines,
		"pick aaa c1",
		"pick bbb c2",
		"pick ccc c3",
		"pick ddd c4",
		"pick eee c5"
	);
}

#[test]
fn reset() {
	let mut history = History::new(3);
	history.redo_history.push_front(HistoryItem::new_add(1, 1));
	history.undo_history.push_front(HistoryItem::new_add(1, 1));
	history.reset();
	assert_empty!(history.undo_history);
	assert_empty!(history.redo_history);
}
