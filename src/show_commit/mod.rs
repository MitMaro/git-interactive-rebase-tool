mod commit;
mod delta;
mod diff_line;
mod file_stat;
mod file_stats_builder;
mod origin;
mod show_commit_state;
mod status;
mod user;
mod util;
mod view_builder;

#[cfg(test)]
mod tests;

use anyhow::anyhow;

use crate::{
	components::Help,
	config::{
		diff_ignore_whitespace_setting::DiffIgnoreWhitespaceSetting,
		diff_show_whitespace_setting::DiffShowWhitespaceSetting,
		Config,
	},
	display::display_color::DisplayColor,
	input::{input_handler::InputMode, Input},
	process::{
		process_module::ProcessModule,
		process_result::ProcessResult,
		state::State,
		util::handle_view_data_scroll,
	},
	show_commit::{
		commit::{Commit, LoadCommitDiffOptions},
		show_commit_state::ShowCommitState,
		util::get_show_commit_help_lines,
		view_builder::{ViewBuilder, ViewBuilderOptions},
	},
	todo_file::TodoFile,
	view::{line_segment::LineSegment, render_context::RenderContext, view_data::ViewData, view_line::ViewLine, View},
};

pub struct ShowCommit<'s> {
	commit: Option<Commit>,
	config: &'s Config,
	help: Help,
	state: ShowCommitState,
	view_builder: ViewBuilder,
	view_data: ViewData,
}

impl<'s> ProcessModule for ShowCommit<'s> {
	fn activate(&mut self, rebase_todo: &TodoFile, _: State) -> ProcessResult {
		if let Some(selected_line) = rebase_todo.get_selected_line() {
			// skip loading commit data if the currently loaded commit has not changed, this retains
			// position after returning to the list view or help
			if let Some(ref commit) = self.commit {
				if commit.get_hash() == selected_line.get_hash() {
					return ProcessResult::new();
				}
			}
			self.view_data.reset();

			let new_commit = Commit::new_from_hash(selected_line.get_hash(), LoadCommitDiffOptions {
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
		else {
			ProcessResult::new()
				.error(anyhow!("No valid commit to show"))
				.state(State::List)
		}
	}

	fn build_view_data(&mut self, context: &RenderContext, _: &TodoFile) -> &mut ViewData {
		if self.help.is_active() {
			return self.help.get_view_data();
		}

		if self.view_data.is_empty() {
			let commit = self.commit.as_ref().unwrap(); // will only fail on programmer error
			let is_full_width = context.is_full_width();

			self.view_data.push_leading_line(ViewLine::from(vec![
				LineSegment::new_with_color(
					if is_full_width { "Commit: " } else { "" },
					DisplayColor::IndicatorColor,
				),
				LineSegment::new(
					if is_full_width {
						commit.get_hash().to_owned()
					}
					else {
						let hash = commit.get_hash();
						let max_index = hash.len().min(8);
						format!("{:8}", hash[0..max_index].to_owned())
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
						.build_view_data_diff(&mut self.view_data, commit, is_full_width);
				},
			}
		}
		&mut self.view_data
	}

	fn handle_input(&mut self, view: &mut View<'_>, _: &mut TodoFile) -> ProcessResult {
		if self.help.is_active() {
			let input = view.get_input(InputMode::Default);
			self.help.handle_input(input);
			return ProcessResult::new().input(input);
		}

		let input = view.get_input(InputMode::ShowCommit);
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
				Input::Help => {
					self.help.set_active();
				},
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
			help: Help::new_from_keybindings(&get_show_commit_help_lines(&config.key_bindings)),
			state: ShowCommitState::Overview,
			view_builder: ViewBuilder::new(view_builder_options),
			view_data,
		}
	}
}
