mod abort_and_rebase;
mod change_action;
mod edit_mode;
mod external_editor;
mod help;
mod insert_line;
mod movement;
mod normal_mode;
mod remove_lines;
mod render;
mod show_commit;
mod swap_lines;
mod toggle_break;
mod undo_redo;
mod visual_mode;

use super::*;
use crate::testutil::module_test;

#[test]
fn resize() {
	module_test(&["pick aaa c1"], &[Event::Resize(100, 200)], |mut test_context| {
		let mut module = List::new(&Config::new());
		let _ = test_context.handle_all_events(&mut module);
		assert_eq!(module.height, 200);
	});
}
