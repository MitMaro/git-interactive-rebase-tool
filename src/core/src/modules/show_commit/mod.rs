mod show_commit_state;
mod util;
mod view_builder;

#[cfg(test)]
mod tests;

use anyhow::{anyhow, Error};
use captur::capture;
use config::{Config, DiffIgnoreWhitespaceSetting, DiffShowWhitespaceSetting};
use git::{CommitDiff, CommitDiffLoaderOptions, Repository};
use input::{InputOptions, StandardEvent};
use todo_file::TodoFile;
use view::{RenderContext, ViewData};

use self::{
	show_commit_state::ShowCommitState,
	util::get_show_commit_help_lines,
	view_builder::{ViewBuilder, ViewBuilderOptions},
};
use crate::{
	components::help::Help,
	events::{Event, KeyBindings, MetaEvent},
	module::{Module, State},
	process::Results,
	select,
	util::handle_view_data_scroll,
};

// TODO Remove `union` call when bitflags/bitflags#180 is resolved
const INPUT_OPTIONS: InputOptions = InputOptions::UNDO_REDO
	.union(InputOptions::MOVEMENT)
	.union(InputOptions::HELP);

pub(crate) struct ShowCommit {
	commit_diff_loader_options: CommitDiffLoaderOptions,
	diff: Option<CommitDiff>,
	diff_view_data: ViewData,
	help: Help,
	overview_view_data: ViewData,
	repository: Repository,
	state: ShowCommitState,
	view_builder: ViewBuilder,
}

impl Module for ShowCommit {
	fn activate(&mut self, rebase_todo: &TodoFile, _: State) -> Results {
		let mut results = Results::new();
		if let Some(selected_line) = rebase_todo.get_selected_line() {
			// skip loading commit data if the currently loaded commit has not changed, this retains
			// position after returning to the list view or help
			if let Some(diff) = self.diff.as_ref() {
				if diff.commit().hash() == selected_line.get_hash() {
					return results;
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

			let new_diff = self
				.repository
				.load_commit_diff(selected_line.get_hash(), &self.commit_diff_loader_options);

			match new_diff {
				Ok(diff) => {
					self.diff = Some(diff);
				},
				Err(e) => {
					results.error_with_return(Error::from(e), State::List);
				},
			}
		}
		else {
			results.error_with_return(anyhow!("No valid commit to show"), State::List);
		}
		results
	}

	fn build_view_data(&mut self, context: &RenderContext, _: &TodoFile) -> &ViewData {
		if self.help.is_active() {
			return self.help.get_view_data();
		}

		let diff = self.diff.as_ref().unwrap(); // will only fail on programmer error
		let state = &self.state;
		let view_builder = &self.view_builder;
		let is_full_width = context.is_full_width();

		match *state {
			ShowCommitState::Overview => {
				if self.overview_view_data.is_empty() {
					self.overview_view_data.update_view_data(|updater| {
						capture!(view_builder, diff);
						view_builder.build_view_data_for_overview(updater, diff, is_full_width);
					});
				}
				&self.overview_view_data
			},
			ShowCommitState::Diff => {
				if self.diff_view_data.is_empty() {
					self.diff_view_data.update_view_data(|updater| {
						capture!(view_builder, diff);
						view_builder.build_view_data_diff(updater, diff, is_full_width);
					});
				}
				&self.diff_view_data
			},
		}
	}

	fn input_options(&self) -> &InputOptions {
		select!(default || &INPUT_OPTIONS, || self.help.input_options())
	}

	fn read_event(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		select!(
			default || {
				key_bindings
					.custom
					.show_diff
					.contains(&event)
					.then(|| Event::from(MetaEvent::ShowDiff))
					.unwrap_or(event)
			},
			|| { Help::read_event(event) }
		)
	}

	fn handle_event(&mut self, event: Event, view_state: &view::State, _: &mut TodoFile) -> Results {
		if self.help.is_active() {
			self.help.handle_event(event, view_state);
			return Results::new();
		}

		let mut results = Results::new();

		let active_view_data = match self.state {
			ShowCommitState::Overview => &mut self.overview_view_data,
			ShowCommitState::Diff => &mut self.diff_view_data,
		};

		if handle_view_data_scroll(event, view_state).is_none() {
			match event {
				Event::MetaEvent(meta_event) if meta_event == MetaEvent::ShowDiff => {
					active_view_data.update_view_data(|updater| updater.clear());
					self.state = match self.state {
						ShowCommitState::Overview => ShowCommitState::Diff,
						ShowCommitState::Diff => ShowCommitState::Overview,
					}
				},
				Event::Standard(standard_event) if standard_event == StandardEvent::Help => self.help.set_active(),
				Event::Key(_) => {
					active_view_data.update_view_data(|updater| updater.clear());
					if self.state == ShowCommitState::Diff {
						self.state = ShowCommitState::Overview;
					}
					else {
						results.state(State::List);
					}
				},
				Event::Resize(..) => active_view_data.update_view_data(|updater| updater.clear()),
				_ => {},
			}
		}
		results
	}
}

impl ShowCommit {
	pub(crate) fn new(config: &Config, repository: Repository) -> Self {
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

		let commit_diff_loader_options = CommitDiffLoaderOptions::new()
			.context_lines(config.git.diff_context)
			.copies(config.git.diff_copies)
			.ignore_whitespace(config.diff_ignore_whitespace == DiffIgnoreWhitespaceSetting::All)
			.ignore_whitespace_change(config.diff_ignore_whitespace == DiffIgnoreWhitespaceSetting::Change)
			.interhunk_context(config.git.diff_interhunk_lines)
			.renames(config.git.diff_renames, config.git.diff_rename_limit);

		Self {
			diff: None,
			diff_view_data,
			help: Help::new_from_keybindings(&get_show_commit_help_lines(&config.key_bindings)),
			commit_diff_loader_options,
			overview_view_data,
			state: ShowCommitState::Overview,
			view_builder: ViewBuilder::new(view_builder_options),
			repository,
		}
	}
}
