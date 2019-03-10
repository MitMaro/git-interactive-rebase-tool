use color::Color;
use git_config::GitConfig;
use std::ffi::OsString;

#[derive(Debug, Clone)]
pub struct Config {
	pub comment_char: String,
	pub foreground_color: Color,
	pub indicator_color: Color,
	pub error_color: Color,
	pub diff_add_color: Color,
	pub diff_remove_color: Color,
	pub pick_color: Color,
	pub reword_color: Color,
	pub edit_color: Color,
	pub exec_color: Color,
	pub squash_color: Color,
	pub fixup_color: Color,
	pub drop_color: Color,
	pub auto_select_next: bool,
	pub editor: OsString,
}

fn string_to_color(color_string: &str, default_color: Color) -> Color {
	Color::try_from(color_string).unwrap_or(default_color)
}

impl Config {
	pub fn new(git_config: GitConfig) -> Self {
		Config {
			comment_char: git_config.comment_char,
			foreground_color: string_to_color(git_config.foreground_color.as_ref(), Color::White),
			indicator_color: string_to_color(git_config.indicator_color.as_ref(), Color::Yellow),
			error_color: string_to_color(git_config.error_color.as_ref(), Color::Red),
			diff_add_color: string_to_color(git_config.diff_add_color.as_ref(), Color::Green),
			diff_remove_color: string_to_color(git_config.diff_add_color.as_ref(), Color::Red),
			pick_color: string_to_color(git_config.pick_color.as_ref(), Color::Green),
			reword_color: string_to_color(git_config.reword_color.as_ref(), Color::Yellow),
			edit_color: string_to_color(git_config.edit_color.as_ref(), Color::Blue),
			exec_color: string_to_color(git_config.edit_color.as_ref(), Color::White),
			squash_color: string_to_color(git_config.squash_color.as_ref(), Color::Cyan),
			fixup_color: string_to_color(git_config.fixup_color.as_ref(), Color::Magenta),
			drop_color: string_to_color(git_config.drop_color.as_ref(), Color::Red),
			auto_select_next: git_config.auto_select_next,
			editor: git_config.editor,
		}
	}
}
