use std::{
	env::{remove_var, set_var},
	path::Path,
};

use rstest::rstest;
use serial_test::serial;
use tempfile::NamedTempFile;

use super::*;
use crate::display::color::Color;

fn load_with_git_config_callback<F>(callback: F) -> Result<Config>
where F: FnOnce(&mut git2::Config) {
	let tmp_file = NamedTempFile::new().unwrap().into_temp_path();
	let mut config = git2::Config::open(tmp_file.to_path_buf().as_path()).unwrap();
	callback(&mut config);
	Config::new_from_config(&config)
}

fn load<F>(callback: F) -> Config
where F: FnOnce(&mut git2::Config) {
	load_with_git_config_callback(callback).unwrap()
}

fn load_error<F>(callback: F) -> String
where F: FnOnce(&mut git2::Config) {
	format!("{:#}", load_with_git_config_callback(callback).err().unwrap())
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
	let config = load(|_| {});
	assert!(!config.auto_select_next);
}

#[test]
fn config_auto_select_next_true() {
	let config = load(|git_config| {
		git_config
			.set_bool("interactive-rebase-tool.autoSelectNext", true)
			.unwrap();
	});
	assert!(config.auto_select_next);
}

#[test]
fn config_auto_select_next_false() {
	let config = load(|git_config| {
		git_config
			.set_bool("interactive-rebase-tool.autoSelectNext", false)
			.unwrap();
	});
	assert!(!config.auto_select_next);
}

#[test]
fn config_auto_select_next_invalid() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.autoSelectNext", "invalid")
				.unwrap();
		}),
		"\"interactive-rebase-tool.autoSelectNext\" is not valid: failed to parse \'invalid\' as a boolean value"
	);
}

#[test]
fn config_diff_ignore_whitespace_default() {
	let config = load(|_| {});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::None);
}

#[test]
fn config_diff_ignore_whitespace_true() {
	let config = load(|git_config| {
		git_config
			.set_bool("interactive-rebase-tool.diffIgnoreWhitespace", true)
			.unwrap();
	});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::All);
}

#[test]
fn config_diff_ignore_whitespace_on() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffIgnoreWhitespace", "on")
			.unwrap();
	});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::All);
}

#[test]
fn config_diff_ignore_whitespace_all() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffIgnoreWhitespace", "all")
			.unwrap();
	});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::All);
}

#[test]
fn config_diff_ignore_whitespace_change() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffIgnoreWhitespace", "change")
			.unwrap();
	});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::Change);
}

#[test]
fn config_diff_ignore_whitespace_false() {
	let config = load(|git_config| {
		git_config
			.set_bool("interactive-rebase-tool.diffIgnoreWhitespace", false)
			.unwrap();
	});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::None);
}

#[test]
fn config_diff_ignore_whitespace_off() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffIgnoreWhitespace", "off")
			.unwrap();
	});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::None);
}

#[test]
fn config_diff_ignore_whitespace_none() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffIgnoreWhitespace", "none")
			.unwrap();
	});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::None);
}

#[test]
fn config_diff_ignore_whitespace_mixed_case() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffIgnoreWhitespace", "ChAnGe")
			.unwrap();
	});
	assert_eq!(config.diff_ignore_whitespace, DiffIgnoreWhitespaceSetting::Change);
}

#[test]
fn config_diff_ignore_whitespace_invalid() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.diffIgnoreWhitespace", "invalid")
				.unwrap();
		}),
		"\"interactive-rebase-tool.diffIgnoreWhitespace\" is not valid: \"invalid\" does not match one of \"true\", \
		 \"on\", \"all\", \"change\", \"false\", \"off\" or \"none\""
	);
}

#[test]
fn config_diff_show_whitespace_default() {
	let config = load(|_| {});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Both);
}

#[test]
fn config_diff_show_whitespace_true() {
	let config = load(|git_config| {
		git_config
			.set_bool("interactive-rebase-tool.diffShowWhitespace", true)
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Both);
}

#[test]
fn config_diff_show_whitespace_on() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffShowWhitespace", "on")
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Both);
}

