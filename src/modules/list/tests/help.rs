use super::*;
use crate::{assert_rendered_output, input::KeyCode};

#[test]
fn normal_mode_help() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Help)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			module.state = ListState::Normal;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				" Key      Action{Pad( )}",
				"{BODY}",
				" Up      |Move selection up",
				" Down    |Move selection down",
				" PageUp  |Move selection up half a page",
				" PageDown|Move selection down half a page",
				" Home    |Move selection to top of the list",
				" End     |Move selection to end of the list",
				" Left    |Scroll content to the left",
				" Right   |Scroll content to the right",
				" q       |Abort interactive rebase",
				" Q       |Immediately abort interactive rebase",
				" w       |Write interactive rebase file",
				" W       |Immediately write interactive rebase file",
				" ?       |Show help",
				" j       |Move selected lines down",
				" k       |Move selected lines up",
				" c       |Show commit information",
				" b       |Toggle break action",
				" p       |Set selected commits to be picked",
				" r       |Set selected commits to be reworded",
				" e       |Set selected commits to be edited",
				" s       |Set selected commits to be squashed",
				" f       |Set selected commits to be fixed-up",
				" d       |Set selected commits to be dropped",
				" E       |Edit an exec, label, reset or merge action's content",
				" I       |Insert a new line",
				" Controld|Duplicate selected line",
				" Delete  |Completely remove the selected lines",
				" Controlz|Undo the last change",
				" Controly|Redo the previous undone change",
				" !       |Open the todo file in the default editor",
				" v       |Enter visual selection mode",
				"{TRAILING}",
				"Press any key to close"
			);
		},
	);
}

#[test]
fn normal_mode_help_event() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Help), Event::from(KeyCode::Enter)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			module.state = ListState::Normal;
			_ = test_context.handle_all_events(&mut module);
			assert!(!module.normal_mode_help.is_active());
		},
	);
}

#[test]
fn visual_mode_help() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Help)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			module.state = ListState::Visual;
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				" Key      Action{Pad( )}",
				"{BODY}",
				" Up      |Move selection up",
				" Down    |Move selection down",
				" PageUp  |Move selection up half a page",
				" PageDown|Move selection down half a page",
				" Home    |Move selection to top of the list",
				" End     |Move selection to end of the list",
				" Left    |Scroll content to the left",
				" Right   |Scroll content to the right",
				" q       |Abort interactive rebase",
				" Q       |Immediately abort interactive rebase",
				" w       |Write interactive rebase file",
				" W       |Immediately write interactive rebase file",
				" ?       |Show help",
				" j       |Move selected lines down",
				" k       |Move selected lines up",
				" p       |Set selected commits to be picked",
				" r       |Set selected commits to be reworded",
				" e       |Set selected commits to be edited",
				" s       |Set selected commits to be squashed",
				" f       |Set selected commits to be fixed-up",
				" d       |Set selected commits to be dropped",
				" Delete  |Completely remove the selected lines",
				" Controlz|Undo the last change",
				" Controly|Redo the previous undone change",
				" !       |Open the todo file in the default editor",
				" v       |Exit visual selection mode",
				"{TRAILING}",
				"Press any key to close"
			);
		},
	);
}

#[test]
fn visual_mode_help_event() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Help), Event::from(KeyCode::Enter)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			module.state = ListState::Visual;
			_ = test_context.handle_all_events(&mut module);
			assert!(!module.visual_mode_help.is_active());
		},
	);
}
