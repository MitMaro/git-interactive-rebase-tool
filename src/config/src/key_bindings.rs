use git::Config;

use crate::{errors::ConfigError, utils::get_input};

fn map_single_ascii_to_lower(s: &str) -> String {
	if s.is_ascii() && s.len() == 1 {
		s.to_lowercase()
	}
	else {
		String::from(s)
	}
}

/// Represents the key binding configuration options.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct KeyBindings {
	/// Key bindings for aborting.
	pub abort: Vec<String>,
	/// Key bindings for the break action.
	pub action_break: Vec<String>,
	/// Key bindings for the drop action.
	pub action_drop: Vec<String>,
	/// Key bindings for the edit action.
	pub action_edit: Vec<String>,
	/// Key bindings for the fixup action.
	pub action_fixup: Vec<String>,
	/// Key bindings for the pick action.
	pub action_pick: Vec<String>,
	/// Key bindings for the reword action.
	pub action_reword: Vec<String>,
	/// Key bindings for the squash action.
	pub action_squash: Vec<String>,
	/// Key bindings for negative confirmation.
	pub confirm_no: Vec<String>,
	/// Key bindings for positive confirmation.
	pub confirm_yes: Vec<String>,
	/// Key bindings for editing.
	pub edit: Vec<String>,
	/// Key bindings for forcing a abort.
	pub force_abort: Vec<String>,
	/// Key bindings for forcing a rebase.
	pub force_rebase: Vec<String>,
	/// Key bindings for showing help.
	pub help: Vec<String>,
	/// Key bindings for inserting a line.
	pub insert_line: Vec<String>,

	/// Key bindings for moving down.
	pub move_down: Vec<String>,
	/// Key bindings for moving to the end.
	pub move_end: Vec<String>,
	/// Key bindings for moving to the start.
	pub move_home: Vec<String>,
	/// Key bindings for moving to the left.
	pub move_left: Vec<String>,
	/// Key bindings for moving to the right.
	pub move_right: Vec<String>,
	/// Key bindings for moving up.
	pub move_up: Vec<String>,
	/// Key bindings for moving down a step.
	pub move_down_step: Vec<String>,
	/// Key bindings for moving up a step.
	pub move_up_step: Vec<String>,
	/// Key bindings for moving the selection down.
	pub move_selection_down: Vec<String>,
	/// Key bindings for moving the selection up.
	pub move_selection_up: Vec<String>,

	/// Key bindings for scrolling down.
	pub scroll_down: Vec<String>,
	/// Key bindings for scrolling to the end.
	pub scroll_end: Vec<String>,
	/// Key bindings for scrolling to the start.
	pub scroll_home: Vec<String>,
	/// Key bindings for scrolling to the left.
	pub scroll_left: Vec<String>,
	/// Key bindings for scrolling to the right.
	pub scroll_right: Vec<String>,
	/// Key bindings for scrolling up.
	pub scroll_up: Vec<String>,
	/// Key bindings for scrolling down a step.
	pub scroll_step_down: Vec<String>,
	/// Key bindings for scrolling up a step.
	pub scroll_step_up: Vec<String>,

	/// Key bindings for opening the external editor.
	pub open_in_external_editor: Vec<String>,
	/// Key bindings for rebasing.
	pub rebase: Vec<String>,
	/// Key bindings for redoing a change.
	pub redo: Vec<String>,
	/// Key bindings for removing a line.
	pub remove_line: Vec<String>,
	/// Key bindings for showing a commit.
	pub show_commit: Vec<String>,
	/// Key bindings for showing a diff.
	pub show_diff: Vec<String>,
	/// Key bindings for toggling visual mode.
	pub toggle_visual_mode: Vec<String>,
	/// Key bindings for undoing a change.
	pub undo: Vec<String>,
}

impl KeyBindings {
	/// Create a new configuration with default values.
	#[must_use]
	#[inline]
	#[allow(clippy::missing_panics_doc)]
	pub fn new() -> Self {
		Self::new_with_config(None).unwrap() // should never error with None config
	}

