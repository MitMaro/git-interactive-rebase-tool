#[cfg(all(unix, test))]
mod tests;
mod utils;

use std::{cmp::min, sync::Arc};

use captur::capture;
use config::Config;
use display::DisplayColor;
use if_chain::if_chain;
use input::{InputOptions, MouseEventKind, StandardEvent};
use parking_lot::Mutex;
use todo_file::{Action, EditContext, Line, Search, TodoFile};

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
	modules::list::utils::get_line_action_maximum_width,
	process::Results,
	select,
	view::{LineSegment, RenderContext, ViewData, ViewLine},
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

#[derive(Debug, Copy, Clone)]
enum CursorUpdate {
	Down(usize),
	Set(usize),
	Up(usize),
	End,
}

pub(crate) struct List {
	auto_select_next: bool,
	edit: Edit,
	height: usize,
	normal_mode_help: Help,
	search: Search,
	search_bar: SearchBar,
	selected_line_action: Option<Action>,
	state: ListState,
	todo_file: Arc<Mutex<TodoFile>>,
	view_data: ViewData,
	visual_index_start: Option<usize>,
	visual_mode_help: Help,
}

impl Module for List {
	fn activate(&mut self, _: State) -> Results {
		self.selected_line_action = self.todo_file.lock().get_selected_line().map(|line| *line.get_action());
		Results::new()
	}

