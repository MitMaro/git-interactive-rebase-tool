pub mod action;
pub mod line;
mod utils;

use crate::config::Config;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::list::action::Action;
use crate::list::line::Line;
use crate::list::utils::{get_list_normal_mode_help_lines, get_list_visual_mode_help_lines, get_todo_line_segments};
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::todo_file::{EditContext, TodoFile};
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use std::cmp;

#[derive(Debug, PartialEq)]
enum ListState {
	Normal,
	Visual,
}

pub struct List<'l> {
	config: &'l Config,
	normal_mode_help_lines: Vec<(String, String)>,
	state: ListState,
	view_data: ViewData,
	visual_index_start: Option<usize>,
	visual_mode_help_lines: Vec<(String, String)>,
}

impl<'l> ProcessModule for List<'l> {
	fn build_view_data(&mut self, view: &View<'_>, todo_file: &TodoFile) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();

		self.view_data.clear();

		let is_visual_mode = self.state == ListState::Visual;
		let visual_index = self
			.visual_index_start
			.unwrap_or_else(|| todo_file.get_selected_line_index())
			- 1;
		let selected_index = todo_file.get_selected_line_index() - 1;

		for (index, line) in todo_file.get_lines().iter().enumerate() {
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

		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		if let Some(visual_index) = self.visual_index_start {
			self.view_data.ensure_line_visible(visual_index - 1);
		}
		self.view_data.ensure_line_visible(selected_index);
		&self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		todo_file: &mut TodoFile,
		view: &View<'_>,
	) -> ProcessResult
	{
		let (_, view_height) = view.get_view_size();
		let input = input_handler.get_input(InputMode::List);
		let mut result = ProcessResult::new().input(input);
		match input {
			Input::MoveCursorLeft => self.view_data.scroll_left(),
			Input::MoveCursorRight => self.view_data.scroll_right(),
			Input::MoveCursorDown => {
				Self::move_cursor_down(todo_file, 1);
			},
			Input::MoveCursorUp => Self::move_cursor_up(todo_file, 1),
			Input::MoveCursorPageDown => Self::move_cursor_down(todo_file, view_height / 2),
			Input::MoveCursorPageUp => Self::move_cursor_up(todo_file, view_height / 2),
			_ => {
				result = match self.state {
					ListState::Normal => self.handle_normal_mode_input(input, result, todo_file),
					ListState::Visual => self.handle_visual_mode_input(input, result, todo_file),
				}
			},
		}
		result
	}

	fn get_help_keybindings_descriptions(&self) -> Option<Vec<(String, String)>> {
		if self.state == ListState::Normal {
			Some(self.normal_mode_help_lines.clone())
		}
		else {
			Some(self.visual_mode_help_lines.clone())
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
			visual_index_start: None,
			visual_mode_help_lines: get_list_visual_mode_help_lines(&config.key_bindings),
		}
	}

	pub(crate) fn move_cursor_up(todo_file: &mut TodoFile, amount: usize) {
		let current_selected_line_index = todo_file.get_selected_line_index();
		todo_file.set_selected_line_index(
			if amount >= current_selected_line_index {
				1
			}
			else {
				current_selected_line_index - amount
			},
		);
	}

	pub(crate) fn move_cursor_down(rebase_todo: &mut TodoFile, amount: usize) {
		let current_selected_line_index = rebase_todo.get_selected_line_index();
		let lines_length = rebase_todo.get_lines().len();
		rebase_todo.set_selected_line_index(cmp::min(current_selected_line_index + amount, lines_length));
	}

	fn set_selected_line_action(&self, rebase_todo: &mut TodoFile, action: Action, advanced_next: bool) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		rebase_todo.update_range(start_index, end_index, &EditContext::new().action(action));
		if advanced_next && self.config.auto_select_next {
			Self::move_cursor_down(rebase_todo, 1);
		}
	}

