use crate::action::Action;
use crate::display::DisplayColor;
use crate::Config;

fn get_input_short_name(input: &str) -> String {
	match input {
		"Left" => String::from("lf"),
		"Right" => String::from("rt"),
		"Down" => String::from("dn"),
		"Up" => String::from("up"),
		"PageUp" => String::from("pup"),
		"PageDown" => String::from("pdn"),
		"Resize" => String::from("rz"),
		"Other" => String::from("ot"),
		_ => String::from(input),
	}
}

pub fn get_action_color(action: Action) -> DisplayColor {
	match action {
		Action::Break => DisplayColor::ActionBreak,
		Action::Drop => DisplayColor::ActionDrop,
		Action::Edit => DisplayColor::ActionEdit,
		Action::Exec => DisplayColor::ActionExec,
		Action::Fixup => DisplayColor::ActionFixup,
		Action::Noop => DisplayColor::Normal,
		Action::Pick => DisplayColor::ActionPick,
		Action::Reword => DisplayColor::ActionReword,
		Action::Squash => DisplayColor::ActionSquash,
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
