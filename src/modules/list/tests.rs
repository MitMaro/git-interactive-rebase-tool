mod abort_and_rebase;
mod activate;
mod change_action;
mod duplicate_line;
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
use crate::test_helpers::testers;
use crate::config::Config;

#[test]
fn resize() {
	testers::module(
		&["pick aaa c1"],
		&[Event::Resize(100, 200)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_all_events(&mut module);
			assert_eq!(module.height, 200);
		},
	);
}
