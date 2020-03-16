mod data;
mod util;

use crate::commit::Commit;
use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::process_result::{ProcessResult, ProcessResultBuilder};
use crate::process::state::State;
use crate::scroll::scroll_position::ScrollPosition;
use crate::show_commit::data::Data;
use crate::view::View;

pub(crate) struct ShowCommit {
	commit: Option<Result<Commit, String>>,
	data: Data,
	scroll_position: ScrollPosition,
}

impl ProcessModule for ShowCommit {
	fn activate(&mut self, _state: State, git_interactive: &GitInteractive) {
		self.scroll_position.reset();
		self.commit = Some(git_interactive.load_commit_stats());
	}

	fn deactivate(&mut self) {
		self.data.reset();
	}

	fn process(&mut self, _git_interactive: &mut GitInteractive, view: &View) -> ProcessResult {
		let (view_width, view_height) = view.get_view_size();
		let mut result = ProcessResultBuilder::new();

		if let Some(commit) = &self.commit {
			match commit {
				Ok(c) => self.data.update(&c, view_width, view_height),
				Err(e) => {
					result = result.error(e.as_str(), State::List(false));
					self.data.reset()
				},
			}
		}

		result.build()
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let (view_width, view_height) = view.get_view_size();

		let input = input_handler.get_input(InputMode::Default);
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::MoveCursorLeft => {
				self.scroll_position
					.scroll_left(view_width, self.get_max_line_length(view_height))
			},
			Input::MoveCursorRight => {
				self.scroll_position
					.scroll_right(view_width, self.get_max_line_length(view_height))
			},
			Input::MoveCursorDown => {
				self.scroll_position
					.scroll_down(view_height, self.get_commit_stats_length())
			},
			Input::MoveCursorUp => {
				self.scroll_position
					.scroll_up(view_height, self.get_commit_stats_length())
			},
			Input::MoveCursorPageDown => {
				self.scroll_position
					.page_down(view_height, self.get_commit_stats_length())
			},
			Input::MoveCursorPageUp => {
				self.scroll_position
					.page_up(view_height, self.get_commit_stats_length())
			},
			Input::Resize => {
				self.scroll_position
					.scroll_up(view_height, self.get_commit_stats_length());
			},
			_ => {
				result = result.state(State::List(false));
			},
		}
		result.build()
	}

	fn render(&self, view: &View, _git_interactive: &GitInteractive) {
		let (_, window_height) = view.get_view_size();
		let view_height = window_height - 2;

		view.draw_title(false);

		match &self.commit {
			None => {
				view.draw_error("Not commit data to show");
				return;
			},
			Some(c) => c.as_ref().unwrap(), // safe unwrap
		};

		view.draw_view_lines(
			self.data.get_lines(),
			self.scroll_position.get_top_position(),
			self.scroll_position.get_left_position(),
			view_height,
		);

		view.set_color(DisplayColor::IndicatorColor, false);
		view.draw_str("Any key to close");
	}
}

impl ShowCommit {
	pub(crate) fn new() -> Self {
		Self {
			commit: None,
			data: Data::new(),
			scroll_position: ScrollPosition::new(3),
		}
	}

	fn get_commit_stats_length(&self) -> usize {
		if let Some(commit) = &self.commit {
			if let Ok(c) = commit {
				let mut len = c.get_file_stats_length();

				match c.get_body() {
					Some(b) => {
						len += b.lines().count();
					},
					None => {},
				}
				return len + 3; // author + date + commit hash
			}
		}
		0
	}

	fn get_max_line_length(&self, view_height: usize) -> usize {
		self.data.get_max_line_length(
			self.scroll_position.get_top_position(),
			self.scroll_position.get_top_position() + view_height,
		)
	}
}
