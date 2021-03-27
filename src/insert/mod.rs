mod insert_state;
mod line_type;

#[cfg(all(unix, test))]
mod tests;

use crate::{
	components::{Choice, Edit},
	input::{input_handler::InputMode, Input},
	insert::{insert_state::InsertState, line_type::LineType},
	process::{process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::{line::Line, TodoFile},
	view::{view_data::ViewData, view_line::ViewLine, View},
};

pub struct Insert {
	action_choices: Choice<LineType>,
	edit: Edit,
	edit_view_data: ViewData,
	line_type: LineType,
	state: InsertState,
}

impl ProcessModule for Insert {
	fn activate(&mut self, _: &TodoFile, _: State) -> ProcessResult {
		self.state = InsertState::Prompt;
		self.edit.clear();
		ProcessResult::new()
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &TodoFile) -> &ViewData {
		let view_width = view.get_view_size().width();
		let view_height = view.get_view_size().height();

		match self.state {
			InsertState::Prompt => self.action_choices.get_view_data(view_width, view_height),
			InsertState::Edit => {
				self.edit_view_data.clear();
				self.edit_view_data.set_view_size(view_width, view_height);
				self.edit.update_view_data(&mut self.edit_view_data);
				&self.edit_view_data
			},
		}
	}

	fn handle_input(&mut self, view: &mut View<'_>, rebase_todo: &mut TodoFile) -> ProcessResult {
		let mut result = ProcessResult::new();
		match self.state {
			InsertState::Prompt => {
				let input = view.get_input(InputMode::Default);
				result = result.input(input);
				if let Some(action) = self.action_choices.handle_input(input) {
					if action == &LineType::Cancel {
						result = result.state(State::List);
					}
					else {
						self.line_type = action.clone();
						self.edit.set_label(format!("{} ", action.to_string()).as_str());
						self.state = InsertState::Edit;
					}
				}
			},
			InsertState::Edit => {
				let input = view.get_input(InputMode::Raw);
				result = result.input(input);
				if !self.edit.handle_input(input) && input == Input::Enter {
					let content = self.edit.get_content();
					result = result.state(State::List);
					if !content.is_empty() {
						let line = match self.line_type {
							LineType::Exec => Line::new_exec(content.as_str()),
							LineType::Label => Line::new_label(content.as_str()),
							LineType::Reset => Line::new_reset(content.as_str()),
							LineType::Merge => Line::new_merge(content.as_str()),
							// this should exit in the prompt state and never get here
							LineType::Cancel => unreachable!(),
						};
						let new_line_index = rebase_todo.get_selected_line_index() + 1;
						rebase_todo.add_line(new_line_index, line);
						rebase_todo.set_selected_line_index(new_line_index);
					}
				}
			},
		}

		result
	}
}

impl Insert {
	pub(crate) fn new() -> Self {
		let mut edit = Edit::new();
		edit.set_description("Enter contents of the new line. Empty content cancels creation of a new line.");

		let mut action_choices = Choice::new(vec![
			(LineType::Exec, 'e', String::from("exec <command>")),
			(LineType::Label, 'l', String::from("label <label>")),
			(LineType::Reset, 'r', String::from("reset <label>")),
			(
				LineType::Merge,
				'm',
				String::from("merge [-C <commit> | -c <commit>] <label> [# <oneline>]"),
			),
			(LineType::Cancel, 'c', String::from("Cancel add line")),
		]);
		action_choices.set_prompt(vec![ViewLine::from("Select the type of line to insert:")]);
		let mut edit_view_data = ViewData::new();
		edit_view_data.set_show_title(true);

		Self {
			state: InsertState::Prompt,
			edit,
			edit_view_data,
			action_choices,
			line_type: LineType::Exec,
		}
	}
}
