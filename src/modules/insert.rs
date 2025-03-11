mod insert_state;
mod line_type;

#[cfg(all(unix, test))]
mod tests;

use std::sync::Arc;

use parking_lot::Mutex;

use self::{insert_state::InsertState, line_type::LineType};
use crate::{
	components::{
		choice::{Choice, INPUT_OPTIONS as CHOICE_INPUT_OPTIONS},
		edit::{Edit, INPUT_OPTIONS as EDIT_INPUT_OPTIONS},
	},
	display::DisplayColor,
	input::{Event, InputOptions},
	module::{Module, State},
	process::Results,
	todo_file::{Line, TodoFile},
	view::{LineSegment, RenderContext, ViewData, ViewDataUpdater, ViewLine},
};

pub(crate) struct Insert {
	action_choices: Choice<LineType>,
	edit: Edit,
	line_type: LineType,
	state: InsertState,
	todo_file: Arc<Mutex<TodoFile>>,
}

impl Module for Insert {
	fn activate(&mut self, _: State) -> Results {
		self.state = InsertState::Prompt;
		self.edit.reset();
		Results::new()
	}

	fn build_view_data(&mut self, _: &RenderContext) -> &ViewData {
		match self.state {
			InsertState::Prompt => self.action_choices.get_view_data(),
			InsertState::Edit => {
				self.edit.build_view_data(
					|updater: &mut ViewDataUpdater<'_>| {
						updater.push_leading_line(ViewLine::from(vec![LineSegment::new_with_color(
							"Enter contents of the new line. Empty content cancels creation of a new line.",
							DisplayColor::IndicatorColor,
						)]));
						updater.push_leading_line(ViewLine::new_empty_line());
					},
					|_| {},
				)
			},
		}
	}

	fn input_options(&self) -> &InputOptions {
		match self.state {
			InsertState::Prompt => &CHOICE_INPUT_OPTIONS,
			InsertState::Edit => &EDIT_INPUT_OPTIONS,
		}
	}

	#[expect(clippy::unreachable, reason = "False positive.")]
	fn handle_event(&mut self, event: Event, view_state: &crate::view::State) -> Results {
		let mut results = Results::new();
		match self.state {
			InsertState::Prompt => {
				let choice = self.action_choices.handle_event(event, view_state);
				if let Some(action) = choice {
					if action == &LineType::Cancel {
						results.state(State::List);
					}
					else {
						self.line_type = action.clone();
						self.edit.set_label(format!("{action} ").as_str());
						self.state = InsertState::Edit;
					}
				}
			},
			InsertState::Edit => {
				self.edit.handle_event(event);
				if self.edit.is_finished() {
					let content = self.edit.get_content();
					results.state(State::List);
					if !content.is_empty() {
						let line = match self.line_type {
							LineType::Exec => Line::new_exec(content),
							LineType::Pick => Line::new_pick(content),
							LineType::Label => Line::new_label(content),
							LineType::Reset => Line::new_reset(content),
							LineType::Merge => Line::new_merge(content),
							LineType::UpdateRef => Line::new_update_ref(content),
							// this should exit in the prompt state and never get here
							LineType::Cancel => unreachable!(),
						};
						let mut todo_file = self.todo_file.lock();
						let new_line_index = todo_file.get_selected_line_index() + 1;
						todo_file.add_line(new_line_index, line);
						_ = todo_file.set_selected_line_index(new_line_index);
					}
				}
			},
		}
		results
	}
}

impl Insert {
	pub(crate) fn new(todo_file: Arc<Mutex<TodoFile>>) -> Self {
		let mut action_choices = Choice::new(vec![
			(LineType::Exec, 'e', String::from("exec <command>")),
			(LineType::Pick, 'p', String::from("pick <hash>")),
			(LineType::Label, 'l', String::from("label <label>")),
			(LineType::Reset, 'r', String::from("reset <label>")),
			(
				LineType::Merge,
				'm',
				String::from("merge [-C <commit> | -c <commit>] <label> [# <oneline>]"),
			),
			(LineType::UpdateRef, 'u', String::from("update-ref <reference>")),
			(LineType::Cancel, 'q', String::from("Cancel add line")),
		]);
		action_choices.set_prompt(vec![ViewLine::from("Select the type of line to insert:")]);

		Self {
			action_choices,
			edit: Edit::new(),
			line_type: LineType::Exec,
			state: InsertState::Prompt,
			todo_file,
		}
	}
}
