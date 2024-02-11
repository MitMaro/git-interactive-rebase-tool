use std::path::Path;

use num_format::{Locale, ToFormattedString};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
	config::KeyBindings,
	display::DisplayColor,
	git::{CommitDiff, Status},
	view::{LineSegment, ViewLine},
};

const TO_FILE_INDICATOR_LONG: &str = " \u{2192} "; // " → "
const TO_FILE_INDICATOR_SHORT: &str = "\u{2192}"; // "→"

pub(super) fn get_show_commit_help_lines(key_bindings: &KeyBindings) -> Vec<(Vec<String>, String)> {
	vec![
		(key_bindings.scroll_up.clone(), String::from("Scroll up")),
		(key_bindings.scroll_down.clone(), String::from("Scroll down")),
		(
			key_bindings.scroll_step_up.clone(),
			String::from("Scroll up half a page"),
		),
		(
			key_bindings.scroll_step_down.clone(),
			String::from("Scroll down half a page"),
		),
		(key_bindings.scroll_home.clone(), String::from("Scroll to the top")),
		(key_bindings.scroll_end.clone(), String::from("Scroll to the bottom")),
		(key_bindings.scroll_right.clone(), String::from("Scroll right")),
		(key_bindings.scroll_left.clone(), String::from("Scroll left")),
		(key_bindings.show_diff.clone(), String::from("Show full diff")),
		(key_bindings.help.clone(), String::from("Show help")),
	]
}

pub(super) fn get_stat_item_segments(
	status: Status,
	to_name: &Path,
	from_name: &Path,
	is_full_width: bool,
) -> Vec<LineSegment> {
	let status_name = if is_full_width {
		match status {
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
		match status {
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

	let color = match status {
		Status::Added | Status::Copied => DisplayColor::DiffAddColor,
		Status::Deleted => DisplayColor::DiffRemoveColor,
		Status::Modified | Status::Renamed | Status::Typechange => DisplayColor::DiffChangeColor,
		// this should never happen in a rebase
		Status::Other => DisplayColor::Normal,
	};

	let to_file_indicator = if is_full_width {
		TO_FILE_INDICATOR_LONG
	}
	else {
		TO_FILE_INDICATOR_SHORT
	};

	match status {
		Status::Copied => {
			vec![
				LineSegment::new_with_color(status_name.as_str(), color),
				LineSegment::new_with_color(to_name.to_str().unwrap_or("invalid"), DisplayColor::Normal),
				LineSegment::new(to_file_indicator),
				LineSegment::new_with_color(from_name.to_str().unwrap_or("invalid"), DisplayColor::DiffAddColor),
			]
		},
		Status::Renamed => {
			vec![
				LineSegment::new_with_color(status_name.as_str(), color),
				LineSegment::new_with_color(to_name.to_str().unwrap_or("invalid"), DisplayColor::DiffRemoveColor),
				LineSegment::new(to_file_indicator),
				LineSegment::new_with_color(from_name.to_str().unwrap_or("invalid"), DisplayColor::DiffAddColor),
			]
		},
		_ => {
			vec![
				LineSegment::new_with_color(status_name.as_str(), color),
				LineSegment::new_with_color(from_name.to_str().unwrap_or("invalid"), color),
			]
		},
	}
}

pub(super) fn get_files_changed_summary(diff: &CommitDiff, is_full_width: bool) -> ViewLine {
	let files_changed = diff.number_files_changed();
	let insertions = diff.number_insertions();
	let deletions = diff.number_deletions();

	if is_full_width {
		ViewLine::from(vec![
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
		ViewLine::from(vec![
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
	let length = graphemes.clone().map(str::len).sum();
	let mut start_partition_index = 0;
	let mut end_partition_index = length;

	let mut index = 0;
	for c in graphemes.clone() {
		if c != " " && c != "\t" && c != "\n" {
			start_partition_index = index;
			break;
		}
		index += c.len();
	}

	index = length;
	for c in graphemes.rev() {
		if c != " " && c != "\t" && c != "\n" {
			end_partition_index = index;
			break;
		}
		index -= c.len();
	}

	(start_partition_index, end_partition_index)
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::empty_string("", 0, 0)]
	#[case::single_character("a", 0, 1)]
	#[case::multiple_characters("abc", 0, 3)]
	#[case::internal_spaces(" a b c ", 1, 6)]
	#[case::leading_whitespace(" a", 1, 2)]
	#[case::trailing_whitespace("a ", 0, 1)]
	#[case::leading_trailing_whitespace(" a ", 1, 2)]
	#[case::all_supported_whitespace_characters(" \ta \t\n", 2, 3)]
	#[case::multi_byte_character("…", 0, 3)]
	#[case::multi_byte_character_leading_whitespace(" …", 1, 4)]
	#[case::multi_byte_character_trailing_whitespace("… ", 0, 3)]
	#[case::multi_byte_character_leading_trailing_whitespace(" … ", 1, 4)]
	#[case::multi_byte_character_in_middle(" a…b ", 1, 6)]
	fn get_partition_index_on_whitespace_for_line_cases(#[case] s: &str, #[case] start: usize, #[case] end: usize) {
		assert_eq!(get_partition_index_on_whitespace_for_line(s), (start, end));
	}
}
