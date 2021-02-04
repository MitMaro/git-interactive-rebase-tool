use anyhow::Result;
use git2::Config;

use crate::config::utils::get_input;

#[derive(Clone, Debug)]
pub struct KeyBindings {
	pub(crate) abort: String,
	pub(crate) action_break: String,
	pub(crate) action_drop: String,
	pub(crate) action_edit: String,
	pub(crate) action_fixup: String,
	pub(crate) action_pick: String,
	pub(crate) action_reword: String,
	pub(crate) action_squash: String,
	pub(crate) confirm_no: String,
	pub(crate) confirm_yes: String,
	pub(crate) edit: String,
	pub(crate) force_abort: String,
	pub(crate) force_rebase: String,
	pub(crate) help: String,
	pub(crate) move_down: String,
	pub(crate) move_down_step: String,
	pub(crate) move_left: String,
	pub(crate) move_right: String,
	pub(crate) move_selection_down: String,
	pub(crate) move_selection_up: String,
	pub(crate) move_up: String,
	pub(crate) move_up_step: String,
	pub(crate) open_in_external_editor: String,
	pub(crate) rebase: String,
	pub(crate) show_commit: String,
	pub(crate) show_diff: String,
	pub(crate) toggle_visual_mode: String,
}

impl KeyBindings {
	pub(super) fn new(git_config: &Config) -> Result<Self> {
		Ok(Self {
			abort: get_input(git_config, "interactive-rebase-tool.inputAbort", "q")?,
			action_break: get_input(git_config, "interactive-rebase-tool.inputActionBreak", "b")?,
			action_drop: get_input(git_config, "interactive-rebase-tool.inputActionDrop", "d")?,
			action_edit: get_input(git_config, "interactive-rebase-tool.inputActionEdit", "e")?,
			action_fixup: get_input(git_config, "interactive-rebase-tool.inputActionFixup", "f")?,
			action_pick: get_input(git_config, "interactive-rebase-tool.inputActionPick", "p")?,
			action_reword: get_input(git_config, "interactive-rebase-tool.inputActionReword", "r")?,
			action_squash: get_input(git_config, "interactive-rebase-tool.inputActionSquash", "s")?,
			confirm_no: get_input(git_config, "interactive-rebase-tool.inputConfirmNo", "n")?,
			confirm_yes: get_input(git_config, "interactive-rebase-tool.inputConfirmYes", "y")?,
			edit: get_input(git_config, "interactive-rebase-tool.inputEdit", "E")?,
			force_abort: get_input(git_config, "interactive-rebase-tool.inputForceAbort", "Q")?,
			force_rebase: get_input(git_config, "interactive-rebase-tool.inputForceRebase", "W")?,
			help: get_input(git_config, "interactive-rebase-tool.inputHelp", "?")?,
			move_down: get_input(git_config, "interactive-rebase-tool.inputMoveDown", "Down")?,
			move_left: get_input(git_config, "interactive-rebase-tool.inputMoveLeft", "Left")?,
			move_right: get_input(git_config, "interactive-rebase-tool.inputMoveRight", "Right")?,
			move_up_step: get_input(git_config, "interactive-rebase-tool.inputMoveStepUp", "PageUp")?,
			move_down_step: get_input(git_config, "interactive-rebase-tool.inputMoveStepDown", "PageDown")?,
			move_selection_down: get_input(git_config, "interactive-rebase-tool.inputMoveSelectionDown", "j")?,
			move_selection_up: get_input(git_config, "interactive-rebase-tool.inputMoveSelectionUp", "k")?,
			move_up: get_input(git_config, "interactive-rebase-tool.inputMoveUp", "Up")?,
			open_in_external_editor: get_input(git_config, "interactive-rebase-tool.inputOpenInExternalEditor", "!")?,
			rebase: get_input(git_config, "interactive-rebase-tool.inputRebase", "w")?,
			show_commit: get_input(git_config, "interactive-rebase-tool.inputShowCommit", "c")?,
			show_diff: get_input(git_config, "interactive-rebase-tool.inputShowDiff", "d")?,
			toggle_visual_mode: get_input(git_config, "interactive-rebase-tool.inputToggleVisualMode", "v")?,
		})
	}
}
