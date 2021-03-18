mod utils;

#[cfg(all(unix, test))]
mod tests;

use std::cmp::min;

use crate::{
	config::Config,
	display::display_color::DisplayColor,
	edit::Edit,
	input::{input_handler::InputMode, Input},
	list::utils::{get_list_normal_mode_help_lines, get_list_visual_mode_help_lines, get_todo_line_segments},
	process::{exit_status::ExitStatus, process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::{action::Action, edit_content::EditContext, line::Line, TodoFile},
	view::{line_segment::LineSegment, view_data::ViewData, view_line::ViewLine, View},
};

#[derive(Debug, PartialEq)]
enum ListState {
	Normal,
	Visual,
	Edit,
}

pub struct List<'l> {
	config: &'l Config,
	edit: Edit,
	normal_mode_help_lines: Vec<(Vec<String>, String)>,
	state: ListState,
	view_data: ViewData,
	visual_index_start: Option<usize>,
	visual_mode_help_lines: Vec<(Vec<String>, String)>,
}

impl<'l> ProcessModule for List<'l> {
	fn build_view_data(&mut self, view: &View<'_>, todo_file: &TodoFile) -> &ViewData {
		let view_width = view.get_view_size().width();
		let view_height = view.get_view_size().height();
		self.view_data.clear();
		self.view_data.set_view_size(view_width, view_height);

		match self.state {
			ListState::Normal | ListState::Visual => {
				let is_visual_mode = self.state == ListState::Visual;
				let selected_index = todo_file.get_selected_line_index();
				let visual_index = self.visual_index_start.unwrap_or(selected_index);

				if todo_file.is_empty() {
					self.view_data
						.push_leading_line(ViewLine::from(LineSegment::new_with_color(
							"Rebase todo file is empty",
							DisplayColor::IndicatorColor,
						)));
				}
				else {
					for (index, line) in todo_file.iter().enumerate() {
						let selected_line = is_visual_mode
							&& ((visual_index <= selected_index && index >= visual_index && index <= selected_index)
								|| (visual_index > selected_index && index >= selected_index && index <= visual_index));
						self.view_data.push_line(
							ViewLine::new_with_pinned_segments(
								get_todo_line_segments(line, selected_index == index, selected_line, view_width),
								if *line.get_action() == Action::Exec { 2 } else { 3 },
							)
							.set_selected(selected_index == index || selected_line),
						);
					}
				}
				self.view_data.rebuild();
				if let Some(visual_index) = self.visual_index_start {
					self.view_data.ensure_line_visible(visual_index);
				}
				self.view_data.ensure_line_visible(selected_index);
			},
			ListState::Edit => self.edit.update_view_data(&mut self.view_data),
		}

		&self.view_data
	}

	fn handle_input(&mut self, view: &mut View<'_>, todo_file: &mut TodoFile) -> ProcessResult {
		let mut result = ProcessResult::new();
		match self.state {
			ListState::Normal => {
				let input = view.get_input(InputMode::List);
				result = result.input(input);
				if !self.handle_move_cursor_inputs(view, todo_file, input) {
					result = self.handle_normal_mode_input(input, result, todo_file);
				}
			},
			ListState::Visual => {
				let input = view.get_input(InputMode::List);
				result = result.input(input);
				if !self.handle_move_cursor_inputs(view, todo_file, input) {
					result = self.handle_visual_mode_input(input, result, todo_file);
				}
			},
			ListState::Edit => {
				let input = view.get_input(InputMode::Raw);
				result = result.input(input);
				if !self.edit.handle_input(input) {
					self.handle_edit_mode_input(input, todo_file);
				}
			},
		}
		result
	}

	fn get_help_keybindings_descriptions(&self) -> Option<Vec<(Vec<String>, String)>> {
		match self.state {
			ListState::Normal => Some(self.normal_mode_help_lines.clone()),
			ListState::Visual => Some(self.visual_mode_help_lines.clone()),
			ListState::Edit => None,
		}
	}
}

impl<'l> List<'l> {
	pub(crate) fn new(config: &'l Config) -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		view_data.set_show_help(true);

		Self {
			config,
			normal_mode_help_lines: get_list_normal_mode_help_lines(&config.key_bindings),
			state: ListState::Normal,
			view_data,
			edit: Edit::new(),
			visual_index_start: None,
			visual_mode_help_lines: get_list_visual_mode_help_lines(&config.key_bindings),
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

	fn set_selected_line_action(&self, rebase_todo: &mut TodoFile, action: Action, advanced_next: bool) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		rebase_todo.update_range(start_index, end_index, &EditContext::new().action(action));
		if advanced_next && self.config.auto_select_next {
			Self::move_cursor_down(rebase_todo, 1);
		}
	}

	pub(crate) fn remove_lines(&mut self, rebase_todo: &mut TodoFile) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		rebase_todo.remove_lines(start_index, end_index);
		let new_index = min(start_index, end_index);

		rebase_todo.set_selected_line_index(new_index);

		if self.state == ListState::Visual {
			self.visual_index_start = Some(rebase_todo.get_selected_line_index());
		}
	}

