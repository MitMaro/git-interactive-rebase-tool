use std::cmp;

use bitflags::bitflags;
use config::KeyBindings;
use display::DisplayColor;
use todo_file::{Action, Line, TodoFile};
use view::LineSegment;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum HelpLinesSelector {
	Normal,
	Visual,
	Common,
}

fn build_help_lines(key_bindings: &KeyBindings, selector: HelpLinesSelector) -> Vec<(Vec<String>, String)> {
	let lines = vec![
		(&key_bindings.move_up, "Move selection up", HelpLinesSelector::Common),
		(
			&key_bindings.move_down,
			"Move selection down",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.move_up_step,
			"Move selection up half a page",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.move_down_step,
			"Move selection down half a page",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.move_home,
			"Move selection to top of the list",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.move_end,
			"Move selection to end of the list",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.move_left,
			"Scroll content to the left",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.move_right,
			"Scroll content to the right",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.abort,
			"Abort interactive rebase",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.force_abort,
			"Immediately abort interactive rebase",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.rebase,
			"Write interactive rebase file",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.force_rebase,
			"Immediately write interactive rebase file",
			HelpLinesSelector::Common,
		),
		(&key_bindings.help, "Show help", HelpLinesSelector::Common),
		(
			&key_bindings.move_selection_down,
			"Move selected lines down",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.move_selection_up,
			"Move selected lines up",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.show_commit,
			"Show commit information",
			HelpLinesSelector::Normal,
		),
		(
			&key_bindings.action_break,
			"Toggle break action",
			HelpLinesSelector::Normal,
		),
		(
			&key_bindings.action_pick,
			"Set selected commits to be picked",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.action_reword,
			"Set selected commits to be reworded",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.action_edit,
			"Set selected commits to be edited",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.action_squash,
			"Set selected commits to be squashed",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.action_fixup,
			"Set selected commits to be fixed-up",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.action_drop,
			"Set selected commits to be dropped",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.edit,
			"Edit an exec, label, reset or merge action's content",
			HelpLinesSelector::Normal,
		),
		(
			&key_bindings.insert_line,
			"Insert a new line",
			HelpLinesSelector::Normal,
		),
		(
			&key_bindings.remove_line,
			"Completely remove the selected lines",
			HelpLinesSelector::Common,
		),
		(&key_bindings.undo, "Undo the last change", HelpLinesSelector::Common),
		(
			&key_bindings.redo,
			"Redo the previous undone change",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.open_in_external_editor,
			"Open the todo file in the default editor",
			HelpLinesSelector::Common,
		),
		(
			&key_bindings.toggle_visual_mode,
			"Enter visual selection mode",
			HelpLinesSelector::Normal,
		),
		(
			&key_bindings.toggle_visual_mode,
			"Exit visual selection mode",
			HelpLinesSelector::Visual,
		),
	];

	lines
		.iter()
		.filter_map(|&(binding, help, line_selector)| {
			let selected = line_selector == selector || line_selector == HelpLinesSelector::Common;
			selected.then(|| ((*binding).clone(), String::from(help)))
		})
		.collect()
}

pub(super) fn get_list_normal_mode_help_lines(key_bindings: &KeyBindings) -> Vec<(Vec<String>, String)> {
	build_help_lines(key_bindings, HelpLinesSelector::Normal)
}

pub(super) fn get_list_visual_mode_help_lines(key_bindings: &KeyBindings) -> Vec<(Vec<String>, String)> {
	build_help_lines(key_bindings, HelpLinesSelector::Visual)
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
		Action::UpdateRef => DisplayColor::ActionUpdateRef,
		// this is technically impossible, since noops should never be rendered
		Action::Noop => DisplayColor::Normal,
	}
}

pub(super) fn get_line_action_maximum_width(todo_file: &TodoFile) -> usize {
	let mut max_width = 0;

	for line in todo_file.lines_iter() {
		let action_length = match line.get_action() {
			// allow these to overflow their bounds
			&Action::Exec | &Action::UpdateRef => 0,
			&Action::Drop | &Action::Edit | &Action::Noop | &Action::Pick => 4,
			&Action::Break | &Action::Label | &Action::Reset | &Action::Merge => 5,
			&Action::Fixup => {
				if line.option().is_some() {
					8 // "fixup -C" = 8
				}
				else {
					5
				}
			},
			&Action::Reword | &Action::Squash => 6,
		};
		if max_width < action_length {
			max_width = action_length;
		}
	}

	max_width
}

bitflags! {
	#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
	pub(crate) struct TodoLineSegmentsOptions: u8 {
		const CURSOR_LINE = 0b0000_0001;
		const SELECTED = 0b0000_0010;
		const FULL_WIDTH = 0b0000_0100;
		const SEARCH_LINE = 0b0000_1000;
	}
}

// safe slice, as it is only on the hash, which is hexadecimal
#[allow(clippy::string_slice)]
pub(super) fn get_todo_line_segments(
	line: &Line,
	search_term: Option<&str>,
	options: TodoLineSegmentsOptions,
	maximum_action_width: usize,
) -> Vec<LineSegment> {
	let mut segments: Vec<LineSegment> = vec![];

	let is_cursor_line = options.contains(TodoLineSegmentsOptions::CURSOR_LINE);
	let selected = options.contains(TodoLineSegmentsOptions::SELECTED);
	let is_full_width = options.contains(TodoLineSegmentsOptions::FULL_WIDTH);
	let is_search_index = options.contains(TodoLineSegmentsOptions::SEARCH_LINE);

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
		if let Some(opt) = line.option() {
			format!("{:maximum_action_width$} ", format!("{action} {opt}"))
		}
		else {
			format!("{:maximum_action_width$} ", action.to_string())
		}
	}
	else {
		format!(
			"{:1}{}",
			action.to_abbreviation(),
			if line.option().is_some() { "*" } else { " " }
		)
	};

	segments.push(LineSegment::new_with_color(
		action_name.as_str(),
		get_action_color(*action),
	));

	// render hash
	match *action {
		Action::Drop | Action::Edit | Action::Fixup | Action::Pick | Action::Reword | Action::Squash => {
			let action_width = if is_full_width { 8 } else { 3 };
			let max_index = cmp::min(line.get_hash().len(), action_width);
			let search_match = search_term.map_or(false, |term| line.get_hash().starts_with(term));

			segments.push(LineSegment::new_with_color_and_style(
				format!(
					"{:width$}",
					line.get_hash()[0..max_index].to_string(), // safe slice, ascii only
					width = action_width
				)
				.as_str(),
				if search_match {
					DisplayColor::IndicatorColor
				}
				else {
					DisplayColor::Normal
				},
				false,
				search_match && is_search_index,
				false,
			));
			segments.push(LineSegment::new(" "));
		},
		Action::Exec
		| Action::Label
		| Action::Reset
		| Action::Merge
		| Action::Break
		| Action::Noop
		| Action::UpdateRef => {},
	}

	let content = line.get_content();
	if !content.is_empty() {
		if let Some(term) = search_term {
			let mut split_iter = content.split(term);
			segments.push(LineSegment::new(split_iter.next().unwrap()));
			for split in split_iter {
				segments.push(LineSegment::new_with_color_and_style(
					term,
					DisplayColor::IndicatorColor,
					false,
					is_search_index,
					false,
				));
				if !split.is_empty() {
					segments.push(LineSegment::new(split));
				}
			}
		}
		else {
			segments.push(LineSegment::new(content));
		}
	}
	segments
}
