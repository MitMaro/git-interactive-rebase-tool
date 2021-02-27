use std::{
	env::{remove_var, set_var},
	path::Path,
};

use serial_test::serial;

use super::*;
use crate::display::color::Color;

fn load_with_config_file(case: &str, test: &str) -> Config {
	Config::new_from_config(
		&git2::Config::open(
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("test")
				.join("fixtures")
				.join("config")
				.join(case)
				.join(test)
				.as_path(),
		)
		.unwrap(),
	)
	.unwrap()
}

fn load_error_with_config_file(case: &str, test: &str) -> String {
	format!(
		"{:#}",
		Config::new_from_config(
			&git2::Config::open(
				Path::new(env!("CARGO_MANIFEST_DIR"))
					.join("test")
					.join("fixtures")
					.join("config")
					.join(case)
					.join(test)
					.as_path(),
			)
			.unwrap(),
		)
		.err()
		.unwrap()
	)
}

#[test]
#[serial]
fn config_new() {
	set_var(
		"GIT_DIR",
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple")
			.to_str()
			.unwrap(),
	);
	Config::new().unwrap();
}

#[test]
#[serial]
fn config_new_invalid_repo() {
	let git_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("test")
		.join("fixtures")
		.join("does-not-exist")
		.into_os_string()
		.into_string()
		.unwrap();
	set_var("GIT_DIR", git_dir.as_str());
	assert_eq!(
		format!("{:#}", Config::new().err().unwrap()).trim(),
		if cfg!(windows) {
			format!(
				"Error loading git config: failed to resolve path '{}': The system cannot find the file specified.",
				git_dir.as_str()
			)
		}
		else {
			format!(
				"Error loading git config: failed to resolve path '{}': No such file or directory",
				git_dir.as_str()
			)
		}
	);
}

#[test]
fn config_auto_select_next_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert!(!config.auto_select_next);
}

#[test]
fn config_auto_select_next_true() {
	let config = load_with_config_file("auto-select-next", "true.gitconfig");
	assert!(config.auto_select_next);
}

#[test]
fn config_auto_select_next_false() {
	let config = load_with_config_file("auto-select-next", "false.gitconfig");
	assert!(!config.auto_select_next);
}

#[test]
fn config_auto_select_next_invalid() {
	assert_eq!(
		load_error_with_config_file("auto-select-next", "invalid.gitconfig"),
		"\"interactive-rebase-tool.autoSelectNext\" is not valid: failed to parse \'invalid\' as a boolean value",
	);
}

#[test]
fn config_diff_ignore_whitespace_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::None);
}

