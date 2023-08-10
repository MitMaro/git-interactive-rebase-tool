use git::Config;

use crate::{
	errors::ConfigError,
	utils::{get_optional_string, get_string},
	Color,
	ConfigErrorCause,
};

fn get_color(config: Option<&Config>, name: &str, default: Color) -> Result<Color, ConfigError> {
	if let Some(value) = get_optional_string(config, name)? {
		Color::try_from(value.to_lowercase().as_str()).map_err(|invalid_color_error| {
			ConfigError::new(
				name,
				value.as_str(),
				ConfigErrorCause::InvalidColor(invalid_color_error),
			)
		})
	}
	else {
		Ok(default)
	}
}

/// Represents the theme configuration options.
#[derive(Clone, Debug)]
#[non_exhaustive]
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
	/// The color for the update-ref action.
	pub color_action_update_ref: Color,
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
	#[allow(clippy::missing_panics_doc)]
	pub fn new() -> Self {
		Self::new_with_config(None).unwrap() // should never error with None config
	}

	/// Create a new theme from a Git Config reference.
	pub(super) fn new_with_config(git_config: Option<&Config>) -> Result<Self, ConfigError> {
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
			color_action_update_ref: get_color(
				git_config,
				"interactive-rebase-tool.updateRefColor",
				Color::DarkMagenta,
			)?,
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

impl TryFrom<&Config> for Theme {
	type Error = ConfigError;

	#[inline]
	fn try_from(config: &Config) -> Result<Self, Self::Error> {
		Self::new_with_config(Some(config))
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_err, assert_ok};
	use rstest::rstest;
	use testutils::assert_err_eq;

	use super::*;
	use crate::{
		errors::InvalidColorError,
		testutils::{invalid_utf, with_git_config},
		ConfigErrorCause,
	};

	macro_rules! config_test {
		($key:ident, $config_name:literal, $default:expr) => {
			let config = Theme::new();
			let value = config.$key;
			assert_eq!(
				value,
				$default,
				"Default for theme configuration '{}' was expected to be '{:?}' but '{:?}' was found",
				stringify!($key),
				$default,
				value
			);

			let config_value = format!("{} = \"42\"", $config_name);
			with_git_config(
				&["[interactive-rebase-tool]", config_value.as_str()],
				|git_config| {
					let config = Theme::new_with_config(Some(&git_config)).unwrap();
					assert_eq!(
						config.$key,
						Color::Index(42),
						"Value for theme configuration '{}' was expected to be changed but was not",
						stringify!($key)
					);
				},
			);
		};
	}

	#[test]
	fn new() {
		let _config = Theme::new();
	}

	#[test]
	fn try_from_git_config() {
		with_git_config(&[], |git_config| {
			assert_ok!(Theme::try_from(&git_config));
		});
	}

	#[test]
	fn try_from_git_config_error() {
		with_git_config(&["[interactive-rebase-tool]", "breakColor = invalid"], |git_config| {
			assert_err!(Theme::try_from(&git_config));
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

	#[test]
	fn theme_color() {
		config_test!(color_action_break, "breakColor", Color::LightWhite);
		config_test!(color_action_drop, "dropColor", Color::LightRed);
		config_test!(color_action_edit, "editColor", Color::LightBlue);
		config_test!(color_action_exec, "execColor", Color::LightWhite);
		config_test!(color_action_fixup, "fixupColor", Color::LightMagenta);
		config_test!(color_action_pick, "pickColor", Color::LightGreen);
		config_test!(color_action_reword, "rewordColor", Color::LightYellow);
		config_test!(color_action_squash, "squashColor", Color::LightCyan);
		config_test!(color_action_label, "labelColor", Color::DarkYellow);
		config_test!(color_action_reset, "resetColor", Color::DarkYellow);
		config_test!(color_action_merge, "mergeColor", Color::DarkYellow);
		config_test!(color_action_update_ref, "updateRefColor", Color::DarkMagenta);
		config_test!(color_background, "backgroundColor", Color::Default);
		config_test!(color_diff_add, "diffAddColor", Color::LightGreen);
		config_test!(color_diff_change, "diffChangeColor", Color::LightYellow);
		config_test!(color_diff_context, "diffContextColor", Color::LightWhite);
		config_test!(color_diff_remove, "diffRemoveColor", Color::LightRed);
		config_test!(color_diff_whitespace, "diffWhitespace", Color::LightBlack);
		config_test!(color_foreground, "foregroundColor", Color::Default);
		config_test!(color_indicator, "indicatorColor", Color::LightCyan);
		config_test!(color_selected_background, "selectedBackgroundColor", Color::Index(237));
	}

	#[test]
	fn value_parsing_invalid_color() {
		with_git_config(&["[interactive-rebase-tool]", "breakColor = -2"], |git_config| {
			assert_err_eq!(
				Theme::new_with_config(Some(&git_config)),
				ConfigError::new(
					"interactive-rebase-tool.breakColor",
					"-2",
					ConfigErrorCause::InvalidColor(InvalidColorError::Indexed)
				)
			);
		});
	}

	#[rstest]
	#[case::color_invalid_utf("breakColor")]
	#[case::color_invalid_utf("verticalSpacingCharacter")]
	fn value_parsing_invalid_utf(#[case] key: &str) {
		with_git_config(
			&[
				"[interactive-rebase-tool]",
				format!("{key} = {}", invalid_utf()).as_str(),
			],
			|git_config| {
				assert_err_eq!(
					Theme::new_with_config(Some(&git_config)),
					ConfigError::new_read_error(
						format!("interactive-rebase-tool.{key}").as_str(),
						ConfigErrorCause::InvalidUtf
					)
				);
			},
		);
	}
}
