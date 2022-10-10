use display::DisplayColor;
use git::{Commit, CommitDiff, DiffLine, Origin};
use view::{LineSegment, ViewDataUpdater, ViewLine};

use super::util::{get_files_changed_summary, get_partition_index_on_whitespace_for_line, get_stat_item_segments};

const PADDING_CHARACTER: char = '\u{2015}'; // '―'

pub(super) struct ViewBuilderOptions {
	space_character: String,
	tab_character: String,
	tab_width: usize,
	show_leading_whitespace: bool,
	show_trailing_whitespace: bool,
}

impl ViewBuilderOptions {
	pub(crate) fn new(
		tab_width: usize,
		tab_character: &str,
		space_character: &str,
		show_leading_whitespace: bool,
		show_trailing_whitespace: bool,
	) -> Self {
		Self {
			space_character: String::from(space_character),
			tab_character: String::from(tab_character),
			tab_width,
			show_leading_whitespace,
			show_trailing_whitespace,
		}
	}
}

pub(super) struct ViewBuilder {
	invisible_tab_string: String,
	visible_tab_string: String,
	visible_space_string: String,
	show_leading_whitespace: bool,
	show_trailing_whitespace: bool,
}

impl ViewBuilder {
	pub(crate) fn new(options: ViewBuilderOptions) -> Self {
		Self {
			invisible_tab_string: " ".repeat(options.tab_width),
			visible_tab_string: format!("{0:width$}", options.tab_character, width = options.tab_width),
			visible_space_string: options.space_character,
			show_leading_whitespace: options.show_leading_whitespace,
			show_trailing_whitespace: options.show_trailing_whitespace,
		}
	}

	fn replace_whitespace(&self, value: &str, visible: bool) -> String {
		if visible {
			value
				.replace(' ', self.visible_space_string.as_str())
				.replace('\t', self.visible_tab_string.as_str())
		}
		else {
			value.replace('\t', self.invisible_tab_string.as_str())
		}
		.replace('\n', "")
	}

	// safe slice, as it is only on the hash, which is hexadecimal
	#[allow(clippy::string_slice)]
	fn build_leading_summary(commit: &Commit, is_full_width: bool) -> ViewLine {
		let mut segments = vec![];
		if is_full_width {
			segments.push(LineSegment::new_with_color("Commit: ", DisplayColor::IndicatorColor));
		}
		let hash = String::from(commit.hash());
		segments.push(LineSegment::new(
			if is_full_width {
				hash
			}
			else {
				let max_index = hash.len().min(8);
				format!("{:8}", hash[0..max_index].to_owned())
			}
			.as_str(),
		));
		ViewLine::from(segments)
	}

	#[allow(clippy::unused_self)]
	pub(super) fn build_view_data_for_overview(
		&self,
		updater: &mut ViewDataUpdater<'_>,
		diff: &CommitDiff,
		is_full_width: bool,
	) {
		let commit = diff.commit();
		updater.push_leading_line(Self::build_leading_summary(commit, is_full_width));
		// TODO handle authored date
		updater.push_line(ViewLine::from(vec![
			LineSegment::new_with_color(
				if is_full_width { "Date: " } else { "D: " },
				DisplayColor::IndicatorColor,
			),
			LineSegment::new(commit.committed_date().format("%c %z").to_string().as_str()),
		]));

		if commit.author().is_some() {
			updater.push_line(ViewLine::from(vec![
				LineSegment::new_with_color(
					if is_full_width { "Author: " } else { "A: " },
					DisplayColor::IndicatorColor,
				),
				LineSegment::new(commit.author().to_string().as_str()),
			]));
		}

		if let Some(committer) = commit.committer().as_ref() {
			updater.push_line(ViewLine::from(vec![
				LineSegment::new_with_color(
					if is_full_width { "Committer: " } else { "C: " },
					DisplayColor::IndicatorColor,
				),
				LineSegment::new(committer.to_string().as_str()),
			]));
		}

		if let Some(summary) = commit.summary() {
			updater.push_lines(summary);
			updater.push_line(ViewLine::from(""));
		}

		if let Some(message) = commit.message() {
			updater.push_lines(message);
			updater.push_line(ViewLine::from(""));
		}

		if commit.summary().is_none() && commit.message().is_none() {
			updater.push_line(ViewLine::from(""));
		}

		updater.push_line(get_files_changed_summary(diff, is_full_width));
		for status in diff.file_statuses() {
			updater.push_line(ViewLine::from(get_stat_item_segments(
				status.status(),
				status.destination_path(),
				status.source_path(),
				is_full_width,
			)));
		}
	}

	fn build_diff_line_line_segment(content: &str, origin: Origin) -> LineSegment {
		LineSegment::new_with_color(content, match origin {
			Origin::Addition => DisplayColor::DiffAddColor,
			Origin::Deletion => DisplayColor::DiffRemoveColor,
			Origin::Context | Origin::Binary | Origin::Header => DisplayColor::DiffContextColor,
		})
	}