#[test]
fn config_diff_show_whitespace_both() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffShowWhitespace", "both")
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Both);
}

#[test]
fn config_diff_show_whitespace_trailing() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffShowWhitespace", "trailing")
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Trailing);
}

#[test]
fn config_diff_show_whitespace_leading() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffShowWhitespace", "leading")
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Leading);
}

#[test]
fn config_diff_show_whitespace_false() {
	let config = load(|git_config| {
		git_config
			.set_bool("interactive-rebase-tool.diffShowWhitespace", false)
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::None);
}

#[test]
fn config_diff_show_whitespace_off() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffShowWhitespace", "off")
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::None);
}

#[test]
fn config_diff_show_whitespace_none() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffShowWhitespace", "none")
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::None);
}

#[test]
fn config_diff_show_whitespace_mixed_case() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffShowWhitespace", "tRaIlInG")
			.unwrap();
	});
	assert_eq!(config.diff_show_whitespace, DiffShowWhitespaceSetting::Trailing);
}

#[test]
fn config_diff_show_whitespace_invalid() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.diffShowWhitespace", "invalid")
				.unwrap();
		}),
		"\"interactive-rebase-tool.diffShowWhitespace\" is not valid: \"invalid\" does not match one of \"true\", \
		 \"on\", \"both\", \"trailing\", \"leading\", \"false\", \"off\" or \"none\""
	);
}

#[test]
fn config_diff_tab_width_default() {
	let config = load(|_| {});
	assert_eq!(config.diff_tab_width, 4);
}

#[test]
fn config_diff_tab_width() {
	let config = load(|git_config| {
		git_config.set_i32("interactive-rebase-tool.diffTabWidth", 14).unwrap();
	});
	assert_eq!(config.diff_tab_width, 14);
}

#[test]
fn config_diff_tab_width_invalid() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.diffTabWidth", "invalid")
				.unwrap();
		}),
		"\"interactive-rebase-tool.diffTabWidth\" is not valid: failed to parse \'invalid\' as a 32-bit integer"
	);
}

#[test]
fn config_diff_tab_width_invalid_range() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.diffTabWidth", "-100")
				.unwrap();
		}),
		"\"interactive-rebase-tool.diffTabWidth\" is not valid: \"-100\" is outside of valid range for an unsigned \
		 32-bit integer"
	);
}

#[test]
fn config_diff_tab_symbol_default() {
	let config = load(|_| {});
	assert_eq!(config.diff_tab_symbol, "→");
}

#[test]
fn config_diff_tab_symbol() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffTabSymbol", "|")
			.unwrap();
	});
	assert_eq!(config.diff_tab_symbol, "|");
}

#[test]
#[allow(unsafe_code)]
fn config_diff_tab_symbol_invalid_utf8() {
	assert_eq!(
		load_error(|git_config| unsafe {
			git_config
				.set_str(
					"interactive-rebase-tool.diffTabSymbol",
					String::from_utf8_unchecked(vec![0xC3, 0x28]).as_str(),
				)
				.unwrap();
		}),
		"\"interactive-rebase-tool.diffTabSymbol\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_diff_space_symbol_default() {
	let config = load(|_| {});
	assert_eq!(config.diff_space_symbol, "·");
}

#[test]
fn config_diff_space_symbol() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffSpaceSymbol", "-")
			.unwrap();
	});
	assert_eq!(config.diff_space_symbol, "-");
}

#[test]
fn config_undo_limit_default() {
	let config = load(|_| {});
	assert_eq!(config.undo_limit, 5000);
}

#[test]
fn config_undo_limit() {
	let config = load(|git_config| {
		git_config.set_i32("interactive-rebase-tool.undoLimit", 14).unwrap();
	});
	assert_eq!(config.undo_limit, 14);
}

#[test]
fn config_undo_limit_invalid() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.undoLimit", "invalid")
				.unwrap();
		}),
		"\"interactive-rebase-tool.undoLimit\" is not valid: failed to parse \'invalid\' as a 32-bit integer"
	);
}

