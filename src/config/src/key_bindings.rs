use anyhow::Result;
use git2::Config;

use super::utils::get_input;

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
	/// Create a new key bindings from a Git Config reference.
	pub(super) fn new(git_config: &Config) -> Result<Self> {
		let confirm_no = get_input(git_config, "interactive-rebase-tool.inputConfirmNo", "n")?
			.iter()
			.map(|s| s.as_str().to_lowercase())
			.collect();
		let confirm_yes = get_input(git_config, "interactive-rebase-tool.inputConfirmYes", "y")?
			.iter()
			.map(|s| s.as_str().to_lowercase())
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
