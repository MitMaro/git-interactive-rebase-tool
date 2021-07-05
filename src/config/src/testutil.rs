//! Utilities for writing tests that interact with the configuration.
use super::{
	diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting,
	diff_show_whitespace_setting::DiffShowWhitespaceSetting,
	git_config::GitConfig,
	theme::Theme,
	Color,
	Config,
	KeyBindings,
};

/// Create a mocked version of the configuration.
#[must_use]
#[inline]
pub fn create_config() -> Config {
	Config {
		auto_select_next: false,
		diff_ignore_whitespace: DiffIgnoreWhitespaceSetting::None,
		diff_show_whitespace: DiffShowWhitespaceSetting::Both,
		diff_space_symbol: String::from("·"),
		diff_tab_symbol: String::from("→"),
		diff_tab_width: 4,
		undo_limit: 5000,
		git: GitConfig {
			comment_char: String::from("#"),
			diff_context: 3,
			diff_interhunk_lines: 0,
			diff_rename_limit: 200,
			diff_renames: true,
			diff_copies: false,
			editor: String::from("true"),
		},
		key_bindings: KeyBindings {
			abort: vec![String::from("q")],
			action_break: vec![String::from("b")],
			action_drop: vec![String::from("d")],
			action_edit: vec![String::from("e")],
			action_fixup: vec![String::from("f")],
			action_pick: vec![String::from("p")],
			action_reword: vec![String::from("r")],
			action_squash: vec![String::from("s")],
			confirm_no: vec![String::from("n")],
			confirm_yes: vec![String::from("y")],
			edit: vec![String::from("E")],
			force_abort: vec![String::from("Q")],
			force_rebase: vec![String::from("W")],
			help: vec![String::from("?")],
			insert_line: vec![String::from("I")],
			move_down: vec![String::from("Down")],
			move_down_step: vec![String::from("PageDown")],
			move_end: vec![String::from("End")],
			move_home: vec![String::from("Home")],
			move_left: vec![String::from("Left")],
			move_right: vec![String::from("Right")],
			move_selection_down: vec![String::from("j")],
			move_selection_up: vec![String::from("k")],
			move_up: vec![String::from("Up")],
			move_up_step: vec![String::from("PageUp")],
			open_in_external_editor: vec![String::from("!")],
			rebase: vec![String::from("w")],
			redo: vec![String::from("Controly")],
			remove_line: vec![String::from("Delete")],
			show_commit: vec![String::from("c")],
			show_diff: vec![String::from("d")],
			toggle_visual_mode: vec![String::from("v")],
			undo: vec![String::from("Controlz")],
		},
		theme: create_theme(),
	}
}

/// Create a mocked version of the configuration theme.
#[must_use]
#[inline]
pub fn create_theme() -> Theme {
	Theme {
		character_vertical_spacing: String::from("~"),
		color_action_break: Color::LightWhite,
		color_action_drop: Color::LightRed,
		color_action_edit: Color::LightBlue,
		color_action_exec: Color::LightWhite,
		color_action_fixup: Color::LightMagenta,
		color_action_pick: Color::LightGreen,
		color_action_reword: Color::LightYellow,
		color_action_squash: Color::LightCyan,
		color_action_label: Color::DarkYellow,
		color_action_reset: Color::DarkYellow,
		color_action_merge: Color::DarkYellow,
		color_background: Color::Default,
		color_diff_add: Color::LightGreen,
		color_diff_change: Color::LightYellow,
		color_diff_context: Color::LightWhite,
		color_diff_remove: Color::LightRed,
		color_diff_whitespace: Color::LightBlack,
		color_foreground: Color::Default,
		color_indicator: Color::LightCyan,
		color_selected_background: Color::Index(237),
	}
}
