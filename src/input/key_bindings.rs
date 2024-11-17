use crate::input::{Event, map_keybindings};

/// Represents a mapping between an input event and an action.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct KeyBindings {
	/// Key bindings for redoing a change.
	pub redo: Vec<Event>,
	/// Key bindings for undoing a change.
	pub undo: Vec<Event>,

	/// Key bindings for scrolling down.
	pub scroll_down: Vec<Event>,
	/// Key bindings for scrolling to the end.
	pub scroll_end: Vec<Event>,
	/// Key bindings for scrolling to the start.
	pub scroll_home: Vec<Event>,
	/// Key bindings for scrolling to the left.
	pub scroll_left: Vec<Event>,
	/// Key bindings for scrolling to the right.
	pub scroll_right: Vec<Event>,
	/// Key bindings for scrolling up.
	pub scroll_up: Vec<Event>,
	/// Key bindings for scrolling down a step.
	pub scroll_step_down: Vec<Event>,
	/// Key bindings for scrolling up a step.
	pub scroll_step_up: Vec<Event>,

	/// Key bindings for help.
	pub help: Vec<Event>,

	/// Key bindings for starting search.
	pub search_start: Vec<Event>,
	/// Key bindings for next search match.
	pub search_next: Vec<Event>,
	/// Key bindings for previous search match.
	pub search_previous: Vec<Event>,

	/// Key bindings for aborting.
	pub abort: Vec<Event>,
	/// Key bindings for the break action.
	pub action_break: Vec<Event>,
	/// Key bindings for the drop action.
	pub action_drop: Vec<Event>,
	/// Key bindings for the edit action.
	pub action_edit: Vec<Event>,
	/// Key bindings for the fixup action.
	pub action_fixup: Vec<Event>,
	/// Key bindings for the pick action.
	pub action_pick: Vec<Event>,
	/// Key bindings for the reword action.
	pub action_reword: Vec<Event>,
	/// Key bindings for the squash action.
	pub action_squash: Vec<Event>,
	/// Key bindings for positive confirmation.
	pub confirm_yes: Vec<Event>,
	/// Key bindings for editing.
	pub edit: Vec<Event>,
	/// Key bindings for forcing an abort.
	pub force_abort: Vec<Event>,
	/// Key bindings for forcing a rebase.
	pub force_rebase: Vec<Event>,
	/// Key bindings for inserting a line.
	pub insert_line: Vec<Event>,
	/// Key bindings for moving down.
	pub move_down: Vec<Event>,
	/// Key bindings for moving down a step.
	pub move_down_step: Vec<Event>,
	/// Key bindings for moving to the end.
	pub move_end: Vec<Event>,
	/// Key bindings for moving to the start.
	pub move_home: Vec<Event>,
	/// Key bindings for moving to the left.
	pub move_left: Vec<Event>,
	/// Key bindings for moving to the right.
	pub move_right: Vec<Event>,
	/// Key bindings for moving the selection down.
	pub move_selection_down: Vec<Event>,
	/// Key bindings for moving the selection up.
	pub move_selection_up: Vec<Event>,
	/// Key bindings for moving up.
	pub move_up: Vec<Event>,
	/// Key bindings for moving up a step.
	pub move_up_step: Vec<Event>,
	/// Key bindings for opening the external editor.
	pub open_in_external_editor: Vec<Event>,
	/// Key bindings for rebasing.
	pub rebase: Vec<Event>,
	/// Key bindings for removing a line.
	pub remove_line: Vec<Event>,
	/// Key bindings for showing a commit.
	pub show_commit: Vec<Event>,
	/// Key bindings for showing a diff.
	pub show_diff: Vec<Event>,
	/// Key bindings for toggling visual mode.
	pub toggle_visual_mode: Vec<Event>,
	/// Key bindings for the fixup specific action to toggle the c option.
	pub fixup_keep_message: Vec<Event>,
	/// Key biding for the fixup specific action to toggle the C option.
	pub fixup_keep_message_with_editor: Vec<Event>,
}

impl KeyBindings {
	/// Create a new instance from the configuration keybindings.
	#[must_use]
	pub(crate) fn new(key_bindings: &crate::config::KeyBindings) -> Self {
		Self {
			redo: map_keybindings(&key_bindings.redo),
			undo: map_keybindings(&key_bindings.undo),
			scroll_down: map_keybindings(&key_bindings.scroll_down),
			scroll_end: map_keybindings(&key_bindings.scroll_end),
			scroll_home: map_keybindings(&key_bindings.scroll_home),
			scroll_left: map_keybindings(&key_bindings.scroll_left),
			scroll_right: map_keybindings(&key_bindings.scroll_right),
			scroll_up: map_keybindings(&key_bindings.scroll_up),
			scroll_step_down: map_keybindings(&key_bindings.scroll_step_down),
			scroll_step_up: map_keybindings(&key_bindings.scroll_step_up),
			help: map_keybindings(&key_bindings.help),
			search_start: map_keybindings(&key_bindings.search_start),
			search_next: map_keybindings(&key_bindings.search_next),
			search_previous: map_keybindings(&key_bindings.search_previous),
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
			fixup_keep_message: map_keybindings(&key_bindings.fixup_keep_message),
			fixup_keep_message_with_editor: map_keybindings(&key_bindings.fixup_keep_message_with_editor),
		}
	}
}

#[cfg(test)]
mod tests {
	use crossterm::event::{KeyCode, KeyModifiers};
	use rstest::rstest;

	use super::*;
	use crate::input::KeyEvent;

	#[test]
	fn map_keybindings_with_modifiers() {
		assert_eq!(map_keybindings(&[String::from("ControlAltShiftUp")]), vec![Event::Key(
			KeyEvent::new(
				KeyCode::Up,
				KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT
			)
		)]);
	}

	#[rstest]
	#[case::backspace("Backspace", KeyCode::Backspace)]
	#[case::back_tab("BackTab", KeyCode::BackTab)]
	#[case::delete("Delete", KeyCode::Delete)]
	#[case::down("Down", KeyCode::Down)]
	#[case::end("End", KeyCode::End)]
	#[case::enter("Enter", KeyCode::Enter)]
	#[case::esc("Esc", KeyCode::Esc)]
	#[case::home("Home", KeyCode::Home)]
	#[case::insert("Insert", KeyCode::Insert)]
	#[case::left("Left", KeyCode::Left)]
	#[case::page_down("PageDown", KeyCode::PageDown)]
	#[case::page_up("PageUp", KeyCode::PageUp)]
	#[case::right("Right", KeyCode::Right)]
	#[case::tab("Tab", KeyCode::Tab)]
	#[case::up("Up", KeyCode::Up)]
	#[case::function_in_range("F10", KeyCode::F(10))]
	#[case::function_out_of_range("F10000", KeyCode::F(1))]
	#[case::char("a", KeyCode::Char('a'))]
	fn map_keybindings_key_code(#[case] binding: &str, #[case] key_code: KeyCode) {
		assert_eq!(map_keybindings(&[String::from(binding)]), vec![Event::from(key_code)]);
	}
}
