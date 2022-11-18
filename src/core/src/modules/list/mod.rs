#[cfg(all(unix, test))]
mod tests;
mod utils;

use std::cmp::min;

use captur::capture;
use config::Config;
use display::DisplayColor;
use if_chain::if_chain;
use input::{InputOptions, MouseEventKind, StandardEvent};
use todo_file::{Action, EditContext, Line, Search, TodoFile};
use view::{LineSegment, RenderContext, ViewData, ViewLine};

use self::utils::{
	get_list_normal_mode_help_lines,
	get_list_visual_mode_help_lines,
	get_todo_line_segments,
	TodoLineSegmentsOptions,
};
use crate::{
	components::{
		edit::Edit,
		help::Help,
		search_bar::{SearchBar, SearchBarAction},
	},
	events::{Event, KeyBindings, MetaEvent},
	module::{ExitStatus, Module, State},
	process::Results,
	select,
};

// TODO Remove `union` call when bitflags/bitflags#180 is resolved
const INPUT_OPTIONS: InputOptions = InputOptions::UNDO_REDO
	.union(InputOptions::RESIZE)
	.union(InputOptions::HELP)
	.union(InputOptions::SEARCH);

#[derive(Debug, PartialEq, Eq)]
enum ListState {
	Normal,
	Visual,
	Edit,
}

pub(crate) struct List {
	auto_select_next: bool,
	edit: Edit,
	height: usize,
	normal_mode_help: Help,
	search: Search,
	search_bar: SearchBar,
	state: ListState,
	view_data: ViewData,
	visual_index_start: Option<usize>,
	visual_mode_help: Help,
}

impl Module for List {
	fn build_view_data(&mut self, context: &RenderContext, todo_file: &TodoFile) -> &ViewData {
		match self.state {
			ListState::Normal => self.get_normal_mode_view_data(todo_file, context),
			ListState::Visual => self.get_visual_mode_view_data(todo_file, context),
			ListState::Edit => {
				if let Some(selected_line) = todo_file.get_selected_line() {
					if selected_line.is_editable() {
						return self.edit.build_view_data(
							|updater| {
								updater.push_leading_line(ViewLine::from(LineSegment::new_with_color(
									format!("Modifying line: {}", selected_line.to_text()).as_str(),
									DisplayColor::IndicatorColor,
								)));
								updater.push_leading_line(ViewLine::new_empty_line());
							},
							|_| {},
						);
					}
				}
				self.edit.get_view_data()
			},
		}
	}

	fn handle_event(&mut self, event: Event, view_state: &view::State, todo_file: &mut TodoFile) -> Results {
		select!(
			default || {
				match self.state {
					ListState::Normal => self.handle_normal_mode_event(event, view_state, todo_file),
					ListState::Visual => self.handle_visual_mode_input(event, view_state, todo_file),
					ListState::Edit => self.handle_edit_mode_input(event, todo_file),
				}
			},
			|| self.handle_normal_help_input(event, view_state),
			|| self.handle_visual_help_input(event, view_state),
			|| self.handle_search_input(event, todo_file)
		)
	}

	fn input_options(&self) -> &InputOptions {
		select!(
			default || &INPUT_OPTIONS,
			|| self.normal_mode_help.input_options(),
			|| self.visual_mode_help.input_options(),
			|| self.search_bar.input_options(),
			|| (self.state == ListState::Edit).then(|| self.edit.input_options())
		)
	}

	fn read_event(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		select!(
			default || Self::read_event_default(event, key_bindings),
			|| (self.state == ListState::Edit).then_some(event),
			|| self.normal_mode_help.read_event(event),
			|| self.visual_mode_help.read_event(event),
			|| self.search_bar.read_event(event)
		)
	}
}