	pub(crate) fn swap_range_up(&mut self, rebase_todo: &mut TodoFile) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);

		if end_index == 1 || start_index == 1 {
			return;
		}

		let range = if end_index <= start_index {
			end_index..=start_index
		}
		else {
			start_index..=end_index
		};

		for index in range {
			rebase_todo.swap_lines(index - 1, index - 2);
		}

		if let Some(visual_index_start) = self.visual_index_start {
			self.visual_index_start = Some(visual_index_start - 1);
		}
		Self::move_cursor_up(rebase_todo, 1);
	}

	pub(crate) fn swap_range_down(&mut self, rebase_todo: &mut TodoFile) {
		let start_index = rebase_todo.get_selected_line_index();
		let end_index = self.visual_index_start.unwrap_or(start_index);
		let lines_length = rebase_todo.get_lines().len();

		if end_index == lines_length || start_index == lines_length {
			return;
		}

		let range = if end_index <= start_index {
			end_index..=start_index
		}
		else {
			start_index..=end_index
		};

		for index in range.rev() {
			rebase_todo.swap_lines(index - 1, index);
		}

		if let Some(visual_index_start) = self.visual_index_start {
			self.visual_index_start = Some(visual_index_start + 1);
		}

		Self::move_cursor_down(rebase_todo, 1);
	}

	fn handle_normal_mode_input(
		&mut self,
		input: Input,
		result: ProcessResult,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult
	{
		let mut result = result;
		match input {
			Input::ShowCommit => {
				if !rebase_todo.get_selected_line().get_hash().is_empty() {
					result = result.state(State::ShowCommit);
				}
			},
			Input::Abort => {
				result = result.state(State::ConfirmAbort);
			},
			Input::ForceAbort => {
				rebase_todo.set_noop();
				result = result.exit_status(ExitStatus::Good);
			},
			Input::Rebase => {
				result = result.state(State::ConfirmRebase);
			},
			Input::ForceRebase => {
				result = result.exit_status(ExitStatus::Good);
			},
			Input::ActionBreak => {
				// TODO - does not stop multiple breaks in a row
				let action = rebase_todo.get_selected_line().get_action();
				if action == &Action::Break {
					rebase_todo.remove_line(rebase_todo.get_selected_line_index());
					Self::move_cursor_up(rebase_todo, 1);
				}
				else {
					rebase_todo.add_line(rebase_todo.get_selected_line_index() + 1, Line::new_break());
					Self::move_cursor_down(rebase_todo, 1);
				}
			},
			Input::ActionDrop => self.set_selected_line_action(rebase_todo, Action::Drop, true),
			Input::ActionEdit => self.set_selected_line_action(rebase_todo, Action::Edit, true),
			Input::ActionFixup => self.set_selected_line_action(rebase_todo, Action::Fixup, true),
			Input::ActionPick => self.set_selected_line_action(rebase_todo, Action::Pick, true),
			Input::ActionReword => self.set_selected_line_action(rebase_todo, Action::Reword, true),
			Input::ActionSquash => self.set_selected_line_action(rebase_todo, Action::Squash, true),
			Input::Edit => {
				if rebase_todo.get_selected_line().get_action() == &Action::Exec {
					result = result.state(State::Edit);
				}
			},
			Input::SwapSelectedDown => self.swap_range_down(rebase_todo),
			Input::SwapSelectedUp => self.swap_range_up(rebase_todo),
			Input::ToggleVisualMode => {
				self.visual_index_start = Some(rebase_todo.get_selected_line_index());
				self.state = ListState::Visual;
				result = result.state(State::List);
			},
			Input::OpenInEditor => result = result.state(State::ExternalEditor),
			_ => {},
		}

		result
	}

	fn handle_visual_mode_input(
		&mut self,
		input: Input,
		result: ProcessResult,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult
	{
		let mut result = result;
		match input {
			Input::Abort => {
				result = result.state(State::ConfirmAbort);
			},
			Input::ForceAbort => {
				rebase_todo.set_noop();
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
				result = result.state(State::List);
			},
			_ => {},
		}
		result
	}
}