	pub(crate) fn swap_range_up(&mut self, rebase_todo: &mut TodoFile) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		if rebase_todo.swap_range_up(start_index, end_index) {
			if let Some(visual_index_start) = self.visual_index_start {
				self.visual_index_start = Some(visual_index_start - 1);
			}
			Self::move_cursor_up(rebase_todo, 1);
		}
	}

	pub(crate) fn swap_range_down(&mut self, rebase_todo: &mut TodoFile) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		if rebase_todo.swap_range_down(start_index, end_index) {
			if let Some(visual_index_start) = self.visual_index_start {
				self.visual_index_start = Some(visual_index_start + 1);
			}

			Self::move_cursor_down(rebase_todo, 1);
		}
	}

	pub(crate) fn undo(&mut self, rebase_todo: &mut TodoFile) {
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
	}

	pub(crate) fn redo(&mut self, rebase_todo: &mut TodoFile) {
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
	}

	fn handle_normal_mode_input(
		&mut self,
		input: Input,
		result: ProcessResult,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		let mut result = result;
		match input {
			Input::ShowCommit => {
				if let Some(selected_line) = rebase_todo.get_selected_line() {
					if selected_line.has_reference() {
						result = result.state(State::ShowCommit);
					}
				}
			},
			Input::Abort => {
				result = result.state(State::ConfirmAbort);
			},
			Input::ForceAbort => {
				rebase_todo.set_lines(vec![]);
				result = result.exit_status(ExitStatus::Good);
			},
			Input::Rebase => {
				result = result.state(State::ConfirmRebase);
			},
			Input::ForceRebase => {
				result = result.exit_status(ExitStatus::Good);
			},
			Input::ActionBreak => {
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
			Input::ActionDrop => self.set_selected_line_action(rebase_todo, Action::Drop, true),
			Input::ActionEdit => self.set_selected_line_action(rebase_todo, Action::Edit, true),
			Input::ActionFixup => self.set_selected_line_action(rebase_todo, Action::Fixup, true),
			Input::ActionPick => self.set_selected_line_action(rebase_todo, Action::Pick, true),
			Input::ActionReword => self.set_selected_line_action(rebase_todo, Action::Reword, true),
			Input::ActionSquash => self.set_selected_line_action(rebase_todo, Action::Squash, true),
			Input::Edit => {
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
			Input::SwapSelectedDown => self.swap_range_down(rebase_todo),
			Input::SwapSelectedUp => self.swap_range_up(rebase_todo),
			Input::ToggleVisualMode => {
				self.visual_index_start = Some(rebase_todo.get_selected_line_index());
				self.state = ListState::Visual;
			},
			Input::OpenInEditor => result = result.state(State::ExternalEditor),
			Input::Undo => self.undo(rebase_todo),
			Input::Redo => self.redo(rebase_todo),
			Input::Delete => self.remove_lines(rebase_todo),
			_ => {},
		}

		result
	}

	fn handle_visual_mode_input(
		&mut self,
		input: Input,
		result: ProcessResult,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		let mut result = result;
		match input {
			Input::Abort => {
				result = result.state(State::ConfirmAbort);
			},
			Input::ForceAbort => {
				rebase_todo.set_lines(vec![]);
				result = result.exit_status(ExitStatus::Good);
			},
			Input::Rebase => {
				result = result.state(State::ConfirmRebase);
			},
			Input::ForceRebase => {
				result = result.exit_status(ExitStatus::Good);
			},
			Input::ActionDrop => self.set_selected_line_action(rebase_todo, Action::Drop, false),
			Input::ActionEdit => self.set_selected_line_action(rebase_todo, Action::Edit, false),
			Input::ActionFixup => self.set_selected_line_action(rebase_todo, Action::Fixup, false),
			Input::ActionPick => self.set_selected_line_action(rebase_todo, Action::Pick, false),
			Input::ActionReword => self.set_selected_line_action(rebase_todo, Action::Reword, false),
			Input::ActionSquash => self.set_selected_line_action(rebase_todo, Action::Squash, false),
			Input::SwapSelectedDown => self.swap_range_down(rebase_todo),
			Input::SwapSelectedUp => self.swap_range_up(rebase_todo),
			Input::ToggleVisualMode => {
				self.visual_index_start = None;
				self.state = ListState::Normal;
			},
			Input::OpenInEditor => result = result.state(State::ExternalEditor),
			Input::Undo => self.undo(rebase_todo),
			Input::Redo => self.redo(rebase_todo),
			Input::Delete => self.remove_lines(rebase_todo),
			_ => {},
		}
		result
	}

	fn handle_edit_mode_input(&mut self, input: Input, todo_file: &mut TodoFile) {
		if input == Input::Enter {
			let selected_index = todo_file.get_selected_line_index();
			todo_file.update_range(
				selected_index,
				selected_index,
				&EditContext::new().content(self.edit.get_content()),
			);
			self.visual_index_start = None;
			self.state = ListState::Normal;
		}
	}

	fn handle_move_cursor_inputs(&mut self, view: &View<'_>, todo_file: &mut TodoFile, input: Input) -> bool {
		match input {
			Input::MoveCursorLeft => self.view_data.scroll_left(),
			Input::MoveCursorRight => self.view_data.scroll_right(),
			Input::MoveCursorDown => {
				Self::move_cursor_down(todo_file, 1);
			},
			Input::MoveCursorUp => Self::move_cursor_up(todo_file, 1),
			Input::MoveCursorPageDown => Self::move_cursor_down(todo_file, view.get_view_size().height() / 2),
			Input::MoveCursorPageUp => Self::move_cursor_up(todo_file, view.get_view_size().height() / 2),
			_ => return false,
		}
		true
	}
}
