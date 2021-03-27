use anyhow::Result;
use git2::Config;

use crate::config::utils::get_input;

#[derive(Clone, Debug)]
pub struct KeyBindings {
	pub(crate) abort: Vec<String>,
	pub(crate) action_break: Vec<String>,
	pub(crate) action_drop: Vec<String>,
	pub(crate) action_edit: Vec<String>,
	pub(crate) action_fixup: Vec<String>,
	pub(crate) action_pick: Vec<String>,
	pub(crate) action_reword: Vec<String>,
	pub(crate) action_squash: Vec<String>,
	pub(crate) confirm_no: Vec<String>,
	pub(crate) confirm_yes: Vec<String>,
	pub(crate) edit: Vec<String>,
	pub(crate) force_abort: Vec<String>,
	pub(crate) force_rebase: Vec<String>,
	pub(crate) help: Vec<String>,
	pub(crate) insert_line: Vec<String>,
	pub(crate) move_down: Vec<String>,
	pub(crate) move_down_step: Vec<String>,
	pub(crate) move_end: Vec<String>,
	pub(crate) move_home: Vec<String>,
	pub(crate) move_left: Vec<String>,
	pub(crate) move_right: Vec<String>,
	pub(crate) move_selection_down: Vec<String>,
	pub(crate) move_selection_up: Vec<String>,
	pub(crate) move_up: Vec<String>,
	pub(crate) move_up_step: Vec<String>,
	pub(crate) open_in_external_editor: Vec<String>,
	pub(crate) rebase: Vec<String>,
	pub(crate) redo: Vec<String>,
	pub(crate) remove_line: Vec<String>,
	pub(crate) show_commit: Vec<String>,
	pub(crate) show_diff: Vec<String>,
	pub(crate) toggle_visual_mode: Vec<String>,
	pub(crate) undo: Vec<String>,
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