impl List {
	pub(crate) fn new(config: &Config) -> Self {
		let view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		});

		Self {
			auto_select_next: config.auto_select_next,
			edit: Edit::new(),
			height: 0,
			normal_mode_help: Help::new_from_keybindings(&get_list_normal_mode_help_lines(&config.key_bindings)),
			search: Search::new(),
			search_bar: SearchBar::new(),
			state: ListState::Normal,
			view_data,
			visual_index_start: None,
			visual_mode_help: Help::new_from_keybindings(&get_list_visual_mode_help_lines(&config.key_bindings)),
		}
	}

	fn set_cursor(&mut self, todo_file: &mut TodoFile, cursor: usize) {
		todo_file.set_selected_line_index(cursor);
		self.search.set_search_start_hint(cursor);
	}

	#[allow(clippy::unused_self)]
	fn move_cursor_left(&self, view_state: &view::State) {
		view_state.scroll_left();
	}

	#[allow(clippy::unused_self)]
	fn move_cursor_right(&self, view_state: &view::State) {
		view_state.scroll_right();
	}

	fn move_cursor_up(&mut self, todo_file: &mut TodoFile, amount: usize) {
		let current_selected_line_index = todo_file.get_selected_line_index();
		let new_selected_line_index = current_selected_line_index.saturating_sub(amount);
		self.set_cursor(todo_file, new_selected_line_index);
	}

	fn move_cursor_down(&mut self, todo_file: &mut TodoFile, amount: usize) {
		let current_selected_line_index = todo_file.get_selected_line_index();
		let new_selected_line_index = current_selected_line_index + amount;
		self.set_cursor(todo_file, new_selected_line_index);
	}

	fn move_cursor_home(&mut self, todo_file: &mut TodoFile) {
		self.set_cursor(todo_file, 0);
	}

	fn move_cursor_end(&mut self, todo_file: &mut TodoFile) {
		let new_selected_line_index = todo_file.get_max_selected_line_index();
		self.set_cursor(todo_file, new_selected_line_index);
	}

	#[allow(clippy::unused_self)]
	fn abort(&self, results: &mut Results) {
		results.state(State::ConfirmAbort);
	}

	#[allow(clippy::unused_self)]
	fn force_abort(&self, results: &mut Results, rebase_todo: &mut TodoFile) {
		rebase_todo.set_lines(vec![]);
		results.exit_status(ExitStatus::Good);
	}

	#[allow(clippy::unused_self)]
	fn rebase(&self, results: &mut Results) {
		results.state(State::ConfirmRebase);
	}

	#[allow(clippy::unused_self)]
	fn force_rebase(&self, results: &mut Results) {
		results.exit_status(ExitStatus::Good);
	}

	fn swap_selected_up(&mut self, todo_file: &mut TodoFile) {
		let start_index = todo_file.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		if todo_file.swap_range_up(start_index, end_index) {
			if let Some(visual_index_start) = self.visual_index_start {
				self.visual_index_start = Some(visual_index_start - 1);
			}
			self.move_cursor_up(todo_file, 1);
		}
	}

	fn swap_selected_down(&mut self, todo_file: &mut TodoFile) {
		let start_index = todo_file.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		if todo_file.swap_range_down(start_index, end_index) {
			if let Some(visual_index_start) = self.visual_index_start {
				self.visual_index_start = Some(visual_index_start + 1);
			}
			self.move_cursor_down(todo_file, 1);
		}
	}

	fn set_selected_line_action(&mut self, rebase_todo: &mut TodoFile, action: Action) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		rebase_todo.update_range(start_index, end_index, &EditContext::new().action(action));
		if self.state == ListState::Normal && self.auto_select_next {
			self.move_cursor_down(rebase_todo, 1);
		}
	}

	fn undo(&mut self, todo_file: &mut TodoFile) {
		if let Some((start_index, end_index)) = todo_file.undo() {
			self.set_cursor(todo_file, start_index);
			if start_index == end_index {
				self.state = ListState::Normal;
				self.visual_index_start = None;
			}
			else {
				self.state = ListState::Visual;
				self.visual_index_start = Some(end_index);
			}
		}
	}

	fn redo(&mut self, todo_file: &mut TodoFile) {
		if let Some((start_index, end_index)) = todo_file.redo() {
			self.set_cursor(todo_file, start_index);
			if start_index == end_index {
				self.state = ListState::Normal;
				self.visual_index_start = None;
			}
			else {
				self.state = ListState::Visual;
				self.visual_index_start = Some(end_index);
			}
		}
	}

	fn delete(&mut self, todo_file: &mut TodoFile) {
		let start_index = todo_file.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		todo_file.remove_lines(start_index, end_index);
		let new_index = min(start_index, end_index);

		self.set_cursor(todo_file, new_index);

		if self.state == ListState::Visual {
			self.visual_index_start = Some(todo_file.get_selected_line_index());
		}
	}

	#[allow(clippy::unused_self)]
	fn open_in_editor(&mut self, results: &mut Results) {
		self.search_bar.reset();
		results.state(State::ExternalEditor);
	}

	fn toggle_visual_mode(&mut self, todo_file: &mut TodoFile) {
		if self.state == ListState::Visual {
			self.state = ListState::Normal;
			self.visual_index_start = None;
		}
		else {
			self.state = ListState::Visual;
			self.visual_index_start = Some(todo_file.get_selected_line_index());
		}
	}

	fn search_start(&mut self) {
		self.search_bar.start_search(None);
	}

	fn help(&mut self) {
		if self.state == ListState::Visual {
			self.visual_mode_help.set_active();
		}
		else {
			self.normal_mode_help.set_active();
		}
	}

	fn resize(&mut self, height: u16) {
		self.height = height as usize;
	}

	#[allow(clippy::unused_self)]
	fn show_commit(&mut self, results: &mut Results, todo_file: &TodoFile) {
		if let Some(selected_line) = todo_file.get_selected_line() {
			if selected_line.has_reference() {
				results.state(State::ShowCommit);
			}
		}
	}

	fn action_break(&mut self, todo_file: &mut TodoFile) {
		let selected_line_index = todo_file.get_selected_line_index();
		let next_action_is_break = todo_file
			.get_line(selected_line_index + 1)
			.map_or(false, |line| line.get_action() == &Action::Break);
		if !next_action_is_break {
			let selected_action_is_break = todo_file
				.get_line(selected_line_index)
				.map_or(false, |line| line.get_action() == &Action::Break);
			if selected_action_is_break {
				todo_file.remove_lines(selected_line_index, selected_line_index);
				self.move_cursor_up(todo_file, 1);
			}
			else {
				todo_file.add_line(selected_line_index + 1, Line::new_break());
				self.move_cursor_down(todo_file, 1);
			}
		}
	}

	fn edit(&mut self, todo_file: &mut TodoFile) {
		if let Some(selected_line) = todo_file.get_selected_line() {
			if selected_line.is_editable() {
				self.state = ListState::Edit;
				self.edit.set_content(selected_line.get_content());
				self.edit.set_label(format!("{} ", selected_line.get_action()).as_str());
			}
		}
	}

	#[allow(clippy::unused_self)]
	fn insert_line(&mut self, results: &mut Results) {
		results.state(State::Insert);
	}

	fn update_list_view_data(&mut self, context: &RenderContext, todo_file: &TodoFile) -> &ViewData {
		let is_visual_mode = self.state == ListState::Visual;
		let selected_index = todo_file.get_selected_line_index();
		let visual_index = self.visual_index_start.unwrap_or(selected_index);
		let search_view_line = self.search_bar.is_editing().then(|| self.search_bar.build_view_line());
		let search_results_total = self.search_bar.is_searching().then(|| self.search.total_results());
		let search_results_current = self.search.current_result_selected();
		let search_term = self.search_bar.search_value();
		let search_index = self.search.current_match();

		self.view_data.update_view_data(|updater| {
			capture!(todo_file);
			updater.clear();
			if todo_file.is_empty() {
				updater.push_leading_line(ViewLine::from(LineSegment::new_with_color(
					"Rebase todo file is empty",
					DisplayColor::IndicatorColor,
				)));
			}
			else {
				for (index, line) in todo_file.lines_iter().enumerate() {
					let selected_line = is_visual_mode
						&& ((visual_index <= selected_index && index >= visual_index && index <= selected_index)
							|| (visual_index > selected_index && index >= selected_index && index <= visual_index));
					let mut todo_line_segment_options = TodoLineSegmentsOptions::empty();
					if selected_index == index {
						todo_line_segment_options.insert(TodoLineSegmentsOptions::CURSOR_LINE);
					}
					if selected_line {
						todo_line_segment_options.insert(TodoLineSegmentsOptions::SELECTED);
					}
					if context.is_full_width() {
						todo_line_segment_options.insert(TodoLineSegmentsOptions::FULL_WIDTH);
					}
					if search_index.map_or(false, |v| v == index) {
						todo_line_segment_options.insert(TodoLineSegmentsOptions::SEARCH_LINE);
					}
					let mut view_line = ViewLine::new_with_pinned_segments(
						get_todo_line_segments(line, search_term, todo_line_segment_options),
						if line.has_reference() { 2 } else { 3 },
					)
					.set_selected(selected_index == index || selected_line);

					if selected_index == index || selected_line {
						view_line = view_line.set_selected(true).set_padding(' ');
					}

					updater.push_line(view_line);
				}
				if let Some(search) = search_view_line {
					updater.push_trailing_line(search);
				}
				else if let Some(s_term) = search_term {
					let mut search_line_segments = vec![];
					search_line_segments.push(LineSegment::new(format!("[{s_term}]: ").as_str()));
					if_chain! {
						if let Some(s_total) = search_results_total;
						if let Some(s_index) = search_results_current;
						if s_total != 0;
						then {
							search_line_segments.push(LineSegment::new(format!("{}/{s_total}", s_index + 1).as_str()));
						}
						else {
							search_line_segments.push(LineSegment::new("No Results"));
						}
					}
					updater.push_trailing_line(ViewLine::from(search_line_segments));
				}
			}
			if visual_index != selected_index {
				updater.ensure_line_visible(visual_index);
			}
			updater.ensure_line_visible(selected_index);
		});
		&self.view_data
	}

	fn get_visual_mode_view_data(&mut self, todo_file: &TodoFile, context: &RenderContext) -> &ViewData {
		if self.visual_mode_help.is_active() {
			self.visual_mode_help.get_view_data()
		}
		else {
			self.update_list_view_data(context, todo_file)
		}
	}

	fn get_normal_mode_view_data(&mut self, todo_file: &TodoFile, context: &RenderContext) -> &ViewData {
		if self.normal_mode_help.is_active() {
			self.normal_mode_help.get_view_data()
		}
		else {
			self.update_list_view_data(context, todo_file)
		}
	}

	#[allow(clippy::cognitive_complexity)]
	fn read_event_default(event: Event, key_bindings: &KeyBindings) -> Event {
		match event {
			e if key_bindings.custom.abort.contains(&e) => Event::from(MetaEvent::Abort),
			e if key_bindings.custom.action_break.contains(&e) => Event::from(MetaEvent::ActionBreak),
			e if key_bindings.custom.action_drop.contains(&e) => Event::from(MetaEvent::ActionDrop),
			e if key_bindings.custom.action_edit.contains(&e) => Event::from(MetaEvent::ActionEdit),
			e if key_bindings.custom.action_fixup.contains(&e) => Event::from(MetaEvent::ActionFixup),
			e if key_bindings.custom.action_pick.contains(&e) => Event::from(MetaEvent::ActionPick),
			e if key_bindings.custom.action_reword.contains(&e) => Event::from(MetaEvent::ActionReword),
			e if key_bindings.custom.action_squash.contains(&e) => Event::from(MetaEvent::ActionSquash),
			e if key_bindings.custom.edit.contains(&e) => Event::from(MetaEvent::Edit),
			e if key_bindings.custom.force_abort.contains(&e) => Event::from(MetaEvent::ForceAbort),
			e if key_bindings.custom.force_rebase.contains(&e) => Event::from(MetaEvent::ForceRebase),
			e if key_bindings.custom.insert_line.contains(&e) => Event::from(MetaEvent::InsertLine),
			e if key_bindings.custom.move_down.contains(&e) => Event::from(MetaEvent::MoveCursorDown),
			e if key_bindings.custom.move_down_step.contains(&e) => Event::from(MetaEvent::MoveCursorPageDown),
			e if key_bindings.custom.move_end.contains(&e) => Event::from(MetaEvent::MoveCursorEnd),
			e if key_bindings.custom.move_home.contains(&e) => Event::from(MetaEvent::MoveCursorHome),
			e if key_bindings.custom.move_left.contains(&e) => Event::from(MetaEvent::MoveCursorLeft),
			e if key_bindings.custom.move_right.contains(&e) => Event::from(MetaEvent::MoveCursorRight),
			e if key_bindings.custom.move_selection_down.contains(&e) => Event::from(MetaEvent::SwapSelectedDown),
			e if key_bindings.custom.move_selection_up.contains(&e) => Event::from(MetaEvent::SwapSelectedUp),
			e if key_bindings.custom.move_up.contains(&e) => Event::from(MetaEvent::MoveCursorUp),
			e if key_bindings.custom.move_up_step.contains(&e) => Event::from(MetaEvent::MoveCursorPageUp),
			e if key_bindings.custom.open_in_external_editor.contains(&e) => Event::from(MetaEvent::OpenInEditor),
			e if key_bindings.custom.rebase.contains(&e) => Event::from(MetaEvent::Rebase),
			e if key_bindings.custom.remove_line.contains(&e) => Event::from(MetaEvent::Delete),
			e if key_bindings.custom.show_commit.contains(&e) => Event::from(MetaEvent::ShowCommit),
			e if key_bindings.custom.toggle_visual_mode.contains(&e) => Event::from(MetaEvent::ToggleVisualMode),
			Event::Mouse(mouse_event) => {
				match mouse_event.kind {
					MouseEventKind::ScrollDown => Event::from(MetaEvent::MoveCursorDown),
					MouseEventKind::ScrollUp => Event::from(MetaEvent::MoveCursorUp),
					_ => event,
				}
			},
			_ => event,
		}
	}

	fn handle_normal_help_input(&mut self, event: Event, view_state: &view::State) -> Option<Results> {
		self.normal_mode_help.is_active().then(|| {
			self.normal_mode_help.handle_event(event, view_state);
			Results::new()
		})
	}

	fn handle_visual_help_input(&mut self, event: Event, view_state: &view::State) -> Option<Results> {
		self.visual_mode_help.is_active().then(|| {
			self.visual_mode_help.handle_event(event, view_state);
			Results::new()
		})
	}

	fn handle_search_input(&mut self, event: Event, todo_file: &mut TodoFile) -> Option<Results> {
		if self.search_bar.is_active() {
			match self.search_bar.handle_event(event) {
				SearchBarAction::Start(term) => {
					if term.is_empty() {
						self.search.cancel();
						self.search_bar.reset();
					}
					else {
						self.search.next(todo_file, term.as_str());
					}
				},
				SearchBarAction::Next(term) => self.search.next(todo_file, term.as_str()),
				SearchBarAction::Previous(term) => self.search.previous(todo_file, term.as_str()),
				SearchBarAction::Cancel => {
					self.search.cancel();
					return Some(Results::from(event));
				},
				SearchBarAction::None => return None,
			}

			if let Some(selected) = self.search.current_match() {
				self.set_cursor(todo_file, selected);
			}
			return Some(Results::from(event));
		}
		None
	}

	#[allow(clippy::integer_division)]
	fn handle_common_list_input(
		&mut self,
		event: Event,
		view_state: &view::State,
		rebase_todo: &mut TodoFile,
	) -> Option<Results> {
		let mut results = Results::new();
		match event {
			Event::MetaEvent(meta_event) => {
				match meta_event {
					MetaEvent::Abort => self.abort(&mut results),
					MetaEvent::ActionDrop => self.set_selected_line_action(rebase_todo, Action::Drop),
					MetaEvent::ActionEdit => self.set_selected_line_action(rebase_todo, Action::Edit),
					MetaEvent::ActionFixup => self.set_selected_line_action(rebase_todo, Action::Fixup),
					MetaEvent::ActionPick => self.set_selected_line_action(rebase_todo, Action::Pick),
					MetaEvent::ActionReword => self.set_selected_line_action(rebase_todo, Action::Reword),
					MetaEvent::ActionSquash => self.set_selected_line_action(rebase_todo, Action::Squash),
					MetaEvent::Delete => self.delete(rebase_todo),
					MetaEvent::ForceAbort => self.force_abort(&mut results, rebase_todo),
					MetaEvent::ForceRebase => self.force_rebase(&mut results),
					MetaEvent::MoveCursorDown => self.move_cursor_down(rebase_todo, 1),
					MetaEvent::MoveCursorEnd => self.move_cursor_end(rebase_todo),
					MetaEvent::MoveCursorHome => self.move_cursor_home(rebase_todo),
					MetaEvent::MoveCursorLeft => self.move_cursor_left(view_state),
					MetaEvent::MoveCursorPageDown => self.move_cursor_down(rebase_todo, self.height / 2),
					MetaEvent::MoveCursorPageUp => self.move_cursor_up(rebase_todo, self.height / 2),
					MetaEvent::MoveCursorRight => self.move_cursor_right(view_state),
					MetaEvent::MoveCursorUp => self.move_cursor_up(rebase_todo, 1),
					MetaEvent::OpenInEditor => self.open_in_editor(&mut results),
					MetaEvent::Rebase => self.rebase(&mut results),
					MetaEvent::SwapSelectedDown => self.swap_selected_down(rebase_todo),
					MetaEvent::SwapSelectedUp => self.swap_selected_up(rebase_todo),
					MetaEvent::ToggleVisualMode => self.toggle_visual_mode(rebase_todo),
					_ => return None,
				}
			},
			Event::Standard(standard_event) => {
				match standard_event {
					StandardEvent::Help => self.help(),
					StandardEvent::Redo => self.redo(rebase_todo),
					StandardEvent::Undo => self.undo(rebase_todo),
					StandardEvent::SearchStart => self.search_start(),
					_ => return None,
				}
			},
			Event::Resize(_, height) => self.resize(height),
			_ => {},
		}

		Some(results)
	}

	fn handle_normal_mode_event(
		&mut self,
		event: Event,
		view_state: &view::State,
		rebase_todo: &mut TodoFile,
	) -> Results {
		if let Some(results) = self.handle_common_list_input(event, view_state, rebase_todo) {
			results
		}
		else {
			let mut results = Results::new();
			if let Event::MetaEvent(meta_event) = event {
				match meta_event {
					MetaEvent::ActionBreak => self.action_break(rebase_todo),
					MetaEvent::Edit => self.edit(rebase_todo),
					MetaEvent::InsertLine => self.insert_line(&mut results),
					MetaEvent::ShowCommit => self.show_commit(&mut results, rebase_todo),
					_ => {},
				}
			}
			results
		}
	}

	fn handle_visual_mode_input(
		&mut self,
		event: Event,
		view_state: &view::State,
		rebase_todo: &mut TodoFile,
	) -> Results {
		self.handle_common_list_input(event, view_state, rebase_todo)
			.unwrap_or_else(Results::new)
	}

	fn handle_edit_mode_input(&mut self, event: Event, rebase_todo: &mut TodoFile) -> Results {
		self.edit.handle_event(event);
		if self.edit.is_finished() {
			let selected_index = rebase_todo.get_selected_line_index();
			rebase_todo.update_range(
				selected_index,
				selected_index,
				&EditContext::new().content(self.edit.get_content()),
			);
			self.visual_index_start = None;
			self.state = ListState::Normal;
		}
		Results::new()
	}
}
