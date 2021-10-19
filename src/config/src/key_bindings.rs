use std::convert::TryFrom;

use anyhow::{Error, Result};
use git::Config;

use super::utils::get_input;
use crate::utils::map_single_ascii_to_lower;

/// Represents the key binding configuration options.
#[derive(Clone, Debug)]
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
	/// Key bindings for moving down a step.
	pub move_down_step: Vec<String>,
	/// Key bindings for moving to the end.
	pub move_end: Vec<String>,
	/// Key bindings for moving to the start.
	pub move_home: Vec<String>,
	/// Key bindings for moving to the left.
	pub move_left: Vec<String>,
	/// Key bindings for moving to the right.
	pub move_right: Vec<String>,
	/// Key bindings for moving the selection down.
	pub move_selection_down: Vec<String>,
	/// Key bindings for moving the selection up.
	pub move_selection_up: Vec<String>,
	/// Key bindings for moving up.
	pub move_up: Vec<String>,
	/// Key bindings for moving up a step.
	pub move_up_step: Vec<String>,
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
	pub fn new() -> Self {
		Self::new_with_config(None).expect("Panic without git config instance") // should never error with None config
	}

	pub(super) fn new_with_config(git_config: Option<&Config>) -> Result<Self> {
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
			move_down_step: get_input(git_config, "interactive-rebase-tool.inputMoveStepDown", "PageDown")?,
			move_end: get_input(git_config, "interactive-rebase-tool.inputMoveEnd", "End")?,
			move_home: get_input(git_config, "interactive-rebase-tool.inputMoveHome", "Home")?,
			move_left: get_input(git_config, "interactive-rebase-tool.inputMoveLeft", "Left")?,
			move_right: get_input(git_config, "interactive-rebase-tool.inputMoveRight", "Right")?,
			move_selection_down: get_input(git_config, "interactive-rebase-tool.inputMoveSelectionDown", "j")?,
			move_selection_up: get_input(git_config, "interactive-rebase-tool.inputMoveSelectionUp", "k")?,
			move_up_step: get_input(git_config, "interactive-rebase-tool.inputMoveStepUp", "PageUp")?,
			move_up: get_input(git_config, "interactive-rebase-tool.inputMoveUp", "Up")?,
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

impl Default for KeyBindings {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl TryFrom<&Config> for KeyBindings {
	type Error = Error;

	#[inline]
	fn try_from(config: &Config) -> core::result::Result<Self, Error> {
		Self::new_with_config(Some(config))
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::testutils::{assert_error, invalid_utf, with_git_config};

	pub(crate) fn with_keybindings<F>(lines: &[&str], callback: F)
	where F: FnOnce(KeyBindings) {
		with_git_config(lines, |config| {
			let key_bindings = KeyBindings::new_with_config(Some(&config)).unwrap();
			callback(key_bindings);
		});
	}

	#[test]
	fn new() {
		let _config = KeyBindings::new();
	}

	#[test]
	fn default() {
		let _config = KeyBindings::default();
	}

	#[test]
	fn try_from_git_config() {
		with_git_config(&[], |git_config| {
			assert!(KeyBindings::try_from(&git_config).is_ok());
		});
	}

	#[test]
	fn try_from_git_config_error() {
		with_git_config(&["[interactive-rebase-tool]", "inputAbort = invalid"], |git_config| {
			assert!(KeyBindings::try_from(&git_config).is_err());
		});
	}

	#[rstest]
	#[case::backspace("backspace", "Backspace")]
	#[case::backtab("backtab", "BackTab")]
	#[case::delete("delete", "Delete")]
	#[case::down("down", "Down")]
	#[case::end("end", "End")]
	#[case::end("enter", "Enter")]
	#[case::end("esc", "Esc")]
	#[case::home("home", "Home")]
	#[case::insert("insert", "Insert")]
	#[case::left("left", "Left")]
	#[case::pagedown("pagedown", "PageDown")]
	#[case::pageup("pageup", "PageUp")]
	#[case::right("right", "Right")]
	#[case::tab("tab", "Tab")]
	#[case::up("up", "Up")]
	#[case::f1("f1", "F1")]
	#[case::f255("f255", "F255")]
	#[case::modifier_character_lowercase("Control+a", "Controla")]
	#[case::modifier_character_uppercase("Control+A", "ControlA")]
	#[case::modifier_character_number("Control+1", "Control1")]
	#[case::modifier_character_special("Control++", "Control+")]
	#[case::modifier_character("Control+a", "Controla")]
	#[case::modifier_special("Control+End", "ControlEnd")]
	#[case::modifier_function("Control+F32", "ControlF32")]
	#[case::modifier_control_alt_shift_out_of_order_1("Alt+Shift+Control+End", "ShiftControlAltEnd")]
	#[case::modifier_control_alt_shift_out_of_order_2("Shift+Control+Alt+End", "ShiftControlAltEnd")]
	#[case::modifier_only_shift("Shift+End", "ShiftEnd")]
	#[case::modifier_only_control("Control+End", "ControlEnd")]
	#[case::modifier_only_control("a b c d", "a,b,c,d")]
	#[case::modifier_only_control("Control+End Control+A", "ControlEnd,ControlA")]
	fn value_parsing(#[case] binding: &str, #[case] expected: &str) {
		with_keybindings(
			&[
				"[interactive-rebase-tool]",
				format!("inputAbort = \"{}\"", binding).as_str(),
			],
			|key_bindings| {
				assert_eq!(
					key_bindings.abort,
					expected.split(',').map(String::from).collect::<Vec<String>>()
				);
			},
		);
	}

	#[rstest]
	#[case::invalid_utf(
		invalid_utf(),
		"\"interactive-rebase-tool.inputAbort\" is not valid: configuration value is not valid utf8"
	)]
	#[case::multiple_characters(
		"abcd",
		"interactive-rebase-tool.inputAbort must contain only one character per binding"
	)]
	#[case::function_key_index(
		"F256",
		"interactive-rebase-tool.inputAbort must contain only one character per binding"
	)]
	#[case::multiple_bindings_one_invalid(
		"f foo",
		"interactive-rebase-tool.inputAbort must contain only one character per binding"
	)]
	fn value_parsing_invalid(#[case] binding: &str, #[case] expected_error: &str) {
		with_git_config(
			&[
				"[interactive-rebase-tool]",
				format!("inputAbort = {}", binding).as_str(),
			],
			|git_config| {
				assert_error(KeyBindings::new_with_config(Some(&git_config)), expected_error);
			},
		);
	}

	#[rstest]
	#[case::abort("inputAbort", "q", |bindings: KeyBindings| bindings.abort)]
	#[case::action_break("inputActionBreak", "b", |bindings: KeyBindings| bindings.action_break)]
	#[case::action_drop("inputActionDrop", "d", |bindings: KeyBindings| bindings.action_drop)]
	#[case::action_edit("inputActionEdit", "e", |bindings: KeyBindings| bindings.action_edit)]
	#[case::action_fixup("inputActionFixup", "f", |bindings: KeyBindings| bindings.action_fixup)]
	#[case::action_pick("inputActionPick", "p", |bindings: KeyBindings| bindings.action_pick)]
	#[case::action_reword("inputActionReword", "r", |bindings: KeyBindings| bindings.action_reword)]
	#[case::action_squash("inputActionSquash", "s", |bindings: KeyBindings| bindings.action_squash)]
	#[case::confirm_no("inputConfirmNo", "n", |bindings: KeyBindings| bindings.confirm_no)]
	#[case::confirm_yes("inputConfirmYes", "y", |bindings: KeyBindings| bindings.confirm_yes)]
	#[case::edit("inputEdit", "E", |bindings: KeyBindings| bindings.edit)]
	#[case::force_abort("inputForceAbort", "Q", |bindings: KeyBindings| bindings.force_abort)]
	#[case::force_rebase("inputForceRebase", "W", |bindings: KeyBindings| bindings.force_rebase)]
	#[case::help("inputHelp", "?", |bindings: KeyBindings| bindings.help)]
	#[case::insert_line("insertLine", "I", |bindings: KeyBindings| bindings.insert_line)]
	#[case::move_down("inputMoveDown", "Down", |bindings: KeyBindings| bindings.move_down)]
	#[case::move_down_step("inputMoveStepDown", "PageDown", |bindings: KeyBindings| bindings.move_down_step)]
	#[case::move_end("inputMoveEnd", "End", |bindings: KeyBindings| bindings.move_end)]
	#[case::move_home("inputMoveHome", "Home", |bindings: KeyBindings| bindings.move_home)]
	#[case::move_left("inputMoveLeft", "Left", |bindings: KeyBindings| bindings.move_left)]
	#[case::move_right("inputMoveRight", "Right", |bindings: KeyBindings| bindings.move_right)]
	#[case::move_selection_down("inputMoveSelectionDown", "j", |bindings: KeyBindings| bindings.move_selection_down)]
	#[case::move_selection_up("inputMoveSelectionUp", "k", |bindings: KeyBindings| bindings.move_selection_up)]
	#[case::move_up("inputMoveUp", "Up", |bindings: KeyBindings| bindings.move_up)]
	#[case::move_up_step("inputMoveStepUp", "PageUp", |bindings: KeyBindings| bindings.move_up_step)]
	#[case::open_in_external_editor(
		"inputOpenInExternalEditor",
		"!",
		|bindings: KeyBindings| bindings.open_in_external_editor)
	]
	#[case::rebase("inputRebase", "w", |bindings: KeyBindings| bindings.rebase)]
	#[case::redo("inputRedo", "Controly", |bindings: KeyBindings| bindings.redo)]
	#[case::remove_line("removeLine", "Delete", |bindings: KeyBindings| bindings.remove_line)]
	#[case::show_commit("inputShowCommit", "c", |bindings: KeyBindings| bindings.show_commit)]
	#[case::show_diff("inputShowDiff", "d", |bindings: KeyBindings| bindings.show_diff)]
	#[case::toggle_visual_mode("inputToggleVisualMode", "v", |bindings: KeyBindings| bindings.toggle_visual_mode)]
	#[case::undo("inputUndo", "Controlz", |bindings: KeyBindings| bindings.undo)]
	pub(crate) fn test_binding<F: 'static>(#[case] config_name: &str, #[case] default: &str, #[case] access: F)
	where F: Fn(KeyBindings) -> Vec<String> {
		let default_keybindings = KeyBindings::new();
		let binding = access(default_keybindings);
		assert_eq!(binding, vec![String::from(default)]);

		let config_value = format!("{} = \"f255\"", config_name);
		with_git_config(&["[interactive-rebase-tool]", config_value.as_str()], |config| {
			let key_bindings = KeyBindings::new_with_config(Some(&config)).unwrap();
			let binding = access(key_bindings);
			assert_eq!(binding, vec![String::from("F255")]);
		});
	}
}