#[test]
fn config_diff_ignore_whitespace_true() {
	let config = load_with_config_file("diff-ignore-whitespace", "true.gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::All);
}

#[test]
fn config_diff_ignore_whitespace_on() {
	let config = load_with_config_file("diff-ignore-whitespace", "on.gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::All);
}

#[test]
fn config_diff_ignore_whitespace_all() {
	let config = load_with_config_file("diff-ignore-whitespace", "all.gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::All);
}

#[test]
fn config_diff_ignore_whitespace_change() {
	let config = load_with_config_file("diff-ignore-whitespace", "change.gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::Change);
}

#[test]
fn config_diff_ignore_whitespace_false() {
	let config = load_with_config_file("diff-ignore-whitespace", "false.gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::None);
}

#[test]
fn config_diff_ignore_whitespace_off() {
	let config = load_with_config_file("diff-ignore-whitespace", "off.gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::None);
}

#[test]
fn config_diff_ignore_whitespace_none() {
	let config = load_with_config_file("diff-ignore-whitespace", "none.gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::None);
}

#[test]
fn config_diff_ignore_whitespace_mixed_case() {
	let config = load_with_config_file("diff-ignore-whitespace", "mixed-case.gitconfig");
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::Change);
}

#[test]
fn config_diff_ignore_whitespace_invalid() {
	assert_eq!(
		load_error_with_config_file("diff-ignore-whitespace", "invalid.gitconfig"),
		"\"interactive-rebase-tool.diffIgnoreWhitespace\" is not valid: \"invalid\" does not match one of \"true\", \
		 \"on\", \"all\", \"change\", \"false\", \"off\" or \"none\""
	);
}

#[test]
fn config_diff_show_whitespace_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Both);
}

#[test]
fn config_diff_show_whitespace_true() {
	let config = load_with_config_file("diff-show-whitespace", "true.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Both);
}

#[test]
fn config_diff_show_whitespace_on() {
	let config = load_with_config_file("diff-show-whitespace", "on.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Both);
}

#[test]
fn config_diff_show_whitespace_both() {
	let config = load_with_config_file("diff-show-whitespace", "both.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Both);
}

#[test]
fn config_diff_show_whitespace_trailing() {
	let config = load_with_config_file("diff-show-whitespace", "trailing.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Trailing);
}

#[test]
fn config_diff_show_whitespace_leading() {
	let config = load_with_config_file("diff-show-whitespace", "leading.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Leading);
}

#[test]
fn config_diff_show_whitespace_false() {
	let config = load_with_config_file("diff-show-whitespace", "false.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::None);
}

#[test]
fn config_diff_show_whitespace_off() {
	let config = load_with_config_file("diff-show-whitespace", "off.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::None);
}

#[test]
fn config_diff_show_whitespace_none() {
	let config = load_with_config_file("diff-show-whitespace", "none.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::None);
}

#[test]
fn config_diff_show_whitespace_mixed_case() {
	let config = load_with_config_file("diff-show-whitespace", "mixed-case.gitconfig");
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Trailing);
}

#[test]
fn config_diff_show_whitespace_invalid() {
	assert_eq!(
		load_error_with_config_file("diff-show-whitespace", "invalid.gitconfig"),
		"\"interactive-rebase-tool.diffShowWhitespace\" is not valid: \"invalid\" does not match one of \"true\", \
		 \"on\", \"both\", \"trailing\", \"leading\", \"false\", \"off\" or \"none\""
	);
}

#[test]
fn config_diff_tab_width_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.diff_tab_width, 4);
}

#[test]
fn config_diff_tab_width() {
	let config = load_with_config_file("diff-tab-width", ".gitconfig");
	assert_eq!(config.diff_tab_width, 14);
}

#[test]
fn config_diff_tab_invalid() {
	assert_eq!(
		load_error_with_config_file("diff-tab-width", "invalid.gitconfig"),
		"\"interactive-rebase-tool.diffTabWidth\" is not valid: failed to parse \'invalid\' as a 32-bit integer"
	);
}

#[test]
fn config_diff_tab_invalid_range() {
	assert_eq!(
		load_error_with_config_file("diff-tab-width", "invalid-range.gitconfig"),
		"\"interactive-rebase-tool.diffTabWidth\" is not valid: \"-100\" is outside of valid range for an unsigned \
		 32-bit integer"
	);
}

#[test]
fn config_diff_tab_symbol_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.diff_tab_symbol, "→");
}

#[test]
fn config_diff_tab_symbol() {
	let config = load_with_config_file("diff-tab-symbol", ".gitconfig");
	assert_eq!(config.diff_tab_symbol, "|");
}

#[test]
fn config_diff_tab_symbol_invalid_utf8() {
	assert_eq!(
		load_error_with_config_file("diff-tab-symbol", "invalid.gitconfig"),
		"\"interactive-rebase-tool.diffTabSymbol\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_diff_space_symbol_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.diff_space_symbol, "·");
}

#[test]
fn config_diff_space_symbol() {
	let config = load_with_config_file("diff-space-symbol", ".gitconfig");
	assert_eq!(config.diff_space_symbol, "-");
}

#[test]
fn config_diff_space_symbol_invalid_utf8() {
	assert_eq!(
		load_error_with_config_file("diff-space-symbol", "invalid.gitconfig"),
		"\"interactive-rebase-tool.diffSpaceSymbol\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_git_comment_char_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.git.comment_char, "#");
}

#[test]
fn config_git_comment_char_auto() {
	let config = load_with_config_file("comment-char", "auto.gitconfig");
	assert_eq!(config.git.comment_char, "#");
}

#[test]
fn config_git_comment_char() {
	let config = load_with_config_file("comment-char", ".gitconfig");
	assert_eq!(config.git.comment_char, ";");
}

#[test]
fn config_git_comment_char_invalid() {
	assert_eq!(
		load_error_with_config_file("comment-char", "invalid.gitconfig"),
		"\"core.commentChar\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_git_diff_context_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.git.diff_context, 3);
}

#[test]
fn config_git_diff_context() {
	let config = load_with_config_file("diff-context", ".gitconfig");
	assert_eq!(config.git.diff_context, 5);
}

#[test]
fn config_git_diff_context_invalid() {
	assert_eq!(
		load_error_with_config_file("diff-context", "invalid.gitconfig"),
		"\"diff.context\" is not valid: failed to parse \'invalid\' as a 32-bit integer"
	);
}

#[test]
fn config_git_diff_context_invalid_range() {
	assert_eq!(
		load_error_with_config_file("diff-context", "invalid-range.gitconfig"),
		"\"diff.context\" is not valid: \"-100\" is outside of valid range for an unsigned 32-bit integer"
	);
}

#[test]
fn config_git_diff_renames_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, false);
}

#[test]
fn config_git_diff_renames_true() {
	let config = load_with_config_file("diff-renames", "true.gitconfig");
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, false);
}

#[test]
fn config_git_diff_renames_false() {
	let config = load_with_config_file("diff-renames", "false.gitconfig");
	assert_eq!(config.git.diff_renames, false);
	assert_eq!(config.git.diff_copies, false);
}

#[test]
fn config_git_diff_renames_copy() {
	let config = load_with_config_file("diff-renames", "copy.gitconfig");
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, true);
}

#[test]
fn config_git_diff_renames_copies() {
	let config = load_with_config_file("diff-renames", "copies.gitconfig");
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, true);
}

#[test]
fn config_git_diff_renames_mixed_case() {
	let config = load_with_config_file("diff-renames", "mixed-case.gitconfig");
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, true);
}

#[test]
fn config_git_diff_renames_invalid() {
	assert_eq!(
		load_error_with_config_file("diff-renames", "invalid.gitconfig"),
		"\"diff.renames\" is not valid: \"invalid\" does not match one of \"true\", \"false\", \"copy\" or \"copies\""
	);
}

#[test]
#[serial]
fn config_git_editor_default_no_env() {
	remove_var("VISUAL");
	remove_var("EDITOR");
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.git.editor, "vi");
}

#[test]
#[serial]
fn config_git_editor_default_visual_env() {
	remove_var("EDITOR");
	set_var("VISUAL", "visual-editor");
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.git.editor, "visual-editor");
}

#[test]
#[serial]
fn config_git_editor_default_editor_env() {
	remove_var("VISUAL");
	set_var("EDITOR", "editor");
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.git.editor, "editor");
}

#[test]
#[serial]
fn config_git_editor() {
	remove_var("VISUAL");
	remove_var("EDITOR");
	let config = load_with_config_file("editor", ".gitconfig");
	assert_eq!(config.git.editor, "custom");
}

#[test]
#[serial]
fn config_git_editor_invalid() {
	remove_var("VISUAL");
	remove_var("EDITOR");
	assert_eq!(
		load_error_with_config_file("editor", "invalid.gitconfig"),
		"\"core.editor\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_key_bindings_key_mixed_case() {
	let config = load_with_config_file("key-bindings", "key-mixed-case.gitconfig");
	assert_eq!(config.key_bindings.abort, "Backspace");
}

#[test]
fn config_key_bindings_key_backspace() {
	let config = load_with_config_file("key-bindings", "key-backspace.gitconfig");
	assert_eq!(config.key_bindings.abort, "Backspace");
}

#[test]
fn config_key_bindings_key_delete() {
	let config = load_with_config_file("key-bindings", "key-delete.gitconfig");
	assert_eq!(config.key_bindings.abort, "Delete");
}

#[test]
fn config_key_bindings_key_down() {
	let config = load_with_config_file("key-bindings", "key-down.gitconfig");
	assert_eq!(config.key_bindings.abort, "Down");
}

#[test]
fn config_key_bindings_key_end() {
	let config = load_with_config_file("key-bindings", "key-end.gitconfig");
	assert_eq!(config.key_bindings.abort, "End");
}

#[test]
fn config_key_bindings_key_enter() {
	let config = load_with_config_file("key-bindings", "key-enter.gitconfig");
	assert_eq!(config.key_bindings.abort, "Enter");
}

#[test]
fn config_key_bindings_key_f0() {
	let config = load_with_config_file("key-bindings", "key-f0.gitconfig");
	assert_eq!(config.key_bindings.abort, "F0");
}

#[test]
fn config_key_bindings_key_f1() {
	let config = load_with_config_file("key-bindings", "key-f1.gitconfig");
	assert_eq!(config.key_bindings.abort, "F1");
}

#[test]
fn config_key_bindings_key_f2() {
	let config = load_with_config_file("key-bindings", "key-f2.gitconfig");
	assert_eq!(config.key_bindings.abort, "F2");
}

#[test]
fn config_key_bindings_key_f3() {
	let config = load_with_config_file("key-bindings", "key-f3.gitconfig");
	assert_eq!(config.key_bindings.abort, "F3");
}

#[test]
fn config_key_bindings_key_f4() {
	let config = load_with_config_file("key-bindings", "key-f4.gitconfig");
	assert_eq!(config.key_bindings.abort, "F4");
}

#[test]
fn config_key_bindings_key_f5() {
	let config = load_with_config_file("key-bindings", "key-f5.gitconfig");
	assert_eq!(config.key_bindings.abort, "F5");
}

#[test]
fn config_key_bindings_key_f6() {
	let config = load_with_config_file("key-bindings", "key-f6.gitconfig");
	assert_eq!(config.key_bindings.abort, "F6");
}

#[test]
fn config_key_bindings_key_f7() {
	let config = load_with_config_file("key-bindings", "key-f7.gitconfig");
	assert_eq!(config.key_bindings.abort, "F7");
}

#[test]
fn config_key_bindings_key_f8() {
	let config = load_with_config_file("key-bindings", "key-f8.gitconfig");
	assert_eq!(config.key_bindings.abort, "F8");
}

#[test]
fn config_key_bindings_key_f9() {
	let config = load_with_config_file("key-bindings", "key-f9.gitconfig");
	assert_eq!(config.key_bindings.abort, "F9");
}

#[test]
fn config_key_bindings_key_f10() {
	let config = load_with_config_file("key-bindings", "key-f10.gitconfig");
	assert_eq!(config.key_bindings.abort, "F10");
}

#[test]
fn config_key_bindings_key_f11() {
	let config = load_with_config_file("key-bindings", "key-f11.gitconfig");
	assert_eq!(config.key_bindings.abort, "F11");
}

#[test]
fn config_key_bindings_key_f12() {
	let config = load_with_config_file("key-bindings", "key-f12.gitconfig");
	assert_eq!(config.key_bindings.abort, "F12");
}

#[test]
fn config_key_bindings_key_f13() {
	let config = load_with_config_file("key-bindings", "key-f13.gitconfig");
	assert_eq!(config.key_bindings.abort, "F13");
}

#[test]
fn config_key_bindings_key_f14() {
	let config = load_with_config_file("key-bindings", "key-f14.gitconfig");
	assert_eq!(config.key_bindings.abort, "F14");
}

#[test]
fn config_key_bindings_key_f15() {
	let config = load_with_config_file("key-bindings", "key-f15.gitconfig");
	assert_eq!(config.key_bindings.abort, "F15");
}

#[test]
fn config_key_bindings_key_home() {
	let config = load_with_config_file("key-bindings", "key-home.gitconfig");
	assert_eq!(config.key_bindings.abort, "Home");
}

#[test]
fn config_key_bindings_key_insert() {
	let config = load_with_config_file("key-bindings", "key-insert.gitconfig");
	assert_eq!(config.key_bindings.abort, "Insert");
}

#[test]
fn config_key_bindings_key_left() {
	let config = load_with_config_file("key-bindings", "key-left.gitconfig");
	assert_eq!(config.key_bindings.abort, "Left");
}

#[test]
fn config_key_bindings_key_pagedown() {
	let config = load_with_config_file("key-bindings", "key-pagedown.gitconfig");
	assert_eq!(config.key_bindings.abort, "PageDown");
}

#[test]
fn config_key_bindings_key_pageup() {
	let config = load_with_config_file("key-bindings", "key-pageup.gitconfig");
	assert_eq!(config.key_bindings.abort, "PageUp");
}

#[test]
fn config_key_bindings_key_right() {
	let config = load_with_config_file("key-bindings", "key-right.gitconfig");
	assert_eq!(config.key_bindings.abort, "Right");
}

#[test]
fn config_key_bindings_key_shift_delete() {
	let config = load_with_config_file("key-bindings", "key-shift-delete.gitconfig");
	assert_eq!(config.key_bindings.abort, "ShiftDelete");
}

#[test]
fn config_key_bindings_key_shift_down() {
	let config = load_with_config_file("key-bindings", "key-shift-down.gitconfig");
	assert_eq!(config.key_bindings.abort, "ShiftDown");
}

#[test]
fn config_key_bindings_key_shift_end() {
	let config = load_with_config_file("key-bindings", "key-shift-end.gitconfig");
	assert_eq!(config.key_bindings.abort, "ShiftEnd");
}

#[test]
fn config_key_bindings_key_shift_home() {
	let config = load_with_config_file("key-bindings", "key-shift-home.gitconfig");
	assert_eq!(config.key_bindings.abort, "ShiftHome");
}

#[test]
fn config_key_bindings_key_shift_left() {
	let config = load_with_config_file("key-bindings", "key-shift-left.gitconfig");
	assert_eq!(config.key_bindings.abort, "ShiftLeft");
}

#[test]
fn config_key_bindings_key_shift_right() {
	let config = load_with_config_file("key-bindings", "key-shift-right.gitconfig");
	assert_eq!(config.key_bindings.abort, "ShiftRight");
}

#[test]
fn config_key_bindings_key_shift_tab() {
	let config = load_with_config_file("key-bindings", "key-shift-tab.gitconfig");
	assert_eq!(config.key_bindings.abort, "ShiftTab");
}

#[test]
fn config_key_bindings_key_shift_up() {
	let config = load_with_config_file("key-bindings", "key-shift-up.gitconfig");
	assert_eq!(config.key_bindings.abort, "ShiftUp");
}

#[test]
fn config_key_bindings_key_tab() {
	let config = load_with_config_file("key-bindings", "key-tab.gitconfig");
	assert_eq!(config.key_bindings.abort, "Tab");
}

#[test]
fn config_key_bindings_key_up() {
	let config = load_with_config_file("key-bindings", "key-up.gitconfig");
	assert_eq!(config.key_bindings.abort, "Up");
}

#[test]
fn config_key_bindings_invalid() {
	assert_eq!(
		load_error_with_config_file("key-bindings", "key-invalid.gitconfig"),
		"\"interactive-rebase-tool.inputAbort\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_key_bindings_key_multiple_characters() {
	assert_eq!(
		load_error_with_config_file("key-bindings", "key-multiple-characters.gitconfig"),
		"Error reading git config: interactive-rebase-tool.inputAbort must contain only one character"
	);
}

#[test]
fn config_key_bindings_key_invalid() {
	assert_eq!(
		load_error_with_config_file("key-bindings", "key-invalid.gitconfig"),
		"\"interactive-rebase-tool.inputAbort\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_key_bindings_abort_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.abort, "q");
}

#[test]
fn config_key_bindings_abort() {
	let config = load_with_config_file("key-bindings", "abort.gitconfig");
	assert_eq!(config.key_bindings.abort, "X");
}

#[test]
fn config_key_bindings_action_break_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.action_break, "b");
}

#[test]
fn config_key_bindings_action_break() {
	let config = load_with_config_file("key-bindings", "action-break.gitconfig");
	assert_eq!(config.key_bindings.action_break, "X");
}

#[test]
fn config_key_bindings_action_drop_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.action_drop, "d");
}

