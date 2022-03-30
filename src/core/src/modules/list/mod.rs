mod utils;

#[cfg(all(unix, test))]
mod tests;

use std::cmp::min;

use captur::capture;
use config::Config;
use display::DisplayColor;
use input::{Event, InputOptions, KeyBindings, MetaEvent, MouseEventKind};
use todo_file::{Action, EditContext, Line, TodoFile};
use view::{LineSegment, RenderContext, ViewData, ViewLine, ViewSender};

use self::utils::{get_list_normal_mode_help_lines, get_list_visual_mode_help_lines, get_todo_line_segments};
use crate::{
	components::{edit::Edit, help::Help},
	module::{ExitStatus, Module, ProcessResult, State},
	select,
};

// TODO Remove `union` call when bitflags/bitflags#180 is resolved
const INPUT_OPTIONS: InputOptions = InputOptions::UNDO_REDO
	.union(InputOptions::RESIZE)
	.union(InputOptions::HELP);

#[derive(Debug, PartialEq)]
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

	fn handle_event(&mut self, event: Event, view_sender: &ViewSender, todo_file: &mut TodoFile) -> ProcessResult {
		select!(
			default || {
				match self.state {
					ListState::Normal => self.handle_normal_mode_event(event, view_sender, todo_file),
					ListState::Visual => self.handle_visual_mode_input(event, view_sender, todo_file),
					ListState::Edit => self.handle_edit_mode_input(event, todo_file),
				}
			},
			|| self.handle_normal_help_input(event, view_sender),
			|| self.handle_visual_help_input(event, view_sender)
		)
	}

	fn input_options(&self) -> &InputOptions {
		select!(
			default || &INPUT_OPTIONS,
			|| self.normal_mode_help.input_options(),
			|| self.visual_mode_help.input_options(),
			|| (self.state == ListState::Edit).then(|| self.edit.input_options())
		)
	}

	fn read_event(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		select!(default || event, || self.read_event_help(event), || {
			Some(self.read_event_default(event, key_bindings))
		})
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
			state: ListState::Normal,
			view_data,
			visual_index_start: None,
			visual_mode_help: Help::new_from_keybindings(&get_list_visual_mode_help_lines(&config.key_bindings)),
		}
	}

	#[allow(clippy::unused_self)]
	fn set_cursor(&self, todo_file: &mut TodoFile, cursor: usize) {
		todo_file.set_selected_line_index(cursor);
	}

	#[allow(clippy::unused_self)]
	fn move_cursor_left(&self, view_sender: &ViewSender) {
		view_sender.scroll_left();
	}

	#[allow(clippy::unused_self)]
	fn move_cursor_right(&self, view_sender: &ViewSender) {
		view_sender.scroll_right();
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
	fn abort(&self, result: ProcessResult) -> ProcessResult {
		result.state(State::ConfirmAbort)
	}

	#[allow(clippy::unused_self)]
	fn force_abort(&self, rebase_todo: &mut TodoFile, result: ProcessResult) -> ProcessResult {
		rebase_todo.set_lines(vec![]);
		result.exit_status(ExitStatus::Good)
	}

	#[allow(clippy::unused_self)]
	fn rebase(&self, result: ProcessResult) -> ProcessResult {
		result.state(State::ConfirmRebase)
	}

	#[allow(clippy::unused_self)]
	const fn force_rebase(&self, result: ProcessResult) -> ProcessResult {
		result.exit_status(ExitStatus::Good)
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
	fn open_in_editor(&mut self, result: ProcessResult) -> ProcessResult {
		result.state(State::ExternalEditor)
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
	fn show_commit(&mut self, todo_file: &TodoFile, mut result: ProcessResult) -> ProcessResult {
		if let Some(selected_line) = todo_file.get_selected_line() {
			if selected_line.has_reference() {
				result = result.state(State::ShowCommit);
			}
		}
		result
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
				self.edit
					.set_label(format!("{} ", selected_line.get_action().as_string()).as_str());
			}
		}
	}

	#[allow(clippy::unused_self)]
	fn insert_line(&mut self, result: ProcessResult) -> ProcessResult {
		result.state(State::Insert)
	}

	fn update_list_view_data(&mut self, context: &RenderContext, todo_file: &TodoFile) -> &ViewData {
		let is_visual_mode = self.state == ListState::Visual;
		let selected_index = todo_file.get_selected_line_index();
		let visual_index = self.visual_index_start.unwrap_or(selected_index);

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
					let mut view_line = ViewLine::new_with_pinned_segments(
						get_todo_line_segments(line, selected_index == index, selected_line, context.is_full_width()),
						if *line.get_action() == Action::Exec { 2 } else { 3 },
					)
					.set_selected(selected_index == index || selected_line);

					if selected_index == index || selected_line {
						view_line = view_line.set_selected(true).set_padding(' ');
					}

					updater.push_line(view_line);
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

	fn read_event_help(&self, event: Event) -> Option<Event> {
		(self.visual_mode_help.is_active() || self.normal_mode_help.is_active()).then(|| event)
	}

	#[allow(clippy::cognitive_complexity)]
	fn read_event_default(&self, event: Event, key_bindings: &KeyBindings) -> Event {
		match self.state {
			ListState::Normal | ListState::Visual => {
				match event {
					e if key_bindings.abort.contains(&e) => Event::from(MetaEvent::Abort),
					e if key_bindings.action_break.contains(&e) => Event::from(MetaEvent::ActionBreak),
					e if key_bindings.action_drop.contains(&e) => Event::from(MetaEvent::ActionDrop),
					e if key_bindings.action_edit.contains(&e) => Event::from(MetaEvent::ActionEdit),
					e if key_bindings.action_fixup.contains(&e) => Event::from(MetaEvent::ActionFixup),
					e if key_bindings.action_pick.contains(&e) => Event::from(MetaEvent::ActionPick),
					e if key_bindings.action_reword.contains(&e) => Event::from(MetaEvent::ActionReword),
					e if key_bindings.action_squash.contains(&e) => Event::from(MetaEvent::ActionSquash),
					e if key_bindings.edit.contains(&e) => Event::from(MetaEvent::Edit),
					e if key_bindings.force_abort.contains(&e) => Event::from(MetaEvent::ForceAbort),
					e if key_bindings.force_rebase.contains(&e) => Event::from(MetaEvent::ForceRebase),
					e if key_bindings.insert_line.contains(&e) => Event::from(MetaEvent::InsertLine),
					e if key_bindings.move_down.contains(&e) => Event::from(MetaEvent::MoveCursorDown),
					e if key_bindings.move_down_step.contains(&e) => Event::from(MetaEvent::MoveCursorPageDown),
					e if key_bindings.move_end.contains(&e) => Event::from(MetaEvent::MoveCursorEnd),
					e if key_bindings.move_home.contains(&e) => Event::from(MetaEvent::MoveCursorHome),
					e if key_bindings.move_left.contains(&e) => Event::from(MetaEvent::MoveCursorLeft),
					e if key_bindings.move_right.contains(&e) => Event::from(MetaEvent::MoveCursorRight),
					e if key_bindings.move_selection_down.contains(&e) => Event::from(MetaEvent::SwapSelectedDown),
					e if key_bindings.move_selection_up.contains(&e) => Event::from(MetaEvent::SwapSelectedUp),
					e if key_bindings.move_up.contains(&e) => Event::from(MetaEvent::MoveCursorUp),
					e if key_bindings.move_up_step.contains(&e) => Event::from(MetaEvent::MoveCursorPageUp),
					e if key_bindings.open_in_external_editor.contains(&e) => Event::from(MetaEvent::OpenInEditor),
					e if key_bindings.rebase.contains(&e) => Event::from(MetaEvent::Rebase),
					e if key_bindings.remove_line.contains(&e) => Event::from(MetaEvent::Delete),
					e if key_bindings.show_commit.contains(&e) => Event::from(MetaEvent::ShowCommit),
					e if key_bindings.toggle_visual_mode.contains(&e) => Event::from(MetaEvent::ToggleVisualMode),
					Event::Mouse(mouse_event) => {
						match mouse_event.kind {
							MouseEventKind::ScrollDown => Event::from(MetaEvent::MoveCursorDown),
							MouseEventKind::ScrollUp => Event::from(MetaEvent::MoveCursorUp),
							_ => event,
						}
					},
					_ => event,
				}
			},
			ListState::Edit => event,
		}
	}

	fn handle_normal_help_input(&mut self, event: Event, view_sender: &ViewSender) -> Option<ProcessResult> {
		self.normal_mode_help.is_active().then(|| {
			self.normal_mode_help.handle_event(event, view_sender);
			ProcessResult::from(event)
		})
	}

	fn handle_visual_help_input(&mut self, event: Event, view_sender: &ViewSender) -> Option<ProcessResult> {
		self.visual_mode_help.is_active().then(|| {
			self.visual_mode_help.handle_event(event, view_sender);
			ProcessResult::from(event)
		})
	}

	#[allow(clippy::integer_division)]
	fn handle_common_list_input(
		&mut self,
		event: Event,
		view_sender: &ViewSender,
		rebase_todo: &mut TodoFile,
	) -> Option<ProcessResult> {
		let mut result = ProcessResult::from(event);
		match event {
			Event::Meta(meta_event) => {
				match meta_event {
					MetaEvent::MoveCursorLeft => self.move_cursor_left(view_sender),
					MetaEvent::MoveCursorRight => self.move_cursor_right(view_sender),
					MetaEvent::MoveCursorUp => self.move_cursor_up(rebase_todo, 1),
					MetaEvent::MoveCursorDown => self.move_cursor_down(rebase_todo, 1),
					MetaEvent::MoveCursorPageUp => self.move_cursor_up(rebase_todo, self.height / 2),
					MetaEvent::MoveCursorPageDown => self.move_cursor_down(rebase_todo, self.height / 2),
					MetaEvent::MoveCursorHome => self.move_cursor_home(rebase_todo),
					MetaEvent::MoveCursorEnd => self.move_cursor_end(rebase_todo),
					MetaEvent::Abort => result = self.abort(result),
					MetaEvent::ForceAbort => result = self.force_abort(rebase_todo, result),
					MetaEvent::Rebase => result = self.rebase(result),
					MetaEvent::ForceRebase => result = self.force_rebase(result),
					MetaEvent::SwapSelectedDown => self.swap_selected_down(rebase_todo),
					MetaEvent::SwapSelectedUp => self.swap_selected_up(rebase_todo),
					MetaEvent::ActionDrop => self.set_selected_line_action(rebase_todo, Action::Drop),
					MetaEvent::ActionEdit => self.set_selected_line_action(rebase_todo, Action::Edit),
					MetaEvent::ActionFixup => self.set_selected_line_action(rebase_todo, Action::Fixup),
					MetaEvent::ActionPick => self.set_selected_line_action(rebase_todo, Action::Pick),
					MetaEvent::ActionReword => self.set_selected_line_action(rebase_todo, Action::Reword),
					MetaEvent::ActionSquash => self.set_selected_line_action(rebase_todo, Action::Squash),
					MetaEvent::Undo => self.undo(rebase_todo),
					MetaEvent::Redo => self.redo(rebase_todo),
					MetaEvent::Delete => self.delete(rebase_todo),
					MetaEvent::OpenInEditor => result = self.open_in_editor(result),
					MetaEvent::ToggleVisualMode => self.toggle_visual_mode(rebase_todo),
					MetaEvent::Help => self.help(),
					_ => return None,
				}
			},
			Event::Resize(_, height) => self.resize(height),
			_ => {},
		}

		Some(result)
	}

	fn handle_normal_mode_event(
		&mut self,
		event: Event,
		view_sender: &ViewSender,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		if let Some(result) = self.handle_common_list_input(event, view_sender, rebase_todo) {
			result
		}
		else {
			let mut result = ProcessResult::from(event);
			if let Event::Meta(meta_event) = event {
				match meta_event {
					MetaEvent::ShowCommit => result = self.show_commit(rebase_todo, result),
					MetaEvent::ActionBreak => self.action_break(rebase_todo),
					MetaEvent::Edit => self.edit(rebase_todo),
					MetaEvent::InsertLine => result = self.insert_line(result),
					_ => {},
				}
			}
			result
		}
	}

	fn handle_visual_mode_input(
		&mut self,
		event: Event,
		view_sender: &ViewSender,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		self.handle_common_list_input(event, view_sender, rebase_todo)
			.unwrap_or_else(|| ProcessResult::from(event))
	}

	fn handle_edit_mode_input(&mut self, event: Event, rebase_todo: &mut TodoFile) -> ProcessResult {
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
		ProcessResult::from(event)
	}
}
