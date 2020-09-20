use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use unicode_segmentation::UnicodeSegmentation;

pub struct Edit {
	content: String,
	cursor_position: usize,
	view_data: ViewData,
}

impl ProcessModule for Edit {
	fn activate(&mut self, git_interactive: &GitInteractive, _: State) -> Result<(), String> {
		self.content = git_interactive.get_selected_line_edit_content().clone();
		self.cursor_position = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
		Ok(())
	}

	fn deactivate(&mut self) {
		self.content.clear();
		self.view_data.clear();
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();

		let line = self.content.as_str();
		let pointer = self.cursor_position;

		let graphemes = UnicodeSegmentation::graphemes(line, true);

		let start = graphemes.clone().take(pointer).collect::<String>();
		let indicator = graphemes.clone().skip(pointer).take(1).collect::<String>();
		let end = graphemes.skip(pointer + 1).collect::<String>();

		let mut segments = vec![
			LineSegment::new(start.as_str()),
			LineSegment::new_with_color_and_style(indicator.as_str(), DisplayColor::Normal, false, true, false),
			LineSegment::new(end.as_str()),
		];
		if indicator.is_empty() {
			segments.push(LineSegment::new_with_color_and_style(
				" ",
				DisplayColor::Normal,
				false,
				true,
				false,
			));
		}
		self.view_data.clear();
		self.view_data.set_content(ViewLine::new(segments));
		self.view_data
			.push_trailing_line(ViewLine::new(vec![LineSegment::new_with_color(
				"Enter to finish",
				DisplayColor::IndicatorColor,
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
		let result = loop {
			let input = input_handler.get_input(InputMode::Raw);
			let result = ProcessResult::new().input(input);
			match input {
				Input::Character(c) => {
					let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.take(self.cursor_position)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.skip(self.cursor_position)
						.collect::<String>();
					self.content = format!("{}{}{}", start, c, end);
					self.cursor_position += 1;
				},
				Input::Backspace => {
					if self.cursor_position != 0 {
						let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
							.take(self.cursor_position - 1)
							.collect::<String>();
						let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
							.skip(self.cursor_position)
							.collect::<String>();
						self.content = format!("{}{}", start, end);
						self.cursor_position -= 1;
					}
				},
				Input::Delete => {
					let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
					if self.cursor_position != length {
						let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
							.take(self.cursor_position)
							.collect::<String>();
						let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
							.skip(self.cursor_position + 1)
							.collect::<String>();
						self.content = format!("{}{}", start, end);
					}
				},
				Input::MoveCursorRight => {
					let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
					if self.cursor_position < length {
						self.cursor_position += 1;
					}
				},
				Input::MoveCursorLeft => {
					if self.cursor_position != 0 {
						self.cursor_position -= 1;
					}
				},
				Input::Enter => {
					git_interactive.edit_selected_line(self.content.as_str());
					break result.state(State::List);
				},
				Input::Resize => {
					let (view_width, view_height) = view.get_view_size();
					self.view_data.set_view_size(view_width, view_height);
				},
				_ => {
					continue;
				},
			}
			break result;
		};
		result
	}
}

impl Edit {
	pub(crate) fn new() -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		Self {
			content: String::from(""),
			cursor_position: 0,
			view_data,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::assert_handle_input_result;
	use crate::build_render_output;
	use crate::config::Config;
	use crate::display::Display;
	use crate::edit::Edit;
	use crate::git_interactive::GitInteractive;
	use crate::input::input_handler::InputHandler;
	use crate::input::Input;
	use crate::process::process_module::ProcessModule;
	use crate::process::state::State;
	use crate::process_module_handle_input_test;
	use crate::process_module_state;
	use crate::process_module_test;
	use crate::view::View;

	process_module_test!(
		edit_move_cursor_end,
		["exec foobar"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}foobar{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_move_cursor_1_left,
		["exec foobar"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}fooba{Normal,Underline}r",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_move_cursor_2_left,
		["exec foobar"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft; 2],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}foob{Normal,Underline}a{Normal}r",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_move_cursor_1_right,
		["exec foobar"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft; 5],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}f{Normal,Underline}o{Normal}obar",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_move_cursor_right,
		["exec foobar"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft; 6],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal,Underline}f{Normal}oobar",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_move_cursor_attempt_past_start,
		["exec foobar"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft; 10],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal,Underline}f{Normal}oobar",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_move_cursor_attempt_past_end,
		["exec foobar"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorRight; 5],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}foobar{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_multiple_width_unicode_single_width,
		["exec a🗳b"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft; 2],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}a{Normal,Underline}🗳{Normal}b",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_multiple_width_unicode_emoji,
		["exec a😀b"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft; 2],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}a{Normal,Underline}😀{Normal}b",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_add_character_end,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::Character('x')],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}abcdx{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_add_character_one_from_end,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft, Input::Character('x')],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}abcx{Normal,Underline}d",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_add_character_one_from_start,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::Character('x')
		],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}ax{Normal,Underline}b{Normal}cd",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_add_character_at_start,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::Character('x')
		],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}x{Normal,Underline}a{Normal}bcd",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_backspace_at_end,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::Backspace],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}abc{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_backspace_one_from_end,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft, Input::Backspace],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}ab{Normal,Underline}d",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_backspace_one_from_start,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::Backspace
		],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal,Underline}b{Normal}cd",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_backspace_at_start,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::Backspace
		],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal,Underline}a{Normal}bcd",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_delete_at_end,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::Delete],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}abcd{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_delete_last_character,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![Input::MoveCursorLeft, Input::Delete],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}abc{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_delete_second_character,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::Delete
		],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal}a{Normal,Underline}c{Normal}d",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_test!(
		edit_cursor_delete_first_character,
		["exec abcd"],
		process_module_state!(new_state = State::Edit, previous_state = State::List),
		vec![
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::MoveCursorLeft,
			Input::Delete
		],
		build_render_output!(
			"{TITLE}",
			"{BODY}",
			"{Normal,Underline}b{Normal}cd",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		),
		|_: &Config, _: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(Edit::new()) }
	);

	process_module_handle_input_test!(
		edit_resize,
		["exec foobar"],
		[Input::Resize],
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut edit = Edit::new();
			edit.activate(git_interactive, State::List).unwrap();
			let result = edit.handle_input(input_handler, git_interactive, view);
			assert_handle_input_result!(result, input = Input::Resize);
		}
	);

	process_module_handle_input_test!(
		edit_finish_edit_no_change,
		["exec foobar"],
		[Input::Enter],
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut edit = Edit::new();
			edit.activate(git_interactive, State::List).unwrap();
			let result = edit.handle_input(input_handler, git_interactive, view);
			assert_handle_input_result!(result, input = Input::Enter, state = State::List);
			assert_eq!(git_interactive.get_selected_line_edit_content(), "foobar");
		}
	);

	process_module_handle_input_test!(
		edit_finish_edit_with_change,
		["exec foobar"],
		[Input::Character('x'), Input::Enter],
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut edit = Edit::new();
			edit.activate(git_interactive, State::List).unwrap();
			edit.handle_input(input_handler, git_interactive, view);
			let result = edit.handle_input(input_handler, git_interactive, view);
			assert_handle_input_result!(result, input = Input::Enter, state = State::List);
			assert_eq!(git_interactive.get_selected_line_edit_content(), "foobarx");
		}
	);

	process_module_handle_input_test!(
		edit_ignore_other_input,
		["exec foobar"],
		[Input::Other, Input::Enter],
		|input_handler: &InputHandler<'_>, git_interactive: &mut GitInteractive, view: &View<'_>| {
			let mut edit = Edit::new();
			edit.activate(git_interactive, State::List).unwrap();
			let result = edit.handle_input(input_handler, git_interactive, view);
			assert_handle_input_result!(result, input = Input::Enter, state = State::List);
		}
	);

	process_module_handle_input_test!(
		edit_deactivate,
		["exec foobar"],
		[Input::MoveCursorLeft],
		|_: &InputHandler<'_>, git_interactive: &mut GitInteractive, _: &View<'_>| {
			let mut edit = Edit::new();
			edit.activate(git_interactive, State::List).unwrap();
			edit.deactivate();
			assert!(edit.content.is_empty());
		}
	);
}