#[test]
fn config_key_bindings_action_drop() {
	let config = load_with_config_file("key-bindings", "action-drop.gitconfig");
	assert_eq!(config.key_bindings.action_drop, "X");
}

#[test]
fn config_key_bindings_action_edit_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.action_edit, "e");
}

#[test]
fn config_key_bindings_action_edit() {
	let config = load_with_config_file("key-bindings", "action-edit.gitconfig");
	assert_eq!(config.key_bindings.action_edit, "X");
}

#[test]
fn config_key_bindings_action_fixup_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.action_fixup, "f");
}

#[test]
fn config_key_bindings_action_fixup() {
	let config = load_with_config_file("key-bindings", "action-fixup.gitconfig");
	assert_eq!(config.key_bindings.action_fixup, "X");
}

#[test]
fn config_key_bindings_action_pick_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.action_pick, "p");
}

#[test]
fn config_key_bindings_action_pick() {
	let config = load_with_config_file("key-bindings", "action-pick.gitconfig");
	assert_eq!(config.key_bindings.action_pick, "X");
}

#[test]
fn config_key_bindings_action_reword_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.action_reword, "r");
}

#[test]
fn config_key_bindings_action_reword() {
	let config = load_with_config_file("key-bindings", "action-reword.gitconfig");
	assert_eq!(config.key_bindings.action_reword, "X");
}

