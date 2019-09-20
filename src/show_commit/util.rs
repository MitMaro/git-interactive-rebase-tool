use crate::display::DisplayColor;
use crate::view::LineSegment;
use git2::Delta;

fn get_file_stat_color(status: Delta) -> DisplayColor {
	match status {
		Delta::Added => DisplayColor::DiffAddColor,
		Delta::Copied => DisplayColor::DiffAddColor,
		Delta::Deleted => DisplayColor::DiffRemoveColor,
		Delta::Modified => DisplayColor::DiffChangeColor,
		Delta::Renamed => DisplayColor::DiffChangeColor,
		Delta::Typechange => DisplayColor::DiffChangeColor,

		// these should never happen in a rebase
		Delta::Conflicted => DisplayColor::Normal,
		Delta::Ignored => DisplayColor::Normal,
		Delta::Unmodified => DisplayColor::Normal,
		Delta::Unreadable => DisplayColor::Normal,
		Delta::Untracked => DisplayColor::Normal,
	}
}

fn get_file_stat_abbreviated(status: Delta) -> String {
	match status {
		Delta::Added => String::from("A "),
		Delta::Copied => String::from("C "),
		Delta::Deleted => String::from("D "),
		Delta::Modified => String::from("M "),
		Delta::Renamed => String::from("R "),
		Delta::Typechange => String::from("T "),

		// these should never happen in a rebase
		Delta::Conflicted => String::from("X "),
		Delta::Ignored => String::from("X "),
		Delta::Unmodified => String::from("X "),
		Delta::Unreadable => String::from("X "),
		Delta::Untracked => String::from("X "),
	}
}

fn get_file_stat_long(status: Delta) -> String {
	match status {
		Delta::Added => format!("{:>8}: ", "added"),
		Delta::Copied => format!("{:>8}: ", "copied"),
		Delta::Deleted => format!("{:>8}: ", "deleted"),
		Delta::Modified => format!("{:>8}: ", "modified"),
		Delta::Renamed => format!("{:>8}: ", "renamed"),
		Delta::Typechange => format!("{:>8}: ", "changed"),

		// these should never happen in a rebase
		Delta::Conflicted => format!("{:>8}: ", "unknown"),
		Delta::Ignored => format!("{:>8}: ", "unknown"),
		Delta::Unmodified => format!("{:>8}: ", "unknown"),
		Delta::Unreadable => format!("{:>8}: ", "unknown"),
		Delta::Untracked => format!("{:>8}: ", "unknown"),
	}
}

pub fn get_stat_item_segments(status: Delta, to_name: &str, from_name: &str, is_full_width: bool) -> Vec<LineSegment> {
	let status_name = if is_full_width {
		get_file_stat_long(status)
	}
	else {
		get_file_stat_abbreviated(status)
	};

	let color = get_file_stat_color(status);

	let to_file_indicator = if is_full_width { " -> " } else { ">" };

	match status {
		Delta::Copied => {
			vec![
				LineSegment::new_with_color(status_name.clone().as_str(), color),
				LineSegment::new_with_color(to_name, DisplayColor::Normal),
				LineSegment::new(to_file_indicator),
				LineSegment::new_with_color(from_name, DisplayColor::DiffAddColor),
			]
		},
		Delta::Renamed => {
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
