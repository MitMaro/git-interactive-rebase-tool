use super::commit::Commit;
use super::diff_line::DiffLine;
use super::origin::Origin;
use super::util::{get_files_changed_summary, get_partition_index_on_whitespace_for_line, get_stat_item_segments};
use crate::display::display_color::DisplayColor;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;

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

	fn replace_whitespace(&self, s: &str, visible: bool) -> String {
		let s = if visible {
			s.replace(" ", self.visible_space_string.as_str())
				.replace("\t", self.visible_tab_string.as_str())
		}
		else {
			s.replace("\t", self.invisible_tab_string.as_str())
		};
		s.replace("\n", "")
	}

	#[allow(clippy::unused_self)]
	pub(super) fn build_view_data_for_overview(&self, view_data: &mut ViewData, commit: &Commit, is_full_width: bool) {
		view_data.push_line(ViewLine::from(vec![
			LineSegment::new_with_color(
				if is_full_width { "Date: " } else { "D: " },
				DisplayColor::IndicatorColor,
			),
			LineSegment::new(commit.get_date().format("%c %z").to_string().as_str()),
		]));

		if let Some(author) = commit.get_author().to_string() {
			view_data.push_line(ViewLine::from(vec![
				LineSegment::new_with_color(
					if is_full_width { "Author: " } else { "A: " },
					DisplayColor::IndicatorColor,
				),
				LineSegment::new(author.as_str()),
			]));
		}

		if let Some(committer) = commit.get_committer().to_string() {
			view_data.push_line(ViewLine::from(vec![
				LineSegment::new_with_color(
					if is_full_width { "Committer: " } else { "C: " },
					DisplayColor::IndicatorColor,
				),
				LineSegment::new(committer.as_str()),
			]));
		}

		if let Some(ref body) = *commit.get_body() {
			for line in body.lines() {
				view_data.push_line(ViewLine::from(line));
			}
		}

		view_data.push_line(ViewLine::from(""));

		view_data.push_line(get_files_changed_summary(commit, is_full_width));
		for stat in commit.get_file_stats() {
			view_data.push_line(ViewLine::from(get_stat_item_segments(
				stat.get_status(),
				stat.get_to_name(),
				stat.get_from_name(),
				is_full_width,
			)));
		}
	}

	fn get_diff_line_segments(
		&self,
		diff_line: &DiffLine,
		old_largest_line_number_length: usize,
		new_largest_line_number_length: usize,
	) -> Vec<LineSegment> {
		let mut line_segments = vec![];
		line_segments.push(match diff_line.old_line_number() {
			Some(line_number) => {
				LineSegment::new(format!("{:<width$}", line_number, width = old_largest_line_number_length).as_str())
			},
			None => LineSegment::new(" ".repeat(old_largest_line_number_length).as_str()),
		});
		line_segments.push(LineSegment::new(" "));

		line_segments.push(match diff_line.new_line_number() {
			Some(line_number) => {
				LineSegment::new(format!("{:<width$}", line_number, width = new_largest_line_number_length).as_str())
			},
			None => LineSegment::new(" ".repeat(new_largest_line_number_length).as_str()),
		});
		line_segments.push(LineSegment::new("| "));

		if self.show_leading_whitespace || self.show_trailing_whitespace {
			let line = diff_line.line();
			let (leading, content, trailing) = if line.trim().is_empty() {
				(
					self.replace_whitespace(line, self.show_leading_whitespace || self.show_trailing_whitespace),
					String::from(""),
					String::from(""),
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

			line_segments.push(LineSegment::new_with_color(
				leading.as_str(),
				DisplayColor::DiffWhitespaceColor,
			));
			line_segments.push(LineSegment::new_with_color(
				content.as_str(),
				match *diff_line.origin() {
					Origin::Addition => DisplayColor::DiffAddColor,
					Origin::Deletion => DisplayColor::DiffRemoveColor,
					Origin::Context => DisplayColor::DiffContextColor,
				},
			));
			line_segments.push(LineSegment::new_with_color(
				trailing.as_str(),
				DisplayColor::DiffWhitespaceColor,
			));
		}
		else {
			line_segments.push(LineSegment::new_with_color(
				self.replace_whitespace(diff_line.line(), false).as_str(),
				match *diff_line.origin() {
					Origin::Addition => DisplayColor::DiffAddColor,
					Origin::Deletion => DisplayColor::DiffRemoveColor,
					Origin::Context => DisplayColor::DiffContextColor,
				},
			));
		}

		line_segments
	}

	pub(super) fn build_view_data_diff(&self, view_data: &mut ViewData, commit: &Commit, is_full_width: bool) {
		view_data.push_leading_line(get_files_changed_summary(commit, is_full_width));
		view_data.push_line(ViewLine::new_empty_line().set_padding_character("―"));

		let file_stats = commit.get_file_stats();
		for (s_i, stat) in file_stats.iter().enumerate() {
			view_data.push_line(ViewLine::from(get_stat_item_segments(
				stat.get_status(),
				stat.get_to_name(),
				stat.get_from_name(),
				true,
			)));

			let old_largest_line_number_length = stat.largest_old_line_number().to_string().len();
			let new_largest_line_number_length = stat.largest_new_line_number().to_string().len();
			for delta in stat.deltas() {
				view_data.push_line(ViewLine::new_empty_line());
				view_data.push_line(ViewLine::from(vec![
					LineSegment::new_with_color_and_style("@@", DisplayColor::Normal, true, false, false),
					LineSegment::new_with_color(
						format!(
							" -{},{} +{},{} ",
							delta.old_start(),
							delta.old_lines(),
							delta.new_start(),
							delta.new_lines(),
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
				view_data.push_line(
					ViewLine::new_pinned(vec![])
						.set_padding_color_and_style(DisplayColor::Normal, true, false, false)
						.set_padding_character("┈"),
				);

				for line in delta.lines() {
					if line.end_of_file() && line.line() != "\n" {
						view_data.push_line(ViewLine::from(vec![
							LineSegment::new(
								" ".repeat(old_largest_line_number_length + new_largest_line_number_length + 3)
									.as_str(),
							),
							LineSegment::new_with_color("\\ No newline at end of file", DisplayColor::DiffContextColor),
						]));
						continue;
					}

					view_data.push_line(ViewLine::from(self.get_diff_line_segments(
						line,
						old_largest_line_number_length,
						new_largest_line_number_length,
					)));
				}
			}
			if s_i + 1 != file_stats.len() {
				view_data.push_line(ViewLine::new_empty_line().set_padding_character("―"));
			}
		}
	}
}
