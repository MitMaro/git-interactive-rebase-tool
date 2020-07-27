use crate::display::display_color::DisplayColor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::handle_input_result::HandleInputResult;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::{ProcessResult, ProcessResultBuilder};
use crate::process::state::State;
use crate::view::View;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy, Debug, PartialEq)]
enum EditState {
	Active,
	Finish,
}

pub(crate) struct Edit {
	content: String,
	cursor_position: usize,
	state: EditState,
}

impl ProcessModule for Edit {
	fn activate(&mut self, _state: State, application: &GitInteractive) {
		self.state = EditState::Active;
		self.content = application.get_selected_line_edit_content().clone();
		self.cursor_position = UnicodeSegmentation::graphemes(self.content.as_str(), true).count();
	}

	fn deactivate(&mut self) {
		self.content.clear();
		self.cursor_position = 0;
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
		_git_interactive: &mut GitInteractive,
		_view: &View<'_>,
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
				_ => {
					continue;
				},
			}
			break;
		}
		HandleInputResult::new(input)
	}

	fn render(&self, view: &View<'_>, _git_interactive: &GitInteractive) {
		let line = self.content.as_str();
		let pointer = self.cursor_position;

		view.draw_title(false);
		view.set_style(false, true, false);
		view.set_color(DisplayColor::Normal, false);

		// this could probably be made way more efficient
		let graphemes = UnicodeSegmentation::graphemes(line, true);
		let segment_length = graphemes.clone().count();
		for (counter, c) in graphemes.enumerate() {
			if counter == pointer {
				view.set_style(false, true, false);
				view.draw_str(c);
				view.set_style(false, false, false);
			}
			else {
				view.draw_str(c);
			}
		}
		if pointer >= segment_length {
			view.set_style(false, true, false);
			view.draw_str(" ");
			view.set_style(false, false, false);
		}

		view.draw_str("\n\n");
		view.set_color(DisplayColor::IndicatorColor, false);
		view.draw_str("Enter to finish");
	}
}

impl Edit {
	pub(crate) fn new() -> Self {
		Self {
			content: String::from(""),
			cursor_position: 0,
			state: EditState::Active,
		}
	}
}
