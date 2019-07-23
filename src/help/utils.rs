use crate::Config;
use unicode_segmentation::UnicodeSegmentation;

fn get_input_short_name(input: &str) -> String {
	if input == "PageUp" {
		String::from("PgUp")
	}
	else if input == "PageDown" {
		String::from("PgDn")
	}
	else if input == "Resize" {
		String::from("Rsze")
	}
	else if input == "Other" {
		String::from("Oth")
	}
	else {
		String::from(input)
	}
}

pub fn get_list_normal_mode_help_lines(config: &Config) -> [(String, &str); 21] {
	[
		(get_input_short_name(config.input_move_up.as_str()), "Move selection up"),
		(
			get_input_short_name(config.input_move_down.as_str()),
			"Move selection down",
		),
		(
			get_input_short_name(config.input_move_up_step.as_str()),
			"Move selection up 5 lines",
		),
		(
			get_input_short_name(config.input_move_down_step.as_str()),
			"Move selection down 5 lines",
		),
		(
			get_input_short_name(config.input_abort.as_str()),
			"Abort interactive rebase",
		),
		(
			get_input_short_name(config.input_force_abort.as_str()),
			"Immediately abort interactive rebase",
		),
		(
			get_input_short_name(config.input_rebase.as_str()),
			"Write interactive rebase file",
		),
		(
			get_input_short_name(config.input_force_rebase.as_str()),
			"Immediately write interactive rebase file",
		),
		(get_input_short_name(config.input_help.as_str()), "Show help"),
		(
			get_input_short_name(config.input_show_commit.as_str()),
			"Show commit information",
		),
		(
			get_input_short_name(config.input_move_selection_down.as_str()),
			"Move selected commit down",
		),
		(
			get_input_short_name(config.input_move_selection_up.as_str()),
			"Move selected commit up",
		),
		(
			get_input_short_name(config.input_action_break.as_str()),
			"Toggle break action",
		),
		(
			get_input_short_name(config.input_action_pick.as_str()),
			"Set selected commit to be picked",
		),
		(
			get_input_short_name(config.input_action_reword.as_str()),
			"Set selected commit to be reworded",
		),
		(
			get_input_short_name(config.input_action_edit.as_str()),
			"Set selected commit to be edited",
		),
		(
			get_input_short_name(config.input_action_squash.as_str()),
			"Set selected commit to be squashed",
		),
		(
			get_input_short_name(config.input_action_fixup.as_str()),
			"Set selected commit to be fixed-up",
		),
		(
			get_input_short_name(config.input_action_drop.as_str()),
			"Set selected commit to be dropped",
		),
		(
			get_input_short_name(config.input_edit.as_str()),
			"Edit an exec action's command",
		),
		(
			get_input_short_name(config.input_open_in_external_editor.as_str()),
			"Open the todo file in the default editor",
		),
	]
}

pub fn get_list_visual_mode_help_lines(config: &Config) -> [(String, &str); 13] {
	[
		(get_input_short_name(config.input_move_up.as_str()), "Move selection up"),
		(
			get_input_short_name(config.input_move_down.as_str()),
			"Move selection down",
		),
		(
			get_input_short_name(config.input_move_up_step.as_str()),
			"Move selection up 5 lines",
		),
		(
			get_input_short_name(config.input_move_down_step.as_str()),
			"Move selection down 5 lines",
		),
		(get_input_short_name(config.input_help.as_str()), "Show help"),
		(
			get_input_short_name(config.input_move_selection_down.as_str()),
			"Move selected commits down",
		),
		(
			get_input_short_name(config.input_move_selection_up.as_str()),
			"Move selected commits up",
		),
		(
			get_input_short_name(config.input_action_pick.as_str()),
			"Set selected commits to be picked",
		),
		(
			get_input_short_name(config.input_action_reword.as_str()),
			"Set selected commits to be reworded",
		),
		(
			get_input_short_name(config.input_action_edit.as_str()),
			"Set selected commits to be edited",
		),
		(
			get_input_short_name(config.input_action_squash.as_str()),
			"Set selected commits to be squashed",
		),
		(
			get_input_short_name(config.input_action_fixup.as_str()),
			"Set selected commits to be fixed-up",
		),
		(
			get_input_short_name(config.input_action_drop.as_str()),
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
