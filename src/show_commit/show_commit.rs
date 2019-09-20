use crate::commit::Commit;
use crate::constants::MINIMUM_FULL_WINDOW_WIDTH;
use crate::display::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::{Input, InputHandler};
use crate::process::{
	HandleInputResult,
	HandleInputResultBuilder,
	ProcessModule,
	ProcessResult,
	ProcessResultBuilder,
	State,
};
use crate::scroll::ScrollPosition;
use crate::show_commit::util::get_stat_item_segments;
use crate::view::{LineSegment, View, ViewLine};
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct ShowCommit {
	scroll_position: ScrollPosition,
}

impl ProcessModule for ShowCommit {
	fn activate(&mut self, _state: State, _git_interactive: &GitInteractive) {
		self.scroll_position.reset();
	}

	fn process(&mut self, git_interactive: &mut GitInteractive, _view: &View) -> ProcessResult {
		let mut result = ProcessResultBuilder::new();
		if let Err(e) = git_interactive.load_commit_stats() {
			result = result.error(e.as_str(), State::List(false));
		}
		result.build()
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler,
		git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let (view_width, view_height) = view.get_view_size();

		let input = input_handler.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorLeft => {
				self.scroll_position.scroll_left(
					view_width,
					self.get_max_line_length(
						git_interactive.get_commit_stats(),
						view_height >= MINIMUM_FULL_WINDOW_WIDTH,
					),
				)
			},
			Input::MoveCursorRight => {
				self.scroll_position.scroll_right(
					view_width,
					self.get_max_line_length(
						git_interactive.get_commit_stats(),
						view_height >= MINIMUM_FULL_WINDOW_WIDTH,
					),
				)
			},
			Input::MoveCursorDown => {
				self.scroll_position.scroll_down(
					view_height,
					self.get_commit_stats_length(git_interactive.get_commit_stats()),
				)
			},
			Input::MoveCursorUp => {
				self.scroll_position.scroll_up(
					view_height,
					self.get_commit_stats_length(git_interactive.get_commit_stats()),
				)
			},
			Input::Resize => {
				self.scroll_position.scroll_up(
					view_height as usize,
					self.get_commit_stats_length(git_interactive.get_commit_stats()),
				);
			},
			_ => {
				result = result.state(State::List(false));
			},
		}
		result.build()
	}

	fn render(&self, view: &View, git_interactive: &GitInteractive) {
		let commit_data = git_interactive.get_commit_stats();
		let (window_width, window_height) = view.get_view_size();
		let view_height = window_height - 2;

		let is_full_width = window_width >= MINIMUM_FULL_WINDOW_WIDTH;

		view.draw_title(false);

		let commit = match commit_data {
			None => {
				view.draw_error("Not commit data to show");
				return;
			},
			Some(c) => c,
		};

		let full_hash = commit.get_hash();
		let author = commit.get_author();
		let committer = commit.get_committer();
		let date = commit.get_date();
		let body = commit.get_body();
		let file_stats = commit.get_file_stats();

		let mut lines: Vec<ViewLine> = vec![];

		lines.push(ViewLine::new(vec![LineSegment::new_with_color(
			if is_full_width {
				format!("Commit: {}", full_hash)
			}
			else {
				let max_index = cmp::min(full_hash.len(), 8);
				format!("{:8} ", full_hash[0..max_index].to_string())
			}
			.as_str(),
			DisplayColor::IndicatorColor,
		)]));

		lines.push(ViewLine::new(vec![LineSegment::new(
			if is_full_width {
				format!("Date: {}", date.format("%c %z"))
			}
			else {
				format!("{}", date.format("%c %z"))
			}
			.as_str(),
		)]));

		if let Some(a) = author.to_string() {
			lines.push(ViewLine::new(vec![LineSegment::new(
				if is_full_width {
					format!("Author: {}", a)
				}
				else {
					format!("A: {}", a)
				}
				.as_str(),
			)]));
		}

		if let Some(c) = committer.to_string() {
			lines.push(ViewLine::new(vec![LineSegment::new(
				if is_full_width {
					format!("Committer: {}", c)
				}
				else {
					format!("C: {}", c)
				}
				.as_str(),
			)]))
		};

		match body {
			Some(b) => {
				for line in b.lines() {
					lines.push(ViewLine::new(vec![LineSegment::new(line)]));
				}
			},
			None => {},
		};

		lines.push(ViewLine::new(vec![LineSegment::new("")]));

		match file_stats {
			Some(stats) => {
				for stat in stats {
					lines.push(ViewLine::new(get_stat_item_segments(
						*stat.get_status(),
						stat.get_to_name().as_str(),
						stat.get_from_name().as_str(),
						is_full_width,
					)))
				}
			},
			None => {},
		}

		view.draw_view_lines(
			lines,
			self.scroll_position.get_top_position(),
			self.scroll_position.get_left_position(),
			view_height,
		);

		view.set_color(DisplayColor::IndicatorColor, false);
		view.draw_str("Any key to close");
	}
}

