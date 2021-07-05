mod input;
mod utils;

#[cfg(all(unix, test))]
mod tests;

use std::cmp::min;

use ::input::{Event, EventHandler, MetaEvent};
use config::Config;
use display::DisplayColor;
use todo_file::{Action, EditContext, Line, TodoFile};
use view::{LineSegment, RenderContext, ViewData, ViewLine, ViewSender};

use self::{
	input::get_event,
	utils::{get_list_normal_mode_help_lines, get_list_visual_mode_help_lines, get_todo_line_segments},
};
use crate::{
	components::{edit::Edit, help::Help},
	core::module::{ExitStatus, Module, ProcessResult, State},
};

#[derive(Debug, PartialEq)]
enum ListState {
	Normal,
	Visual,
	Edit,
}

pub struct List {
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
			ListState::Edit => self.edit.get_view_data(),
		}
	}

	fn handle_events(
		&mut self,
		event_handler: &EventHandler,
		view_sender: &ViewSender,
		todo_file: &mut TodoFile,
	) -> ProcessResult {
		match self.state {
			ListState::Normal => self.handle_normal_mode_input(event_handler, view_sender, todo_file),
			ListState::Visual => self.handle_visual_mode_input(event_handler, view_sender, todo_file),
			ListState::Edit => self.handle_edit_mode_input(event_handler, todo_file),
		}
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

	pub(crate) fn move_cursor_up(todo_file: &mut TodoFile, amount: usize) {
		let current_selected_line_index = todo_file.get_selected_line_index();
		todo_file.set_selected_line_index(
			if amount > current_selected_line_index {
				0
			}
			else {
				current_selected_line_index - amount
			},
		);
	}

	pub(crate) fn move_cursor_down(rebase_todo: &mut TodoFile, amount: usize) {
		let current_selected_line_index = rebase_todo.get_selected_line_index();
		rebase_todo.set_selected_line_index(current_selected_line_index + amount);
	}

	fn set_selected_line_action(&self, rebase_todo: &mut TodoFile, action: Action) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		rebase_todo.update_range(start_index, end_index, &EditContext::new().action(action));
		if self.state == ListState::Normal && self.auto_select_next {
			Self::move_cursor_down(rebase_todo, 1);
		}
	}

	fn update_list_view_data(&mut self, context: &RenderContext, todo_file: &TodoFile) -> &ViewData {
		let is_visual_mode = self.state == ListState::Visual;
		let selected_index = todo_file.get_selected_line_index();
		let visual_index = self.visual_index_start.unwrap_or(selected_index);

		self.view_data.update_view_data(|updater| {
			updater.clear();
			if todo_file.is_empty() {
				updater.push_leading_line(ViewLine::from(LineSegment::new_with_color(
					"Rebase todo file is empty",
					DisplayColor::IndicatorColor,
				)));
			}
			else {
				for (index, line) in todo_file.iter().enumerate() {
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
					MetaEvent::MoveCursorLeft => view_sender.scroll_left(),
					MetaEvent::MoveCursorRight => view_sender.scroll_right(),
					MetaEvent::MoveCursorDown => Self::move_cursor_down(rebase_todo, 1),
					MetaEvent::MoveCursorUp => Self::move_cursor_up(rebase_todo, 1),
					MetaEvent::MoveCursorPageDown => Self::move_cursor_down(rebase_todo, self.height / 2),
					MetaEvent::MoveCursorPageUp => Self::move_cursor_up(rebase_todo, self.height / 2),
					MetaEvent::MoveCursorHome => rebase_todo.set_selected_line_index(0),
					MetaEvent::MoveCursorEnd => {
						rebase_todo.set_selected_line_index(rebase_todo.get_max_selected_line_index());
					},
					MetaEvent::Abort => result = result.state(State::ConfirmAbort),
					MetaEvent::ForceAbort => {
						rebase_todo.set_lines(vec![]);
						result = result.exit_status(ExitStatus::Good);
					},
					MetaEvent::Rebase => result = result.state(State::ConfirmRebase),
					MetaEvent::ForceRebase => result = result.exit_status(ExitStatus::Good),
					MetaEvent::SwapSelectedDown => {
						let start_index = rebase_todo.get_selected_line_index();
						let end_index = self.visual_index_start.unwrap_or(start_index);

						if rebase_todo.swap_range_down(start_index, end_index) {
							if let Some(visual_index_start) = self.visual_index_start {
								self.visual_index_start = Some(visual_index_start + 1);
							}
							Self::move_cursor_down(rebase_todo, 1);
						}
					},
					MetaEvent::SwapSelectedUp => {
						let start_index = rebase_todo.get_selected_line_index();
						let end_index = self.visual_index_start.unwrap_or(start_index);

						if rebase_todo.swap_range_up(start_index, end_index) {
							if let Some(visual_index_start) = self.visual_index_start {
								self.visual_index_start = Some(visual_index_start - 1);
							}
							Self::move_cursor_up(rebase_todo, 1);
						}
					},
					MetaEvent::ActionDrop => self.set_selected_line_action(rebase_todo, Action::Drop),
					MetaEvent::ActionEdit => self.set_selected_line_action(rebase_todo, Action::Edit),
					MetaEvent::ActionFixup => self.set_selected_line_action(rebase_todo, Action::Fixup),
					MetaEvent::ActionPick => self.set_selected_line_action(rebase_todo, Action::Pick),
					MetaEvent::ActionReword => self.set_selected_line_action(rebase_todo, Action::Reword),
					MetaEvent::ActionSquash => self.set_selected_line_action(rebase_todo, Action::Squash),
					MetaEvent::Undo => {
						if let Some((start_index, end_index)) = rebase_todo.undo() {
							rebase_todo.set_selected_line_index(start_index);
							if start_index == end_index {
								self.state = ListState::Normal;
								self.visual_index_start = None;
							}
							else {
								self.state = ListState::Visual;
								self.visual_index_start = Some(end_index);
							}
						}
					},
					MetaEvent::Redo => {
						if let Some((start_index, end_index)) = rebase_todo.redo() {
							rebase_todo.set_selected_line_index(start_index);
							if start_index == end_index {
								self.state = ListState::Normal;
								self.visual_index_start = None;
							}
							else {
								self.state = ListState::Visual;
								self.visual_index_start = Some(end_index);
							}
						}
					},
					MetaEvent::Delete => {
						let start_index = rebase_todo.get_selected_line_index();
						let end_index = self.visual_index_start.unwrap_or(start_index);

						rebase_todo.remove_lines(start_index, end_index);
						let new_index = min(start_index, end_index);

						rebase_todo.set_selected_line_index(new_index);

						if self.state == ListState::Visual {
							self.visual_index_start = Some(rebase_todo.get_selected_line_index());
						}
					},
					MetaEvent::OpenInEditor => result = result.state(State::ExternalEditor),
					MetaEvent::ToggleVisualMode => {
						if self.state == ListState::Visual {
							self.state = ListState::Normal;
							self.visual_index_start = None;
						}
						else {
							self.state = ListState::Visual;
							self.visual_index_start = Some(rebase_todo.get_selected_line_index());
						}
					},
					MetaEvent::Help => {
						if self.state == ListState::Visual {
							self.visual_mode_help.set_active();
						}
						else {
							self.normal_mode_help.set_active();
						}
					},
					_ => return None,
				}
			},
			Event::Resize(_, height) => {
				self.height = height as usize;
			},
			_ => {},
		}

		Some(result)
	}

	fn handle_normal_mode_input(
		&mut self,
		event_handler: &EventHandler,
		view_sender: &ViewSender,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		if self.normal_mode_help.is_active() {
			return ProcessResult::from(self.normal_mode_help.handle_event(event_handler, view_sender));
		}

		let event = get_event(event_handler);
		if let Some(result) = self.handle_common_list_input(event, view_sender, rebase_todo) {
			result
		}
		else {
			let mut result = ProcessResult::from(event);
			if let Event::Meta(meta_event) = event {
				match meta_event {
					MetaEvent::ShowCommit => {
						if let Some(selected_line) = rebase_todo.get_selected_line() {
							if selected_line.has_reference() {
								result = result.state(State::ShowCommit);
							}
						}
					},
					MetaEvent::ActionBreak => {
						let selected_line_index = rebase_todo.get_selected_line_index();
						let next_action_is_break = rebase_todo
							.get_line(selected_line_index + 1)
							.map_or(false, |line| line.get_action() == &Action::Break);
						if !next_action_is_break {
							let selected_action_is_break = rebase_todo
								.get_line(selected_line_index)
								.map_or(false, |line| line.get_action() == &Action::Break);
							if selected_action_is_break {
								rebase_todo.remove_lines(selected_line_index, selected_line_index);
								Self::move_cursor_up(rebase_todo, 1);
							}
							else {
								rebase_todo.add_line(selected_line_index + 1, Line::new_break());
								Self::move_cursor_down(rebase_todo, 1);
							}
						}
					},
					MetaEvent::Edit => {
						if let Some(selected_line) = rebase_todo.get_selected_line() {
							if selected_line.is_editable() {
								self.state = ListState::Edit;
								self.edit.set_content(selected_line.get_content());
								self.edit
									.set_label(format!("{} ", selected_line.get_action().as_string()).as_str());
								self.edit
									.set_description(format!("Modifying line: {}", selected_line.to_text()).as_str());
							}
						}
					},
					MetaEvent::InsertLine => result = result.state(State::Insert),
					_ => {},
				}
			}
			result
		}
	}

	fn handle_visual_mode_input(
		&mut self,
		event_handler: &EventHandler,
		view_sender: &ViewSender,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		if self.visual_mode_help.is_active() {
			return ProcessResult::from(self.visual_mode_help.handle_event(event_handler, view_sender));
		}

		let event = get_event(event_handler);
		self.handle_common_list_input(event, view_sender, rebase_todo)
			.unwrap_or_else(|| ProcessResult::from(event))
	}

	fn handle_edit_mode_input(&mut self, event_handler: &EventHandler, rebase_todo: &mut TodoFile) -> ProcessResult {
		let result = ProcessResult::from(self.edit.handle_event(event_handler));
		if self.edit.is_finished() {
			let selected_index = rebase_todo.get_selected_line_index();
			rebase_todo.update_range(
				selected_index,
				selected_index,
				&EditContext::new().content(self.edit.get_content().as_str()),
			);
			self.visual_index_start = None;
			self.state = ListState::Normal;
		}
		result
	}
}