	pub(super) fn new_with_config(git_config: Option<&Config>) -> Result<Self, ConfigError> {
		let confirm_no = get_input(git_config, "interactive-rebase-tool.inputConfirmNo", "n")?
			.iter()
			.map(|s| map_single_ascii_to_lower(s))
			.collect();
		let confirm_yes = get_input(git_config, "interactive-rebase-tool.inputConfirmYes", "y")?
			.iter()
			.map(|s| map_single_ascii_to_lower(s))
			.collect();
		Ok(Self {
			abort: get_input(git_config, "interactive-rebase-tool.inputAbort", "q")?,
			action_break: get_input(git_config, "interactive-rebase-tool.inputActionBreak", "b")?,
			action_drop: get_input(git_config, "interactive-rebase-tool.inputActionDrop", "d")?,
			action_edit: get_input(git_config, "interactive-rebase-tool.inputActionEdit", "e")?,
			action_fixup: get_input(git_config, "interactive-rebase-tool.inputActionFixup", "f")?,
			action_pick: get_input(git_config, "interactive-rebase-tool.inputActionPick", "p")?,
			action_reword: get_input(git_config, "interactive-rebase-tool.inputActionReword", "r")?,
			action_squash: get_input(git_config, "interactive-rebase-tool.inputActionSquash", "s")?,
			confirm_no,
			confirm_yes,
			edit: get_input(git_config, "interactive-rebase-tool.inputEdit", "E")?,
			force_abort: get_input(git_config, "interactive-rebase-tool.inputForceAbort", "Q")?,
			force_rebase: get_input(git_config, "interactive-rebase-tool.inputForceRebase", "W")?,
			help: get_input(git_config, "interactive-rebase-tool.inputHelp", "?")?,
			insert_line: get_input(git_config, "interactive-rebase-tool.insertLine", "I")?,
			move_down: get_input(git_config, "interactive-rebase-tool.inputMoveDown", "Down")?,
			move_end: get_input(git_config, "interactive-rebase-tool.inputMoveEnd", "End")?,
			move_home: get_input(git_config, "interactive-rebase-tool.inputMoveHome", "Home")?,
			move_left: get_input(git_config, "interactive-rebase-tool.inputMoveLeft", "Left")?,
			move_right: get_input(git_config, "interactive-rebase-tool.inputMoveRight", "Right")?,
			move_down_step: get_input(git_config, "interactive-rebase-tool.inputMoveStepDown", "PageDown")?,
			move_up_step: get_input(git_config, "interactive-rebase-tool.inputMoveStepUp", "PageUp")?,
			move_up: get_input(git_config, "interactive-rebase-tool.inputMoveUp", "Up")?,
			move_selection_down: get_input(git_config, "interactive-rebase-tool.inputMoveSelectionDown", "j")?,
			move_selection_up: get_input(git_config, "interactive-rebase-tool.inputMoveSelectionUp", "k")?,
			scroll_down: get_input(git_config, "interactive-rebase-tool.inputScrollDown", "Down")?,
			scroll_end: get_input(git_config, "interactive-rebase-tool.inputScrollEnd", "End")?,
			scroll_home: get_input(git_config, "interactive-rebase-tool.inputScrollHome", "Home")?,
			scroll_left: get_input(git_config, "interactive-rebase-tool.inputScrollLeft", "Left")?,
			scroll_right: get_input(git_config, "interactive-rebase-tool.inputScrollRight", "Right")?,
			scroll_up: get_input(git_config, "interactive-rebase-tool.inputScrollUp", "Up")?,
			scroll_step_down: get_input(git_config, "interactive-rebase-tool.inputScrollStepDown", "PageDown")?,
			scroll_step_up: get_input(git_config, "interactive-rebase-tool.inputScrollStepUp", "PageUp")?,
			open_in_external_editor: get_input(git_config, "interactive-rebase-tool.inputOpenInExternalEditor", "!")?,
			rebase: get_input(git_config, "interactive-rebase-tool.inputRebase", "w")?,
			redo: get_input(git_config, "interactive-rebase-tool.inputRedo", "control+y")?,
			remove_line: get_input(git_config, "interactive-rebase-tool.removeLine", "delete")?,
			show_commit: get_input(git_config, "interactive-rebase-tool.inputShowCommit", "c")?,
			show_diff: get_input(git_config, "interactive-rebase-tool.inputShowDiff", "d")?,
			toggle_visual_mode: get_input(git_config, "interactive-rebase-tool.inputToggleVisualMode", "v")?,
			undo: get_input(git_config, "interactive-rebase-tool.inputUndo", "control+z")?,
		})
	}
}

