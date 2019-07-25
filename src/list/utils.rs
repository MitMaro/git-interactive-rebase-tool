use crate::action::Action;
use crate::window::WindowColor;
use crate::Config;

fn get_input_short_name(input: &str) -> String {
	if input == "Left" {
		String::from("lf")
	}
	else if input == "Right" {
		String::from("rt")
	}
	else if input == "Down" {
		String::from("dn")
	}
	else if input == "Up" {
		String::from("up")
	}
	else if input == "PageUp" {
		String::from("pup")
	}
	else if input == "PageDown" {
		String::from("pdn")
	}
	else if input == "Resize" {
		String::from("rz")
	}
	else if input == "Other" {
		String::from("ot")
	}
	else {
		String::from(input)
	}
}

pub fn get_action_color(action: Action) -> WindowColor {
	match action {
		Action::Break => WindowColor::ActionBreak,
		Action::Drop => WindowColor::ActionDrop,
		Action::Edit => WindowColor::ActionEdit,
		Action::Exec => WindowColor::ActionExec,
		Action::Fixup => WindowColor::ActionFixup,
		Action::Noop => WindowColor::Foreground,
		Action::Pick => WindowColor::ActionPick,
		Action::Reword => WindowColor::ActionReword,
		Action::Squash => WindowColor::ActionSquash,
	}
}

pub fn get_normal_footer_full(config: &Config) -> String {
	format!(
		" {}, {}, {}/{}, {}/{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
		config.input_move_up,
		config.input_move_down,
		config.input_abort,
		config.input_force_abort,
		config.input_rebase,
		config.input_force_rebase,
		config.input_show_commit,
		config.input_move_selection_down,
		config.input_move_selection_up,
		config.input_action_break,
		config.input_action_pick,
		config.input_action_reword,
		config.input_action_edit,
		config.input_action_squash,
		config.input_action_fixup,
		config.input_action_drop,
		config.input_edit,
		config.input_open_in_external_editor,
		config.input_help,
	)
}

pub fn get_visual_footer_full(config: &Config) -> String {
	format!(
		" {}, {}, {}/{}, {}/{}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
		config.input_move_up,
		config.input_move_down,
		config.input_abort,
		config.input_force_abort,
		config.input_rebase,
		config.input_force_rebase,
		config.input_move_selection_down,
		config.input_move_selection_up,
		config.input_action_pick,
		config.input_action_reword,
		config.input_action_edit,
		config.input_action_squash,
		config.input_action_fixup,
		config.input_action_drop,
		config.input_help,
	)
}

pub fn get_normal_footer_compact(config: &Config) -> String {
	format!(
		"{},{},{}/{},{}/{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
		get_input_short_name(config.input_move_up.as_str()),
		get_input_short_name(config.input_move_down.as_str()),
		get_input_short_name(config.input_abort.as_str()),
		get_input_short_name(config.input_force_abort.as_str()),
		get_input_short_name(config.input_rebase.as_str()),
		get_input_short_name(config.input_force_rebase.as_str()),
		get_input_short_name(config.input_show_commit.as_str()),
		get_input_short_name(config.input_move_selection_down.as_str()),
		get_input_short_name(config.input_move_selection_up.as_str()),
		get_input_short_name(config.input_action_break.as_str()),
		get_input_short_name(config.input_action_pick.as_str()),
		get_input_short_name(config.input_action_reword.as_str()),
		get_input_short_name(config.input_action_edit.as_str()),
		get_input_short_name(config.input_action_squash.as_str()),
		get_input_short_name(config.input_action_fixup.as_str()),
		get_input_short_name(config.input_action_drop.as_str()),
		get_input_short_name(config.input_edit.as_str()),
		get_input_short_name(config.input_open_in_external_editor.as_str()),
		get_input_short_name(config.input_help.as_str()),
	)
}

pub fn get_visual_footer_compact(config: &Config) -> String {
	format!(
		"{},{},{}/{},{}/{},{},{},{},{},{},{},{},{},{}",
		get_input_short_name(config.input_move_up.as_str()),
		get_input_short_name(config.input_move_down.as_str()),
		get_input_short_name(config.input_abort.as_str()),
		get_input_short_name(config.input_force_abort.as_str()),
		get_input_short_name(config.input_rebase.as_str()),
		get_input_short_name(config.input_force_rebase.as_str()),
		get_input_short_name(config.input_move_selection_down.as_str()),
		get_input_short_name(config.input_move_selection_up.as_str()),
		get_input_short_name(config.input_action_pick.as_str()),
		get_input_short_name(config.input_action_reword.as_str()),
		get_input_short_name(config.input_action_edit.as_str()),
		get_input_short_name(config.input_action_squash.as_str()),
		get_input_short_name(config.input_action_fixup.as_str()),
		get_input_short_name(config.input_action_drop.as_str()),
		get_input_short_name(config.input_help.as_str()),
	)
}
