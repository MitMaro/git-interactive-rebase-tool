mod commit;
mod file_stat;
mod status;
mod user;
mod util;

use crate::constants::MINIMUM_FULL_WINDOW_WIDTH;
use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::process_result::{ProcessResult, ProcessResultBuilder};
use crate::process::state::State;
use crate::show_commit::commit::Commit;
use crate::show_commit::util::get_stat_item_segments;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;

pub(crate) struct ShowCommit {
	commit: Option<Result<Commit, String>>,
	view_data: ViewData,
	no_commit_view_data: ViewData,
}

impl ProcessModule for ShowCommit {
	fn activate(&mut self, _: State, git_interactive: &GitInteractive) {
		self.commit = Some(Commit::from_commit_hash(
			git_interactive.get_selected_line_hash().as_str(),
		));
	}

	fn deactivate(&mut self) {
		self.view_data.reset();
	}

	fn process(&mut self, _git_interactive: &mut GitInteractive, _: &View) -> ProcessResult {
		let mut result = ProcessResultBuilder::new();

		if let Some(commit) = &self.commit {
			if let Err(e) = commit {
				result = result.error(e.as_str(), State::List(false));
			}
		}

		result.build()
	}

	fn handle_input(&mut self, input_handler: &InputHandler, _: &mut GitInteractive, view: &View) -> HandleInputResult {
		let input = input_handler.get_input(InputMode::Default);
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorLeft => self.view_data.scroll_left(),
			Input::MoveCursorRight => self.view_data.scroll_right(),
			Input::MoveCursorDown => self.view_data.scroll_down(),
			Input::MoveCursorUp => self.view_data.scroll_up(),
			Input::MoveCursorPageDown => self.view_data.page_down(),
			Input::MoveCursorPageUp => self.view_data.page_up(),
			Input::Resize => {
				let (view_width, view_height) = view.get_view_size();
				self.view_data.set_view_size(view_width, view_height);
			},
			_ => {
				result = result.state(State::List(false));
			},
		}
		result.build()
	}

	fn render(&self, _view: &View, _git_interactive: &GitInteractive) {}
}

impl ShowCommit {
	pub(crate) fn new() -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		view_data.set_show_help(false);

		Self {
			commit: None,
			view_data,
			no_commit_view_data: ViewData::new_error("Not commit data to show"),
		}
	}

	pub(crate) fn build_view_data(&mut self, view: &View, _: &GitInteractive) -> &ViewData {
		let (window_width, window_height) = view.get_view_size();

		match &self.commit {
			Some(commit) => {
				self.view_data.clear();
				self.view_data.set_view_size(window_width, window_height);
				let commit = commit.as_ref().unwrap(); // if commit is error it will be caught in process
				let is_full_width = window_width >= MINIMUM_FULL_WINDOW_WIDTH;
				let full_hash = commit.get_hash();
				let author = commit.get_author();
				let committer = commit.get_committer();
				let date = commit.get_date();
				let body = commit.get_body();
				let file_stats = commit.get_file_stats();

				self.view_data.push_leading_line(ViewLine::new(vec![
					LineSegment::new_with_color(
						if is_full_width { "Commit: " } else { "" },
						DisplayColor::IndicatorColor,
					),
					LineSegment::new(
						if is_full_width {
							full_hash.clone()
						}
						else {
							let max_index = full_hash.len().min(8);
							format!("{:8} ", full_hash[0..max_index].to_string())
						}
						.as_str(),
					),
				]));

				self.view_data.push_line(ViewLine::new(vec![
					LineSegment::new_with_color(
						if is_full_width { "Date: " } else { "D: " },
						DisplayColor::IndicatorColor,
					),
					LineSegment::new(date.format("%c %z").to_string().as_str()),
				]));

				if let Some(author) = author.to_string() {
					self.view_data.push_line(ViewLine::new(vec![
						LineSegment::new_with_color(
							if is_full_width { "Author: " } else { "A: " },
							DisplayColor::IndicatorColor,
						),
						LineSegment::new(author.as_str()),
					]));
				}

				if let Some(committer) = committer.to_string() {
					self.view_data.push_line(ViewLine::new(vec![
						LineSegment::new_with_color(
							if is_full_width { "Committer: " } else { "C: " },
							DisplayColor::IndicatorColor,
						),
						LineSegment::new(committer.as_str()),
					]));
				}

				self.view_data.push_line(ViewLine::new(vec![LineSegment::new("")]));

				if let Some(body) = body {
					for line in body.lines() {
						self.view_data.push_line(ViewLine::new(vec![LineSegment::new(line)]));
					}
				}

				if let Some(file_stats) = file_stats {
					for stat in file_stats {
						self.view_data.push_line(ViewLine::new(get_stat_item_segments(
							stat.get_status(),
							stat.get_to_name().as_str(),
							stat.get_from_name().as_str(),
							is_full_width,
						)));
					}
				}

				self.view_data
					.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new("Any key to close")]));

				self.view_data.rebuild();
				&self.view_data
			},
			None => &self.no_commit_view_data,
		}
	}
}