impl TryFrom<&Config> for KeyBindings {
	type Error = ConfigError;

	#[inline]
	fn try_from(config: &Config) -> Result<Self, Self::Error> {
		Self::new_with_config(Some(config))
	}
}

#[cfg(test)]
mod tests {
	use claim::assert_ok;

	use super::*;
	use crate::testutils::with_git_config;

	macro_rules! config_test {
		($key:ident, $config_name:literal, $default:literal) => {
			let config = KeyBindings::new();
			let value = config.$key[0].as_str();
			assert_eq!(
				value,
				String::from($default),
				"Default value for key binding '{}' was expected to be '{}' but '{}' was found",
				stringify!($key),
				$default,
				value
			);

			let config_value = format!("{} = \"f255\"", $config_name);
			with_git_config(
				&["[interactive-rebase-tool]", config_value.as_str()],
				|git_config| {
					let config = KeyBindings::new_with_config(Some(&git_config)).unwrap();
					assert_eq!(
						config.$key[0].as_str(),
						"F255",
						"Value for key binding '{}' was expected to be changed but was not",
						stringify!($key)
					);
				},
			);
		};
	}

	#[test]
	fn new() {
		let _config = KeyBindings::new();
	}

	#[test]
	fn try_from_git_config() {
		with_git_config(&[], |git_config| {
			assert_ok!(KeyBindings::try_from(&git_config));
		});
	}

	#[test]
	fn try_from_git_config_error() {
		with_git_config(&["[interactive-rebase-tool]", "inputAbort = invalid"], |git_config| {
			let _ = KeyBindings::try_from(&git_config).unwrap_err();
		});
	}

	#[test]
	fn key_bindings() {
		config_test!(abort, "inputAbort", "q");
		config_test!(action_break, "inputActionBreak", "b");
		config_test!(action_drop, "inputActionDrop", "d");
		config_test!(action_edit, "inputActionEdit", "e");
		config_test!(action_fixup, "inputActionFixup", "f");
		config_test!(action_pick, "inputActionPick", "p");
		config_test!(action_reword, "inputActionReword", "r");
		config_test!(action_squash, "inputActionSquash", "s");
		config_test!(confirm_no, "inputConfirmNo", "n");
		config_test!(confirm_yes, "inputConfirmYes", "y");
		config_test!(edit, "inputEdit", "E");
		config_test!(force_abort, "inputForceAbort", "Q");
		config_test!(force_rebase, "inputForceRebase", "W");
		config_test!(help, "inputHelp", "?");
		config_test!(insert_line, "insertLine", "I");
		config_test!(move_down, "inputMoveDown", "Down");
		config_test!(move_end, "inputMoveEnd", "End");
		config_test!(move_home, "inputMoveHome", "Home");
		config_test!(move_left, "inputMoveLeft", "Left");
		config_test!(move_right, "inputMoveRight", "Right");
		config_test!(move_up, "inputMoveUp", "Up");
		config_test!(move_down_step, "inputMoveStepDown", "PageDown");
		config_test!(move_up_step, "inputMoveStepUp", "PageUp");
		config_test!(move_selection_down, "inputMoveSelectionDown", "j");
		config_test!(move_selection_up, "inputMoveSelectionUp", "k");
		config_test!(scroll_down, "inputScrollDown", "Down");
		config_test!(scroll_end, "inputScrollEnd", "End");
		config_test!(scroll_home, "inputScrollHome", "Home");
		config_test!(scroll_left, "inputScrollLeft", "Left");
		config_test!(scroll_right, "inputScrollRight", "Right");
		config_test!(scroll_up, "inputScrollUp", "Up");
		config_test!(scroll_step_down, "inputScrollStepDown", "PageDown");
		config_test!(scroll_step_up, "inputScrollStepUp", "PageUp");
		config_test!(open_in_external_editor, "inputOpenInExternalEditor", "!");
		config_test!(rebase, "inputRebase", "w");
		config_test!(redo, "inputRedo", "Controly");
		config_test!(remove_line, "removeLine", "Delete");
		config_test!(show_commit, "inputShowCommit", "c");
		config_test!(show_diff, "inputShowDiff", "d");
		config_test!(toggle_visual_mode, "inputToggleVisualMode", "v");
		config_test!(undo, "inputUndo", "Controlz");
	}
}
