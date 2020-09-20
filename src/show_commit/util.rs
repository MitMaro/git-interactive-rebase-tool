use crate::config::key_bindings::KeyBindings;
use crate::display::display_color::DisplayColor;
use crate::show_commit::commit::Commit;
use crate::show_commit::status::Status;
use crate::view::line_segment::LineSegment;
use crate::view::view_line::ViewLine;
use num_format::{Locale, ToFormattedString};
use unicode_segmentation::UnicodeSegmentation;

pub(super) fn get_show_commit_help_lines(key_bindings: &KeyBindings) -> Vec<(String, String)> {
	vec![
		(key_bindings.move_up.clone(), String::from("Scroll up")),
		(key_bindings.move_down.clone(), String::from("Scroll down")),
		(key_bindings.move_up_step.clone(), String::from("Scroll up half a page")),
		(
			key_bindings.move_down_step.clone(),
			String::from("Scroll down half a page"),
		),
		(key_bindings.move_right.clone(), String::from("Scroll right")),
		(key_bindings.move_left.clone(), String::from("Scroll left")),
		(key_bindings.show_diff.clone(), String::from("Show full diff")),
		(key_bindings.help.clone(), String::from("Show help")),
	]
}

pub(super) fn get_stat_item_segments(
	status: &Status,
	to_name: &str,
	from_name: &str,
	is_full_width: bool,
) -> Vec<LineSegment>
{
	let status_name = if is_full_width {
		match *status {
			Status::Added => format!("{:>8}: ", "added"),
			Status::Copied => format!("{:>8}: ", "copied"),
			Status::Deleted => format!("{:>8}: ", "deleted"),
			Status::Modified => format!("{:>8}: ", "modified"),
			Status::Renamed => format!("{:>8}: ", "renamed"),
			Status::Typechange => format!("{:>8}: ", "changed"),
			// this should never happen in a rebase
			Status::Other => format!("{:>8}: ", "unknown"),
		}
	}
	else {
		match *status {
			Status::Added => String::from("A "),
			Status::Copied => String::from("C "),
			Status::Deleted => String::from("D "),
			Status::Modified => String::from("M "),
			Status::Renamed => String::from("R "),
			Status::Typechange => String::from("T "),
			// this should never happen in a rebase
			Status::Other => String::from("X "),
		}
	};

	let color = match *status {
		Status::Added | Status::Copied => DisplayColor::DiffAddColor,
		Status::Deleted => DisplayColor::DiffRemoveColor,
		Status::Modified | Status::Renamed | Status::Typechange => DisplayColor::DiffChangeColor,
		// this should never happen in a rebase
		Status::Other => DisplayColor::Normal,
	};

	let to_file_indicator = if is_full_width { " → " } else { "→" };

	match *status {
		Status::Copied => {
			vec![
				LineSegment::new_with_color(status_name.as_str(), color),
				LineSegment::new_with_color(to_name, DisplayColor::Normal),
				LineSegment::new(to_file_indicator),
				LineSegment::new_with_color(from_name, DisplayColor::DiffAddColor),
			]
		},
		Status::Renamed => {
			vec![
				LineSegment::new_with_color(status_name.as_str(), color),
				LineSegment::new_with_color(to_name, DisplayColor::DiffRemoveColor),
				LineSegment::new(to_file_indicator),
				LineSegment::new_with_color(from_name, DisplayColor::DiffAddColor),
			]
		},
		_ => {
			vec![
				LineSegment::new_with_color(status_name.as_str(), color),
				LineSegment::new_with_color(from_name, color),
			]
		},
	}
}

pub(super) fn get_files_changed_summary(commit: &Commit, is_full_width: bool) -> ViewLine {
	let files_changed = commit.get_number_files_changed();
	let insertions = commit.get_number_insertions();
	let deletions = commit.get_number_deletions();

	if is_full_width {
		ViewLine::new(vec![
			LineSegment::new_with_color(
				files_changed.to_formatted_string(&Locale::en).as_str(),
				DisplayColor::IndicatorColor,
			),
			LineSegment::new(if files_changed == 1 { " file" } else { " files" }),
			LineSegment::new(" with "),
			LineSegment::new_with_color(
				insertions.to_formatted_string(&Locale::en).as_str(),
				DisplayColor::DiffAddColor,
			),
			LineSegment::new(if insertions == 1 { " insertion" } else { " insertions" }),
			LineSegment::new(" and "),
			LineSegment::new_with_color(
				deletions.to_formatted_string(&Locale::en).as_str(),
				DisplayColor::DiffRemoveColor,
			),
			LineSegment::new(if deletions == 1 { " deletion" } else { " deletions" }),
		])
	}
	else {
		ViewLine::new(vec![
			LineSegment::new_with_color(
				files_changed.to_formatted_string(&Locale::en).as_str(),
				DisplayColor::IndicatorColor,
			),
			LineSegment::new(" / "),
			LineSegment::new_with_color(
				insertions.to_formatted_string(&Locale::en).as_str(),
				DisplayColor::DiffAddColor,
			),
			LineSegment::new(" / "),
			LineSegment::new_with_color(
				deletions.to_formatted_string(&Locale::en).as_str(),
				DisplayColor::DiffRemoveColor,
			),
		])
	}
}

pub(super) fn get_partition_index_on_whitespace_for_line(line: &str) -> (usize, usize) {
	let graphemes = UnicodeSegmentation::graphemes(line, true);
	let length = graphemes.clone().count();
	let mut start_partition_index = 0;
	let mut end_partition_index = 0;

	for (index, c) in graphemes.clone().enumerate() {
		if c != " " && c != "\t" && c != "\n" {
			start_partition_index = index;
			break;
		}
	}

	for (index, c) in graphemes.rev().enumerate() {
		if c != " " && c != "\t" && c != "\n" {
			end_partition_index = length - index;
			break;
		}
	}

	if start_partition_index >= end_partition_index {
		start_partition_index = 0;
		end_partition_index = length;
	}

	(start_partition_index, end_partition_index)
}