#[test]
fn config_key_bindings_action_squash_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.action_squash, "s");
}

#[test]
fn config_key_bindings_action_squash() {
	let config = load_with_config_file("key-bindings", "action-squash.gitconfig");
	assert_eq!(config.key_bindings.action_squash, "X");
}

#[test]
fn config_key_bindings_confirm_no_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.confirm_no, "n");
}

#[test]
fn config_key_bindings_confirm_no() {
	let config = load_with_config_file("key-bindings", "confirm-no.gitconfig");
	assert_eq!(config.key_bindings.confirm_no, "X");
}

#[test]
fn config_key_bindings_confirm_yes_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.confirm_yes, "y");
}

#[test]
fn config_key_bindings_confirm_yes() {
	let config = load_with_config_file("key-bindings", "confirm-yes.gitconfig");
	assert_eq!(config.key_bindings.confirm_yes, "X");
}

#[test]
fn config_key_bindings_edit_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.edit, "E");
}

#[test]
fn config_key_bindings_confirm_edit() {
	let config = load_with_config_file("key-bindings", "edit.gitconfig");
	assert_eq!(config.key_bindings.edit, "X");
}

#[test]
fn config_key_bindings_force_abort_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.force_abort, "Q");
}

#[test]
fn config_key_bindings_force_abort() {
	let config = load_with_config_file("key-bindings", "force-abort.gitconfig");
	assert_eq!(config.key_bindings.force_abort, "X");
}

