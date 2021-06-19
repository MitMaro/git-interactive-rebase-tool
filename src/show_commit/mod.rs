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
use lazy_static::lazy_static;

use crate::{
	components::help::Help,
	config::{Config, DiffIgnoreWhitespaceSetting, DiffShowWhitespaceSetting},
	input::{Event, EventHandler, InputOptions, MetaEvent},
	process::{Module, ProcessResult, State},
	show_commit::{
		commit::{Commit, LoadCommitDiffOptions},
		show_commit_state::ShowCommitState,
		util::get_show_commit_help_lines,
		view_builder::{ViewBuilder, ViewBuilderOptions},
	},
	todo_file::TodoFile,
	view::{handle_view_data_scroll, RenderContext, ViewData, ViewSender},
};

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::new()
		.movement(true)
		.undo_redo(true)
		.help(true)
		.resize(false);
}

pub struct ShowCommit {
	commit: Option<Commit>,
	diff_view_data: ViewData,
	help: Help,
	load_commit_diff_options: LoadCommitDiffOptions,
	overview_view_data: ViewData,
	state: ShowCommitState,
	view_builder: ViewBuilder,
}

impl Module for ShowCommit {
	fn activate(&mut self, rebase_todo: &TodoFile, _: State) -> ProcessResult {
		if let Some(selected_line) = rebase_todo.get_selected_line() {
			// skip loading commit data if the currently loaded commit has not changed, this retains
			// position after returning to the list view or help
			if let Some(ref commit) = self.commit {
				if commit.get_hash() == selected_line.get_hash() {
					return ProcessResult::new();
				}
			}
			self.overview_view_data.update_view_data(|updater| {
				updater.clear();
				updater.reset_scroll_position();
			});

			self.diff_view_data.update_view_data(|updater| {
				updater.clear();
				updater.reset_scroll_position();
			});

			let new_commit = Commit::new_from_hash(selected_line.get_hash(), &self.load_commit_diff_options);

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

	fn build_view_data(&mut self, context: &RenderContext, _: &TodoFile) -> &ViewData {
		if self.help.is_active() {
			return self.help.get_view_data();
		}

		let commit = self.commit.as_ref().unwrap(); // will only fail on programmer error
		let state = &self.state;
		let view_builder = &self.view_builder;
		let is_full_width = context.is_full_width();

		match *state {
			ShowCommitState::Overview => {
				if self.overview_view_data.is_empty() {
					self.overview_view_data.update_view_data(|updater| {
						view_builder.build_view_data_for_overview(updater, commit, is_full_width);
					});
				}
				&self.overview_view_data
			},
			ShowCommitState::Diff => {
				if self.diff_view_data.is_empty() {
					self.diff_view_data.update_view_data(|updater| {
						view_builder.build_view_data_diff(updater, commit, is_full_width);
					});
				}
				&self.diff_view_data
			},
		}
	}

	fn handle_events(
		&mut self,
		event_handler: &EventHandler,
		view_sender: &ViewSender,
		_: &mut TodoFile,
	) -> ProcessResult {
		if self.help.is_active() {
			return ProcessResult::from(self.help.handle_event(event_handler, view_sender));
		}

		let event = event_handler.read_event(&INPUT_OPTIONS, |event, key_bindings| {
			if key_bindings.show_diff.contains(&event) {
				Event::from(MetaEvent::ShowDiff)
			}
			else {
				event
			}
		});

		let mut result = ProcessResult::from(event);

		let active_view_data = match self.state {
			ShowCommitState::Overview => &mut self.overview_view_data,
			ShowCommitState::Diff => &mut self.diff_view_data,
		};

		if handle_view_data_scroll(event, view_sender).is_none() {
			match event {
				Event::Meta(meta_event) if meta_event == MetaEvent::ShowDiff => {
					active_view_data.update_view_data(|updater| updater.clear());
					self.state = match self.state {
						ShowCommitState::Overview => ShowCommitState::Diff,
						ShowCommitState::Diff => ShowCommitState::Overview,
					}
				},
				Event::Meta(meta_event) if meta_event == MetaEvent::Help => self.help.set_active(),
				Event::Key(_) => {
					active_view_data.update_view_data(|updater| updater.clear());
					if self.state == ShowCommitState::Diff {
						self.state = ShowCommitState::Overview;
					}
					else {
						result = result.state(State::List);
					}
				},
				Event::Resize(..) => active_view_data.update_view_data(|updater| updater.clear()),
				_ => {},
			}
		}
		result
	}
}

impl ShowCommit {
	pub(crate) fn new(config: &Config) -> Self {
		let overview_view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		});
		let diff_view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		});
		let view_builder_options = ViewBuilderOptions::new(
			config.diff_tab_width as usize,
			config.diff_tab_symbol.as_str(),
			config.diff_space_symbol.as_str(),
			config.diff_show_whitespace == DiffShowWhitespaceSetting::Both
				|| config.diff_show_whitespace == DiffShowWhitespaceSetting::Leading,
			config.diff_show_whitespace == DiffShowWhitespaceSetting::Both
				|| config.diff_show_whitespace == DiffShowWhitespaceSetting::Trailing,
		);
		let load_commit_diff_options = LoadCommitDiffOptions {
			context_lines: config.git.diff_context,
			copies: config.git.diff_copies,
			ignore_whitespace: config.diff_ignore_whitespace == DiffIgnoreWhitespaceSetting::All,
			ignore_whitespace_change: config.diff_ignore_whitespace == DiffIgnoreWhitespaceSetting::Change,
			interhunk_lines: config.git.diff_interhunk_lines,
			rename_limit: config.git.diff_rename_limit,
			renames: config.git.diff_renames,
		};
		Self {
			commit: None,
			diff_view_data,
			help: Help::new_from_keybindings(&get_show_commit_help_lines(&config.key_bindings)),
			load_commit_diff_options,
			overview_view_data,
			state: ShowCommitState::Overview,
			view_builder: ViewBuilder::new(view_builder_options),
		}
	}
}
