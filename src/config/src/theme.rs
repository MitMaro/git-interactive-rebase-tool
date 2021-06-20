use anyhow::Result;
use git2::Config;

use super::{
	utils::{get_color, get_string},
	Color,
};

#[derive(Clone, Debug)]
pub struct Theme {
	pub character_vertical_spacing: String,
	pub color_action_break: Color,
	pub color_action_drop: Color,
	pub color_action_edit: Color,
	pub color_action_exec: Color,
	pub color_action_fixup: Color,
	pub color_action_pick: Color,
	pub color_action_reword: Color,
	pub color_action_squash: Color,
	pub color_action_label: Color,
	pub color_action_reset: Color,
	pub color_action_merge: Color,
	pub color_background: Color,
	pub color_diff_add: Color,
	pub color_diff_change: Color,
	pub color_diff_context: Color,
	pub color_diff_remove: Color,
	pub color_diff_whitespace: Color,
	pub color_foreground: Color,
	pub color_indicator: Color,
	pub color_selected_background: Color,
}

impl Theme {
	pub(super) fn new(git_config: &Config) -> Result<Self> {
		Ok(Self {
			character_vertical_spacing: get_string(
				git_config,
				"interactive-rebase-tool.verticalSpacingCharacter",
				"~",
			)?,
			color_action_break: get_color(git_config, "interactive-rebase-tool.breakColor", Color::LightWhite)?,
			color_action_drop: get_color(git_config, "interactive-rebase-tool.dropColor", Color::LightRed)?,
			color_action_edit: get_color(git_config, "interactive-rebase-tool.editColor", Color::LightBlue)?,
			color_action_exec: get_color(git_config, "interactive-rebase-tool.execColor", Color::LightWhite)?,
			color_action_fixup: get_color(git_config, "interactive-rebase-tool.fixupColor", Color::LightMagenta)?,
			color_action_pick: get_color(git_config, "interactive-rebase-tool.pickColor", Color::LightGreen)?,
			color_action_reword: get_color(git_config, "interactive-rebase-tool.rewordColor", Color::LightYellow)?,
			color_action_squash: get_color(git_config, "interactive-rebase-tool.squashColor", Color::LightCyan)?,
			color_action_label: get_color(git_config, "interactive-rebase-tool.labelColor", Color::DarkYellow)?,
			color_action_reset: get_color(git_config, "interactive-rebase-tool.resetColor", Color::DarkYellow)?,
			color_action_merge: get_color(git_config, "interactive-rebase-tool.mergeColor", Color::DarkYellow)?,
			color_background: get_color(git_config, "interactive-rebase-tool.backgroundColor", Color::Default)?,
			color_diff_add: get_color(git_config, "interactive-rebase-tool.diffAddColor", Color::LightGreen)?,
			color_diff_change: get_color(
				git_config,
				"interactive-rebase-tool.diffChangeColor",
				Color::LightYellow,
			)?,
			color_diff_context: get_color(
				git_config,
				"interactive-rebase-tool.diffContextColor",
				Color::LightWhite,
			)?,
			color_diff_remove: get_color(git_config, "interactive-rebase-tool.diffRemoveColor", Color::LightRed)?,
			color_diff_whitespace: get_color(git_config, "interactive-rebase-tool.diffWhitespace", Color::LightBlack)?,
			color_foreground: get_color(git_config, "interactive-rebase-tool.foregroundColor", Color::Default)?,
			color_indicator: get_color(git_config, "interactive-rebase-tool.indicatorColor", Color::LightCyan)?,
			color_selected_background: get_color(
				git_config,
				"interactive-rebase-tool.selectedBackgroundColor",
				Color::Index(237),
			)?,
		})
	}
}