#[test]
fn config_key_bindings_force_rebase_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.force_rebase, "W");
}

#[test]
fn config_key_bindings_force_rebase() {
	let config = load_with_config_file("key-bindings", "force-rebase.gitconfig");
	assert_eq!(config.key_bindings.force_rebase, "X");
}

#[test]
fn config_key_bindings_help_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.help, "?");
}

#[test]
fn config_key_bindings_help() {
	let config = load_with_config_file("key-bindings", "help.gitconfig");
	assert_eq!(config.key_bindings.help, "X");
}

#[test]
fn config_key_bindings_move_down_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.move_down, "Down");
}

#[test]
fn config_key_bindings_move_down() {
	let config = load_with_config_file("key-bindings", "move-down.gitconfig");
	assert_eq!(config.key_bindings.move_down, "X");
}

#[test]
fn config_key_bindings_move_left_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.move_left, "Left");
}

#[test]
fn config_key_bindings_move_left() {
	let config = load_with_config_file("key-bindings", "move-left.gitconfig");
	assert_eq!(config.key_bindings.move_left, "X");
}

#[test]
fn config_key_bindings_move_right_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.move_right, "Right");
}

#[test]
fn config_key_bindings_move_right() {
	let config = load_with_config_file("key-bindings", "move-right.gitconfig");
	assert_eq!(config.key_bindings.move_right, "X");
}

