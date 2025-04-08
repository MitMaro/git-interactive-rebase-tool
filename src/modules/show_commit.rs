mod show_commit_state;
mod util;
mod view_builder;

#[cfg(test)]
mod tests;

use std::sync::Arc;

use anyhow::anyhow;
use captur::capture;
use parking_lot::Mutex;

use self::{
	show_commit_state::ShowCommitState,
	util::get_show_commit_help_lines,
	view_builder::{ViewBuilder, ViewBuilderOptions},
};
use crate::{
	application::AppData,
	components::help::Help,
	config::DiffShowWhitespaceSetting,
	diff,
	diff::thread::LoadStatus,
	input::{Event, InputOptions, KeyBindings, StandardEvent},
	module::{Module, State},
	process::Results,
	select,
	todo_file::TodoFile,
	util::handle_view_data_scroll,
	view::{self, RenderContext, ViewData, ViewLine},
};

// TODO Remove `union` call when bitflags/bitflags#180 is resolved
const INPUT_OPTIONS: InputOptions = InputOptions::UNDO_REDO
	.union(InputOptions::MOVEMENT)
	.union(InputOptions::HELP);

pub(crate) struct ShowCommit {
	diff_state: diff::thread::State,
	diff_view_data: ViewData,
	help: Help,
	overview_view_data: ViewData,
	state: ShowCommitState,
	view_state: view::State,
	todo_file: Arc<Mutex<TodoFile>>,
	view_builder: ViewBuilder,
}

impl Module for ShowCommit {
	fn activate(&mut self, _: State) -> Results {
		let mut results = Results::new();
		if let Some(selected_line) = self.todo_file.lock().get_selected_line() {
			{
				// skip loading commit data if the currently loaded commit has not changed, this retains
				// position after returning to the list view or help
				let diff = self.diff_state.diff();
				if diff.read().commit().hash() == selected_line.get_hash() {
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

			results.load_diff(selected_line.get_hash());
		}
		else {
			results.error_with_return(anyhow!("No valid commit to show"), State::List);
		}
		results
	}

	fn build_view_data(&mut self, context: &RenderContext) -> &ViewData {
		if self.help.is_active() {
			return self.help.get_view_data();
		}

		let diff_arc = self.diff_state.diff();
		let diff = diff_arc.read();
		let load_status = self.diff_state.load_status();

		if let LoadStatus::Error { code, msg, .. } = load_status {
			self.overview_view_data.update_view_data(|updater| {
				updater.clear();
				self.view_builder.build_diff_error(updater, code, msg.as_str());
			});
			return &self.overview_view_data;
		}

		// There is a small race condition where sometimes the diff loader is still in the process
		// of cancelling the previous diff and still has that diff loaded. In that case, we want to
		// show a general loading diff.
		let todo_line = self.todo_file.lock();
		let selected_line = todo_line.get_selected_line().map_or("", |l| l.get_hash());
		if self.diff_state.is_cancelled() || selected_line.is_empty() || selected_line != diff.commit().hash() {
			self.overview_view_data.update_view_data(|updater| {
				updater.clear();
				updater.push_line(ViewLine::from("Loading Diff"));
			});
			return &self.overview_view_data;
		}

		let state = &self.state;
		let view_builder = &mut self.view_builder;
		let is_full_width = context.is_full_width();

		match *state {
			ShowCommitState::Overview => {
				self.overview_view_data.update_view_data(|updater| {
					capture!(view_builder, diff);
					view_builder.build_view_data_for_overview(updater, &diff, &load_status, is_full_width);
				});
				&self.overview_view_data
			},
			ShowCommitState::Diff => {
				self.diff_view_data.update_view_data(|updater| {
					capture!(view_builder, diff);
					view_builder.build_view_data_diff(updater, &diff, &load_status, is_full_width);
				});
				&self.diff_view_data
			},
		}
	}

	fn input_options(&self) -> &InputOptions {
		select!(default & INPUT_OPTIONS, self.help.input_options())
	}

	fn read_event(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		select!(
			default {
				let has_event = key_bindings.show_diff.contains(&event);
				if has_event {
					Event::from(StandardEvent::ShowDiff)
				}
				else {
					event
				}
			},
			self.help.read_event(event)
		)
	}

	fn handle_event(&mut self, event: Event) -> Results {
		select!(
			default {
				let mut results = Results::new();

				match event {
					Event::Standard(StandardEvent::ShowDiff) => {
						self.state = match self.state {
							ShowCommitState::Overview => ShowCommitState::Diff,
							ShowCommitState::Diff => ShowCommitState::Overview,
						}
					},
					Event::Standard(StandardEvent::Help) => self.help.set_active(),
					Event::Key(_) => {
						if self.state == ShowCommitState::Diff {
							self.state = ShowCommitState::Overview;
						}
						else {
							results.cancel_diff();
							results.state(State::List);
						}
					},
					_ => {},
				}
				results
			},
			self.help.handle_event(event, &self.view_state),
			handle_view_data_scroll(event, &self.view_state)
		)
	}
}

impl ShowCommit {
	pub(crate) fn new(app_data: &AppData) -> Self {
		let overview_view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		});
		let diff_view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		});
		let config = app_data.config();
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
			diff_state: app_data.diff_state(),
			diff_view_data,
			help: Help::new_from_keybindings(&get_show_commit_help_lines(&config.key_bindings)),
			overview_view_data,
			state: ShowCommitState::Overview,
			view_state: app_data.view_state(),
			todo_file: app_data.todo_file(),
			view_builder: ViewBuilder::new(view_builder_options),
		}
	}
}
