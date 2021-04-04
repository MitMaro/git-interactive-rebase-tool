mod utils;

#[cfg(all(unix, test))]
mod tests;

use std::cmp::min;

use crate::{
	components::{Edit, Help},
	config::Config,
	display::display_color::DisplayColor,
	input::{
		input_handler::{InputHandler, InputMode},
		Input,
	},
	list::utils::{get_list_normal_mode_help_lines, get_list_visual_mode_help_lines, get_todo_line_segments},
	process::{exit_status::ExitStatus, process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::{action::Action, edit_content::EditContext, line::Line, TodoFile},
	view::{line_segment::LineSegment, render_context::RenderContext, view_data::ViewData, view_line::ViewLine, View},
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
	normal_mode_help: Help,
	state: ListState,
	view_data: ViewData,
	visual_index_start: Option<usize>,
	visual_mode_help: Help,
}

impl<'l> ProcessModule for List<'l> {
	fn build_view_data(&mut self, context: &RenderContext, todo_file: &TodoFile) -> &mut ViewData {
		self.view_data.clear();

		match self.state {
			ListState::Normal => self.get_normal_mode_view_data(todo_file, context),
			ListState::Visual => self.get_visual_mode_view_data(todo_file, context),
			ListState::Edit => {
				self.edit.update_view_data(&mut self.view_data);
				&mut self.view_data
			},
		}
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		view: &mut View<'_>,
		todo_file: &mut TodoFile,
	) -> ProcessResult {
		match self.state {
			ListState::Normal => self.handle_normal_mode_input(input_handler, view, todo_file),
			ListState::Visual => self.handle_visual_mode_input(input_handler, view, todo_file),
			ListState::Edit => self.handle_edit_mode_input(input_handler, todo_file),
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
			edit: Edit::new(),
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
		if self.state == ListState::Normal && self.config.auto_select_next {
			Self::move_cursor_down(rebase_todo, 1);
		}
	}

	fn update_list_view_data(&mut self, context: &RenderContext, todo_file: &TodoFile) {
		self.view_data.clear();
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
						get_todo_line_segments(line, selected_index == index, selected_line, context.is_full_width()),
						if *line.get_action() == Action::Exec { 2 } else { 3 },
					)
					.set_selected(selected_index == index || selected_line),
				);
			}
		}
		if let Some(visual_index) = self.visual_index_start {
			self.view_data.ensure_line_visible(visual_index);
		}
		self.view_data.ensure_line_visible(selected_index);
	}

	fn get_visual_mode_view_data(&mut self, todo_file: &TodoFile, context: &RenderContext) -> &mut ViewData {
		if self.visual_mode_help.is_active() {
			self.visual_mode_help.get_view_data()
		}
		else {
			self.update_list_view_data(context, todo_file);
			&mut self.view_data
		}
	}

	fn get_normal_mode_view_data(&mut self, todo_file: &TodoFile, context: &RenderContext) -> &mut ViewData {
		if self.normal_mode_help.is_active() {
			self.normal_mode_help.get_view_data()
		}
		else {
			self.update_list_view_data(context, todo_file);
			&mut self.view_data
		}
	}

	fn handle_common_list_input(
		&mut self,
		input: Input,
		view: &View<'_>,
		rebase_todo: &mut TodoFile,
	) -> Option<ProcessResult> {
		let mut result = ProcessResult::new().input(input);
		match input {
			Input::MoveCursorLeft => self.view_data.scroll_left(),
			Input::MoveCursorRight => self.view_data.scroll_right(),
			Input::MoveCursorDown => Self::move_cursor_down(rebase_todo, 1),
			Input::MoveCursorUp => Self::move_cursor_up(rebase_todo, 1),
			Input::MoveCursorPageDown => Self::move_cursor_down(rebase_todo, view.get_render_context().height() / 2),
			Input::MoveCursorPageUp => Self::move_cursor_up(rebase_todo, view.get_render_context().height() / 2),
			Input::MoveCursorHome => rebase_todo.set_selected_line_index(0),
			Input::MoveCursorEnd => rebase_todo.set_selected_line_index(rebase_todo.get_max_selected_line_index()),
			Input::Abort => result = result.state(State::ConfirmAbort),
			Input::ForceAbort => {
				rebase_todo.set_lines(vec![]);
				result = result.exit_status(ExitStatus::Good);
			},
			Input::Rebase => result = result.state(State::ConfirmRebase),
			Input::ForceRebase => result = result.exit_status(ExitStatus::Good),
			Input::SwapSelectedDown => {
				let start_index = rebase_todo.get_selected_line_index();
				let end_index = self.visual_index_start.unwrap_or(start_index);

				if rebase_todo.swap_range_down(start_index, end_index) {
					if let Some(visual_index_start) = self.visual_index_start {
						self.visual_index_start = Some(visual_index_start + 1);
					}

					Self::move_cursor_down(rebase_todo, 1);
				}
			},
			Input::SwapSelectedUp => {
				let start_index = rebase_todo.get_selected_line_index();
				let end_index = self.visual_index_start.unwrap_or(start_index);

				if rebase_todo.swap_range_up(start_index, end_index) {
					if let Some(visual_index_start) = self.visual_index_start {
						self.visual_index_start = Some(visual_index_start - 1);
					}
					Self::move_cursor_up(rebase_todo, 1);
				}
			},
			Input::ActionDrop => self.set_selected_line_action(rebase_todo, Action::Drop),
			Input::ActionEdit => self.set_selected_line_action(rebase_todo, Action::Edit),
			Input::ActionFixup => self.set_selected_line_action(rebase_todo, Action::Fixup),
			Input::ActionPick => self.set_selected_line_action(rebase_todo, Action::Pick),
			Input::ActionReword => self.set_selected_line_action(rebase_todo, Action::Reword),
			Input::ActionSquash => self.set_selected_line_action(rebase_todo, Action::Squash),
			Input::Undo => {
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
			Input::Redo => {
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
			Input::Delete => {
				let start_index = rebase_todo.get_selected_line_index();
				let end_index = self.visual_index_start.unwrap_or(start_index);

				rebase_todo.remove_lines(start_index, end_index);
				let new_index = min(start_index, end_index);

				rebase_todo.set_selected_line_index(new_index);

				if self.state == ListState::Visual {
					self.visual_index_start = Some(rebase_todo.get_selected_line_index());
				}
			},
			Input::OpenInEditor => result = result.state(State::ExternalEditor),
			Input::ToggleVisualMode => {
				if self.state == ListState::Visual {
					self.state = ListState::Normal;
					self.visual_index_start = None;
				}
				else {
					self.state = ListState::Visual;
					self.visual_index_start = Some(rebase_todo.get_selected_line_index());
				}
			},
			Input::Help => {
				if self.state == ListState::Visual {
					self.visual_mode_help.set_active();
				}
				else {
					self.normal_mode_help.set_active();
				}
			},
			_ => return None,
		}

		Some(result)
	}

	fn handle_normal_mode_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		view: &View<'_>,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		if self.normal_mode_help.is_active() {
			let input = input_handler.get_input(InputMode::Default);
			self.normal_mode_help.handle_input(input);
			return ProcessResult::new().input(input);
		}

		let input = input_handler.get_input(InputMode::List);
		if let Some(result) = self.handle_common_list_input(input, view, rebase_todo) {
			result
		}
		else {
			let mut result = ProcessResult::new().input(input);
			match input {
				Input::ShowCommit => {
					if let Some(selected_line) = rebase_todo.get_selected_line() {
						if selected_line.has_reference() {
							result = result.state(State::ShowCommit);
						}
					}
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
				Input::InsertLine => result = result.state(State::Insert),
				_ => {},
			}
			result
		}
	}

	fn handle_visual_mode_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		view: &View<'_>,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		if self.visual_mode_help.is_active() {
			let input = input_handler.get_input(InputMode::Default);
			self.visual_mode_help.handle_input(input);
			return ProcessResult::new().input(input);
		}

		let input = input_handler.get_input(InputMode::List);
		self.handle_common_list_input(input, view, rebase_todo)
			.map_or_else(|| ProcessResult::new().input(input), |result| result)
	}

	fn handle_edit_mode_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		let input = input_handler.get_input(InputMode::Raw);
		if !self.edit.handle_input(input) && input == Input::Enter {
			let selected_index = rebase_todo.get_selected_line_index();
			rebase_todo.update_range(
				selected_index,
				selected_index,
				&EditContext::new().content(self.edit.get_content().as_str()),
			);
			self.visual_index_start = None;
			self.state = ListState::Normal;
		}
		ProcessResult::new().input(input)
	}
}
