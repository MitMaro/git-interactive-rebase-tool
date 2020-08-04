use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::HandleInputResult;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::{ProcessResult, ProcessResultBuilder};
use crate::process::state::State;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy, Debug, PartialEq)]
enum EditState {
	Active,
	Finish,
}

pub struct Edit {
	content: String,
	cursor_position: usize,
	state: EditState,
	view_data: ViewData,
}

impl ProcessModule for Edit {
	fn activate(&mut self, _state: &State, application: &GitInteractive) {
		self.state = EditState::Active;
		self.content = application.get_selected_line_edit_content().clone();
		self.cursor_position = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
	}

	fn deactivate(&mut self) {
		self.content.clear();
		self.cursor_position = 0;
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
		if end.is_empty() {
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

	fn process(&mut self, git_interactive: &mut GitInteractive, _view: &View<'_>) -> ProcessResult {
		let mut result = ProcessResultBuilder::new();
		match self.state {
			EditState::Active => {},
			EditState::Finish => {
				git_interactive.edit_selected_line(self.content.as_str());
				result = result.state(State::List(false));
			},
		};
		result.build()
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_: &mut GitInteractive,
		view: &View<'_>,
	) -> HandleInputResult
	{
		if self.state == EditState::Finish {
			return HandleInputResult::new(Input::Enter);
		}
		let mut input;
		loop {
			input = input_handler.get_input(InputMode::Raw);
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
					if self.cursor_position == 0 {
						break;
					}
					let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.take(self.cursor_position - 1)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.skip(self.cursor_position)
						.collect::<String>();
					self.content = format!("{}{}", start, end);
					self.cursor_position -= 1;
				},
				Input::Delete => {
					let length = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
					if self.cursor_position == length {
						break;
					}
					let start = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.take(self.cursor_position)
						.collect::<String>();
					let end = UnicodeSegmentation::graphemes(self.content.as_str(), true)
						.skip(self.cursor_position + 1)
						.collect::<String>();
					self.content = format!("{}{}", start, end);
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
				Input::Enter => self.state = EditState::Finish,
				Input::Resize => {
					let (view_width, view_height) = view.get_view_size();
					self.view_data.set_view_size(view_width, view_height);
				},
				_ => {
					continue;
				},
			}
			break;
		}
		HandleInputResult::new(input)
	}
}

impl Edit {
	pub(crate) fn new() -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);
		Self {
			content: String::from(""),
			cursor_position: 0,
			state: EditState::Active,
			view_data,
		}
	}
}
