use anyhow::Result;
use git2::Config;

use super::utils::get_input;

#[derive(Clone, Debug)]
pub struct KeyBindings {
	pub abort: Vec<String>,
	pub action_break: Vec<String>,
	pub action_drop: Vec<String>,
	pub action_edit: Vec<String>,
	pub action_fixup: Vec<String>,
	pub action_pick: Vec<String>,
	pub action_reword: Vec<String>,
	pub action_squash: Vec<String>,
	pub confirm_no: Vec<String>,
	pub confirm_yes: Vec<String>,
	pub edit: Vec<String>,
	pub force_abort: Vec<String>,
	pub force_rebase: Vec<String>,
	pub help: Vec<String>,
	pub insert_line: Vec<String>,
	pub move_down: Vec<String>,
	pub move_down_step: Vec<String>,
	pub move_end: Vec<String>,
	pub move_home: Vec<String>,
	pub move_left: Vec<String>,
	pub move_right: Vec<String>,
	pub move_selection_down: Vec<String>,
	pub move_selection_up: Vec<String>,
	pub move_up: Vec<String>,
	pub move_up_step: Vec<String>,
	pub open_in_external_editor: Vec<String>,
	pub rebase: Vec<String>,
	pub redo: Vec<String>,
	pub remove_line: Vec<String>,
	pub show_commit: Vec<String>,
	pub show_diff: Vec<String>,
	pub toggle_visual_mode: Vec<String>,
	pub undo: Vec<String>,
}

impl KeyBindings {
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
