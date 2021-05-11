mod action;
mod argument_tokenizer;
mod external_editor_state;

#[cfg(all(unix, test))]
mod tests;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

use crate::{
	components::choice::Choice,
	external_editor::{action::Action, argument_tokenizer::tokenize, external_editor_state::ExternalEditorState},
	input::{Event, EventHandler, InputOptions, MetaEvent},
	process::{exit_status::ExitStatus, process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::{line::Line, TodoFile},
	view::{render_context::RenderContext, view_data::ViewData, view_line::ViewLine},
};

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::new();
}

pub struct ExternalEditor {
	editor: String,
	empty_choice: Choice<Action>,
	error_choice: Choice<Action>,
	external_command: (String, Vec<String>),
	lines: Vec<Line>,
	state: ExternalEditorState,
	view_data: ViewData,
}

impl ProcessModule for ExternalEditor {
	fn activate(&mut self, todo_file: &TodoFile, _: State) -> ProcessResult {
		let result = ProcessResult::new();
		if let Err(err) = todo_file.write_file() {
			return result.error(err).state(State::List);
		}

		if self.lines.is_empty() {
			self.lines = todo_file.get_lines_owned();
		}

		match self.get_command(todo_file) {
			Ok(external_command) => self.external_command = external_command,
			Err(err) => return result.error(err).state(State::List),
		}
		self.set_state(result, ExternalEditorState::Active)
	}

	fn deactivate(&mut self) {
		self.lines.clear();
		self.view_data.reset();
	}

	fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &mut ViewData {
		match self.state {
			ExternalEditorState::Active => {
				self.view_data.clear();
				self.view_data.push_leading_line(ViewLine::from("Editing..."));
				&mut self.view_data
			},
			ExternalEditorState::Empty => self.empty_choice.get_view_data(),
			ExternalEditorState::Error(ref error) => {
				self.error_choice
					.set_prompt(error.chain().map(|c| ViewLine::from(format!("{:#}", c))).collect());
				self.error_choice.get_view_data()
			},
		}
	}

	fn handle_events(
		&mut self,
		event_handler: &EventHandler,
		_: &RenderContext,
		todo_file: &mut TodoFile,
	) -> ProcessResult {
		let mut result = ProcessResult::new();
		match self.state {
			ExternalEditorState::Active => {
				let event = event_handler.read_event(&INPUT_OPTIONS, |event, _| event);
				result = result.event(event);
				match event {
					Event::Meta(MetaEvent::ExternalCommandSuccess) => {
						match todo_file.load_file() {
							Ok(_) => {
								if todo_file.is_empty() || todo_file.is_noop() {
									result = self.set_state(result, ExternalEditorState::Empty);
								}
								else {
									result = result.state(State::List);
								}
							},
							Err(e) => result = self.set_state(result, ExternalEditorState::Error(e)),
						}
					},
					Event::Meta(MetaEvent::ExternalCommandError) => {
						result = self.set_state(
							result,
							ExternalEditorState::Error(anyhow!("Editor returned a non-zero exit status")),
						);
					},
					_ => {},
				}
			},
			ExternalEditorState::Empty => {
				let (choice, event) = self.empty_choice.handle_event(event_handler);
				result = result.event(event);
				if let Some(action) = choice {
					match *action {
						Action::AbortRebase => result = result.exit_status(ExitStatus::Good),
						Action::EditRebase => result = self.set_state(result, ExternalEditorState::Active),
						Action::UndoAndEdit => {
							todo_file.set_lines(self.lines.clone());
							result = self.undo_and_edit(result, todo_file);
						},
						Action::RestoreAndAbortEdit => {},
					}
				}
			},
			ExternalEditorState::Error(_) => {
				let (choice, event) = self.error_choice.handle_event(event_handler);
				result = result.event(event);
				if let Some(action) = choice {
					match *action {
						Action::AbortRebase => {
							todo_file.set_lines(vec![]);
							result = result.exit_status(ExitStatus::Good);
						},
						Action::EditRebase => result = self.set_state(result, ExternalEditorState::Active),
						Action::RestoreAndAbortEdit => {
							todo_file.set_lines(self.lines.clone());
							result = result.state(State::List);
							if let Err(err) = todo_file.write_file() {
								result = result.error(err);
							}
						},
						Action::UndoAndEdit => {
							todo_file.set_lines(self.lines.clone());
							result = self.undo_and_edit(result, todo_file);
						},
					}
				}
			},
		}
		result
	}
}

impl ExternalEditor {
	pub(crate) fn new(editor: &str) -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);

		let mut empty_choice = Choice::new(vec![
			(Action::AbortRebase, '1', String::from("Abort rebase")),
			(Action::EditRebase, '2', String::from("Edit rebase file")),
			(
				Action::UndoAndEdit,
				'3',
				String::from("Undo modifications and edit rebase file"),
			),
		]);
		empty_choice.set_prompt(vec![ViewLine::from("The rebase file is empty.")]);

		let error_choice = Choice::new(vec![
			(Action::AbortRebase, '1', String::from("Abort rebase")),
			(Action::EditRebase, '2', String::from("Edit rebase file")),
			(
				Action::RestoreAndAbortEdit,
				'3',
				String::from("Restore rebase file and abort edit"),
			),
			(
				Action::UndoAndEdit,
				'4',
				String::from("Undo modifications and edit rebase file"),
			),
		]);

		Self {
			editor: String::from(editor),
			empty_choice,
			error_choice,
			external_command: (String::from(""), vec![]),
			lines: vec![],
			state: ExternalEditorState::Active,
			view_data,
		}
	}

	fn set_state(&mut self, result: ProcessResult, new_state: ExternalEditorState) -> ProcessResult {
		let result = match new_state {
			ExternalEditorState::Active => {
				result.external_command(self.external_command.0.clone(), self.external_command.1.clone())
			},
			ExternalEditorState::Empty | ExternalEditorState::Error(_) => result,
		};
		self.state = new_state;
		result
	}

	fn undo_and_edit(&mut self, result: ProcessResult, todo_file: &mut TodoFile) -> ProcessResult {
		todo_file.set_lines(self.lines.clone());
		if let Err(err) = todo_file.write_file() {
			return result.error(err).state(State::List);
		}
		self.set_state(result, ExternalEditorState::Active)
	}

	fn get_command(&mut self, todo_file: &TodoFile) -> Result<(String, Vec<String>)> {
		let mut parameters = tokenize(self.editor.as_str())
			.map_or(Err(anyhow!("Invalid editor: \"{}\"", self.editor)), |args| {
				if args.is_empty() {
					Err(anyhow!("No editor configured"))
				}
				else {
					Ok(args.into_iter())
				}
			})
			.map_err(|e| anyhow!("Please see the git \"core.editor\" configuration for details").context(e))?;

		let filepath = todo_file.get_filepath();
		let mut file_pattern_found = false;
		let command = parameters.next().unwrap_or_else(|| String::from("false"));
		let mut arguments = parameters
			.map(|a| {
				if a.as_str() == "%" {
					file_pattern_found = true;
					String::from(filepath)
				}
				else {
					a
				}
			})
			.collect::<Vec<String>>();
		if !file_pattern_found {
			arguments.push(String::from(filepath));
		}
		Ok((command, arguments))
	}
}