#[test]
fn config_undo_limit_invalid_range() {
	assert_eq!(
		load_error(|git_config| {
			git_config.set_str("interactive-rebase-tool.undoLimit", "-100").unwrap();
		}),
		"\"interactive-rebase-tool.undoLimit\" is not valid: \"-100\" is outside of valid range for an unsigned \
		 32-bit integer"
	);
}

#[test]
#[allow(unsafe_code)]
fn config_diff_space_symbol_invalid_utf8() {
	assert_eq!(
		load_error(|git_config| unsafe {
			git_config
				.set_str(
					"interactive-rebase-tool.diffSpaceSymbol",
					String::from_utf8_unchecked(vec![0xC3, 0x28]).as_str(),
				)
				.unwrap();
		}),
		"\"interactive-rebase-tool.diffSpaceSymbol\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_git_comment_char_default() {
	let config = load(|_| {});
	assert_eq!(config.git.comment_char, "#");
}

#[test]
fn config_git_comment_char_auto() {
	let config = load(|git_config| {
		git_config.set_str("core.commentChar", "auto").unwrap();
	});

	assert_eq!(config.git.comment_char, "#");
}

#[test]
fn config_git_comment_char() {
	let config = load(|git_config| {
		git_config.set_str("core.commentChar", ";").unwrap();
	});
	assert_eq!(config.git.comment_char, ";");
}

#[test]
#[allow(unsafe_code)]
fn config_git_comment_char_invalid() {
	assert_eq!(
		load_error(|git_config| unsafe {
			git_config
				.set_str(
					"core.commentChar",
					String::from_utf8_unchecked(vec![0xC3, 0x28]).as_str(),
				)
				.unwrap();
		}),
		"\"core.commentChar\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_git_diff_context_default() {
	let config = load(|_| {});
	assert_eq!(config.git.diff_context, 3);
}

#[test]
fn config_git_diff_context() {
	let config = load(|git_config| {
		git_config.set_str("diff.context", "5").unwrap();
	});
	assert_eq!(config.git.diff_context, 5);
}

#[test]
fn config_git_diff_context_invalid() {
	assert_eq!(
		load_error(|git_config| {
			git_config.set_str("diff.context", "invalid").unwrap();
		}),
		"\"diff.context\" is not valid: failed to parse \'invalid\' as a 32-bit integer"
	);
}

#[test]
fn config_git_diff_context_invalid_range() {
	assert_eq!(
		load_error(|git_config| {
			git_config.set_str("diff.context", "-100").unwrap();
		}),
		"\"diff.context\" is not valid: \"-100\" is outside of valid range for an unsigned 32-bit integer"
	);
}

#[test]
fn config_git_diff_renames_default() {
	let config = load(|_| {});
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, false);
}

#[test]
fn config_git_diff_renames_true() {
	let config = load(|git_config| {
		git_config.set_str("diff.renames", "true").unwrap();
	});
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, false);
}

#[test]
fn config_git_diff_renames_false() {
	let config = load(|git_config| {
		git_config.set_str("diff.renames", "false").unwrap();
	});
	assert_eq!(config.git.diff_renames, false);
	assert_eq!(config.git.diff_copies, false);
}

#[test]
fn config_git_diff_renames_copy() {
	let config = load(|git_config| {
		git_config.set_str("diff.renames", "copy").unwrap();
	});
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, true);
}

#[test]
fn config_git_diff_renames_copies() {
	let config = load(|git_config| {
		git_config.set_str("diff.renames", "copies").unwrap();
	});
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, true);
}

#[test]
fn config_git_diff_renames_mixed_case() {
	let config = load(|git_config| {
		git_config.set_str("diff.renames", "cOpIeS").unwrap();
	});
	assert_eq!(config.git.diff_renames, true);
	assert_eq!(config.git.diff_copies, true);
}

#[test]
fn config_git_diff_renames_invalid() {
	assert_eq!(
		load_error(|git_config| {
			git_config.set_str("diff.renames", "invalid").unwrap();
		}),
		"\"diff.renames\" is not valid: \"invalid\" does not match one of \"true\", \"false\", \"copy\" or \"copies\""
	);
}

