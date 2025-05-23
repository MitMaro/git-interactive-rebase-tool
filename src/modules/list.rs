mod search;
#[cfg(all(unix, test))]
mod tests;
mod utils;

use std::{cmp::min, sync::Arc};

use captur::capture;
use parking_lot::Mutex;

use self::{
	search::Search,
	utils::{
		TodoLineSegmentsOptions,
		get_list_normal_mode_help_lines,
		get_list_visual_mode_help_lines,
		get_todo_line_segments,
	},
};
use crate::{
	application::AppData,
	components::{
		edit::Edit,
		help::Help,
		search_bar::{SearchBar, SearchBarAction},
		spin_indicator::SpinIndicator,
	},
	display::DisplayColor,
	input::{Event, InputOptions, KeyBindings, MouseEventKind, StandardEvent},
	module::{ExitStatus, Module, State},
	modules::list::utils::get_line_action_maximum_width,
	process::Results,
	search::Searchable,
	select,
	todo_file::{Action, EditContext, Line, TodoFile},
	view,
	view::{LineSegment, RenderContext, ViewData, ViewLine},
};

// TODO Remove `union` call when bitflags/bitflags#180 is resolved
const INPUT_OPTIONS: InputOptions = InputOptions::UNDO_REDO
	.union(InputOptions::RESIZE)
	.union(InputOptions::HELP)
	.union(InputOptions::SEARCH_START);

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
	spin_indicator: SpinIndicator,
	state: ListState,
	todo_file: Arc<Mutex<TodoFile>>,
	view_data: ViewData,
	view_state: view::State,
	visual_index_start: Option<usize>,
	visual_mode_help: Help,
}

impl Module for List {
	fn activate(&mut self, _: State) -> Results {
		self.selected_line_action = self.todo_file.lock().get_selected_line().map(|line| *line.get_action());
		let searchable: Box<dyn Searchable> = Box::new(self.search.clone());
		let mut results = Results::from(searchable);
		if let Some(term) = self.search_bar.search_value() {
			results.search_term(term);
		}
		results
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

	fn handle_event(&mut self, event: Event) -> Results {
		select!(
			default {
				match self.state {
					ListState::Normal => self.handle_normal_mode_event(event),
					ListState::Visual => self.handle_visual_mode_input(event),
					ListState::Edit => self.handle_edit_mode_input(event),
				}
			},
			self.normal_mode_help.handle_event(event, &self.view_state),
			self.visual_mode_help.handle_event(event, &self.view_state),
			self.handle_search_input(event)
		)
	}

	fn input_options(&self) -> &InputOptions {
		select!(
			default & INPUT_OPTIONS,
			(self.state == ListState::Edit).then(|| self.edit.input_options()),
			self.normal_mode_help.input_options(),
			self.visual_mode_help.input_options(),
			self.search_bar.input_options()
		)
	}

	fn read_event(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		select!(
			default self.read_event_default(event, key_bindings),
			(self.state == ListState::Edit).then_some(event),
			self.normal_mode_help.read_event(event),
			self.visual_mode_help.read_event(event),
			self.search_bar.read_event(event)
		)
	}
}

impl List {
	pub(crate) fn new(app_data: &AppData) -> Self {
		let view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
			updater.set_show_help(true);
		});

		let config = app_data.config();

