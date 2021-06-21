use anyhow::Result;
use git2::Config;

use super::{
	utils::{get_color, get_string},
	Color,
};

/// Represents the theme configuration options.
#[derive(Clone, Debug)]
pub struct Theme {
	/// The character for filling vertical spacing.
	pub character_vertical_spacing: String,
	/// The color for the break action.
	pub color_action_break: Color,
	/// The color for the drop action.
	pub color_action_drop: Color,
	/// The color for the edit action.
	pub color_action_edit: Color,
	/// The color for the exec action.
	pub color_action_exec: Color,
	/// The color for the fixup action.
	pub color_action_fixup: Color,
	/// The color for the pick action.
	pub color_action_pick: Color,
	/// The color for the reword action.
	pub color_action_reword: Color,
	/// The color for the squash action.
	pub color_action_squash: Color,
	/// The color for the label action.
	pub color_action_label: Color,
	/// The color for the reset action.
	pub color_action_reset: Color,
	/// The color for the merge action.
	pub color_action_merge: Color,
	/// The color for the background.
	pub color_background: Color,
	/// The color for added lines in a diff.
	pub color_diff_add: Color,
	/// The color for changed lines in a diff.
	pub color_diff_change: Color,
	/// The color for context lines in a diff.
	pub color_diff_context: Color,
	/// The color for removed lines in a diff.
	pub color_diff_remove: Color,
	/// The color for whitespace characters in a diff.
	pub color_diff_whitespace: Color,
	/// The color for the standard text.
	pub color_foreground: Color,
	/// The color for indicator text.
	pub color_indicator: Color,
	/// The background color for selected lines.
	pub color_selected_background: Color,
}

impl Theme {
	/// Create a new theme from a Git Config reference.
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
