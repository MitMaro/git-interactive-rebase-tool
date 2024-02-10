use crate::{
	events::{AppKeyBindings, Event, KeyBindings},
	input::KeyCode,
};

pub(crate) fn create_test_keybindings() -> KeyBindings {
	crate::input::testutil::create_test_keybindings(create_test_custom_keybindings())
}

pub(crate) fn create_test_custom_keybindings() -> AppKeyBindings {
	AppKeyBindings {
		abort: vec![Event::from(KeyCode::Char('q'))],
		action_break: vec![Event::from(KeyCode::Char('b'))],
		action_drop: vec![Event::from(KeyCode::Char('d'))],
		action_edit: vec![Event::from(KeyCode::Char('e'))],
		action_fixup: vec![Event::from(KeyCode::Char('f'))],
		action_pick: vec![Event::from(KeyCode::Char('p'))],
		action_reword: vec![Event::from(KeyCode::Char('r'))],
		action_squash: vec![Event::from(KeyCode::Char('s'))],
		confirm_yes: vec![Event::from(KeyCode::Char('y'))],
		edit: vec![Event::from(KeyCode::Char('E'))],
		force_abort: vec![Event::from(KeyCode::Char('Q'))],
		force_rebase: vec![Event::from(KeyCode::Char('W'))],
		insert_line: vec![Event::from(KeyCode::Char('I'))],
		move_down: vec![Event::from(KeyCode::Down)],
		move_down_step: vec![Event::from(KeyCode::PageDown)],
		move_end: vec![Event::from(KeyCode::End)],
		move_home: vec![Event::from(KeyCode::Home)],
		move_left: vec![Event::from(KeyCode::Left)],
		move_right: vec![Event::from(KeyCode::Right)],
		move_selection_down: vec![Event::from(KeyCode::Char('j'))],
		move_selection_up: vec![Event::from(KeyCode::Char('k'))],
		move_up: vec![Event::from(KeyCode::Up)],
		move_up_step: vec![Event::from(KeyCode::PageUp)],
		open_in_external_editor: vec![Event::from(KeyCode::Char('!'))],
		rebase: vec![Event::from(KeyCode::Char('w'))],
		remove_line: vec![Event::from(KeyCode::Delete)],
		show_commit: vec![Event::from(KeyCode::Char('c'))],
		show_diff: vec![Event::from(KeyCode::Char('d'))],
		toggle_visual_mode: vec![Event::from(KeyCode::Char('v'))],
		fixup_keep_message: vec![Event::from(KeyCode::Char('u'))],
		fixup_keep_message_with_editor: vec![Event::from(KeyCode::Char('U'))],
	}
}
