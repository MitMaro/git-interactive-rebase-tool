use crate::action::Action;
use crate::config::Config;
use crate::constants::{
	LIST_FOOTER_COMPACT,
	LIST_FOOTER_COMPACT_WIDTH,
	LIST_FOOTER_FULL,
	LIST_FOOTER_FULL_WIDTH,
	LIST_HELP_LINES,
	MINIMUM_FULL_WINDOW_WIDTH,
	VISUAL_MODE_FOOTER_COMPACT,
	VISUAL_MODE_FOOTER_COMPACT_WIDTH,
	VISUAL_MODE_FOOTER_FULL,
	VISUAL_MODE_FOOTER_FULL_WIDTH,
	VISUAL_MODE_HELP_LINES,
};
use crate::git_interactive::GitInteractive;
use crate::input::{Input, InputHandler};
use crate::line::Line;
use crate::list::get_action_color;
use crate::process::{ExitStatus, HandleInputResult, HandleInputResultBuilder, ProcessModule, ProcessResult, State};
use crate::scroll::ScrollPosition;
use crate::view::{LineSegment, View, ViewLine};
use crate::window::WindowColor;
use std::cmp;

#[derive(Debug, PartialEq)]
enum ListState {
	Normal,
	Visual,
}

pub struct List<'l> {
	config: &'l Config,
	scroll_position: ScrollPosition,
	state: ListState,
}

impl<'l> ProcessModule for List<'l> {
	#[allow(clippy::nonminimal_bool)]
	fn render(&self, view: &View, git_interactive: &GitInteractive) {
		let (view_width, view_height) = view.get_view_size();

		let is_visual_mode = self.state == ListState::Visual;
		let visual_index = git_interactive.get_visual_start_index() - 1;

		let mut view_lines: Vec<ViewLine> = vec![];

		let selected_index = *git_interactive.get_selected_line_index() - 1;

		for (index, line) in git_interactive.get_lines().iter().enumerate() {
			view_lines.push(ViewLine::new(self.get_todo_line_segments(
				line,
				selected_index == index,
				is_visual_mode
					&& ((visual_index <= selected_index && index >= visual_index && index <= selected_index)
						|| (visual_index > selected_index && index >= selected_index && index <= visual_index)),
				view_width,
			)));
		}

		view.draw_title(true);

		view.draw_view_lines(view_lines, self.scroll_position.get_position(), view_height - 2);

		view.set_color(WindowColor::Foreground);
		view.set_style(true, false, false);
		if is_visual_mode {
			if view_width >= VISUAL_MODE_FOOTER_FULL_WIDTH {
				view.draw_str(VISUAL_MODE_FOOTER_FULL);
			}
			else if view_width >= VISUAL_MODE_FOOTER_COMPACT_WIDTH {
				view.draw_str(VISUAL_MODE_FOOTER_COMPACT);
			}
			else {
				view.draw_str("(Visual) Help: ?");
			}
		}
		else if view_width >= LIST_FOOTER_FULL_WIDTH {
			view.draw_str(LIST_FOOTER_FULL);
		}
		else if view_width >= LIST_FOOTER_COMPACT_WIDTH {
			view.draw_str(LIST_FOOTER_COMPACT);
		}
		else {
			view.draw_str("Help: ?");
		}
		view.set_style(false, false, false);
	}
}

impl<'l> List<'l> {
	pub fn new(config: &'l Config) -> Self {
		Self {
			config,
			scroll_position: ScrollPosition::new(2, 1, 1),
			state: ListState::Normal,
		}
	}

	fn set_selected_line_action(&self, git_interactive: &mut GitInteractive, action: Action) {
		git_interactive.set_selected_line_action(action);
		if self.config.auto_select_next {
			git_interactive.move_cursor_down(1);
		}
	}

	pub fn process_with_view(&mut self, git_interactive: &mut GitInteractive, view: &View) -> ProcessResult {
		let (_, view_height) = view.get_view_size();
		let lines = git_interactive.get_lines();
		let selected_index = *git_interactive.get_selected_line_index() - 1;
		self.scroll_position
			.ensure_cursor_visible(selected_index, view_height, lines.len());
		ProcessResult::new()
	}

