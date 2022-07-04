use std::cmp;

use config::KeyBindings;
use display::DisplayColor;
use todo_file::{Action, Line};
use view::LineSegment;

pub(super) fn get_list_normal_mode_help_lines(key_bindings: &KeyBindings) -> Vec<(Vec<String>, String)> {
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
		(
			key_bindings.move_home.clone(),
			String::from("Move selection to top of the list"),
		),
		(
			key_bindings.move_end.clone(),
			String::from("Move selection to end of the list"),
		),
		(
			key_bindings.move_left.clone(),
			String::from("Scroll content to the left"),
		),
		(
			key_bindings.move_right.clone(),
			String::from("Scroll content to the right"),
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
		(key_bindings.insert_line.clone(), String::from("Insert a new line")),
		(
			key_bindings.remove_line.clone(),
			String::from("Completely remove the selected line"),
		),
		(key_bindings.undo.clone(), String::from("Undo the last change")),
		(
			key_bindings.redo.clone(),
			String::from("Redo the previous undone change"),
		),
		(
			key_bindings.open_in_external_editor.clone(),
			String::from("Open the todo file in the default editor"),
		),
	]
}

pub(super) fn get_list_visual_mode_help_lines(key_bindings: &KeyBindings) -> Vec<(Vec<String>, String)> {
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
		(
			key_bindings.move_home.clone(),
			String::from("Move selection to top of the list"),
		),
		(
			key_bindings.move_end.clone(),
			String::from("Move selection to end of the list"),
		),
		(
			key_bindings.move_left.clone(),
			String::from("Scroll content to the left"),
		),
		(
			key_bindings.move_right.clone(),
			String::from("Scroll content to the right"),
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
			key_bindings.remove_line.clone(),
			String::from("Completely remove the selected lines"),
		),
		(key_bindings.undo.clone(), String::from("Undo the last change")),
		(
			key_bindings.redo.clone(),
			String::from("Redo the previous undone change"),
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
		Action::Pick => DisplayColor::ActionPick,
		Action::Reword => DisplayColor::ActionReword,
		Action::Squash => DisplayColor::ActionSquash,
		Action::Label => DisplayColor::ActionLabel,
		Action::Reset => DisplayColor::ActionReset,
		Action::Merge => DisplayColor::ActionMerge,
		// this is technically impossible, since noops should never be rendered
		Action::Noop => DisplayColor::Normal,
	}
}

// safe slice, as it is only on the hash, which is hexadecimal
#[allow(clippy::string_slice, clippy::indexing_slicing)]
pub(super) fn get_todo_line_segments(
	line: &Line,
	is_cursor_line: bool,
	selected: bool,
	is_full_width: bool,
) -> Vec<LineSegment> {
	let mut segments: Vec<LineSegment> = vec![];

	let action = line.get_action();

	let indicator = if is_cursor_line || selected {
		if is_full_width { " > " } else { ">" }
	}
	else if is_full_width {
		"   "
	}
	else {
		" "
	};

	segments.push(LineSegment::new_with_color_and_style(
		indicator,
		DisplayColor::Normal,
		!is_cursor_line && selected,
		false,
		false,
	));

	let action_name = if is_full_width {
		format!("{:6} ", action.to_string())
	}
	else {
		format!("{:1} ", action.to_abbreviation())
	};

	segments.push(LineSegment::new_with_color(
		action_name.as_str(),
		get_action_color(*action),
	));

	match *action {
		Action::Drop | Action::Edit | Action::Fixup | Action::Pick | Action::Reword | Action::Squash => {
			let action_width = if is_full_width { 8 } else { 3 };
			let max_index = cmp::min(line.get_hash().len(), action_width);
			segments.push(LineSegment::new(
				format!(
					"{:width$} ",
					line.get_hash()[0..max_index].to_string(), // safe slice, ascii only
					width = action_width
				)
				.as_str(),
			));
		},
		Action::Exec | Action::Label | Action::Reset | Action::Merge | Action::Break | Action::Noop => {},
	}
	let content = line.get_content();
	if !content.is_empty() {
		segments.push(LineSegment::new(content));
	}
	segments
}
