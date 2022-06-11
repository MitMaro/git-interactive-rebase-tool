mod action;
mod argument_tokenizer;
mod external_editor_state;

#[cfg(all(unix, test))]
mod tests;

use anyhow::{anyhow, Result};
use input::InputOptions;
use lazy_static::lazy_static;
use todo_file::{Line, TodoFile};
use view::{RenderContext, ViewData, ViewLine};

use self::{action::Action, argument_tokenizer::tokenize, external_editor_state::ExternalEditorState};
use crate::{
	components::choice::{Choice, INPUT_OPTIONS as CHOICE_INPUT_OPTIONS},
	events::{Event, MetaEvent},
	module::{ExitStatus, Module, State},
	process::Results,
};

lazy_static! {
	static ref INPUT_OPTIONS: InputOptions = InputOptions::RESIZE;
}

pub(crate) struct ExternalEditor {
	editor: String,
	empty_choice: Choice<Action>,
	error_choice: Choice<Action>,
	external_command: (String, Vec<String>),
	lines: Vec<Line>,
	state: ExternalEditorState,
	view_data: ViewData,
}

impl Module for ExternalEditor {
	fn activate(&mut self, todo_file: &TodoFile, _: State) -> Results {
		let mut results = Results::new();
		if let Err(err) = todo_file.write_file() {
			results.error_with_return(err, State::List);
			return results;
		}

		if self.lines.is_empty() {
			self.lines = todo_file.get_lines_owned();
		}

		match self.get_command(todo_file) {
			Ok(external_command) => self.external_command = external_command,
			Err(err) => {
				results.error_with_return(err, State::List);
				return results;
			},
		}
		self.set_state(&mut results, ExternalEditorState::Active);
		results
	}

	fn deactivate(&mut self) -> Results {
		self.lines.clear();
		self.view_data.update_view_data(|updater| updater.clear());
		Results::new()
	}

	fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &ViewData {
		match self.state {
			ExternalEditorState::Active => {
				self.view_data.update_view_data(|updater| {
					updater.clear();
					updater.push_leading_line(ViewLine::from("Editing..."));
				});
				&self.view_data
			},
			ExternalEditorState::Empty => self.empty_choice.get_view_data(),
			ExternalEditorState::Error(ref error) => {
				self.error_choice
					.set_prompt(error.chain().map(|c| ViewLine::from(format!("{:#}", c))).collect());
				self.error_choice.get_view_data()
			},
		}
	}

	fn input_options(&self) -> &InputOptions {
		match self.state {
			ExternalEditorState::Active => &INPUT_OPTIONS,
			ExternalEditorState::Empty | ExternalEditorState::Error(_) => &CHOICE_INPUT_OPTIONS,
		}
	}

	fn handle_event(&mut self, event: Event, view_state: &view::State, todo_file: &mut TodoFile) -> Results {
		let mut results = Results::new();
		match self.state {
			ExternalEditorState::Active => {
				match event {
					Event::MetaEvent(MetaEvent::ExternalCommandSuccess) => {
						match todo_file.load_file() {
							Ok(_) => {
								if todo_file.is_empty() || todo_file.is_noop() {
									self.set_state(&mut results, ExternalEditorState::Empty);
								}
								else {
									results.state(State::List);
								}
							},
							Err(e) => self.set_state(&mut results, ExternalEditorState::Error(e)),
						}
					},
					Event::MetaEvent(MetaEvent::ExternalCommandError) => {
						self.set_state(
							&mut results,
							ExternalEditorState::Error(anyhow!("Editor returned a non-zero exit status")),
						);
					},
					_ => {},
				}
			},
			ExternalEditorState::Empty => {
				let choice = self.empty_choice.handle_event(event, view_state);
				if let Some(action) = choice {
					match *action {
						Action::AbortRebase => results.exit_status(ExitStatus::Good),
						Action::EditRebase => self.set_state(&mut results, ExternalEditorState::Active),
						Action::UndoAndEdit => {
							todo_file.set_lines(self.lines.clone());
							self.undo_and_edit(&mut results, todo_file);
						},
						Action::RestoreAndAbortEdit => {},
					}
				}
			},
			ExternalEditorState::Error(_) => {
				let choice = self.error_choice.handle_event(event, view_state);
				if let Some(action) = choice {
					match *action {
						Action::AbortRebase => {
							todo_file.set_lines(vec![]);
							results.exit_status(ExitStatus::Good);
						},
						Action::EditRebase => self.set_state(&mut results, ExternalEditorState::Active),
						Action::RestoreAndAbortEdit => {
							todo_file.set_lines(self.lines.clone());
							results.state(State::List);
							if let Err(err) = todo_file.write_file() {
								results.error(err);
							}
						},
						Action::UndoAndEdit => {
							todo_file.set_lines(self.lines.clone());
							self.undo_and_edit(&mut results, todo_file);
						},
					}
				}
			},
		}
		results
	}
}

impl ExternalEditor {
	pub(crate) fn new(editor: &str) -> Self {
		let view_data = ViewData::new(|updater| {
			updater.set_show_title(true);
		});

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

	fn set_state(&mut self, results: &mut Results, new_state: ExternalEditorState) {
		self.state = new_state;
		match self.state {
			ExternalEditorState::Active => {
				results.external_command(self.external_command.0.clone(), self.external_command.1.clone());
			},
			ExternalEditorState::Empty | ExternalEditorState::Error(_) => {},
		}
	}

	fn undo_and_edit(&mut self, results: &mut Results, todo_file: &mut TodoFile) {
		todo_file.set_lines(self.lines.clone());
		if let Err(err) = todo_file.write_file() {
			results.error_with_return(err, State::List);
			return;
		}
		self.set_state(results, ExternalEditorState::Active);
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

		let filepath = todo_file.get_filepath().to_str().ok_or_else(|| {
			anyhow!(
				"The file path {} is invalid",
				todo_file.get_filepath().to_string_lossy()
			)
		})?;
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