	pub fn handle_input_with_view(
		&mut self,
		input_handler: &InputHandler,
		git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		match self.state {
			ListState::Normal => self.handle_normal_mode_input(input_handler, git_interactive, view),
			ListState::Visual => self.handle_visual_mode_input(input_handler, git_interactive, view),
		}
	}

	fn handle_normal_mode_input(
		&mut self,
		input_handler: &InputHandler,
		git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let input = input_handler.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Help => {
				view.update_help_top(false, true, LIST_HELP_LINES);
				result = result.help(State::List(false));
			},
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
				result = result.exit_status(ExitStatus::Good).state(State::Exiting);
			},
			Input::Rebase => {
				result = result.state(State::ConfirmRebase);
			},
			Input::ForceRebase => {
				result = result.exit_status(ExitStatus::Good).state(State::Exiting);
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
			Input::MoveCursorDown => git_interactive.move_cursor_down(1),
			Input::MoveCursorUp => git_interactive.move_cursor_up(1),
			Input::MoveCursorPageDown => git_interactive.move_cursor_down(5),
			Input::MoveCursorPageUp => git_interactive.move_cursor_up(5),
			Input::ToggleVisualMode => {
				git_interactive.start_visual_mode();
				self.state = ListState::Visual;
				result = result.state(State::List(true));
			},
			Input::OpenInEditor => result = result.state(State::ExternalEditor),
			_ => {},
		}
		result.build()
	}

	fn handle_visual_mode_input(
		&mut self,
		input_handler: &InputHandler,
		git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let input = input_handler.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Help => {
				view.update_help_top(false, true, VISUAL_MODE_HELP_LINES);
				result = result.help(State::List(true));
			},
			Input::MoveCursorDown => {
				git_interactive.move_cursor_down(1);
			},
			Input::MoveCursorUp => {
				git_interactive.move_cursor_up(1);
			},
			Input::MoveCursorPageDown => {
				git_interactive.move_cursor_down(5);
			},
			Input::MoveCursorPageUp => {
				git_interactive.move_cursor_up(5);
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
				result = result.state(State::List(false));
			},
			_ => {},
		}
		result.build()
	}

	fn get_todo_line_segments(
		&self,
		line: &Line,
		is_cursor_line: bool,
		selected: bool,
		view_width: usize,
	) -> Vec<LineSegment>
	{
		let mut segments: Vec<LineSegment> = vec![];

		let action = line.get_action();

		if view_width >= MINIMUM_FULL_WINDOW_WIDTH {
			segments.push(LineSegment::new_with_color_and_style(
				if is_cursor_line || selected { " > " } else { "   " },
				WindowColor::Foreground,
				!is_cursor_line && selected,
				false,
				false,
			));

			segments.push(LineSegment::new_with_color(
				format!("{:6} ", action.as_string()).as_str(),
				get_action_color(*action),
			));

			segments.push(LineSegment::new(
				if *action == Action::Exec {
					line.get_command().clone()
				}
				else if *action == Action::Break {
					String::from("         ")
				}
				else {
					let max_index = cmp::min(line.get_hash().len(), 8);
					format!("{:8} ", line.get_hash()[0..max_index].to_string())
				}
				.as_str(),
			));
		}
		else {
			segments.push(LineSegment::new_with_color_and_style(
				if is_cursor_line || selected { ">" } else { " " },
				WindowColor::Foreground,
				!is_cursor_line && selected,
				false,
				false,
			));

			segments.push(LineSegment::new_with_color(
				format!("{:1} ", line.get_action().to_abbreviation()).as_str(),
				get_action_color(*action),
			));

			segments.push(LineSegment::new(
				if *action == Action::Exec {
					line.get_command().clone()
				}
				else if *action == Action::Break {
					String::from("    ")
				}
				else {
					let max_index = cmp::min(line.get_hash().len(), 3);
					format!("{:3} ", line.get_hash()[0..max_index].to_string())
				}
				.as_str(),
			));
		}
		if *action != Action::Exec && *action != Action::Break {
			segments.push(LineSegment::new(line.get_comment().as_str()));
		}
		segments
	}
}