	// safe slice, only slices across graphemes whitespace
	#[allow(clippy::string_slice)]
	fn get_diff_line_segments(
		&self,
		diff_line: &DiffLine,
		old_largest_line_number_length: usize,
		new_largest_line_number_length: usize,
	) -> Vec<LineSegment> {
		let mut line_segments = vec![
			match diff_line.old_line_number() {
				Some(line_number) => {
					LineSegment::new(format!("{line_number:<old_largest_line_number_length$}").as_str())
				},
				None => LineSegment::new(" ".repeat(old_largest_line_number_length).as_str()),
			},
			LineSegment::new(" "),
			match diff_line.new_line_number() {
				Some(line_number) => {
					LineSegment::new(format!("{line_number:<new_largest_line_number_length$}").as_str())
				},
				None => LineSegment::new(" ".repeat(new_largest_line_number_length).as_str()),
			},
			LineSegment::new("| "),
		];

		if self.show_leading_whitespace || self.show_trailing_whitespace {
			let line = diff_line.line();
			let (leading, content, trailing) = if line.trim().is_empty() {
				(
					self.replace_whitespace(line, self.show_leading_whitespace || self.show_trailing_whitespace),
					String::new(),
					String::new(),
				)
			}
			else {
				let (start, end) = get_partition_index_on_whitespace_for_line(line);
				(
					self.replace_whitespace(&line[0..start], self.show_leading_whitespace),
					self.replace_whitespace(&line[start..end], false),
					self.replace_whitespace(&line[end..], self.show_trailing_whitespace),
				)
			};

			if !leading.is_empty() {
				line_segments.push(LineSegment::new_with_color(
					leading.as_str(),
					DisplayColor::DiffWhitespaceColor,
				));
			}
			if !content.is_empty() {
				line_segments.push(Self::build_diff_line_line_segment(content.as_str(), diff_line.origin()));
			}
			if !trailing.is_empty() {
				line_segments.push(LineSegment::new_with_color(
					trailing.as_str(),
					DisplayColor::DiffWhitespaceColor,
				));
			}
		}
		else {
			line_segments.push(Self::build_diff_line_line_segment(
				self.replace_whitespace(diff_line.line(), false).as_str(),
				diff_line.origin(),
			));
		}

		line_segments
	}

	pub(super) fn build_view_data_diff(
		&self,
		updater: &mut ViewDataUpdater<'_>,
		diff: &CommitDiff,
		is_full_width: bool,
	) {
		updater.push_leading_line(Self::build_leading_summary(diff.commit(), is_full_width));
		updater.push_leading_line(get_files_changed_summary(diff, is_full_width));
		updater.push_line(ViewLine::new_empty_line().set_padding(PADDING_CHARACTER));

		let file_statuses = diff.file_statuses();
		for (s_i, status) in file_statuses.iter().enumerate() {
			updater.push_line(ViewLine::from(get_stat_item_segments(
				status.status(),
				status.destination_path(),
				status.source_path(),
				true,
			)));

			let old_largest_line_number_length = status.last_old_line_number().to_string().len();
			let new_largest_line_number_length = status.last_new_line_number().to_string().len();
			for delta in status.deltas() {
				updater.push_line(ViewLine::new_empty_line());
				updater.push_line(ViewLine::from(vec![
					LineSegment::new_with_color_and_style("@@", DisplayColor::Normal, true, false, false),
					LineSegment::new_with_color(
						format!(
							" -{},{} +{},{} ",
							delta.old_lines_start(),
							delta.old_number_lines(),
							delta.new_lines_start(),
							delta.new_number_lines(),
						)
						.as_str(),
						DisplayColor::DiffContextColor,
					),
					LineSegment::new_with_color_and_style("@@", DisplayColor::Normal, true, false, false),
					LineSegment::new_with_color(
						format!(" {}", delta.context()).as_str(),
						DisplayColor::DiffContextColor,
					),
				]));
				updater.push_line(ViewLine::new_pinned(vec![]).set_padding_with_color_and_style(
					PADDING_CHARACTER,
					DisplayColor::Normal,
					true,
					false,
					false,
				));

				for line in delta.lines() {
					if line.end_of_file() && line.line() != "\n" {
						updater.push_line(ViewLine::from(vec![
							LineSegment::new(
								" ".repeat(old_largest_line_number_length + new_largest_line_number_length + 3)
									.as_str(),
							),
							LineSegment::new_with_color("\\ No newline at end of file", DisplayColor::DiffContextColor),
						]));
						continue;
					}

					updater.push_line(ViewLine::from(self.get_diff_line_segments(
						line,
						old_largest_line_number_length,
						new_largest_line_number_length,
					)));
				}
			}
			if s_i + 1 != file_statuses.len() {
				updater.push_line(ViewLine::new_empty_line().set_padding(PADDING_CHARACTER));
			}
		}
	}
}