#[test]
fn config_key_bindings_move_step_up_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.move_up_step, "PageUp");
}

#[test]
fn config_key_bindings_move_step_up() {
	let config = load_with_config_file("key-bindings", "move-step-up.gitconfig");
	assert_eq!(config.key_bindings.move_up_step, "X");
}

#[test]
fn config_key_bindings_move_step_down_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.move_down_step, "PageDown");
}

#[test]
fn config_key_bindings_move_step_down() {
	let config = load_with_config_file("key-bindings", "move-step-down.gitconfig");
	assert_eq!(config.key_bindings.move_down_step, "X");
}

#[test]
fn config_key_bindings_move_selection_down_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.move_selection_down, "j");
}

#[test]
fn config_key_bindings_move_selection_down() {
	let config = load_with_config_file("key-bindings", "move-selection-down.gitconfig");
	assert_eq!(config.key_bindings.move_selection_down, "X");
}

#[test]
fn config_key_bindings_move_selection_up_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.move_selection_up, "k");
}

#[test]
fn config_key_bindings_move_selection_up() {
	let config = load_with_config_file("key-bindings", "move-selection-up.gitconfig");
	assert_eq!(config.key_bindings.move_selection_up, "X");
}

#[test]
fn config_key_bindings_move_up_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.move_up, "Up");
}

