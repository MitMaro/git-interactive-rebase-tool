use crate::action::Action;
use crate::window::WindowColor;
use crate::Config;

pub fn get_action_color(action: Action) -> WindowColor {
	match action {
		Action::Break => WindowColor::ActionBreak,
		Action::Drop => WindowColor::ActionDrop,
		Action::Edit => WindowColor::ActionEdit,
		Action::Exec => WindowColor::ActionExec,
		Action::Fixup => WindowColor::ActionFixup,
		Action::Pick => WindowColor::ActionPick,
		Action::Reword => WindowColor::ActionReword,
		Action::Squash => WindowColor::ActionSquash,
	}
}

pub fn get_normal_footer_full(config: &Config) -> String {
	format!(
		" up, down, {}/{}, {}/{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
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
		" up, down, {}, {}, {}, {}, {}, {}, {}, {}, {}",
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
		"up,dn,{}/{},{}/{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
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

pub fn get_visual_footer_compact(config: &Config) -> String {
	format!(
		"up,dn,{},{},{},{},{},{},{},{},{}",
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
