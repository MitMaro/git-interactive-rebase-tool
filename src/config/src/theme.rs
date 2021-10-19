use std::convert::TryFrom;

use anyhow::{Error, Result};
use git::Config;

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
	/// Create a new configuration with default values.
	#[must_use]
	#[inline]
	pub fn new() -> Self {
		Self::new_with_config(None).expect("Panic without git config instance") // should never error with None config
	}

	/// Create a new theme from a Git Config reference.
	pub(super) fn new_with_config(git_config: Option<&Config>) -> Result<Self> {
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

impl Default for Theme {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl TryFrom<&Config> for Theme {
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

	#[test]
	fn new() {
		let _config = Theme::new();
	}

	#[test]
	fn default() {
		let _config = Theme::default();
	}

	#[test]
	fn try_from_git_config() {
		with_git_config(&[], |git_config| {
			assert!(Theme::try_from(&git_config).is_ok());
		});
	}

	#[test]
	fn try_from_git_config_error() {
		with_git_config(&["[interactive-rebase-tool]", "breakColor = invalid"], |git_config| {
			assert!(Theme::try_from(&git_config).is_err());
		});
	}

	#[test]
	fn character_vertical_spacing() {
		assert_eq!(Theme::new().character_vertical_spacing, "~");
		with_git_config(
			&["[interactive-rebase-tool]", "verticalSpacingCharacter = \"X\""],
			|config| {
				let theme = Theme::new_with_config(Some(&config)).unwrap();
				assert_eq!(theme.character_vertical_spacing, "X");
			},
		);
	}

	#[rstest]
	#[case::color_action_break("breakColor", Color::LightWhite, |theme: Theme| theme.color_action_break)]
	#[case::color_action_drop("dropColor", Color::LightRed, |theme: Theme| theme.color_action_drop)]
	#[case::color_action_edit("editColor", Color::LightBlue, |theme: Theme| theme.color_action_edit)]
	#[case::color_action_exec("execColor", Color::LightWhite, |theme: Theme| theme.color_action_exec)]
	#[case::color_action_fixup("fixupColor", Color::LightMagenta, |theme: Theme| theme.color_action_fixup)]
	#[case::color_action_pick("pickColor", Color::LightGreen, |theme: Theme| theme.color_action_pick)]
	#[case::color_action_reword("rewordColor", Color::LightYellow, |theme: Theme| theme.color_action_reword)]
	#[case::color_action_squash("squashColor", Color::LightCyan, |theme: Theme| theme.color_action_squash)]
	#[case::color_action_label("labelColor", Color::DarkYellow, |theme: Theme| theme.color_action_label)]
	#[case::color_action_reset("resetColor", Color::DarkYellow, |theme: Theme| theme.color_action_reset)]
	#[case::color_action_merge("mergeColor", Color::DarkYellow, |theme: Theme| theme.color_action_merge)]
	#[case::color_background("backgroundColor", Color::Default, |theme: Theme| theme.color_background)]
	#[case::color_diff_add("diffAddColor", Color::LightGreen, |theme: Theme| theme.color_diff_add)]
	#[case::color_diff_change("diffChangeColor", Color::LightYellow, |theme: Theme| theme.color_diff_change)]
	#[case::color_diff_context("diffContextColor", Color::LightWhite, |theme: Theme| theme.color_diff_context)]
	#[case::color_diff_remove("diffRemoveColor", Color::LightRed, |theme: Theme| theme.color_diff_remove)]
	#[case::color_diff_whitespace("diffWhitespace", Color::LightBlack, |theme: Theme| theme.color_diff_whitespace)]
	#[case::color_foreground("foregroundColor", Color::Default, |theme: Theme| theme.color_foreground)]
	#[case::color_indicator("indicatorColor", Color::LightCyan, |theme: Theme| theme.color_indicator)]
	#[case::color_selected_background(
		"selectedBackgroundColor",
		Color::Index(237),
		|theme: Theme| theme.color_selected_background)
	]
	pub(crate) fn theme_color<F: 'static>(#[case] config_name: &str, #[case] default: Color, #[case] access: F)
	where F: Fn(Theme) -> Color {
		let default_theme = Theme::new();
		let color = access(default_theme);
		assert_eq!(color, default);

		let config_value = format!("{} = \"42\"", config_name);
		with_git_config(&["[interactive-rebase-tool]", config_value.as_str()], |config| {
			let theme = Theme::new_with_config(Some(&config)).unwrap();
			let color = access(theme);
			assert_eq!(color, Color::Index(42));
		});
	}

	#[rstest]
	#[case::color_invalid_utf(
		invalid_utf(),
		"\"interactive-rebase-tool.breakColor\" is not valid: configuration value is not valid utf8"
	)]
	#[case::color_invalid_range_under(
		"-2",
		"\"interactive-rebase-tool.breakColor\" is not valid: \"-2\" is not a valid color index. Index must be \
		 between 0-255."
	)]
	#[case::color_invalid_range_above(
		"256",
		"\"interactive-rebase-tool.breakColor\" is not valid: \"256\" is not a valid color index. Index must be \
		 between 0-255."
	)]
	fn value_parsing_invalid(#[case] value: &str, #[case] expected_error: &str) {
		with_git_config(
			&["[interactive-rebase-tool]", format!("breakColor = {}", value).as_str()],
			|git_config| {
				assert_error(Theme::new_with_config(Some(&git_config)), expected_error);
			},
		);
	}
}