#[test]
fn config_key_bindings_move_up() {
	let config = load_with_config_file("key-bindings", "move-up.gitconfig");
	assert_eq!(config.key_bindings.move_up, "X");
}

#[test]
fn config_key_bindings_open_in_external_editor_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.open_in_external_editor, "!");
}

#[test]
fn config_key_bindings_open_in_external_editor() {
	let config = load_with_config_file("key-bindings", "open-in-external-editor.gitconfig");
	assert_eq!(config.key_bindings.open_in_external_editor, "X");
}

#[test]
fn config_key_bindings_rebase_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.rebase, "w");
}

#[test]
fn config_key_bindings_rebase() {
	let config = load_with_config_file("key-bindings", "rebase.gitconfig");
	assert_eq!(config.key_bindings.rebase, "X");
}

#[test]
fn config_key_bindings_show_commit_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.show_commit, "c");
}

#[test]
fn config_key_bindings_show_commit() {
	let config = load_with_config_file("key-bindings", "show-commit.gitconfig");
	assert_eq!(config.key_bindings.show_commit, "X");
}

#[test]
fn config_key_bindings_show_diff_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.show_diff, "d");
}

#[test]
fn config_key_bindings_show_diff() {
	let config = load_with_config_file("key-bindings", "show-diff.gitconfig");
	assert_eq!(config.key_bindings.show_diff, "X");
}

#[test]
fn config_key_bindings_toggle_visual_mode_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.key_bindings.toggle_visual_mode, "v");
}

#[test]
fn config_key_bindings_toggle_visual_mode() {
	let config = load_with_config_file("key-bindings", "toggle-visual-mode.gitconfig");
	assert_eq!(config.key_bindings.toggle_visual_mode, "X");
}

#[test]
fn config_theme_character_vertical_spacing_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.character_vertical_spacing, "~");
}

#[test]
fn config_theme_character_vertical_spacing() {
	let config = load_with_config_file("theme", "character-vertical-spacing.gitconfig");
	assert_eq!(config.theme.character_vertical_spacing, "X");
}

#[test]
fn config_theme_color_action_break_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_action_break, Color::LightWhite);
}

#[test]
fn config_theme_color_action_break() {
	let config = load_with_config_file("theme", "color-action-break.gitconfig");
	assert_eq!(config.theme.color_action_break, Color::Index(10));
}

#[test]
fn config_theme_color_action_drop_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_action_drop, Color::LightRed);
}

#[test]
fn config_theme_color_action_drop() {
	let config = load_with_config_file("theme", "color-action-drop.gitconfig");
	assert_eq!(config.theme.color_action_drop, Color::Index(10));
}

#[test]
fn config_theme_color_action_edit_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_action_edit, Color::LightBlue);
}

#[test]
fn config_theme_color_action_edit() {
	let config = load_with_config_file("theme", "color-action-edit.gitconfig");
	assert_eq!(config.theme.color_action_edit, Color::Index(10));
}

#[test]
fn config_theme_color_action_exec_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_action_exec, Color::LightWhite);
}

#[test]
fn config_theme_color_action_exec() {
	let config = load_with_config_file("theme", "color-action-exec.gitconfig");
	assert_eq!(config.theme.color_action_exec, Color::Index(10));
}

#[test]
fn config_theme_color_action_fixup_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_action_fixup, Color::LightMagenta);
}

#[test]
fn config_theme_color_action_fixup() {
	let config = load_with_config_file("theme", "color-action-fixup.gitconfig");
	assert_eq!(config.theme.color_action_fixup, Color::Index(10));
}

#[test]
fn config_theme_color_action_pick_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_action_pick, Color::LightGreen);
}

#[test]
fn config_theme_color_action_pick() {
	let config = load_with_config_file("theme", "color-action-pick.gitconfig");
	assert_eq!(config.theme.color_action_pick, Color::Index(10));
}

#[test]
fn config_theme_color_action_reword_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_action_reword, Color::LightYellow);
}

#[test]
fn config_theme_color_action_reword() {
	let config = load_with_config_file("theme", "color-action-reword.gitconfig");
	assert_eq!(config.theme.color_action_reword, Color::Index(10));
}

#[test]
fn config_theme_color_action_squash_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_action_squash, Color::LightCyan);
}

