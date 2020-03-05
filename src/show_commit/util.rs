use crate::commit::status::Status;
use crate::display::display_color::DisplayColor;
use crate::view::line_segment::LineSegment;

pub(super) fn get_file_stat_color(status: &Status) -> DisplayColor {
	match status {
		Status::Added => DisplayColor::DiffAddColor,
		Status::Copied => DisplayColor::DiffAddColor,
		Status::Deleted => DisplayColor::DiffRemoveColor,
		Status::Modified => DisplayColor::DiffChangeColor,
		Status::Renamed => DisplayColor::DiffChangeColor,
		Status::Typechange => DisplayColor::DiffChangeColor,
		// this should never happen in a rebase
		Status::Other => DisplayColor::Normal,
	}
}

pub(super) fn get_file_stat_abbreviated(status: &Status) -> String {
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
}

pub(super) fn get_file_stat_long(status: &Status) -> String {
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

pub(super) fn get_stat_item_segments(
	status: &Status,
	to_name: &str,
	from_name: &str,
	is_full_width: bool,
) -> Vec<LineSegment>
{
	let status_name = if is_full_width {
		get_file_stat_long(&status)
	}
	else {
		get_file_stat_abbreviated(&status)
	};

	let color = get_file_stat_color(&status);

	let to_file_indicator = if is_full_width { " -> " } else { ">" };

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
