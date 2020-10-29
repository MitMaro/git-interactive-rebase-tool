use crate::display::display_color::DisplayColor;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::todo_file::{EditContext, TodoFile};
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
	fn activate(&mut self, todo_file: &TodoFile, _: State) -> ProcessResult {
		self.content = todo_file.get_selected_line().get_edit_content().to_string();
		self.cursor_position = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
		ProcessResult::new()
	}

	fn deactivate(&mut self) {
		self.content.clear();
		self.view_data.clear();
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &TodoFile) -> &ViewData {
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
		self.view_data.set_content(ViewLine::from(segments));
		self.view_data
			.push_trailing_line(ViewLine::from(LineSegment::new_with_color(
				"Enter to finish",
				DisplayColor::IndicatorColor,
			)));
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		todo_file: &mut TodoFile,
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
				Input::Home => self.cursor_position = 0,
				Input::End => {
					self.cursor_position = UnicodeSegmentation::graphemes(self.content.as_str(), true).count()
				},
				Input::Right => {
					let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
					if self.cursor_position < length {
						self.cursor_position += 1;
					}
				},
				Input::Left => {
					if self.cursor_position != 0 {
						self.cursor_position -= 1;
					}
				},
				Input::Enter => {
					todo_file.update_selected(&EditContext::new().content(self.content.as_str()));
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
	use crate::assert_process_result;
	use crate::assert_rendered_output;
	use crate::edit::Edit;
	use crate::input::Input;
	use crate::process::state::State;
	use crate::process::testutil::{process_module_test, TestContext, ViewState};

	#[test]
	#[serial_test::serial]
	fn move_cursor_end() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::MoveCursorRight],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}foobar{Normal,Underline} ",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_1_left() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::MoveCursorLeft],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}fooba{Normal,Underline}r",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_2_from_start() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::MoveCursorLeft; 2],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}foob{Normal,Underline}a{Normal}r",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_1_from_start() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::MoveCursorLeft; 5],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}f{Normal,Underline}o{Normal}obar",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_to_start() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::MoveCursorLeft; 6],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal,Underline}f{Normal}oobar",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_to_home() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::Home],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal,Underline}f{Normal}oobar",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_to_end() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::End,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}foobar{Normal,Underline} ",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_on_empty_content() {
		process_module_test(
			&["exec "],
			ViewState::default(),
			&[Input::MoveCursorLeft, Input::MoveCursorRight, Input::End, Input::Home],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal,Underline} ",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_attempt_past_start() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::MoveCursorLeft; 10],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal,Underline}f{Normal}oobar",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn move_cursor_attempt_past_end() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::MoveCursorLeft; 10],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal,Underline}f{Normal}oobar",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn multiple_width_unicode_single_width() {
		process_module_test(
			&["exec a🗳b"],
			ViewState::default(),
			&[Input::MoveCursorLeft; 2],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}a{Normal,Underline}🗳{Normal}b",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn multiple_width_unicode_emoji() {
		process_module_test(
			&["exec a😀b"],
			ViewState::default(),
			&[Input::MoveCursorLeft; 2],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}a{Normal,Underline}😀{Normal}b",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn add_character_end() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[Input::Character('x')],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}abcdx{Normal,Underline} ",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn add_character_one_from_end() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[Input::MoveCursorLeft, Input::Character('x')],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}abcx{Normal,Underline}d",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn add_character_one_from_start() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::Character('x'),
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}ax{Normal,Underline}b{Normal}cd",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn add_character_at_start() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::Character('x'),
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}x{Normal,Underline}a{Normal}bcd",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn backspace_at_end() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[Input::Backspace],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}abc{Normal,Underline} ",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn backspace_one_from_end() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[Input::MoveCursorLeft, Input::Backspace],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}ab{Normal,Underline}d",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn backspace_one_from_start() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::Backspace,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal,Underline}b{Normal}cd",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn backspace_at_start() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::Backspace,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal,Underline}a{Normal}bcd",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn delete_at_end() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[Input::Delete],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}abcd{Normal,Underline} ",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn delete_last_character() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[Input::MoveCursorLeft, Input::Delete],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}abc{Normal,Underline} ",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn delete_second_character() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::Delete,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}a{Normal,Underline}c{Normal}d",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn delete_first_character() {
		process_module_test(
			&["exec abcd"],
			ViewState::default(),
			&[
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::MoveCursorLeft,
				Input::Delete,
			],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_all_inputs(&mut module);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal,Underline}b{Normal}cd",
					"{TRAILING}",
					"{IndicatorColor}Enter to finish"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn resize() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Resize);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn finish_edit_no_change() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::Enter],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Enter,
					state = State::List
				);
				assert_eq!(
					test_context.rebase_todo_file.get_selected_line().get_edit_content(),
					"foobar"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn finish_edit_with_change() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::Character('x'), Input::Enter],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.handle_input(&mut module);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Enter,
					state = State::List
				);
				assert_eq!(
					test_context.rebase_todo_file.get_selected_line().get_edit_content(),
					"foobarx"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn ignore_other_input() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::Other, Input::Enter],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Enter,
					state = State::List
				);
				assert_eq!(
					test_context.rebase_todo_file.get_selected_line().get_edit_content(),
					"foobar"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn deactivate() {
		process_module_test(
			&["exec foobar"],
			ViewState::default(),
			&[Input::Other, Input::Enter],
			|mut test_context: TestContext<'_>| {
				let mut module = Edit::new();
				test_context.activate(&mut module, State::List);
				test_context.deactivate(&mut module);
				assert!(module.content.is_empty());
			},
		);
	}
}
