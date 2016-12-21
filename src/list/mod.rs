pub(crate) mod action;
pub(crate) mod line;
mod utils;

use crate::config::Config;
use crate::constants::MINIMUM_FULL_WINDOW_WIDTH;
use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::list::action::Action;
use crate::list::line::Line;
use crate::list::utils::{
	get_action_color,
	get_normal_footer_compact,
	get_normal_footer_full,
	get_visual_footer_compact,
	get_visual_footer_full,
};
use crate::process::exit_status::ExitStatus;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::scroll::scroll_position::ScrollPosition;
use crate::view::line_segment::LineSegment;
use crate::view::view_line::ViewLine;
use crate::view::View;
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
enum ListState {
	Normal,
	Visual,
}

pub(crate) struct List<'l> {
	config: &'l Config,
	normal_footer_compact: String,
	normal_footer_full: String,
	scroll_position: ScrollPosition,
	state: ListState,
	visual_footer_compact: String,
	visual_footer_full: String,
}

fn get_maximum_line_length(is_full_width: bool, lines: &[Line]) -> usize {
	let mut length = 0;
	if is_full_width {
		for line in lines {
			let line_length = match *line.get_action() {
				Action::Exec => UnicodeSegmentation::graphemes(line.get_command().as_str(), true).count(),
				Action::Break => 0,
				_ => 9 + UnicodeSegmentation::graphemes(line.get_comment().as_str(), true).count(),
			} + 10;

			if line_length > length {
				length = line_length;
			}
		}
	}
	else {
		for line in lines {
			let line_length = match *line.get_action() {
				Action::Exec => UnicodeSegmentation::graphemes(line.get_command().as_str(), true).count(),
				Action::Break => 0,
				_ => 4 + UnicodeSegmentation::graphemes(line.get_comment().as_str(), true).count(),
			} + 3;

			if line_length > length {
				length = line_length;
			}
		}
	}
	length
}

impl<'l> ProcessModule for List<'l> {
	fn process(&mut self, git_interactive: &mut GitInteractive, view: &View) -> ProcessResult {
		let (_, view_height) = view.get_view_size();
		let lines = git_interactive.get_lines();
		let selected_index = *git_interactive.get_selected_line_index() - 1;
		self.scroll_position
			.ensure_cursor_visible(selected_index, view_height, lines.len());

		ProcessResult::new()
	}

	fn handle_input(
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

	#[allow(clippy::nonminimal_bool)]
	fn render(&self, view: &View, git_interactive: &GitInteractive) {
		let (view_width, view_height) = view.get_view_size();

		let is_visual_mode = self.state == ListState::Visual;
		let visual_index = git_interactive.get_visual_start_index() - 1;

		let mut view_lines: Vec<ViewLine> = vec![];

		let selected_index = *git_interactive.get_selected_line_index() - 1;

		for (index, line) in git_interactive.get_lines().iter().enumerate() {
			let selected_line = is_visual_mode
				&& ((visual_index <= selected_index && index >= visual_index && index <= selected_index)
					|| (visual_index > selected_index && index >= selected_index && index <= visual_index));
			view_lines.push(
				ViewLine::new_with_pinned_segments(
					self.get_todo_line_segments(line, selected_index == index, selected_line, view_width),
					if *line.get_action() == Action::Exec { 2 } else { 3 },
				)
				.set_selected(selected_index == index || selected_line),
			);
		}

		view.draw_title(true);

		view.draw_view_lines(
			&view_lines,
			self.scroll_position.get_top_position(),
			self.scroll_position.get_left_position(),
			view_height - 2,
		);

		view.set_color(DisplayColor::Normal, false);
		view.set_style(true, false, false);
		if is_visual_mode {
			if view_width >= self.visual_footer_full.len() {
				view.draw_str(self.visual_footer_full.as_str());
			}
			else if view_width >= self.visual_footer_compact.len() {
				view.draw_str(self.visual_footer_compact.as_str());
			}
			else {
				view.draw_str(format!("(Visual) Help: {}", self.config.input_help).as_str());
			}
		}
		else if view_width >= self.normal_footer_full.len() {
			view.draw_str(self.normal_footer_full.as_str());
		}
		else if view_width >= self.normal_footer_compact.len() {
			view.draw_str(self.normal_footer_compact.as_str());
		}
		else {
			view.draw_str(format!("Help: {}", self.config.input_help).as_str());
		}
		view.set_style(false, false, false);
	}
}

impl<'l> List<'l> {
	pub(crate) fn new(config: &'l Config) -> Self {
		Self {
			config,
			normal_footer_compact: get_normal_footer_compact(config),
			normal_footer_full: get_normal_footer_full(config),
			scroll_position: ScrollPosition::new(2, 1, 1),
			state: ListState::Normal,
			visual_footer_compact: get_visual_footer_compact(config),
			visual_footer_full: get_visual_footer_full(config),
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
		input_handler: &InputHandler,
		git_interactive: &mut GitInteractive,
		view: &View,
	) -> HandleInputResult
	{
		let input = input_handler.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		let (view_width, _) = view.get_view_size();
		match input {
			Input::Help => {
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
			Input::MoveCursorLeft => {
				self.scroll_position.scroll_left(
					view_width,
					get_maximum_line_length(view_width >= MINIMUM_FULL_WINDOW_WIDTH, git_interactive.get_lines()),
				)
			},
			Input::MoveCursorRight => {
				self.scroll_position.scroll_right(
					view_width,
					get_maximum_line_length(view_width >= MINIMUM_FULL_WINDOW_WIDTH, git_interactive.get_lines()),
				)
			},
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
		let (view_width, _) = view.get_view_size();
		match input {
			Input::Help => {
				result = result.help(State::List(true));
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
			Input::MoveCursorLeft => {
				self.scroll_position.scroll_left(
					view_width,
					get_maximum_line_length(view_width >= MINIMUM_FULL_WINDOW_WIDTH, git_interactive.get_lines()),
				)
			},
			Input::MoveCursorRight => {
				self.scroll_position.scroll_right(
					view_width,
					get_maximum_line_length(view_width >= MINIMUM_FULL_WINDOW_WIDTH, git_interactive.get_lines()),
				)
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
				DisplayColor::Normal,
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
					String::from("")
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
				DisplayColor::Normal,
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
