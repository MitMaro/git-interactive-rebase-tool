use crate::config::key_bindings::KeyBindings;
use crate::constants::MINIMUM_FULL_WINDOW_WIDTH;
use crate::display::display_color::DisplayColor;
use crate::input::utils::get_input_short_name;
use crate::list::action::Action;
use crate::list::line::Line;
use crate::view::line_segment::LineSegment;
use std::cmp;

fn get_action_color(action: Action) -> DisplayColor {
	match action {
		Action::Break => DisplayColor::ActionBreak,
		Action::Drop => DisplayColor::ActionDrop,
		Action::Edit => DisplayColor::ActionEdit,
		Action::Exec => DisplayColor::ActionExec,
		Action::Fixup => DisplayColor::ActionFixup,
		Action::Noop => DisplayColor::Normal,
		Action::Pick => DisplayColor::ActionPick,
		Action::Reword => DisplayColor::ActionReword,
		Action::Squash => DisplayColor::ActionSquash,
	}
}

pub(super) fn get_normal_footer_full(key_bindings: &KeyBindings) -> String {
	format!(
		" {}, {}, {}/{}, {}/{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
		key_bindings.move_up,
		key_bindings.move_down,
		key_bindings.abort,
		key_bindings.force_abort,
		key_bindings.rebase,
		key_bindings.force_rebase,
		key_bindings.show_commit,
		key_bindings.move_selection_down,
		key_bindings.move_selection_up,
		key_bindings.action_break,
		key_bindings.action_pick,
		key_bindings.action_reword,
		key_bindings.action_edit,
		key_bindings.action_squash,
		key_bindings.action_fixup,
		key_bindings.action_drop,
		key_bindings.edit,
		key_bindings.open_in_external_editor,
		key_bindings.help,
	)
}

pub(super) fn get_visual_footer_full(key_bindings: &KeyBindings) -> String {
	format!(
		" {}, {}, {}/{}, {}/{}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
		key_bindings.move_up,
		key_bindings.move_down,
		key_bindings.abort,
		key_bindings.force_abort,
		key_bindings.rebase,
		key_bindings.force_rebase,
		key_bindings.move_selection_down,
		key_bindings.move_selection_up,
		key_bindings.action_pick,
		key_bindings.action_reword,
		key_bindings.action_edit,
		key_bindings.action_squash,
		key_bindings.action_fixup,
		key_bindings.action_drop,
		key_bindings.help,
	)
}

pub(super) fn get_normal_footer_compact(key_bindings: &KeyBindings) -> String {
	format!(
		"{},{},{}/{},{}/{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
		get_input_short_name(key_bindings.move_up.as_str()),
		get_input_short_name(key_bindings.move_down.as_str()),
		get_input_short_name(key_bindings.abort.as_str()),
		get_input_short_name(key_bindings.force_abort.as_str()),
		get_input_short_name(key_bindings.rebase.as_str()),
		get_input_short_name(key_bindings.force_rebase.as_str()),
		get_input_short_name(key_bindings.show_commit.as_str()),
		get_input_short_name(key_bindings.move_selection_down.as_str()),
		get_input_short_name(key_bindings.move_selection_up.as_str()),
		get_input_short_name(key_bindings.action_break.as_str()),
		get_input_short_name(key_bindings.action_pick.as_str()),
		get_input_short_name(key_bindings.action_reword.as_str()),
		get_input_short_name(key_bindings.action_edit.as_str()),
		get_input_short_name(key_bindings.action_squash.as_str()),
		get_input_short_name(key_bindings.action_fixup.as_str()),
		get_input_short_name(key_bindings.action_drop.as_str()),
		get_input_short_name(key_bindings.edit.as_str()),
		get_input_short_name(key_bindings.open_in_external_editor.as_str()),
		get_input_short_name(key_bindings.help.as_str()),
	)
}

pub(super) fn get_visual_footer_compact(key_bindings: &KeyBindings) -> String {
	format!(
		"{},{},{}/{},{}/{},{},{},{},{},{},{},{},{},{}",
		get_input_short_name(key_bindings.move_up.as_str()),
		get_input_short_name(key_bindings.move_down.as_str()),
		get_input_short_name(key_bindings.abort.as_str()),
		get_input_short_name(key_bindings.force_abort.as_str()),
		get_input_short_name(key_bindings.rebase.as_str()),
		get_input_short_name(key_bindings.force_rebase.as_str()),
		get_input_short_name(key_bindings.move_selection_down.as_str()),
		get_input_short_name(key_bindings.move_selection_up.as_str()),
		get_input_short_name(key_bindings.action_pick.as_str()),
		get_input_short_name(key_bindings.action_reword.as_str()),
		get_input_short_name(key_bindings.action_edit.as_str()),
		get_input_short_name(key_bindings.action_squash.as_str()),
		get_input_short_name(key_bindings.action_fixup.as_str()),
		get_input_short_name(key_bindings.action_drop.as_str()),
		get_input_short_name(key_bindings.help.as_str()),
	)
}

pub(super) fn get_todo_line_segments(
	line: &Line,
	is_cursor_line: bool,
	selected: bool,
	view_width: usize,
) -> Vec<LineSegment>
{
	let mut segments: Vec<LineSegment> = vec![];

	let action = line.get_action();

	if view_width >= MINIMUM_FULL_WINDOW_WIDTH {
		segments.push(LineSegment::new_with_color_and_style(
			if is_cursor_line || selected { " > " } else { "   " },
			DisplayColor::Normal,
			!is_cursor_line && selected,
			false,
			false,
		));

		segments.push(LineSegment::new_with_color(
			format!("{:6} ", action.as_string()).as_str(),
			get_action_color(*action),
		));

		segments.push(LineSegment::new(
			if *action == Action::Exec {
				line.get_command().clone()
			}
			else if *action == Action::Break {
				String::from("")
			}
			else {
				let max_index = cmp::min(line.get_hash().len(), 8);
				format!("{:8} ", line.get_hash()[0..max_index].to_string())
			}
			.as_str(),
		));
	}
	else {
		segments.push(LineSegment::new_with_color_and_style(
			if is_cursor_line || selected { ">" } else { " " },
			DisplayColor::Normal,
			!is_cursor_line && selected,
			false,
			false,
		));

		segments.push(LineSegment::new_with_color(
			format!("{:1} ", line.get_action().to_abbreviation()).as_str(),
			get_action_color(*action),
		));

		segments.push(LineSegment::new(
			if *action == Action::Exec {
				line.get_command().clone()
			}
			else if *action == Action::Break {
				String::from("    ")
			}
			else {
				let max_index = cmp::min(line.get_hash().len(), 3);
				format!("{:3} ", line.get_hash()[0..max_index].to_string())
			}
			.as_str(),
		));
	}
	if *action != Action::Exec && *action != Action::Break {
		segments.push(LineSegment::new(line.get_comment().as_str()));
	}
	segments
}