		Self {
			auto_select_next: config.auto_select_next,
			edit: Edit::new(),
			height: 0,
			normal_mode_help: Help::new_from_keybindings(&get_list_normal_mode_help_lines(&config.key_bindings)),
			search: Search::new(app_data.todo_file()),
			search_bar: SearchBar::new(),
			selected_line_action: None,
			spin_indicator: SpinIndicator::new(),
			state: ListState::Normal,
			todo_file: app_data.todo_file(),
			view_data,
			view_state: app_data.view_state(),
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

	fn move_cursor_left(&self) {
		self.view_state.scroll_left();
	}

	fn move_cursor_right(&self) {
		self.view_state.scroll_right();
	}

	fn abort(&self, results: &mut Results) {
		results.state(State::ConfirmAbort);
	}

	fn force_abort(&self, results: &mut Results) {
		let mut todo_file = self.todo_file.lock();
		todo_file.set_lines(vec![]);
		results.exit_status(ExitStatus::Good);
	}

	fn rebase(&self, results: &mut Results) {
		results.state(State::ConfirmRebase);
	}

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

	fn open_in_editor(&mut self, results: &mut Results) {
		results.search_cancel();
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
		self.search_bar.start_search(Some(""));
	}

	fn search_update(&mut self) {
		self.spin_indicator.refresh();
		// select the first match, if it is available and has not been previously selected
		if let Some(selected) = self.search.current_match() {
			_ = self.update_cursor(CursorUpdate::Set(selected.index()));
		}
		else if !self.search_bar.is_editing() {
			if let Some(selected) = self.search.next() {
				_ = self.update_cursor(CursorUpdate::Set(selected));
			}
		}
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
			.is_some_and(|line| line.get_action() == &Action::Break);

		// no need to add an additional break when the next line is already a break
		if next_action_is_break {
			return;
		}

		let selected_action_is_break = todo_file
			.get_line(selected_line_index)
			.is_some_and(|line| line.get_action() == &Action::Break);

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
				self.edit.reset();
				self.edit.set_content(selected_line.get_content());
				self.edit.set_label(format!("{} ", selected_line.get_action()).as_str());
			}
		}
	}

	fn insert_line(&mut self, results: &mut Results) {
		results.state(State::Insert);
	}

	fn duplicate_line(&mut self) {
		let mut todo_file = self.todo_file.lock();

		if let Some(selected_line) = todo_file.get_selected_line() {
			if selected_line.is_duplicatable() {
				let new_line = selected_line.clone();
				let selected_line_index = todo_file.get_selected_line_index();
				todo_file.add_line(selected_line_index + 1, new_line);
			}
		}
	}

	fn update_list_view_data(&mut self, context: &RenderContext) -> &ViewData {
		let todo_file = self.todo_file.lock();
		let is_visual_mode = self.state == ListState::Visual;
		let selected_index = todo_file.get_selected_line_index();
		let visual_index = self.visual_index_start.unwrap_or(selected_index);
		let search_view_line = self.search_bar.is_editing().then(|| self.search_bar.build_view_line());
		let search_results_total = self.search.total_results();
		let search_results_current = self.search.current_result_selected().unwrap_or(0);
		let search_term = self.search_bar.search_value();
		let search_index = self.search.current_match();
		let search_active = self.search.is_active();
		let spin_indicator = self.spin_indicator.indicator();

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
					let search_match = self.search.match_at_index(index);
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
					if search_index.is_some_and(|v| v.index() == index) {
						todo_line_segment_options.insert(TodoLineSegmentsOptions::SEARCH_LINE);
					}
					let mut view_line = ViewLine::new_with_pinned_segments(
						get_todo_line_segments(
							line,
							search_term,
							search_match,
							todo_line_segment_options,
							maximum_action_width,
						),
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
					if search_results_total == 0 && !search_active {
						search_line_segments.push(LineSegment::new("No Results"));
					}
					else {
						search_line_segments.push(LineSegment::new(
							format!("{}/{search_results_total}", search_results_current + 1).as_str(),
						));
					}

					if search_active {
						search_line_segments.push(LineSegment::new(format!(" Searching [{spin_indicator}]").as_str()));
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

	#[expect(clippy::cognitive_complexity, reason = "Legacy: needs refactor")]
	fn read_event_default(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		// handle action level events
		if let Some(action) = self.selected_line_action {
			if action == Action::Fixup {
				match event {
					e if key_bindings.fixup_keep_message.contains(&e) => {
						return Event::from(StandardEvent::FixupKeepMessage);
					},
					e if key_bindings.fixup_keep_message_with_editor.contains(&e) => {
						return Event::from(StandardEvent::FixupKeepMessageWithEditor);
					},
					_ => {},
				}
			}
		}

		match event {
			e if key_bindings.abort.contains(&e) => Event::from(StandardEvent::Abort),
			e if key_bindings.action_break.contains(&e) => Event::from(StandardEvent::ActionBreak),
			e if key_bindings.action_drop.contains(&e) => Event::from(StandardEvent::ActionDrop),
			e if key_bindings.action_edit.contains(&e) => Event::from(StandardEvent::ActionEdit),
			e if key_bindings.action_fixup.contains(&e) => Event::from(StandardEvent::ActionFixup),
			e if key_bindings.action_pick.contains(&e) => Event::from(StandardEvent::ActionPick),
			e if key_bindings.action_reword.contains(&e) => Event::from(StandardEvent::ActionReword),
			e if key_bindings.action_squash.contains(&e) => Event::from(StandardEvent::ActionSquash),
			e if key_bindings.edit.contains(&e) => Event::from(StandardEvent::Edit),
			e if key_bindings.force_abort.contains(&e) => Event::from(StandardEvent::ForceAbort),
			e if key_bindings.force_rebase.contains(&e) => Event::from(StandardEvent::ForceRebase),
			e if key_bindings.insert_line.contains(&e) => Event::from(StandardEvent::InsertLine),
			e if key_bindings.duplicate_line.contains(&e) => Event::from(StandardEvent::DuplicateLine),
			e if key_bindings.move_down.contains(&e) => Event::from(StandardEvent::MoveCursorDown),
			e if key_bindings.move_down_step.contains(&e) => Event::from(StandardEvent::MoveCursorPageDown),
			e if key_bindings.move_end.contains(&e) => Event::from(StandardEvent::MoveCursorEnd),
			e if key_bindings.move_home.contains(&e) => Event::from(StandardEvent::MoveCursorHome),
			e if key_bindings.move_left.contains(&e) => Event::from(StandardEvent::MoveCursorLeft),
			e if key_bindings.move_right.contains(&e) => Event::from(StandardEvent::MoveCursorRight),
			e if key_bindings.move_selection_down.contains(&e) => Event::from(StandardEvent::SwapSelectedDown),
			e if key_bindings.move_selection_up.contains(&e) => Event::from(StandardEvent::SwapSelectedUp),
			e if key_bindings.move_up.contains(&e) => Event::from(StandardEvent::MoveCursorUp),
			e if key_bindings.move_up_step.contains(&e) => Event::from(StandardEvent::MoveCursorPageUp),
			e if key_bindings.open_in_external_editor.contains(&e) => Event::from(StandardEvent::OpenInEditor),
			e if key_bindings.rebase.contains(&e) => Event::from(StandardEvent::Rebase),
			e if key_bindings.remove_line.contains(&e) => Event::from(StandardEvent::Delete),
			e if key_bindings.show_commit.contains(&e) => Event::from(StandardEvent::ShowCommit),
			e if key_bindings.toggle_visual_mode.contains(&e) => Event::from(StandardEvent::ToggleVisualMode),
			Event::Mouse(mouse_event) => {
				match mouse_event.kind {
					MouseEventKind::ScrollDown => Event::from(StandardEvent::MoveCursorDown),
					MouseEventKind::ScrollUp => Event::from(StandardEvent::MoveCursorUp),
					_ => event,
				}
			},
			_ => event,
		}
	}

	fn handle_search_input(&mut self, event: Event) -> Option<Results> {
		if !self.search_bar.is_active() {
			return None;
		}

		let mut results = Results::from(event);
		let todo_file = self.todo_file.lock();
		match self.search_bar.handle_event(event) {
			SearchBarAction::Update(term) => {
				if term.is_empty() {
					results.search_cancel();
				}
				else {
					results.search_term(term.as_str());
				}
			},
			SearchBarAction::Start(term) => {
				if term.is_empty() {
					results.search_cancel();
					self.search_bar.reset();
				}
				else {
					results.search_term(term.as_str());
				}
			},
			SearchBarAction::Next(term) => {
				results.search_term(term.as_str());
				_ = self.search.next();
			},
			SearchBarAction::Previous(term) => {
				results.search_term(term.as_str());
				_ = self.search.previous();
			},
			SearchBarAction::Cancel => {
				results.search_cancel();
				return Some(results);
			},
			SearchBarAction::None => return None,
		}
		drop(todo_file);

		self.search_update();
		Some(results)
	}

	#[expect(clippy::integer_division, reason = "Truncation desired")]
	fn handle_common_list_input(&mut self, event: Event) -> Option<Results> {
		let mut results = Results::new();
		match event {
			Event::Standard(standard_event) => {
				match standard_event {
					StandardEvent::Abort => self.abort(&mut results),
					StandardEvent::ActionDrop => self.set_selected_line_action(Action::Drop),
					StandardEvent::ActionEdit => self.set_selected_line_action(Action::Edit),
					StandardEvent::ActionFixup => self.set_selected_line_action(Action::Fixup),
					StandardEvent::ActionPick => self.set_selected_line_action(Action::Pick),
					StandardEvent::ActionReword => self.set_selected_line_action(Action::Reword),
					StandardEvent::ActionSquash => self.set_selected_line_action(Action::Squash),
					StandardEvent::Delete => self.delete(),
					StandardEvent::ForceAbort => self.force_abort(&mut results),
					StandardEvent::ForceRebase => self.force_rebase(&mut results),
					StandardEvent::MoveCursorDown => {
						_ = self.update_cursor(CursorUpdate::Down(1));
					},
					StandardEvent::MoveCursorEnd => {
						_ = self.update_cursor(CursorUpdate::End);
					},
					StandardEvent::MoveCursorHome => {
						_ = self.update_cursor(CursorUpdate::Set(0));
					},
					StandardEvent::MoveCursorLeft => self.move_cursor_left(),
					StandardEvent::MoveCursorPageDown => {
						_ = self.update_cursor(CursorUpdate::Down(self.height / 2));
					},
					StandardEvent::MoveCursorPageUp => {
						_ = self.update_cursor(CursorUpdate::Up(self.height / 2));
					},
					StandardEvent::MoveCursorRight => self.move_cursor_right(),
					StandardEvent::MoveCursorUp => {
						_ = self.update_cursor(CursorUpdate::Up(1));
					},
					StandardEvent::OpenInEditor => self.open_in_editor(&mut results),
					StandardEvent::Rebase => self.rebase(&mut results),
					StandardEvent::SwapSelectedDown => self.swap_selected_down(),
					StandardEvent::SwapSelectedUp => self.swap_selected_up(),
					StandardEvent::ToggleVisualMode => self.toggle_visual_mode(),
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

	fn handle_normal_mode_event(&mut self, event: Event) -> Results {
		if let Some(results) = self.handle_common_list_input(event) {
			results
		}
		else {
			let mut results = Results::new();
			if let Event::Standard(standard_event) = event {
				match standard_event {
					StandardEvent::ActionBreak => self.action_break(),
					StandardEvent::Edit => self.edit(),
					StandardEvent::InsertLine => self.insert_line(&mut results),
					StandardEvent::DuplicateLine => self.duplicate_line(),
					StandardEvent::ShowCommit => self.show_commit(&mut results),
					StandardEvent::FixupKeepMessage => self.toggle_option("-C"),
					StandardEvent::FixupKeepMessageWithEditor => self.toggle_option("-c"),
					_ => {},
				}
			}
			results
		}
	}

	fn handle_visual_mode_input(&mut self, event: Event) -> Results {
		self.handle_common_list_input(event).unwrap_or_else(Results::new)
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