#[test]
#[serial]
fn config_git_editor_default_no_env() {
	remove_var("VISUAL");
	remove_var("EDITOR");
	let config = load(|_| {});
	assert_eq!(config.git.editor, "vi");
}

#[test]
#[serial]
fn config_git_editor_default_visual_env() {
	remove_var("EDITOR");
	set_var("VISUAL", "visual-editor");
	let config = load(|_| {});
	assert_eq!(config.git.editor, "visual-editor");
}

#[test]
#[serial]
fn config_git_editor_default_editor_env() {
	remove_var("VISUAL");
	set_var("EDITOR", "editor");
	let config = load(|_| {});
	assert_eq!(config.git.editor, "editor");
}

#[test]
#[serial]
fn config_git_editor() {
	remove_var("VISUAL");
	remove_var("EDITOR");
	let config = load(|git_config| {
		git_config.set_str("core.editor", "custom").unwrap();
	});
	assert_eq!(config.git.editor, "custom");
}

#[test]
#[serial]
#[allow(unsafe_code)]
fn config_git_editor_invalid() {
	remove_var("VISUAL");
	remove_var("EDITOR");
	assert_eq!(
		load_error(|git_config| unsafe {
			git_config
				.set_str("core.editor", String::from_utf8_unchecked(vec![0xC3, 0x28]).as_str())
				.unwrap();
		}),
		"\"core.editor\" is not valid: configuration value is not valid utf8"
	);
}

#[rstest(
	binding,
	expected,
	case::backspace("backspace", "Backspace"),
	case::backtab("backtab", "BackTab"),
	case::delete("delete", "Delete"),
	case::down("down", "Down"),
	case::end("end", "End"),
	case::home("home", "Home"),
	case::insert("insert", "Insert"),
	case::left("left", "Left"),
	case::pagedown("pagedown", "PageDown"),
	case::pageup("pageup", "PageUp"),
	case::right("right", "Right"),
	case::tab("tab", "Tab"),
	case::up("up", "Up"),
	case::f1("f1", "F1"),
	case::f255("f255", "F255"),
	case::modifier_character_lowercase("Control+a", "Controla"),
	case::modifier_character_uppercase("Control+A", "ControlA"),
	case::modifier_character_number("Control+1", "Control1"),
	case::modifier_character_special("Control++", "Control+"),
	case::modifier_character("Control+a", "Controla"),
	case::modifier_special("Control+End", "ControlEnd"),
	case::modifier_function("Control+F32", "ControlF32"),
	case::modifier_control_alt_shift_out_of_order_1("Alt+Shift+Control+End", "ShiftControlAltEnd"),
	case::modifier_control_alt_shift_out_of_order_2("Shift+Control+Alt+End", "ShiftControlAltEnd"),
	case::modifier_only_shift("Shift+End", "ShiftEnd"),
	case::modifier_only_control("Control+End", "ControlEnd"),
	case::modifier_only_control("a b c d", "a,b,c,d"),
	case::modifier_only_control("Control+End Control+A", "ControlEnd,ControlA")
)]
fn config_key_bindings(binding: &str, expected: &str) {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputAbort", binding)
			.unwrap();
	});
	assert_eq!(
		config.key_bindings.abort,
		expected.split(',').map(String::from).collect::<Vec<String>>()
	);
}

#[test]
#[allow(unsafe_code)]
fn config_key_bindings_invalid() {
	assert_eq!(
		load_error(|git_config| unsafe {
			git_config
				.set_str(
					"interactive-rebase-tool.inputAbort",
					String::from_utf8_unchecked(vec![0xC3, 0x28]).as_str(),
				)
				.unwrap();
		}),
		"\"interactive-rebase-tool.inputAbort\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_key_bindings_key_multiple_characters() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.inputAbort", "abcd")
				.unwrap();
		}),
		"Error reading git config: interactive-rebase-tool.inputAbort must contain only one character per binding"
	);
}

#[test]
fn config_key_bindings_key_invalid_function_index() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.inputAbort", "F256")
				.unwrap();
		}),
		"Error reading git config: interactive-rebase-tool.inputAbort must contain only one character per binding"
	);
}

