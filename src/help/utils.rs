use crate::Config;
use unicode_segmentation::UnicodeSegmentation;

pub(super) fn get_list_normal_mode_help_lines(config: &Config) -> [(&str, &str); 22] {
	[
		(config.input_move_up.as_str(), "Move selection up"),
		(config.input_move_down.as_str(), "Move selection down"),
		(config.input_move_up_step.as_str(), "Move selection up 5 lines"),
		(config.input_move_down_step.as_str(), "Move selection down 5 lines"),
		(config.input_abort.as_str(), "Abort interactive rebase"),
		(
			config.input_force_abort.as_str(),
			"Immediately abort interactive rebase",
		),
		(config.input_rebase.as_str(), "Write interactive rebase file"),
		(
			config.input_force_rebase.as_str(),
			"Immediately write interactive rebase file",
		),
		(config.input_toggle_visual_mode.as_str(), "Enter visual mode"),
		(config.input_help.as_str(), "Show help"),
		(config.input_show_commit.as_str(), "Show commit information"),
		(config.input_move_selection_down.as_str(), "Move selected commit down"),
		(config.input_move_selection_up.as_str(), "Move selected commit up"),
		(config.input_action_break.as_str(), "Toggle break action"),
		(config.input_action_pick.as_str(), "Set selected commit to be picked"),
		(
			config.input_action_reword.as_str(),
			"Set selected commit to be reworded",
		),
		(config.input_action_edit.as_str(), "Set selected commit to be edited"),
		(
			config.input_action_squash.as_str(),
			"Set selected commit to be squashed",
		),
		(config.input_action_fixup.as_str(), "Set selected commit to be fixed-up"),
		(config.input_action_drop.as_str(), "Set selected commit to be dropped"),
		(config.input_edit.as_str(), "Edit an exec action's command"),
		(
			config.input_open_in_external_editor.as_str(),
			"Open the todo file in the default editor",
		),
	]
}

pub(super) fn get_list_visual_mode_help_lines(config: &Config) -> [(&str, &str); 14] {
	[
		(config.input_move_up.as_str(), "Move selection up"),
		(config.input_move_down.as_str(), "Move selection down"),
		(config.input_move_up_step.as_str(), "Move selection up 5 lines"),
		(config.input_move_down_step.as_str(), "Move selection down 5 lines"),
		(config.input_help.as_str(), "Show help"),
		(config.input_move_selection_down.as_str(), "Move selected commits down"),
		(config.input_move_selection_up.as_str(), "Move selected commits up"),
		(config.input_action_pick.as_str(), "Set selected commits to be picked"),
		(
			config.input_action_reword.as_str(),
			"Set selected commits to be reworded",
		),
		(config.input_action_edit.as_str(), "Set selected commits to be edited"),
		(
			config.input_action_squash.as_str(),
			"Set selected commits to be squashed",
		),
		(
			config.input_action_fixup.as_str(),
			"Set selected commits to be fixed-up",
		),
		(config.input_action_drop.as_str(), "Set selected commits to be dropped"),
		(config.input_toggle_visual_mode.as_str(), "Exit visual mode"),
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
