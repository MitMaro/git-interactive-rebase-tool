pub(crate) mod git_config;
pub(crate) mod key_bindings;
pub(crate) mod theme;
mod utils;

use crate::config::git_config::GitConfig;
use crate::config::key_bindings::KeyBindings;
use crate::config::theme::Theme;
use crate::config::utils::{editor_from_env, get_bool, get_color, get_input, get_string, open_git_config};
use crate::display::color::Color;

#[derive(Clone, Debug)]
pub(crate) struct Config {
	pub(crate) auto_select_next: bool,
	pub(crate) git: GitConfig,
	pub(crate) key_bindings: KeyBindings,
	pub(crate) theme: Theme,
}

impl Config {
	pub(crate) fn new() -> Result<Self, String> {
		let git_config = open_git_config()?;

		let comment_char = get_string(&git_config, "core.commentChar", "#")?;
		let comment_char = if comment_char.as_str().eq("auto") {
			String::from("#")
		}
		else {
			comment_char
		};

		Ok(Config {
			theme: Theme {
				color_foreground: get_color(&git_config, "interactive-rebase-tool.foregroundColor", Color::Default)?,
				color_background: get_color(&git_config, "interactive-rebase-tool.backgroundColor", Color::Default)?,
				color_selected_background: get_color(
					&git_config,
					"interactive-rebase-tool.selectedBackgroundColor",
					Color::Index(237),
				)?,
				color_indicator: get_color(&git_config, "interactive-rebase-tool.indicatorColor", Color::LightCyan)?,
				color_action_break: get_color(&git_config, "interactive-rebase-tool.breakColor", Color::LightWhite)?,
				color_action_drop: get_color(&git_config, "interactive-rebase-tool.dropColor", Color::LightRed)?,
				color_action_edit: get_color(&git_config, "interactive-rebase-tool.editColor", Color::LightBlue)?,
				color_action_exec: get_color(&git_config, "interactive-rebase-tool.execColor", Color::LightWhite)?,
				color_action_fixup: get_color(&git_config, "interactive-rebase-tool.fixupColor", Color::LightMagenta)?,
				color_action_pick: get_color(&git_config, "interactive-rebase-tool.pickColor", Color::LightGreen)?,
				color_action_reword: get_color(&git_config, "interactive-rebase-tool.rewordColor", Color::LightYellow)?,
				color_action_squash: get_color(&git_config, "interactive-rebase-tool.squashColor", Color::LightCyan)?,
				color_diff_add: get_color(&git_config, "interactive-rebase-tool.diffAddColor", Color::LightGreen)?,
				color_diff_change: get_color(
					&git_config,
					"interactive-rebase-tool.diffChangeColor",
					Color::LightYellow,
				)?,
				color_diff_remove: get_color(&git_config, "interactive-rebase-tool.diffRemoveColor", Color::LightRed)?,
				character_vertical_spacing: get_string(
					&git_config,
					"interactive-rebase-tool.verticalSpacingCharacter",
					"~",
				)?,
			},
			auto_select_next: get_bool(&git_config, "interactive-rebase-tool.autoSelectNext", false)?,
			git: GitConfig {
				comment_char,
				editor: get_string(&git_config, "core.editor", editor_from_env().as_str())?,
			},
			key_bindings: KeyBindings {
				abort: get_input(&git_config, "interactive-rebase-tool.inputAbort", "q")?,
				action_break: get_input(&git_config, "interactive-rebase-tool.inputActionBreak", "b")?,
				action_drop: get_input(&git_config, "interactive-rebase-tool.inputActionDrop", "d")?,
				action_edit: get_input(&git_config, "interactive-rebase-tool.inputActionEdit", "e")?,
				action_fixup: get_input(&git_config, "interactive-rebase-tool.inputActionFixup", "f")?,
				action_pick: get_input(&git_config, "interactive-rebase-tool.inputActionPick", "p")?,
				action_reword: get_input(&git_config, "interactive-rebase-tool.inputActionReword", "r")?,
				action_squash: get_input(&git_config, "interactive-rebase-tool.inputActionSquash", "s")?,
				confirm_no: get_input(&git_config, "interactive-rebase-tool.inputConfirmNo", "n")?,
				confirm_yes: get_input(&git_config, "interactive-rebase-tool.inputConfirmYes", "y")?,
				edit: get_input(&git_config, "interactive-rebase-tool.inputEdit", "E")?,
				force_abort: get_input(&git_config, "interactive-rebase-tool.inputForceAbort", "Q")?,
				force_rebase: get_input(&git_config, "interactive-rebase-tool.inputForceRebase", "W")?,
				help: get_input(&git_config, "interactive-rebase-tool.inputHelp", "?")?,
				move_down: get_input(&git_config, "interactive-rebase-tool.inputMoveDown", "Down")?,
				move_left: get_input(&git_config, "interactive-rebase-tool.inputMoveLeft", "Left")?,
				move_right: get_input(&git_config, "interactive-rebase-tool.inputMoveRight", "Right")?,
				move_up_step: get_input(&git_config, "interactive-rebase-tool.inputMoveStepUp", "PageUp")?,
				move_down_step: get_input(&git_config, "interactive-rebase-tool.inputMoveStepDown", "PageDown")?,
				move_selection_down: get_input(&git_config, "interactive-rebase-tool.inputMoveSelectionDown", "j")?,
				move_selection_up: get_input(&git_config, "interactive-rebase-tool.inputMoveSelectionUp", "k")?,
				move_up: get_input(&git_config, "interactive-rebase-tool.inputMoveUp", "Up")?,
				open_in_external_editor: get_input(
					&git_config,
					"interactive-rebase-tool.inputOpenInExternalEditor",
					"!",
				)?,
				rebase: get_input(&git_config, "interactive-rebase-tool.inputRebase", "w")?,
				show_commit: get_input(&git_config, "interactive-rebase-tool.inputShowCommit", "c")?,
				toggle_visual_mode: get_input(&git_config, "interactive-rebase-tool.inputToggleVisualMode", "v")?,
			},
		})
	}
}