#[test]
fn config_key_bindings_multiple_invalid() {
	assert_eq!(
		load_error(|git_config| {
			git_config
				.set_str("interactive-rebase-tool.inputAbort", "f foo")
				.unwrap();
		}),
		"Error reading git config: interactive-rebase-tool.inputAbort must contain only one character per binding"
	);
}

#[test]
#[allow(unsafe_code)]
fn config_key_bindings_key_invalid() {
	assert_eq!(
		load_error(|git_config| unsafe {
			git_config
				.set_str(
					"interactive-rebase-tool.inputAbort",
					String::from_utf8_unchecked(vec![0xC3, 0x28]).as_str(),
				)
				.unwrap();
		}),
		"\"interactive-rebase-tool.inputAbort\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_key_bindings_abort_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.abort, vec![String::from("q")]);
}

#[test]
fn config_key_bindings_abort() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.inputAbort", "X").unwrap();
	});
	assert_eq!(config.key_bindings.abort, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_action_break_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.action_break, vec![String::from("b")]);
}

#[test]
fn config_key_bindings_action_break() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputActionBreak", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.action_break, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_action_drop_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.action_drop, vec![String::from("d")]);
}

#[test]
fn config_key_bindings_action_drop() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputActionDrop", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.action_drop, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_action_edit_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.action_edit, vec![String::from("e")]);
}

#[test]
fn config_key_bindings_action_edit() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputActionEdit", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.action_edit, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_action_fixup_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.action_fixup, vec![String::from("f")]);
}

#[test]
fn config_key_bindings_action_fixup() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputActionFixup", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.action_fixup, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_action_pick_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.action_pick, vec![String::from("p")]);
}

#[test]
fn config_key_bindings_action_pick() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputActionPick", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.action_pick, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_action_reword_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.action_reword, vec![String::from("r")]);
}

#[test]
fn config_key_bindings_action_reword() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputActionReword", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.action_reword, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_action_squash_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.action_squash, vec![String::from("s")]);
}

#[test]
fn config_key_bindings_action_squash() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputActionSquash", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.action_squash, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_confirm_no_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.confirm_no, vec![String::from("n")]);
}

#[test]
fn config_key_bindings_confirm_no() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputConfirmNo", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.confirm_no, vec![String::from("x")]);
}

#[test]
fn config_key_bindings_confirm_yes_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.confirm_yes, vec![String::from("y")]);
}

#[test]
fn config_key_bindings_confirm_yes() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputConfirmYes", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.confirm_yes, vec![String::from("x")]);
}

#[test]
fn config_key_bindings_edit_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.edit, vec![String::from("E")]);
}

#[test]
fn config_key_bindings_confirm_edit() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.inputEdit", "X").unwrap();
	});
	assert_eq!(config.key_bindings.edit, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_force_abort_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.force_abort, vec![String::from("Q")]);
}

#[test]
fn config_key_bindings_force_abort() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputForceAbort", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.force_abort, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_force_rebase_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.force_rebase, vec![String::from("W")]);
}

#[test]
fn config_key_bindings_force_rebase() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputForceRebase", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.force_rebase, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_help_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.help, vec![String::from("?")]);
}

#[test]
fn config_key_bindings_help() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.inputHelp", "X").unwrap();
	});
	assert_eq!(config.key_bindings.help, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_move_down_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.move_down, vec![String::from("Down")]);
}

#[test]
fn config_key_bindings_move_down() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputMoveDown", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.move_down, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_move_left_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.move_left, vec![String::from("Left")]);
}

#[test]
fn config_key_bindings_move_left() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputMoveLeft", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.move_left, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_move_right_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.move_right, vec![String::from("Right")]);
}

#[test]
fn config_key_bindings_move_right() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputMoveRight", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.move_right, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_move_step_up_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.move_up_step, vec![String::from("PageUp")]);
}

#[test]
fn config_key_bindings_move_step_up() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputMoveStepUp", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.move_up_step, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_move_step_down_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.move_down_step, vec![String::from("PageDown")]);
}

