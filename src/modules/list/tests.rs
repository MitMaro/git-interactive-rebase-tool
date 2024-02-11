mod abort_and_rebase;
mod change_action;
mod edit_mode;
mod external_editor;
mod help;
mod insert_line;
mod movement;
mod normal_mode;
mod read_event;
mod remove_lines;
mod render;
mod search;
mod show_commit;
mod swap_lines;
mod toggle_break;
mod toggle_option;
mod undo_redo;
mod visual_mode;

use super::*;
use crate::testutil::module_test;

pub(crate) fn create_list(config: &Config, todo_file: TodoFile) -> List {
	List::new(config, Arc::new(Mutex::new(todo_file)))
}

#[test]
fn resize() {
	module_test(&["pick aaa c1"], &[Event::Resize(100, 200)], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		_ = test_context.handle_all_events(&mut module);
		assert_eq!(module.height, 200);
	});
}
