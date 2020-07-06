use crate::config::key_bindings::KeyBindings;
use unicode_segmentation::UnicodeSegmentation;

pub(super) fn get_list_normal_mode_help_lines(key_bindings: &KeyBindings) -> [(&str, &str); 22] {
	[
		(key_bindings.move_up.as_str(), "Move selection up"),
		(key_bindings.move_down.as_str(), "Move selection down"),
		(key_bindings.move_up_step.as_str(), "Move selection up 5 lines"),
		(key_bindings.move_down_step.as_str(), "Move selection down 5 lines"),
		(key_bindings.abort.as_str(), "Abort interactive rebase"),
		(
			key_bindings.force_abort.as_str(),
			"Immediately abort interactive rebase",
		),
		(key_bindings.rebase.as_str(), "Write interactive rebase file"),
		(
			key_bindings.force_rebase.as_str(),
			"Immediately write interactive rebase file",
		),
		(key_bindings.toggle_visual_mode.as_str(), "Enter visual mode"),
		(key_bindings.help.as_str(), "Show help"),
		(key_bindings.show_commit.as_str(), "Show commit information"),
		(key_bindings.move_selection_down.as_str(), "Move selected commit down"),
		(key_bindings.move_selection_up.as_str(), "Move selected commit up"),
		(key_bindings.action_break.as_str(), "Toggle break action"),
		(key_bindings.action_pick.as_str(), "Set selected commit to be picked"),
		(
			key_bindings.action_reword.as_str(),
			"Set selected commit to be reworded",
		),
		(key_bindings.action_edit.as_str(), "Set selected commit to be edited"),
		(
			key_bindings.action_squash.as_str(),
			"Set selected commit to be squashed",
		),
		(key_bindings.action_fixup.as_str(), "Set selected commit to be fixed-up"),
		(key_bindings.action_drop.as_str(), "Set selected commit to be dropped"),
		(key_bindings.edit.as_str(), "Edit an exec action's command"),
		(
			key_bindings.open_in_external_editor.as_str(),
			"Open the todo file in the default editor",
		),
	]
}

pub(super) fn get_list_visual_mode_help_lines(key_bindings: &KeyBindings) -> [(&str, &str); 14] {
	[
		(key_bindings.move_up.as_str(), "Move selection up"),
		(key_bindings.move_down.as_str(), "Move selection down"),
		(key_bindings.move_up_step.as_str(), "Move selection up 5 lines"),
		(key_bindings.move_down_step.as_str(), "Move selection down 5 lines"),
		(key_bindings.help.as_str(), "Show help"),
		(key_bindings.move_selection_down.as_str(), "Move selected commits down"),
		(key_bindings.move_selection_up.as_str(), "Move selected commits up"),
		(key_bindings.action_pick.as_str(), "Set selected commits to be picked"),
		(
			key_bindings.action_reword.as_str(),
			"Set selected commits to be reworded",
		),
		(key_bindings.action_edit.as_str(), "Set selected commits to be edited"),
		(
			key_bindings.action_squash.as_str(),
			"Set selected commits to be squashed",
		),
		(
			key_bindings.action_fixup.as_str(),
			"Set selected commits to be fixed-up",
		),
		(key_bindings.action_drop.as_str(), "Set selected commits to be dropped"),
		(key_bindings.toggle_visual_mode.as_str(), "Exit visual mode"),
	]
}

pub(super) fn get_show_commit_help_lines(key_bindings: &KeyBindings) -> [(&str, &str); 7] {
	[
		(key_bindings.move_up.as_str(), "Scroll up"),
		(key_bindings.move_down.as_str(), "Scroll down"),
		(key_bindings.move_up_step.as_str(), "Scroll up half a page"),
		(key_bindings.move_down_step.as_str(), "Scroll down half a page"),
		(key_bindings.move_right.as_str(), "Scroll right"),
		(key_bindings.move_left.as_str(), "Scroll left"),
		(key_bindings.help.as_str(), "Show help"),
	]
}

pub(super) fn get_max_help_key_length(lines: &[(&str, &str)]) -> usize {
	let mut max_length = 0;
	for (key, _) in lines {
		let len = UnicodeSegmentation::graphemes(*key, true).count();
		if len > max_length {
			max_length = len;
		}
	}
	max_length
}