#[test]
fn config_key_bindings_move_step_down() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputMoveStepDown", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.move_down_step, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_move_selection_down_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.move_selection_down, vec![String::from("j")]);
}

#[test]
fn config_key_bindings_move_selection_down() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputMoveSelectionDown", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.move_selection_down, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_move_selection_up_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.move_selection_up, vec![String::from("k")]);
}

#[test]
fn config_key_bindings_move_selection_up() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputMoveSelectionUp", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.move_selection_up, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_move_up_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.move_up, vec![String::from("Up")]);
}

#[test]
fn config_key_bindings_move_up() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.inputMoveUp", "X").unwrap();
	});
	assert_eq!(config.key_bindings.move_up, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_open_in_external_editor_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.open_in_external_editor, vec![String::from("!")]);
}

#[test]
fn config_key_bindings_open_in_external_editor() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputOpenInExternalEditor", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.open_in_external_editor, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_rebase_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.rebase, vec![String::from("w")]);
}

#[test]
fn config_key_bindings_rebase() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.inputRebase", "X").unwrap();
	});
	assert_eq!(config.key_bindings.rebase, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_redo_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.redo, vec![String::from("Controly")]);
}

#[test]
fn config_key_bindings_redo() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.inputRedo", "X").unwrap();
	});
	assert_eq!(config.key_bindings.redo, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_show_commit_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.show_commit, vec![String::from("c")]);
}

#[test]
fn config_key_bindings_show_commit() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputShowCommit", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.show_commit, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_show_diff_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.show_diff, vec![String::from("d")]);
}

#[test]
fn config_key_bindings_show_diff() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputShowDiff", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.show_diff, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_toggle_visual_mode_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.toggle_visual_mode, vec![String::from("v")]);
}

#[test]
fn config_key_bindings_toggle_visual_mode() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.inputToggleVisualMode", "X")
			.unwrap();
	});
	assert_eq!(config.key_bindings.toggle_visual_mode, vec![String::from("X")]);
}

#[test]
fn config_key_bindings_undo_default() {
	let config = load(|_| {});
	assert_eq!(config.key_bindings.undo, vec![String::from("Controlz")]);
}

#[test]
fn config_key_bindings_undo() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.inputUndo", "X").unwrap();
	});
	assert_eq!(config.key_bindings.undo, vec![String::from("X")]);
}

#[test]
fn config_theme_character_vertical_spacing_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.character_vertical_spacing, "~");
}

#[test]
fn config_theme_character_vertical_spacing() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.verticalSpacingCharacter", "X")
			.unwrap();
	});
	assert_eq!(config.theme.character_vertical_spacing, "X");
}

#[test]
fn config_theme_color_action_break_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_break, Color::LightWhite);
}

#[test]
fn config_theme_color_action_break() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.breakColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_break, Color::Index(10));
}

#[test]
fn config_theme_color_action_drop_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_drop, Color::LightRed);
}

#[test]
fn config_theme_color_action_drop() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.dropColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_drop, Color::Index(10));
}

#[test]
fn config_theme_color_action_edit_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_edit, Color::LightBlue);
}

#[test]
fn config_theme_color_action_edit() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.editColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_edit, Color::Index(10));
}

#[test]
fn config_theme_color_action_exec_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_exec, Color::LightWhite);
}

#[test]
fn config_theme_color_action_exec() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.execColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_exec, Color::Index(10));
}

#[test]
fn config_theme_color_action_fixup_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_fixup, Color::LightMagenta);
}

#[test]
fn config_theme_color_action_fixup() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.fixupColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_fixup, Color::Index(10));
}

#[test]
fn config_theme_color_action_pick_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_pick, Color::LightGreen);
}

#[test]
fn config_theme_color_action_pick() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.pickColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_pick, Color::Index(10));
}

#[test]
fn config_theme_color_action_reword_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_reword, Color::LightYellow);
}

#[test]
fn config_theme_color_action_reword() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.rewordColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_reword, Color::Index(10));
}

#[test]
fn config_theme_color_action_squash_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_squash, Color::LightCyan);
}

