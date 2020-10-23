use crate::config::key_bindings::KeyBindings;
use crate::constants::MINIMUM_FULL_WINDOW_WIDTH;
use crate::display::display_color::DisplayColor;
use crate::list::action::Action;
use crate::list::line::Line;
use crate::view::line_segment::LineSegment;
use std::cmp;

pub(super) fn get_list_normal_mode_help_lines(key_bindings: &KeyBindings) -> Vec<(String, String)> {
	vec![
		(key_bindings.move_up.clone(), String::from("Move selection up")),
		(key_bindings.move_down.clone(), String::from("Move selection down")),
		(
			key_bindings.move_up_step.clone(),
			String::from("Move selection up 5 lines"),
		),
		(
			key_bindings.move_down_step.clone(),
			String::from("Move selection down 5 lines"),
		),
		(key_bindings.abort.clone(), String::from("Abort interactive rebase")),
		(
			key_bindings.force_abort.clone(),
			String::from("Immediately abort interactive rebase"),
		),
		(
			key_bindings.rebase.clone(),
			String::from("Write interactive rebase file"),
		),
		(
			key_bindings.force_rebase.clone(),
			String::from("Immediately write interactive rebase file"),
		),
		(
			key_bindings.toggle_visual_mode.clone(),
			String::from("Enter visual mode"),
		),
		(key_bindings.help.clone(), String::from("Show help")),
		(
			key_bindings.show_commit.clone(),
			String::from("Show commit information"),
		),
		(
			key_bindings.move_selection_down.clone(),
			String::from("Move selected commit down"),
		),
		(
			key_bindings.move_selection_up.clone(),
			String::from("Move selected commit up"),
		),
		(key_bindings.action_break.clone(), String::from("Toggle break action")),
		(
			key_bindings.action_pick.clone(),
			String::from("Set selected commit to be picked"),
		),
		(
			key_bindings.action_reword.clone(),
			String::from("Set selected commit to be reworded"),
		),
		(
			key_bindings.action_edit.clone(),
			String::from("Set selected commit to be edited"),
		),
		(
			key_bindings.action_squash.clone(),
			String::from("Set selected commit to be squashed"),
		),
		(
			key_bindings.action_fixup.clone(),
			String::from("Set selected commit to be fixed-up"),
		),
		(
			key_bindings.action_drop.clone(),
			String::from("Set selected commit to be dropped"),
		),
		(key_bindings.edit.clone(), String::from("Edit an exec action's command")),
		(
			key_bindings.open_in_external_editor.clone(),
			String::from("Open the todo file in the default editor"),
		),
	]
}

pub(super) fn get_list_visual_mode_help_lines(key_bindings: &KeyBindings) -> Vec<(String, String)> {
	vec![
		(key_bindings.move_up.clone(), String::from("Move selection up")),
		(key_bindings.move_down.clone(), String::from("Move selection down")),
		(
			key_bindings.move_up_step.clone(),
			String::from("Move selection up 5 lines"),
		),
		(
			key_bindings.move_down_step.clone(),
			String::from("Move selection down 5 lines"),
		),
		(key_bindings.help.clone(), String::from("Show help")),
		(
			key_bindings.move_selection_down.clone(),
			String::from("Move selected commits down"),
		),
		(
			key_bindings.move_selection_up.clone(),
			String::from("Move selected commits up"),
		),
		(
			key_bindings.action_pick.clone(),
			String::from("Set selected commits to be picked"),
		),
		(
			key_bindings.action_reword.clone(),
			String::from("Set selected commits to be reworded"),
		),
		(
			key_bindings.action_edit.clone(),
			String::from("Set selected commits to be edited"),
		),
		(
			key_bindings.action_squash.clone(),
			String::from("Set selected commits to be squashed"),
		),
		(
			key_bindings.action_fixup.clone(),
			String::from("Set selected commits to be fixed-up"),
		),
		(
			key_bindings.action_drop.clone(),
			String::from("Set selected commits to be dropped"),
		),
		(
			key_bindings.toggle_visual_mode.clone(),
			String::from("Exit visual mode"),
		),
	]
}

const fn get_action_color(action: Action) -> DisplayColor {
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
