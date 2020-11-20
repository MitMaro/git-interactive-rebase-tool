mod utils;

use crate::config::Config;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::list::utils::{get_list_normal_mode_help_lines, get_list_visual_mode_help_lines, get_todo_line_segments};
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::todo_file::action::Action;
use crate::todo_file::edit_content::EditContext;
use crate::todo_file::line::Line;
use crate::todo_file::TodoFile;
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
		self.view_data.set_view_size(view_width, view_height);

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
			},
			_ => {},
		}
		result
	}
}

#[cfg(all(unix, test))]
mod tests {
	use super::*;
	use crate::assert_process_result;
	use crate::assert_rendered_output;
	use crate::process::testutil::{process_module_test, TestContext, ViewState};

	#[test]
	#[serial_test::serial]
	fn render_full() {
		process_module_test(
			&[
				"pick aaaaaaaa comment 1",
				"drop bbbbbbbb comment 2",
				"fixup cccccccc comment 3",
				"exec echo 'foo'",
				"pick dddddddd comment 4",
				"reword eeeeeeee comment 5",
				"break",
				"squash ffffffff comment 6",
				"edit 11111111 comment 7",
			],
			ViewState::default(),
			&[],
			|test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaaaaaaa \
					 {Normal(selected)}comment 1",
					"{Normal}   {ActionDrop}drop   {Normal}bbbbbbbb {Normal}comment 2",
					"{Normal}   {ActionFixup}fixup  {Normal}cccccccc {Normal}comment 3",
					"{Normal}   {ActionExec}exec   {Normal}echo 'foo'",
					"{Normal}   {ActionPick}pick   {Normal}dddddddd {Normal}comment 4",
					"{Normal}   {ActionReword}reword {Normal}eeeeeeee {Normal}comment 5",
					"{Normal}   {ActionBreak}break  ",
					"{Normal}   {ActionSquash}squash {Normal}ffffffff {Normal}comment 6",
					"{Normal}   {ActionEdit}edit   {Normal}11111111 {Normal}comment 7"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn render_compact() {
		process_module_test(
			&[
				"pick aaaaaaaa comment 1",
				"drop bbbbbbbb comment 2",
				"fixup cccccccc comment 3",
				"exec echo 'foo'",
				"pick dddddddd comment 4",
				"reword eeeeeeee comment 5",
				"break",
				"squash ffffffff comment 6",
				"edit 11111111 comment 7",
			],
			ViewState {
				size: (30, 100),
				..ViewState::default()
			},
			&[],
			|test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)}>{ActionPick(selected)}p {Normal(selected)}aaa {Normal(selected)}comment 1",
					"{Normal} {ActionDrop}d {Normal}bbb {Normal}comment 2",
					"{Normal} {ActionFixup}f {Normal}ccc {Normal}comment 3",
					"{Normal} {ActionExec}x {Normal}echo 'foo'",
					"{Normal} {ActionPick}p {Normal}ddd {Normal}comment 4",
					"{Normal} {ActionReword}r {Normal}eee {Normal}comment 5",
					"{Normal} {ActionBreak}b ",
					"{Normal} {ActionSquash}s {Normal}fff {Normal}comment 6",
					"{Normal} {ActionEdit}e {Normal}111 {Normal}comment 7"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_down_1() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorDown],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_down_view_end() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorDown; 2],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_down_scroll_1() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorDown; 3],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_down_scroll_bottom() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorDown; 4],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_down_scroll_bottom_move_up_one() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 4);
				test_context.build_view_data(&mut module);
				test_context.handle_input(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_down_scroll_bottom_move_up_top() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorUp,
				Input::MoveCursorUp,
				Input::MoveCursorUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 4);
				test_context.build_view_data(&mut module);
				test_context.handle_n_inputs(&mut module, 3);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_up_attempt_above_top() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorUp, Input::MoveCursorUp],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_down_attempt_below_bottom() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorDown; 4],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_page_up_from_top() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorPageUp],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_page_up_from_one_page_down() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
				"pick aaa c6",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorDown, Input::MoveCursorDown, Input::MoveCursorPageUp],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 2);
				test_context.build_view_data(&mut module);
				test_context.handle_input(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_page_up_from_one_page_down_plus_1() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
				"pick aaa c6",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorPageUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 3);
				test_context.build_view_data(&mut module);
				test_context.handle_input(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_page_up_from_one_page_down_minus_1() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
				"pick aaa c6",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorDown, Input::MoveCursorDown, Input::MoveCursorPageUp],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 2);
				test_context.build_view_data(&mut module);
				test_context.handle_input(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_page_up_from_bottom() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
				"pick aaa c6",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorPageUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 5);
				test_context.build_view_data(&mut module);
				test_context.handle_input(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c6"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_page_down_from_bottom() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
				"pick aaa c6",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorPageDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 5);
				test_context.build_view_data(&mut module);
				test_context.handle_input(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c6"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_page_down_one_from_bottom() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
				"pick aaa c6",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorPageDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 4);
				test_context.build_view_data(&mut module);
				test_context.handle_input(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c6"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_page_down_one_page_from_bottom() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
				"pick aaa c6",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorPageDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_n_inputs(&mut module, 3);
				test_context.build_view_data(&mut module);
				test_context.handle_input(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c6"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_start() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ToggleVisualMode],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_start_cursor_down_one() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ToggleVisualMode, Input::MoveCursorDown],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c1",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_start_move_down_below_view() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_start_cursor_page_down() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ToggleVisualMode, Input::MoveCursorPageDown],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c1",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_start_cursor_page_down_below_view() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::ToggleVisualMode,
				Input::MoveCursorPageDown,
				Input::MoveCursorPageDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c4",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_start_cursor_from_bottom_move_up() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorPageDown,
				Input::MoveCursorPageDown,
				Input::MoveCursorPageDown,
				Input::ToggleVisualMode,
				Input::MoveCursorUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_start_cursor_from_bottom_to_top() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorPageDown,
				Input::MoveCursorPageDown,
				Input::MoveCursorPageDown,
				Input::ToggleVisualMode,
				Input::MoveCursorPageUp,
				Input::MoveCursorPageUp,
				Input::MoveCursorPageUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_to_drop() {
		process_module_test(
			&["pick aaa c1"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionDrop],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionDrop(selected)}drop   {Normal(selected)}aaa      {Normal(selected)}c1"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_to_edit() {
		process_module_test(
			&["pick aaa c1"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionEdit],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionEdit(selected)}edit   {Normal(selected)}aaa      {Normal(selected)}c1"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_to_fixup() {
		process_module_test(
			&["pick aaa c1"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionFixup],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionFixup(selected)}fixup  {Normal(selected)}aaa      \
					 {Normal(selected)}c1"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_to_pick() {
		process_module_test(
			&["drop aaa c1"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionPick],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_to_reword() {
		process_module_test(
			&["pick aaa c1"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionReword],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c1"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_to_squash() {
		process_module_test(
			&["pick aaa c1"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionSquash],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionSquash(selected)}squash {Normal(selected)}aaa      \
					 {Normal(selected)}c1"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_toggle_break_add() {
		process_module_test(
			&["pick aaa c1"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionBreak],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal(selected)} > {ActionBreak(selected)}break  "
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_toggle_break_remove() {
		process_module_test(
			&["pick aaa c1", "break"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorDown, Input::ActionBreak],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1"
				);
			},
		);
	}

	// TODO - this is a known bug
	#[test]
	#[ignore]
	#[serial_test::serial]
	fn change_selected_line_toggle_break_above_existing() {
		process_module_test(
			&["pick aaa c1", "break"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionBreak],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal}   {ActionBreak}break  "
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_auto_select_next_with_next_line() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2"],
			ViewState {
				size: (120, 4),
				..ViewState::default()
			},
			&[Input::ActionSquash],
			|mut test_context: TestContext<'_>| {
				let mut config = test_context.config.clone();
				config.auto_select_next = true;
				let mut module = List::new(&config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionSquash}squash {Normal}aaa      {Normal}c1",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_swap_down() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
			ViewState::default(),
			&[Input::SwapSelectedDown],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_swap_down_past_bottom() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
			ViewState::default(),
			&[Input::SwapSelectedDown; 3],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_swap_up() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
			ViewState::default(),
			&[Input::MoveCursorDown, Input::MoveCursorDown, Input::SwapSelectedUp],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn change_selected_line_swap_up_past_top() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::SwapSelectedUp,
				Input::SwapSelectedUp,
				Input::SwapSelectedUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_show_commit_when_hash_available() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ShowCommit],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::ShowCommit,
					state = State::ShowCommit
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_do_not_show_commit_when_hash_not_available() {
		process_module_test(
			&["exec echo foo"],
			ViewState::default(),
			&[Input::ShowCommit],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ShowCommit);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_abort() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::Abort],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Abort,
					state = State::ConfirmAbort
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_force_abort() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ForceAbort],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::ForceAbort,
					exit_status = ExitStatus::Good
				);
				assert!(test_context.rebase_todo_file.is_noop())
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_rebase() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::Rebase],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Rebase,
					state = State::ConfirmRebase
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_force_rebase() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ForceRebase],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::ForceRebase,
					exit_status = ExitStatus::Good
				);
				assert!(!test_context.rebase_todo_file.is_noop());
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_edit_with_edit_content() {
		process_module_test(
			&["exec echo foo"],
			ViewState::default(),
			&[Input::Edit],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Edit,
					state = State::Edit
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_edit_without_edit_content() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::Edit],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Edit);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_open_external_editor() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::OpenInEditor],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::OpenInEditor,
					state = State::ExternalEditor
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_toggle_visual_mode() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ToggleVisualMode],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ToggleVisualMode);
				assert_eq!(module.visual_index_start, Some(1));
				assert_eq!(module.state, ListState::Visual);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_other_input() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::Other],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_action_change_top_bottom() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
			ViewState::default(),
			&[
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ActionReword,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c1",
					"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected)} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_action_change_bottom_top() {
		process_module_test(
			&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorUp,
				Input::MoveCursorUp,
				Input::ActionReword,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c1",
					"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c3"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_action_change_drop() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ActionDrop,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal(selected),Dimmed} > {ActionDrop(selected)}drop   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionDrop(selected)}drop   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionDrop(selected)}drop   {Normal(selected)}aaa      {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_action_change_edit() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ActionEdit,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal(selected),Dimmed} > {ActionEdit(selected)}edit   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionEdit(selected)}edit   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionEdit(selected)}edit   {Normal(selected)}aaa      {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_action_change_fixup() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ActionFixup,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal(selected),Dimmed} > {ActionFixup(selected)}fixup  {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionFixup(selected)}fixup  {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionFixup(selected)}fixup  {Normal(selected)}aaa      \
					 {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_action_change_pick() {
		process_module_test(
			&[
				"drop aaa c1",
				"drop aaa c2",
				"drop aaa c3",
				"drop aaa c4",
				"drop aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ActionPick,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionDrop}drop   {Normal}aaa      {Normal}c1",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
					"{Normal}   {ActionDrop}drop   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_action_change_reword() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ActionReword,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
					 {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_action_change_squash() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ActionSquash,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal(selected),Dimmed} > {ActionSquash(selected)}squash {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionSquash(selected)}squash {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionSquash(selected)}squash {Normal(selected)}aaa      \
					 {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_abort() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ToggleVisualMode, Input::Abort],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_input(&mut module);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Abort,
					state = State::ConfirmAbort
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_force_abort() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ToggleVisualMode, Input::ForceAbort],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_input(&mut module);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::ForceAbort,
					exit_status = ExitStatus::Good
				);
				assert!(test_context.rebase_todo_file.is_noop())
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_rebase() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ToggleVisualMode, Input::Rebase],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_input(&mut module);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Rebase,
					state = State::ConfirmRebase
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_force_rebase() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ToggleVisualMode, Input::ForceRebase],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_input(&mut module);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::ForceRebase,
					exit_status = ExitStatus::Good
				);
				assert!(!test_context.rebase_todo_file.is_noop())
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_swap_down_from_top_to_bottom_selection() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::SwapSelectedDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_swap_down_from_bottom_to_top_selection() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorUp,
				Input::MoveCursorUp,
				Input::SwapSelectedDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_swap_up_from_top_to_bottom_selection() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::SwapSelectedUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_swap_up_from_bottom_to_top_selection() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorUp,
				Input::MoveCursorUp,
				Input::SwapSelectedUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_swap_down_to_limit_from_bottom_to_top_selection() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorUp,
				Input::MoveCursorUp,
				Input::SwapSelectedDown,
				Input::SwapSelectedDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_swap_down_to_limit_from_top_to_bottom_selection() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::SwapSelectedDown,
				Input::SwapSelectedDown,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_swap_up_to_limit_from_top_to_bottom_selection() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::SwapSelectedUp,
				Input::SwapSelectedUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_swap_up_to_limit_from_bottom_to_top_selection() {
		process_module_test(
			&[
				"pick aaa c1",
				"pick aaa c2",
				"pick aaa c3",
				"pick aaa c4",
				"pick aaa c5",
			],
			ViewState::default(),
			&[
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::MoveCursorDown,
				Input::ToggleVisualMode,
				Input::MoveCursorUp,
				Input::MoveCursorUp,
				Input::SwapSelectedUp,
				Input::SwapSelectedUp,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c3",
					"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
					 {Normal(selected)}c4",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
					"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_toggle_visual_mode() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::ToggleVisualMode, Input::ToggleVisualMode],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.handle_input(&mut module);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::ToggleVisualMode);
				assert_eq!(module.visual_index_start, None);
				assert_eq!(module.state, ListState::Normal);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_other_input() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[Input::Other],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn scroll_right() {
		process_module_test(
			&[
				"pick aaaaaaaaaaaa this comment needs to be longer than the width of the view",
				"pick bbbbbbbbbbbb this comment needs to be longer than the width of the view",
				"pick cccccccccccc this comment needs to be longer than the width of the view",
			],
			ViewState {
				size: (50, 4),
				..ViewState::default()
			},
			&[Input::MoveCursorRight; 3],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaaaaaaa {Normal(selected)}s \
					 comment needs to be longer th",
					"{Normal}   {ActionPick}pick   {Normal}bbbbbbbb {Normal}s comment needs to be longer th",
					"{Normal}   {ActionPick}pick   {Normal}cccccccc {Normal}s comment needs to be longer th"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn scroll_left() {
		process_module_test(
			&[
				"pick aaaaaaaaaaaa this comment needs to be longer than the width of the view",
				"pick bbbbbbbbbbbb this comment needs to be longer than the width of the view",
				"pick cccccccccccc this comment needs to be longer than the width of the view",
			],
			ViewState {
				size: (50, 4),
				..ViewState::default()
			},
			&[
				Input::MoveCursorRight,
				Input::MoveCursorRight,
				Input::MoveCursorRight,
				Input::MoveCursorLeft,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context.build_view_data(&mut module);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaaaaaaa \
					 {Normal(selected)}is comment needs to be longer t",
					"{Normal}   {ActionPick}pick   {Normal}bbbbbbbb {Normal}is comment needs to be longer t",
					"{Normal}   {ActionPick}pick   {Normal}cccccccc {Normal}is comment needs to be longer t"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn normal_mode_help() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[],
			|test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				module.state = ListState::Normal;
				let help = module.get_help_keybindings_descriptions().unwrap();
				assert_eq!(help.len(), 22);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn visual_mode_help() {
		process_module_test(
			&["pick aaa c1"],
			ViewState::default(),
			&[],
			|test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				module.state = ListState::Visual;
				let help = module.get_help_keybindings_descriptions().unwrap();
				assert_eq!(help.len(), 14);
			},
		);
	}

	// this can technically never happen, but it's worth testing, just in case of an invalid state
	#[test]
	#[serial_test::serial]
	fn render_noop_list() {
		process_module_test(
			&["break"],
			ViewState::default(),
			&[],
			|test_context: TestContext<'_>| {
				let mut module = List::new(test_context.config);
				test_context
					.rebase_todo_file
					.update_range(1, 1, &EditContext::new().action(Action::Noop));
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}{HELP}",
					"{BODY}",
					"{Normal(selected)} > {Normal(selected)}noop   {Normal(selected)}         "
				);
			},
		);
	}
}