#[test]
fn config_theme_color_action_squash() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.squashColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_squash, Color::Index(10));
}

#[test]
fn config_theme_color_action_label_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_label, Color::DarkYellow);
}

#[test]
fn config_theme_color_action_label() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.labelColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_label, Color::Index(10));
}

#[test]
fn config_theme_color_action_reset_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_reset, Color::DarkYellow);
}

#[test]
fn config_theme_color_action_reset() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.resetColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_reset, Color::Index(10));
}

#[test]
fn config_theme_color_action_merge_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_action_merge, Color::DarkYellow);
}

#[test]
fn config_theme_color_action_merge() {
	let config = load(|git_config| {
		git_config.set_str("interactive-rebase-tool.mergeColor", "10").unwrap();
	});
	assert_eq!(config.theme.color_action_merge, Color::Index(10));
}

#[test]
fn config_theme_color_background_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_background, Color::Default);
}

#[test]
fn config_theme_color_background() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.backgroundColor", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_background, Color::Index(10));
}

#[test]
fn config_theme_color_diff_add_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_diff_add, Color::LightGreen);
}

#[test]
fn config_theme_color_diff_add() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffAddColor", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_diff_add, Color::Index(10));
}

#[test]
fn config_theme_color_diff_change_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_diff_change, Color::LightYellow);
}

#[test]
fn config_theme_color_diff_change() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffChangeColor", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_diff_change, Color::Index(10));
}

#[test]
fn config_theme_color_diff_context_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_diff_context, Color::LightWhite);
}

#[test]
fn config_theme_color_diff_context() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffContextColor", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_diff_context, Color::Index(10));
}

#[test]
fn config_theme_color_diff_remove_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_diff_remove, Color::LightRed);
}

#[test]
fn config_theme_color_diff_remove() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffRemoveColor", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_diff_remove, Color::Index(10));
}

#[test]
fn config_theme_color_diff_whitespace_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_diff_whitespace, Color::LightBlack);
}

#[test]
fn config_theme_color_diff_whitespace() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.diffWhitespace", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_diff_whitespace, Color::Index(10));
}

#[test]
fn config_theme_color_foreground_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_foreground, Color::Default);
}

#[test]
fn config_theme_color_foreground() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.foregroundColor", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_foreground, Color::Index(10));
}

#[test]
fn config_theme_color_indicator_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_indicator, Color::LightCyan);
}

#[test]
fn config_theme_color_indicator() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.indicatorColor", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_indicator, Color::Index(10));
}

#[test]
fn config_theme_color_selected_background_default() {
	let config = load(|_| {});
	assert_eq!(config.theme.color_selected_background, Color::Index(237));
}

#[test]
fn config_theme_color_selected() {
	let config = load(|git_config| {
		git_config
			.set_str("interactive-rebase-tool.selectedBackgroundColor", "10")
			.unwrap();
	});
	assert_eq!(config.theme.color_selected_background, Color::Index(10));
}

#[test]
#[allow(unsafe_code)]
fn config_theme_color_invalid() {
	assert_eq!(
		load_error(|git_config| unsafe {
			git_config
				.set_str(
					"interactive-rebase-tool.breakColor",
					String::from_utf8_unchecked(vec![0xC3, 0x28]).as_str(),
				)
				.unwrap();
		}),
		"\"interactive-rebase-tool.breakColor\" is not valid: configuration value is not valid utf8"
	);
}

#[test]
fn config_theme_color_invalid_range_under() {
	assert_eq!(
		load_error(|git_config| {
			git_config.set_str("interactive-rebase-tool.breakColor", "-2").unwrap();
		}),
		"\"interactive-rebase-tool.breakColor\" is not valid: \"-2\" is not a valid color index. Index must be \
		 between 0-255."
	);
}

#[test]
fn config_theme_color_invalid_range_above() {
	assert_eq!(
		load_error(|git_config| {
			git_config.set_str("interactive-rebase-tool.breakColor", "256").unwrap();
		}),
		"\"interactive-rebase-tool.breakColor\" is not valid: \"256\" is not a valid color index. Index must be \
		 between 0-255."
	);
}