	fn build_view_data(&mut self, context: &RenderContext) -> &ViewData {
		match self.state {
			ListState::Normal => self.get_normal_mode_view_data(context),
			ListState::Visual => self.get_visual_mode_view_data(context),
			ListState::Edit => {
				if let Some(selected_line) = self.todo_file.lock().get_selected_line() {
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

	fn handle_event(&mut self, event: Event, view_state: &crate::view::State) -> Results {
		select!(
			default || {
				match self.state {
					ListState::Normal => self.handle_normal_mode_event(event, view_state),
					ListState::Visual => self.handle_visual_mode_input(event, view_state),
					ListState::Edit => self.handle_edit_mode_input(event),
				}
			},
			|| self.handle_normal_help_input(event, view_state),
			|| self.handle_visual_help_input(event, view_state),
			|| self.handle_search_input(event)
		)
	}

	fn input_options(&self) -> &InputOptions {
		select!(
			default || &INPUT_OPTIONS,
			|| (self.state == ListState::Edit).then(|| self.edit.input_options()),
			|| self.normal_mode_help.input_options(),
			|| self.visual_mode_help.input_options(),
			|| self.search_bar.input_options()
		)
	}

	fn read_event(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		select!(
			default || self.read_event_default(event, key_bindings),
			|| (self.state == ListState::Edit).then_some(event),
			|| self.normal_mode_help.read_event(event),
			|| self.visual_mode_help.read_event(event),
			|| self.search_bar.read_event(event)
		)
	}
}

impl List {
	pub(crate) fn new(config: &Config, todo_file: Arc<Mutex<TodoFile>>) -> Self {
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
			selected_line_action: None,
			state: ListState::Normal,
			todo_file,
			view_data,
			visual_index_start: None,
			visual_mode_help: Help::new_from_keybindings(&get_list_visual_mode_help_lines(&config.key_bindings)),
		}
	}

	fn update_cursor(&mut self, cursor_update: CursorUpdate) -> usize {
		let mut todo_file = self.todo_file.lock();
		let new_selected_line_index = match cursor_update {
			CursorUpdate::Down(amount) => todo_file.get_selected_line_index().saturating_add(amount),
			CursorUpdate::Up(amount) => todo_file.get_selected_line_index().saturating_sub(amount),
			CursorUpdate::Set(value) => value,
			CursorUpdate::End => todo_file.get_max_selected_line_index(),
		};
		let selected_line_index = todo_file.set_selected_line_index(new_selected_line_index);
		self.selected_line_action = todo_file.get_selected_line().map(|line| *line.get_action());
		self.search.set_search_start_hint(selected_line_index);
		selected_line_index
	}

	#[allow(clippy::unused_self)]
	fn move_cursor_left(&self, view_state: &crate::view::State) {
		view_state.scroll_left();
	}

	#[allow(clippy::unused_self)]
	fn move_cursor_right(&self, view_state: &crate::view::State) {
		view_state.scroll_right();
	}

	#[allow(clippy::unused_self)]
	fn abort(&self, results: &mut Results) {
		results.state(State::ConfirmAbort);
	}

	#[allow(clippy::unused_self)]
	fn force_abort(&self, results: &mut Results) {
		let mut todo_file = self.todo_file.lock();
		todo_file.set_lines(vec![]);
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

	fn swap_selected_up(&mut self) {
		let mut todo_file = self.todo_file.lock();
		let start_index = todo_file.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		let swapped = todo_file.swap_range_up(start_index, end_index);
		drop(todo_file);

		if swapped {
			if let Some(visual_index_start) = self.visual_index_start {
				self.visual_index_start = Some(visual_index_start - 1);
			}
			_ = self.update_cursor(CursorUpdate::Up(1));
		}
	}

	fn swap_selected_down(&mut self) {
		let mut todo_file = self.todo_file.lock();
		let start_index = todo_file.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		let swapped = todo_file.swap_range_down(start_index, end_index);
		drop(todo_file);

		if swapped {
			if let Some(visual_index_start) = self.visual_index_start {
				self.visual_index_start = Some(visual_index_start + 1);
			}
			_ = self.update_cursor(CursorUpdate::Down(1));
		}
	}

	fn set_selected_line_action(&mut self, action: Action) {
		let mut todo_file = self.todo_file.lock();
		let start_index = todo_file.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		todo_file.update_range(start_index, end_index, &EditContext::new().action(action));
		drop(todo_file);

		if self.state == ListState::Normal && self.auto_select_next {
			_ = self.update_cursor(CursorUpdate::Down(1));
		}
	}

	fn undo(&mut self) {
		let mut todo_file = self.todo_file.lock();
		let undo_result = todo_file.undo();
		drop(todo_file);

		if let Some((start_index, end_index)) = undo_result {
			let new_start_index = self.update_cursor(CursorUpdate::Set(start_index));
			if new_start_index == end_index {
				self.state = ListState::Normal;
				self.visual_index_start = None;
			}
			else {
				self.state = ListState::Visual;
				self.visual_index_start = Some(end_index);
			}
		}
	}

	fn redo(&mut self) {
		let mut todo_file = self.todo_file.lock();
		let redo_result = todo_file.redo();
		drop(todo_file);

		if let Some((start_index, end_index)) = redo_result {
			let new_start_index = self.update_cursor(CursorUpdate::Set(start_index));
			if new_start_index == end_index {
				self.state = ListState::Normal;
				self.visual_index_start = None;
			}
			else {
				self.state = ListState::Visual;
				self.visual_index_start = Some(end_index);
			}
		}
	}

	fn delete(&mut self) {
		let mut todo_file = self.todo_file.lock();
		let start_index = todo_file.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		todo_file.remove_lines(start_index, end_index);
		drop(todo_file);

		let new_index = min(start_index, end_index);
		let selected_line_index = self.update_cursor(CursorUpdate::Set(new_index));

		if self.state == ListState::Visual {
			self.visual_index_start = Some(selected_line_index);
		}
	}

	#[allow(clippy::unused_self)]
	fn open_in_editor(&mut self, results: &mut Results) {
		self.search_bar.reset();
		results.state(State::ExternalEditor);
	}

	fn toggle_visual_mode(&mut self) {
		if self.state == ListState::Visual {
			self.state = ListState::Normal;
			self.visual_index_start = None;
		}
		else {
			self.state = ListState::Visual;
			self.visual_index_start = Some(self.todo_file.lock().get_selected_line_index());
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

	fn show_commit(&mut self, results: &mut Results) {
		let todo_file = self.todo_file.lock();
		if let Some(selected_line) = todo_file.get_selected_line() {
			if selected_line.has_reference() {
				results.state(State::ShowCommit);
			}
		}
	}

	fn action_break(&mut self) {
		let mut todo_file = self.todo_file.lock();
		let selected_line_index = todo_file.get_selected_line_index();
		let next_action_is_break = todo_file
			.get_line(selected_line_index + 1)
			.map_or(false, |line| line.get_action() == &Action::Break);

		// no need to add an additional break when the next line is already a break
		if next_action_is_break {
			return;
		}

		let selected_action_is_break = todo_file
			.get_line(selected_line_index)
			.map_or(false, |line| line.get_action() == &Action::Break);

		let cursor_update = if selected_action_is_break {
			todo_file.remove_lines(selected_line_index, selected_line_index);
			CursorUpdate::Up(1)
		}
		else {
			todo_file.add_line(selected_line_index + 1, Line::new_break());
			CursorUpdate::Down(1)
		};

		drop(todo_file);

		_ = self.update_cursor(cursor_update);
	}

	#[allow(clippy::unused_self)]
	fn toggle_option(&mut self, option: &str) {
		let mut todo_file = self.todo_file.lock();
		let selected_line_index = todo_file.get_selected_line_index();
		todo_file.update_range(
			selected_line_index,
			selected_line_index,
			&EditContext::new().option(option),
		);
	}

	fn edit(&mut self) {
		let todo_file = self.todo_file.lock();
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

	fn update_list_view_data(&mut self, context: &RenderContext) -> &ViewData {
		let todo_file = self.todo_file.lock();
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
				let maximum_action_width = get_line_action_maximum_width(&todo_file);
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
						get_todo_line_segments(line, search_term, todo_line_segment_options, maximum_action_width),
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

	fn get_visual_mode_view_data(&mut self, context: &RenderContext) -> &ViewData {
		if self.visual_mode_help.is_active() {
			self.visual_mode_help.get_view_data()
		}
		else {
			self.update_list_view_data(context)
		}
	}

	fn get_normal_mode_view_data(&mut self, context: &RenderContext) -> &ViewData {
		if self.normal_mode_help.is_active() {
			self.normal_mode_help.get_view_data()
		}
		else {
			self.update_list_view_data(context)
		}
	}

	#[allow(clippy::cognitive_complexity)]
	fn read_event_default(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		// handle action level events
		if let Some(action) = self.selected_line_action {
			if action == Action::Fixup {
				match event {
					e if key_bindings.custom.fixup_keep_message.contains(&e) => {
						return Event::from(MetaEvent::FixupKeepMessage);
					},
					e if key_bindings.custom.fixup_keep_message_with_editor.contains(&e) => {
						return Event::from(MetaEvent::FixupKeepMessageWithEditor);
					},
					_ => {},
				}
			}
		}

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

	fn handle_normal_help_input(&mut self, event: Event, view_state: &crate::view::State) -> Option<Results> {
		self.normal_mode_help.is_active().then(|| {
			self.normal_mode_help.handle_event(event, view_state);
			Results::new()
		})
	}

	fn handle_visual_help_input(&mut self, event: Event, view_state: &crate::view::State) -> Option<Results> {
		self.visual_mode_help.is_active().then(|| {
			self.visual_mode_help.handle_event(event, view_state);
			Results::new()
		})
	}

	fn handle_search_input(&mut self, event: Event) -> Option<Results> {
		if self.search_bar.is_active() {
			let todo_file = self.todo_file.lock();
			match self.search_bar.handle_event(event) {
				SearchBarAction::Start(term) => {
					if term.is_empty() {
						self.search.cancel();
						self.search_bar.reset();
					}
					else {
						self.search.next(&todo_file, term.as_str());
					}
				},
				SearchBarAction::Next(term) => self.search.next(&todo_file, term.as_str()),
				SearchBarAction::Previous(term) => self.search.previous(&todo_file, term.as_str()),
				SearchBarAction::Cancel => {
					self.search.cancel();
					return Some(Results::from(event));
				},
				SearchBarAction::None | SearchBarAction::Update(_) => return None,
			}
			drop(todo_file);

			if let Some(selected) = self.search.current_match() {
				_ = self.update_cursor(CursorUpdate::Set(selected));
			}
			return Some(Results::from(event));
		}
		None
	}

	#[allow(clippy::integer_division)]
	fn handle_common_list_input(&mut self, event: Event, view_state: &crate::view::State) -> Option<Results> {
		let mut results = Results::new();
		match event {
			Event::MetaEvent(meta_event) => {
				match meta_event {
					MetaEvent::Abort => self.abort(&mut results),
					MetaEvent::ActionDrop => self.set_selected_line_action(Action::Drop),
					MetaEvent::ActionEdit => self.set_selected_line_action(Action::Edit),
					MetaEvent::ActionFixup => self.set_selected_line_action(Action::Fixup),
					MetaEvent::ActionPick => self.set_selected_line_action(Action::Pick),
					MetaEvent::ActionReword => self.set_selected_line_action(Action::Reword),
					MetaEvent::ActionSquash => self.set_selected_line_action(Action::Squash),
					MetaEvent::Delete => self.delete(),
					MetaEvent::ForceAbort => self.force_abort(&mut results),
					MetaEvent::ForceRebase => self.force_rebase(&mut results),
					MetaEvent::MoveCursorDown => {
						_ = self.update_cursor(CursorUpdate::Down(1));
					},
					MetaEvent::MoveCursorEnd => {
						_ = self.update_cursor(CursorUpdate::End);
					},
					MetaEvent::MoveCursorHome => {
						_ = self.update_cursor(CursorUpdate::Set(0));
					},
					MetaEvent::MoveCursorLeft => self.move_cursor_left(view_state),
					MetaEvent::MoveCursorPageDown => {
						_ = self.update_cursor(CursorUpdate::Down(self.height / 2));
					},
					MetaEvent::MoveCursorPageUp => {
						_ = self.update_cursor(CursorUpdate::Up(self.height / 2));
					},
					MetaEvent::MoveCursorRight => self.move_cursor_right(view_state),
					MetaEvent::MoveCursorUp => {
						_ = self.update_cursor(CursorUpdate::Up(1));
					},
					MetaEvent::OpenInEditor => self.open_in_editor(&mut results),
					MetaEvent::Rebase => self.rebase(&mut results),
					MetaEvent::SwapSelectedDown => self.swap_selected_down(),
					MetaEvent::SwapSelectedUp => self.swap_selected_up(),
					MetaEvent::ToggleVisualMode => self.toggle_visual_mode(),
					_ => return None,
				}
			},
			Event::Standard(standard_event) => {
				match standard_event {
					StandardEvent::Help => self.help(),
					StandardEvent::Redo => self.redo(),
					StandardEvent::Undo => self.undo(),
					StandardEvent::SearchStart => self.search_start(),
					_ => return None,
				}
			},
			Event::Resize(_, height) => self.resize(height),
			_ => {},
		}

		Some(results)
	}

	fn handle_normal_mode_event(&mut self, event: Event, view_state: &crate::view::State) -> Results {
		if let Some(results) = self.handle_common_list_input(event, view_state) {
			results
		}
		else {
			let mut results = Results::new();
			if let Event::MetaEvent(meta_event) = event {
				match meta_event {
					MetaEvent::ActionBreak => self.action_break(),
					MetaEvent::Edit => self.edit(),
					MetaEvent::InsertLine => self.insert_line(&mut results),
					MetaEvent::ShowCommit => self.show_commit(&mut results),
					MetaEvent::FixupKeepMessage => self.toggle_option("-C"),
					MetaEvent::FixupKeepMessageWithEditor => self.toggle_option("-c"),
					_ => {},
				}
			}
			results
		}
	}

	fn handle_visual_mode_input(&mut self, event: Event, view_state: &crate::view::State) -> Results {
		self.handle_common_list_input(event, view_state)
			.unwrap_or_else(Results::new)
	}

	fn handle_edit_mode_input(&mut self, event: Event) -> Results {
		self.edit.handle_event(event);
		if self.edit.is_finished() {
			let mut todo_file = self.todo_file.lock();
			let selected_index = todo_file.get_selected_line_index();
			todo_file.update_range(
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