impl ShowCommit {
	pub fn new() -> Self {
		Self {
			scroll_position: ScrollPosition::new(3, 6, 3),
		}
	}

	fn get_commit_stats_length(&self, commit: &Option<Commit>) -> usize {
		match commit {
			Some(c) => {
				let mut len = c.get_file_stats_length();

				match c.get_body() {
					Some(b) => {
						len += b.lines().count();
					},
					None => {},
				}
				len + 3 // author + date + commit hash
			},
			None => 0,
		}
	}

	fn get_max_line_length(&self, commit: &Option<Commit>, is_full_width: bool) -> usize {
		match commit {
			Some(c) => {
				let full_hash = c.get_hash();
				let author = c.get_author();
				let committer = c.get_committer();
				let body = c.get_body();
				let file_stats = c.get_file_stats();

				let mut max_line_length = if is_full_width {
					full_hash.len() + 8 // 8 = "Commit: "
				}
				else {
					cmp::min(full_hash.len(), 8)
				};

				max_line_length = cmp::max(
					if is_full_width {
						35 // "Date: Sun Jul 8 00:34:60 2001+09:30"
					}
					else {
						29 // "Sun Jul 8 00:34:60 2001+09:30"
					},
					max_line_length,
				);

				if let Some(a) = author.to_string() {
					max_line_length = cmp::max(
						if is_full_width {
							UnicodeSegmentation::graphemes(a.as_str(), true).count() + 8 // 8 = "Author: "
						}
						else {
							UnicodeSegmentation::graphemes(a.as_str(), true).count() + 3 // 3 = "A: "
						},
						max_line_length,
					);
				}

				if let Some(c) = committer.to_string() {
					max_line_length = cmp::max(
						if is_full_width {
							UnicodeSegmentation::graphemes(c.as_str(), true).count() + 11 // 11 = "Committer: "
						}
						else {
							UnicodeSegmentation::graphemes(c.as_str(), true).count() + 3 // 3 = "C: "
						},
						max_line_length,
					);
				};

				if let Some(b) = body {
					for line in b.lines() {
						let line_length = UnicodeSegmentation::graphemes(line, true).count();
						if line_length > max_line_length {
							max_line_length = line_length;
						}
					}
				}

				if let Some(stats) = file_stats {
					let additional_line_length = if is_full_width {
						13 // stat name + arrow
					}
					else {
						3 // stat name + arrow
					};

					for stat in stats {
						let stat_line_length =
							UnicodeSegmentation::graphemes(stat.get_to_name().as_str(), true).count()
								+ UnicodeSegmentation::graphemes(stat.get_from_name().as_str(), true).count()
								+ additional_line_length;

						if stat_line_length > max_line_length {
							max_line_length = stat_line_length;
						}
					}
				}

				max_line_length
			},
			None => 0,
		}
	}
}
