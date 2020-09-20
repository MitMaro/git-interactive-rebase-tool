pub mod action;
pub mod line;
mod utils;

use crate::config::Config;
use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::list::action::Action;
use crate::list::utils::{
	get_list_normal_mode_help_lines,
	get_list_visual_mode_help_lines,
	get_normal_footer_compact,
	get_normal_footer_full,
	get_todo_line_segments,
	get_visual_footer_compact,
	get_visual_footer_full,
};
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;

#[derive(Debug, PartialEq)]
enum ListState {
	Normal,
	Visual,
}

pub struct List<'l> {
	config: &'l Config,
	normal_footer_compact: String,
	normal_footer_full: String,
	normal_mode_help_lines: Vec<(String, String)>,
	state: ListState,
	view_data: ViewData,
	visual_footer_compact: String,
	visual_footer_full: String,
	visual_mode_help_lines: Vec<(String, String)>,
}

impl<'l> ProcessModule for List<'l> {
	fn build_view_data(&mut self, view: &View<'_>, git_interactive: &GitInteractive) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();

		self.view_data.clear();

		let is_visual_mode = self.state == ListState::Visual;
		let visual_index = git_interactive.get_visual_start_index() - 1;
		let selected_index = *git_interactive.get_selected_line_index() - 1;

		for (index, line) in git_interactive.get_lines().iter().enumerate() {
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

		let footer = if is_visual_mode {
			if view_width >= self.visual_footer_full.len() {
				self.visual_footer_full.clone()
			}
			else if view_width >= self.visual_footer_compact.len() {
				self.visual_footer_compact.clone()
			}
			else {
				format!("(Visual) Help: {}", self.config.key_bindings.help)
			}
		}
		else if view_width >= self.normal_footer_full.len() {
			self.normal_footer_full.clone()
		}
		else if view_width >= self.normal_footer_compact.len() {
			self.normal_footer_compact.clone()
		}
		else {
			format!("Help: {}", self.config.key_bindings.help)
		};

		self.view_data
			.push_trailing_line(ViewLine::new_pinned(vec![LineSegment::new_with_color_and_style(
				footer.as_str(),
				DisplayColor::Normal,
				true,
				false,
				false,
			)]));

		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		git_interactive: &mut GitInteractive,
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
				git_interactive.move_cursor_down(1);
			},
			Input::MoveCursorUp => git_interactive.move_cursor_up(1),
			Input::MoveCursorPageDown => git_interactive.move_cursor_down(view_height / 2),
			Input::MoveCursorPageUp => git_interactive.move_cursor_up(view_height / 2),
			_ => {
				result = match self.state {
					ListState::Normal => self.handle_normal_mode_input(input, result, git_interactive),
					ListState::Visual => self.handle_visual_mode_input(input, result, git_interactive),
				}
			},
		}
		let selected_index = *git_interactive.get_selected_line_index() - 1;
		self.view_data.ensure_line_visible(selected_index);
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
			normal_footer_compact: get_normal_footer_compact(&config.key_bindings),
			normal_footer_full: get_normal_footer_full(&config.key_bindings),
			normal_mode_help_lines: get_list_normal_mode_help_lines(&config.key_bindings),
			state: ListState::Normal,
			view_data,
			visual_footer_compact: get_visual_footer_compact(&config.key_bindings),
			visual_footer_full: get_visual_footer_full(&config.key_bindings),
			visual_mode_help_lines: get_list_visual_mode_help_lines(&config.key_bindings),
		}
	}

	fn set_selected_line_action(&self, git_interactive: &mut GitInteractive, action: Action) {
		git_interactive.set_selected_line_action(action);
		if self.config.auto_select_next {
			git_interactive.move_cursor_down(1);
		}
	}

	fn handle_normal_mode_input(
		&mut self,
		input: Input,
		result: ProcessResult,
		git_interactive: &mut GitInteractive,
	) -> ProcessResult
	{
		let mut result = result;
		match input {
			Input::ShowCommit => {
				if !git_interactive.get_selected_line_hash().is_empty() {
					result = result.state(State::ShowCommit);
				}
			},
			Input::Abort => {
				result = result.state(State::ConfirmAbort);
			},
			Input::ForceAbort => {
				git_interactive.clear();
				result = result.exit_status(ExitStatus::Good);
			},
			Input::Rebase => {
				result = result.state(State::ConfirmRebase);
			},
			Input::ForceRebase => {
				result = result.exit_status(ExitStatus::Good);
			},
			Input::ActionBreak => git_interactive.toggle_break(),
			Input::ActionDrop => self.set_selected_line_action(git_interactive, Action::Drop),
			Input::ActionEdit => self.set_selected_line_action(git_interactive, Action::Edit),
			Input::ActionFixup => self.set_selected_line_action(git_interactive, Action::Fixup),
			Input::ActionPick => self.set_selected_line_action(git_interactive, Action::Pick),
			Input::ActionReword => self.set_selected_line_action(git_interactive, Action::Reword),
			Input::ActionSquash => self.set_selected_line_action(git_interactive, Action::Squash),
			Input::Edit => {
				if *git_interactive.get_selected_line_action() == Action::Exec {
					result = result.state(State::Edit);
				}
			},
			Input::SwapSelectedDown => git_interactive.swap_selected_down(),
			Input::SwapSelectedUp => git_interactive.swap_selected_up(),
			Input::ToggleVisualMode => {
				git_interactive.start_visual_mode();
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
		git_interactive: &mut GitInteractive,
	) -> ProcessResult
	{
		let mut result = result;
		match input {
			Input::Abort => {
				result = result.state(State::ConfirmAbort);
			},
			Input::ForceAbort => {
				git_interactive.clear();
				result = result.exit_status(ExitStatus::Good);
			},
			Input::Rebase => {
				result = result.state(State::ConfirmRebase);
			},
			Input::ForceRebase => {
				result = result.exit_status(ExitStatus::Good);
			},
			Input::ActionDrop => git_interactive.set_visual_range_action(Action::Drop),
			Input::ActionEdit => git_interactive.set_visual_range_action(Action::Edit),
			Input::ActionFixup => git_interactive.set_visual_range_action(Action::Fixup),
			Input::ActionPick => git_interactive.set_visual_range_action(Action::Pick),
			Input::ActionReword => git_interactive.set_visual_range_action(Action::Reword),
			Input::ActionSquash => git_interactive.set_visual_range_action(Action::Squash),
			Input::SwapSelectedDown => git_interactive.swap_visual_range_down(),
			Input::SwapSelectedUp => git_interactive.swap_visual_range_up(),
			Input::ToggleVisualMode => {
				self.state = ListState::Normal;
				result = result.state(State::List);
			},
			_ => {},
		}
		result
	}
}