#[test]
fn config_theme_color_action_squash() {
	let config = load_with_config_file("theme", "color-action-squash.gitconfig");
	assert_eq!(config.theme.color_action_squash, Color::Index(10));
}

#[test]
fn config_theme_color_action_label() {
	let config = load_with_config_file("theme", "color-action-label.gitconfig");
	assert_eq!(config.theme.color_action_label, Color::Index(10));
}

#[test]
fn config_theme_color_action_reset() {
	let config = load_with_config_file("theme", "color-action-reset.gitconfig");
	assert_eq!(config.theme.color_action_reset, Color::Index(10));
}

#[test]
fn config_theme_color_action_merge() {
	let config = load_with_config_file("theme", "color-action-merge.gitconfig");
	assert_eq!(config.theme.color_action_merge, Color::Index(10));
}

#[test]
fn config_theme_color_background_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_background, Color::Default);
}

#[test]
fn config_theme_color_background() {
	let config = load_with_config_file("theme", "color-background.gitconfig");
	assert_eq!(config.theme.color_background, Color::Index(10));
}

#[test]
fn config_theme_color_diff_add_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_diff_add, Color::LightGreen);
}

#[test]
fn config_theme_color_diff_add() {
	let config = load_with_config_file("theme", "color-diff-add.gitconfig");
	assert_eq!(config.theme.color_diff_add, Color::Index(10));
}

#[test]
fn config_theme_color_diff_change_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_diff_change, Color::LightYellow);
}

#[test]
fn config_theme_color_diff_change() {
	let config = load_with_config_file("theme", "color-diff-change.gitconfig");
	assert_eq!(config.theme.color_diff_change, Color::Index(10));
}

#[test]
fn config_theme_color_diff_context_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_diff_context, Color::LightWhite);
}

#[test]
fn config_theme_color_diff_context() {
	let config = load_with_config_file("theme", "color-diff-context.gitconfig");
	assert_eq!(config.theme.color_diff_context, Color::Index(10));
}

#[test]
fn config_theme_color_diff_remove_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_diff_remove, Color::LightRed);
}

#[test]
fn config_theme_color_diff_remove() {
	let config = load_with_config_file("theme", "color-diff-remove.gitconfig");
	assert_eq!(config.theme.color_diff_remove, Color::Index(10));
}

#[test]
fn config_theme_color_diff_whitespace_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_diff_whitespace, Color::LightBlack);
}

#[test]
fn config_theme_color_diff_whitespace() {
	let config = load_with_config_file("theme", "color-diff-whitespace.gitconfig");
	assert_eq!(config.theme.color_diff_whitespace, Color::Index(10));
}

#[test]
fn config_theme_color_foreground_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_foreground, Color::Default);
}

#[test]
fn config_theme_color_foreground() {
	let config = load_with_config_file("theme", "color-foreground.gitconfig");
	assert_eq!(config.theme.color_foreground, Color::Index(10));
}

#[test]
fn config_theme_color_indicator_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_indicator, Color::LightCyan);
}

#[test]
fn config_theme_color_indicator() {
	let config = load_with_config_file("theme", "color-indicator.gitconfig");
	assert_eq!(config.theme.color_indicator, Color::Index(10));
}

#[test]
fn config_theme_color_selected_background_default() {
	let config = load_with_config_file("empty", ".gitconfig");
	assert_eq!(config.theme.color_selected_background, Color::Index(237));
}

#[test]
fn config_theme_color_selected() {
	let config = load_with_config_file("theme", "color-selected-background.gitconfig");
	assert_eq!(config.theme.color_selected_background, Color::Index(10));
}

#[test]
fn config_theme_color_invalid() {
	assert_eq!(
		load_error_with_config_file("theme", "color-invalid.gitconfig"),
		"\"interactive-rebase-tool.breakColor\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_theme_color_invalid_range_under() {
	assert_eq!(
		load_error_with_config_file("theme", "color-invalid-range-under.gitconfig"),
		"\"interactive-rebase-tool.breakColor\" is not valid: \"-2\" is not a valid color index. Index must be \
		 between 0-255."
	);
}

#[test]
fn config_theme_color_invalid_range_above() {
	assert_eq!(
		load_error_with_config_file("theme", "color-invalid-range-above.gitconfig"),
		"\"interactive-rebase-tool.breakColor\" is not valid: \"256\" is not a valid color index. Index must be \
		 between 0-255."
	);
}
