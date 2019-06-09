use crate::constants::MINIMUM_FULL_WINDOW_WIDTH;
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
use crate::window::WindowColor;
use std::cmp;

pub struct ShowCommit {
	scroll_position: ScrollPosition,
}

impl ProcessModule for ShowCommit {
	fn activate(&mut self, _state: State, _git_interactive: &GitInteractive) {
		self.scroll_position.reset();
	}

	fn process(&mut self, git_interactive: &mut GitInteractive) -> ProcessResult {
		let mut result = ProcessResultBuilder::new();
		if let Err(e) = git_interactive.load_commit_stats() {
			result = result.error(e.as_str(), State::List);
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
			WindowColor::IndicatorColor,
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

		view.draw_view_lines(lines, self.scroll_position.get_position(), view_height);

		view.set_color(WindowColor::IndicatorColor);
		view.draw_str("Any key to close");
	}
}

impl ShowCommit {
	pub fn new() -> Self {
		Self {
			scroll_position: ScrollPosition::new(3, 6, 3),
		}
	}

	pub fn handle_input_with_view(
		&mut self,
		input_handler: &InputHandler,
		git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let (_, window_height) = view.get_view_size();

		let input = input_handler.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorDown => {
				self.scroll_position
					.scroll_down(window_height, git_interactive.get_commit_stats_length())
			},
			Input::MoveCursorUp => {
				self.scroll_position
					.scroll_up(window_height, git_interactive.get_commit_stats_length())
			},
			Input::Resize => {
				self.scroll_position
					.scroll_up(window_height as usize, git_interactive.get_commit_stats_length());
			},
			_ => {
				result = result.state(State::List);
			},
		}
		result.build()
	}
}
