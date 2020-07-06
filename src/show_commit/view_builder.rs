use crate::config::key_bindings::KeyBindings;
use crate::display::display_color::DisplayColor;
use crate::input::utils::get_input_short_name;
use crate::show_commit::commit::Commit;
use crate::show_commit::util::{get_files_changed_summary, get_stat_item_segments};
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;

pub(super) struct ViewBuilder<'d> {
	key_bindings: &'d KeyBindings,
}

impl<'d> ViewBuilder<'d> {
	pub(crate) fn new(key_bindings: &'d KeyBindings) -> Self {
		Self { key_bindings }
	}

	fn get_overview_footer(&self, is_full_width: bool) -> String {
		if is_full_width {
			format!(
				" {}, {}, {}, {}, {}, {}, {}, Any other key to close",
				self.key_bindings.move_up,
				self.key_bindings.move_down,
				self.key_bindings.move_up_step,
				self.key_bindings.move_down_step,
				self.key_bindings.move_right,
				self.key_bindings.move_left,
				self.key_bindings.help,
			)
		}
		else {
			format!(
				" {}, {}, {}, {}, {}, {}, {}, Any to close",
				get_input_short_name(self.key_bindings.move_up.as_str()),
				get_input_short_name(self.key_bindings.move_down.as_str()),
				get_input_short_name(self.key_bindings.move_up_step.as_str()),
				get_input_short_name(self.key_bindings.move_down_step.as_str()),
				get_input_short_name(self.key_bindings.move_right.as_str()),
				get_input_short_name(self.key_bindings.move_left.as_str()),
				get_input_short_name(self.key_bindings.help.as_str()),
			)
		}
	}

	pub(super) fn build_view_data_for_overview(&self, view_data: &mut ViewData, commit: &Commit, is_full_width: bool) {
		view_data.push_line(ViewLine::new(vec![
			LineSegment::new_with_color(
				if is_full_width { "Date: " } else { "D: " },
				DisplayColor::IndicatorColor,
			),
			LineSegment::new(commit.get_date().format("%c %z").to_string().as_str()),
		]));

		if let Some(author) = commit.get_author().to_string() {
			view_data.push_line(ViewLine::new(vec![
				LineSegment::new_with_color(
					if is_full_width { "Author: " } else { "A: " },
					DisplayColor::IndicatorColor,
				),
				LineSegment::new(author.as_str()),
			]));
		}

		if let Some(committer) = commit.get_committer().to_string() {
			view_data.push_line(ViewLine::new(vec![
				LineSegment::new_with_color(
					if is_full_width { "Committer: " } else { "C: " },
					DisplayColor::IndicatorColor,
				),
				LineSegment::new(committer.as_str()),
			]));
		}

		if let Some(body) = commit.get_body() {
			for line in body.lines() {
				view_data.push_line(ViewLine::new(vec![LineSegment::new(line)]));
			}
		}

		view_data.push_line(ViewLine::new(vec![LineSegment::new("")]));

		view_data.push_line(get_files_changed_summary(commit, is_full_width));
		for stat in commit.get_file_stats() {
			view_data.push_line(ViewLine::new(get_stat_item_segments(
				stat.get_status(),
				stat.get_to_name().as_str(),
				stat.get_from_name().as_str(),
				is_full_width,
			)));
		}

		view_data.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new(
			self.get_overview_footer(is_full_width).as_str(),
		)]));
	}
}
