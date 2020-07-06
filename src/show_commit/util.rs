use crate::display::display_color::DisplayColor;
use crate::show_commit::commit::Commit;
use crate::show_commit::status::Status;
use crate::view::line_segment::LineSegment;
use crate::view::view_line::ViewLine;
use num_format::{Locale, ToFormattedString};

pub(super) fn get_stat_item_segments(
	status: &Status,
	to_name: &str,
	from_name: &str,
	is_full_width: bool,
) -> Vec<LineSegment>
{
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
		Status::Added => DisplayColor::DiffAddColor,
		Status::Copied => DisplayColor::DiffAddColor,
		Status::Deleted => DisplayColor::DiffRemoveColor,
		Status::Modified => DisplayColor::DiffChangeColor,
		Status::Renamed => DisplayColor::DiffChangeColor,
		Status::Typechange => DisplayColor::DiffChangeColor,
		// this should never happen in a rebase
		Status::Other => DisplayColor::Normal,
	};
	let to_file_indicator = if is_full_width { " → " } else { "→" };

	match status {
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
