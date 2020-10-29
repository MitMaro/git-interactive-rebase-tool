mod commit;
mod delta;
mod diff_line;
mod file_stat;
mod file_stats_builder;
mod show_commit_state;
mod status;
mod user;
mod util;
mod view_builder;

use crate::config::diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting;
use crate::config::diff_show_whitespace_setting::DiffShowWhitespaceSetting;
use crate::config::Config;
use crate::constants::MINIMUM_FULL_WINDOW_WIDTH;
use crate::display::display_color::DisplayColor;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::process::util::handle_view_data_scroll;
use crate::show_commit::commit::{Commit, LoadCommitDiffOptions};
use crate::show_commit::show_commit_state::ShowCommitState;
use crate::show_commit::util::get_show_commit_help_lines;
use crate::show_commit::view_builder::{ViewBuilder, ViewBuilderOptions};
use crate::todo_file::TodoFile;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;

pub struct ShowCommit<'s> {
	commit: Option<Commit>,
	config: &'s Config,
	show_commit_help_lines: Vec<(String, String)>,
	state: ShowCommitState,
	view_builder: ViewBuilder,
	view_data: ViewData,
}

impl<'s> ProcessModule for ShowCommit<'s> {
	fn activate(&mut self, rebase_todo: &TodoFile, _: State) -> ProcessResult {
		// skip loading commit data if the currently loaded commit has not changed, this retains
		// position after returning to the list view or help
		if let Some(ref commit) = self.commit {
			if commit.get_hash() == rebase_todo.get_selected_line().get_hash() {
				return ProcessResult::new();
			}
		}
		self.view_data.reset();

		let new_commit = Commit::new_from_hash(rebase_todo.get_selected_line().get_hash(), LoadCommitDiffOptions {
			context_lines: self.config.git.diff_context,
			copies: self.config.git.diff_copies,
			ignore_whitespace: self.config.diff_ignore_whitespace == DiffIgnoreWhitespaceSetting::All,
			ignore_whitespace_change: self.config.diff_ignore_whitespace == DiffIgnoreWhitespaceSetting::Change,
			interhunk_lines: self.config.git.diff_interhunk_lines,
			rename_limit: self.config.git.diff_rename_limit,
			renames: self.config.git.diff_renames,
		});

		match new_commit {
			Ok(c) => {
				self.commit = Some(c);
				ProcessResult::new()
			},
			Err(e) => ProcessResult::new().error(e).state(State::List),
		}
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &TodoFile) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();
		let commit = self.commit.as_ref().unwrap(); // will only fail on programmer error
		if self.view_data.is_empty() {
			let is_full_width = view_width >= MINIMUM_FULL_WINDOW_WIDTH;

			self.view_data.push_leading_line(ViewLine::from(vec![
				LineSegment::new_with_color(
					if is_full_width { "Commit: " } else { "" },
					DisplayColor::IndicatorColor,
				),
				LineSegment::new(
					if is_full_width {
						commit.get_hash().to_string()
					}
					else {
						let hash = commit.get_hash();
						let max_index = hash.len().min(8);
						format!("{:8} ", hash[0..max_index].to_string())
					}
					.as_str(),
				),
			]));

			match self.state {
				ShowCommitState::Overview => {
					self.view_builder
						.build_view_data_for_overview(&mut self.view_data, commit, is_full_width);
				},
				ShowCommitState::Diff => {
					self.view_builder
						.build_view_data_diff(&mut self.view_data, commit, is_full_width)
				},
			}
		}
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(&mut self, input_handler: &InputHandler<'_>, _: &mut TodoFile, _: &View<'_>) -> ProcessResult {
		let input = input_handler.get_input(InputMode::ShowCommit);
		let mut result = ProcessResult::new().input(input);

		if handle_view_data_scroll(input, &mut self.view_data).is_none() {
			match input {
				Input::ShowDiff => {
					self.view_data.reset();
					self.state = match self.state {
						ShowCommitState::Overview => ShowCommitState::Diff,
						ShowCommitState::Diff => ShowCommitState::Overview,
					}
				},
				Input::Resize => {
					self.view_data.clear();
				},
				Input::Help => {},
				_ => {
					if self.state == ShowCommitState::Diff {
						self.view_data.reset();
						self.state = ShowCommitState::Overview;
					}
					else {
						self.view_data.clear();
						result = result.state(State::List);
					}
				},
			}
		}
		result
	}

	fn get_help_keybindings_descriptions(&self) -> Option<Vec<(String, String)>> {
		Some(self.show_commit_help_lines.clone())
	}
}

impl<'s> ShowCommit<'s> {
	pub(crate) fn new(config: &'s Config) -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		view_data.set_show_help(true);
		let view_builder_options = ViewBuilderOptions::new(
			config.diff_tab_width as usize,
			config.diff_tab_symbol.as_str(),
			config.diff_space_symbol.as_str(),
			config.diff_show_whitespace == DiffShowWhitespaceSetting::Both
				|| config.diff_show_whitespace == DiffShowWhitespaceSetting::Leading,
			config.diff_show_whitespace == DiffShowWhitespaceSetting::Both
				|| config.diff_show_whitespace == DiffShowWhitespaceSetting::Trailing,
		);
		Self {
			commit: None,
			config,
			show_commit_help_lines: get_show_commit_help_lines(&config.key_bindings),
			state: ShowCommitState::Overview,
			view_builder: ViewBuilder::new(view_builder_options),
			view_data,
		}
	}
}
