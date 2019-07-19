use crate::Config;
use unicode_segmentation::UnicodeSegmentation;

pub fn get_list_normal_mode_help_lines(config: &Config) -> [(String, &str); 21] {
	[
		(String::from("Up"), "Move selection up"),
		(String::from("Down"), "Move selection down"),
		(String::from("PgUp"), "Move selection up 5 lines"),
		(String::from("PgDn"), "Move selection down 5 lines"),
		(config.input_abort.to_string(), "Abort interactive rebase"),
		(
			config.input_force_abort.to_string(),
			"Immediately abort interactive rebase",
		),
		(config.input_rebase.to_string(), "Write interactive rebase file"),
		(
			config.input_force_rebase.to_string(),
			"Immediately write interactive rebase file",
		),
		(config.input_help.to_string(), "Show help"),
		(config.input_show_commit.to_string(), "Show commit information"),
		(
			config.input_move_selection_down.to_string(),
			"Move selected commit down",
		),
		(config.input_move_selection_up.to_string(), "Move selected commit up"),
		(config.input_action_break.to_string(), "Toggle break action"),
		(config.input_action_pick.to_string(), "Set selected commit to be picked"),
		(
			config.input_action_reword.to_string(),
			"Set selected commit to be reworded",
		),
		(config.input_action_edit.to_string(), "Set selected commit to be edited"),
		(
			config.input_action_squash.to_string(),
			"Set selected commit to be squashed",
		),
		(
			config.input_action_fixup.to_string(),
			"Set selected commit to be fixed-up",
		),
		(
			config.input_action_drop.to_string(),
			"Set selected commit to be dropped",
		),
		(config.input_edit.to_string(), "Edit an exec action's command"),
		(
			config.input_open_in_external_editor.to_string(),
			"Open the todo file in the default editor",
		),
	]
}

pub fn get_list_visual_mode_help_lines(config: &Config) -> [(String, &str); 13] {
	[
		(String::from("Up"), "Move selection up"),
		(String::from("Down"), "Move selection down"),
		(String::from("PgUp"), "Move selection up 5 lines"),
		(String::from("PgDn"), "Move selection down 5 lines"),
		(config.input_help.to_string(), "Show help"),
		(
			config.input_move_selection_down.to_string(),
			"Move selected commits down",
		),
		(config.input_move_selection_up.to_string(), "Move selected commits up"),
		(
			config.input_action_pick.to_string(),
			"Set selected commits to be picked",
		),
		(
			config.input_action_reword.to_string(),
			"Set selected commits to be reworded",
		),
		(
			config.input_action_edit.to_string(),
			"Set selected commits to be edited",
		),
		(
			config.input_action_squash.to_string(),
			"Set selected commits to be squashed",
		),
		(
			config.input_action_fixup.to_string(),
			"Set selected commits to be fixed-up",
		),
		(
			config.input_action_drop.to_string(),
			"Set selected commits to be dropped",
		),
	]
}

pub fn get_max_help_description_length(lines: &[(String, &str)]) -> usize {
	let mut max_length = 0;
	for (_, desc) in lines {
		let len = UnicodeSegmentation::graphemes(*desc, true).count();
		if len > max_length {
			max_length = len;
		}
	}
	max_length
}
