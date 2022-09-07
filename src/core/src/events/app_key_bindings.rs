use config::KeyBindings;
use input::CustomKeybinding;

use crate::events::{Event, MetaEvent};

pub(crate) fn map_keybindings(bindings: &[String]) -> Vec<Event> {
	input::map_keybindings::<MetaEvent>(bindings)
}

/// Represents a mapping between an input event and an action.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct AppKeyBindings {
	/// Key bindings for aborting.
	pub(crate) abort: Vec<Event>,
	/// Key bindings for the break action.
	pub(crate) action_break: Vec<Event>,
	/// Key bindings for the drop action.
	pub(crate) action_drop: Vec<Event>,
	/// Key bindings for the edit action.
	pub(crate) action_edit: Vec<Event>,
	/// Key bindings for the fixup action.
	pub(crate) action_fixup: Vec<Event>,
	/// Key bindings for the pick action.
	pub(crate) action_pick: Vec<Event>,
	/// Key bindings for the reword action.
	pub(crate) action_reword: Vec<Event>,
	/// Key bindings for the squash action.
	pub(crate) action_squash: Vec<Event>,
	/// Key bindings for positive confirmation.
	pub(crate) confirm_yes: Vec<Event>,
	/// Key bindings for editing.
	pub(crate) edit: Vec<Event>,
	/// Key bindings for forcing an abort.
	pub(crate) force_abort: Vec<Event>,
	/// Key bindings for forcing a rebase.
	pub(crate) force_rebase: Vec<Event>,
	/// Key bindings for inserting a line.
	pub(crate) insert_line: Vec<Event>,
	/// Key bindings for moving down.
	pub(crate) move_down: Vec<Event>,
	/// Key bindings for moving down a step.
	pub(crate) move_down_step: Vec<Event>,
	/// Key bindings for moving to the end.
	pub(crate) move_end: Vec<Event>,
	/// Key bindings for moving to the start.
	pub(crate) move_home: Vec<Event>,
	/// Key bindings for moving to the left.
	pub(crate) move_left: Vec<Event>,
	/// Key bindings for moving to the right.
	pub(crate) move_right: Vec<Event>,
	/// Key bindings for moving the selection down.
	pub(crate) move_selection_down: Vec<Event>,
	/// Key bindings for moving the selection up.
	pub(crate) move_selection_up: Vec<Event>,
	/// Key bindings for moving up.
	pub(crate) move_up: Vec<Event>,
	/// Key bindings for moving up a step.
	pub(crate) move_up_step: Vec<Event>,
	/// Key bindings for opening the external editor.
	pub(crate) open_in_external_editor: Vec<Event>,
	/// Key bindings for rebasing.
	pub(crate) rebase: Vec<Event>,
	/// Key bindings for removing a line.
	pub(crate) remove_line: Vec<Event>,
	/// Key bindings for showing a commit.
	pub(crate) show_commit: Vec<Event>,
	/// Key bindings for showing a diff.
	pub(crate) show_diff: Vec<Event>,
	/// Key bindings for toggling visual mode.
	pub(crate) toggle_visual_mode: Vec<Event>,
}

impl CustomKeybinding for AppKeyBindings {
	/// Create a new instance from the configuration keybindings.
	fn new(key_bindings: &KeyBindings) -> Self {
		Self {
			abort: map_keybindings(&key_bindings.abort),
			action_break: map_keybindings(&key_bindings.action_break),
			action_drop: map_keybindings(&key_bindings.action_drop),
			action_edit: map_keybindings(&key_bindings.action_edit),
			action_fixup: map_keybindings(&key_bindings.action_fixup),
			action_pick: map_keybindings(&key_bindings.action_pick),
			action_reword: map_keybindings(&key_bindings.action_reword),
			action_squash: map_keybindings(&key_bindings.action_squash),
			edit: map_keybindings(&key_bindings.edit),
			force_abort: map_keybindings(&key_bindings.force_abort),
			force_rebase: map_keybindings(&key_bindings.force_rebase),
			insert_line: map_keybindings(&key_bindings.insert_line),
			move_down: map_keybindings(&key_bindings.move_down),
			move_down_step: map_keybindings(&key_bindings.move_down_step),
			move_end: map_keybindings(&key_bindings.move_end),
			move_home: map_keybindings(&key_bindings.move_home),
			move_left: map_keybindings(&key_bindings.move_left),
			move_right: map_keybindings(&key_bindings.move_right),
			move_selection_down: map_keybindings(&key_bindings.move_selection_down),
			move_selection_up: map_keybindings(&key_bindings.move_selection_up),
			move_up: map_keybindings(&key_bindings.move_up),
			move_up_step: map_keybindings(&key_bindings.move_up_step),
			open_in_external_editor: map_keybindings(&key_bindings.open_in_external_editor),
			rebase: map_keybindings(&key_bindings.rebase),
			remove_line: map_keybindings(&key_bindings.remove_line),
			show_commit: map_keybindings(&key_bindings.show_commit),
			show_diff: map_keybindings(&key_bindings.show_diff),
			toggle_visual_mode: map_keybindings(&key_bindings.toggle_visual_mode),
			confirm_yes: map_keybindings(&key_bindings.confirm_yes),
		}
	}
}
