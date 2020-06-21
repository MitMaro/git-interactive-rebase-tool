use crate::config::key_bindings::KeyBindings;
use crate::display::display_color::DisplayColor;
use crate::input::utils::get_input_short_name;
use crate::list::action::Action;

pub(super) fn get_action_color(action: Action) -> DisplayColor {
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

pub(super) fn get_normal_footer_full(key_bindings: &KeyBindings) -> String {
	format!(
		" {}, {}, {}/{}, {}/{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
		key_bindings.move_up,
		key_bindings.move_down,
		key_bindings.abort,
		key_bindings.force_abort,
		key_bindings.rebase,
		key_bindings.force_rebase,
		key_bindings.show_commit,
		key_bindings.move_selection_down,
		key_bindings.move_selection_up,
		key_bindings.action_break,
		key_bindings.action_pick,
		key_bindings.action_reword,
		key_bindings.action_edit,
		key_bindings.action_squash,
		key_bindings.action_fixup,
		key_bindings.action_drop,
		key_bindings.edit,
		key_bindings.open_in_external_editor,
		key_bindings.help,
	)
}

pub(super) fn get_visual_footer_full(key_bindings: &KeyBindings) -> String {
	format!(
		" {}, {}, {}/{}, {}/{}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
		key_bindings.move_up,
		key_bindings.move_down,
		key_bindings.abort,
		key_bindings.force_abort,
		key_bindings.rebase,
		key_bindings.force_rebase,
		key_bindings.move_selection_down,
		key_bindings.move_selection_up,
		key_bindings.action_pick,
		key_bindings.action_reword,
		key_bindings.action_edit,
		key_bindings.action_squash,
		key_bindings.action_fixup,
		key_bindings.action_drop,
		key_bindings.help,
	)
}

pub(super) fn get_normal_footer_compact(key_bindings: &KeyBindings) -> String {
	format!(
		"{},{},{}/{},{}/{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
		get_input_short_name(key_bindings.move_up.as_str()),
		get_input_short_name(key_bindings.move_down.as_str()),
		get_input_short_name(key_bindings.abort.as_str()),
		get_input_short_name(key_bindings.force_abort.as_str()),
		get_input_short_name(key_bindings.rebase.as_str()),
		get_input_short_name(key_bindings.force_rebase.as_str()),
		get_input_short_name(key_bindings.show_commit.as_str()),
		get_input_short_name(key_bindings.move_selection_down.as_str()),
		get_input_short_name(key_bindings.move_selection_up.as_str()),
		get_input_short_name(key_bindings.action_break.as_str()),
		get_input_short_name(key_bindings.action_pick.as_str()),
		get_input_short_name(key_bindings.action_reword.as_str()),
		get_input_short_name(key_bindings.action_edit.as_str()),
		get_input_short_name(key_bindings.action_squash.as_str()),
		get_input_short_name(key_bindings.action_fixup.as_str()),
		get_input_short_name(key_bindings.action_drop.as_str()),
		get_input_short_name(key_bindings.edit.as_str()),
		get_input_short_name(key_bindings.open_in_external_editor.as_str()),
		get_input_short_name(key_bindings.help.as_str()),
	)
}

pub(super) fn get_visual_footer_compact(key_bindings: &KeyBindings) -> String {
	format!(
		"{},{},{}/{},{}/{},{},{},{},{},{},{},{},{},{}",
		get_input_short_name(key_bindings.move_up.as_str()),
		get_input_short_name(key_bindings.move_down.as_str()),
		get_input_short_name(key_bindings.abort.as_str()),
		get_input_short_name(key_bindings.force_abort.as_str()),
		get_input_short_name(key_bindings.rebase.as_str()),
		get_input_short_name(key_bindings.force_rebase.as_str()),
		get_input_short_name(key_bindings.move_selection_down.as_str()),
		get_input_short_name(key_bindings.move_selection_up.as_str()),
		get_input_short_name(key_bindings.action_pick.as_str()),
		get_input_short_name(key_bindings.action_reword.as_str()),
		get_input_short_name(key_bindings.action_edit.as_str()),
		get_input_short_name(key_bindings.action_squash.as_str()),
		get_input_short_name(key_bindings.action_fixup.as_str()),
		get_input_short_name(key_bindings.action_drop.as_str()),
		get_input_short_name(key_bindings.help.as_str()),
	)
}
