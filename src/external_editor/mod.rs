mod action;
mod argument_tokenizer;
mod external_editor_state;

#[cfg(all(unix, test))]
mod tests;

use std::{
	ffi::OsString,
	process::{Command, ExitStatus as ProcessExitStatus},
};

use anyhow::{anyhow, Result};

use crate::{
	components::Choice,
	external_editor::{action::Action, argument_tokenizer::tokenize, external_editor_state::ExternalEditorState},
	input::{input_handler::InputMode, Input},
	process::{exit_status::ExitStatus, process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::{line::Line, TodoFile},
	view::{view_data::ViewData, view_line::ViewLine, View},
};

pub struct ExternalEditor {
	empty_choice: Choice<Action>,
	error_choice: Choice<Action>,
	editor: String,
	state: ExternalEditorState,
	view_data: ViewData,
	lines: Vec<Line>,
}

impl ProcessModule for ExternalEditor {
	fn activate(&mut self, todo_file: &TodoFile, _: State) -> ProcessResult {
		let mut result = ProcessResult::new();
		self.state = ExternalEditorState::Active;
		if let Err(err) = todo_file.write_file() {
			result = result.error(err).state(State::List);
		}
		else if self.lines.is_empty() {
			self.lines = todo_file.get_lines_owned();
		}
		result
	}

	fn deactivate(&mut self) {
		self.lines.clear();
		self.view_data.reset();
	}

	fn build_view_data(&mut self, _: &View<'_>, _: &TodoFile) -> &mut ViewData {
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

	fn handle_input(&mut self, view: &mut View<'_>, todo_file: &mut TodoFile) -> ProcessResult {
		let mut result = ProcessResult::new();
		match self.state {
			ExternalEditorState::Active => {
				if let Err(e) = self.run_editor(view, todo_file) {
					self.state = ExternalEditorState::Error(e);
				}
				else {
					match todo_file.load_file() {
						Ok(_) => {
							if todo_file.is_empty() || todo_file.is_noop() {
								self.state = ExternalEditorState::Empty;
							}
							else {
								result = result.state(State::List);
							}
						},
						Err(e) => self.state = ExternalEditorState::Error(e),
					}
				}
				result = result.input(Input::Other);
			},
			ExternalEditorState::Empty => {
				let input = view.get_input(InputMode::Default);
				result = result.input(input);
				if let Some(action) = self.empty_choice.handle_input(input) {
					match *action {
						Action::AbortRebase => result = result.exit_status(ExitStatus::Good),
						Action::EditRebase => self.state = ExternalEditorState::Active,
						Action::UndoAndEdit => {
							todo_file.set_lines(self.lines.to_vec());
							self.activate(todo_file, State::ExternalEditor);
						},
						Action::RestoreAndAbortEdit => {},
					}
				}
			},
			ExternalEditorState::Error(_) => {
				let input = view.get_input(InputMode::Default);
				result = result.input(input);
				if let Some(action) = self.error_choice.handle_input(input) {
					match *action {
						Action::AbortRebase => {
							todo_file.set_lines(vec![]);
							result = result.exit_status(ExitStatus::Good);
						},
						Action::EditRebase => self.state = ExternalEditorState::Active,
						Action::RestoreAndAbortEdit => {
							todo_file.set_lines(self.lines.to_vec());
							result = result.state(State::List);
							if let Err(err) = todo_file.write_file() {
								result = result.error(err);
							}
						},
						Action::UndoAndEdit => {
							todo_file.set_lines(self.lines.to_vec());
							self.activate(todo_file, State::ExternalEditor);
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
			empty_choice,
			error_choice,
			editor: String::from(editor),
			state: ExternalEditorState::Active,
			view_data,
			lines: vec![],
		}
	}

	fn run_editor(&mut self, view: &mut View<'_>, todo_file: &TodoFile) -> Result<()> {
		let mut arguments = tokenize(self.editor.as_str())
			.map_or(Err(anyhow!("Invalid editor: \"{}\"", self.editor)), |args| {
				if args.is_empty() {
					Err(anyhow!("No editor configured"))
				}
				else {
					Ok(args.into_iter().map(OsString::from))
				}
			})
			.map_err(|e| anyhow!("Please see the git \"core.editor\" configuration for details").context(e))?;

		let filepath = todo_file.get_filepath();
		let callback = || -> Result<ProcessExitStatus> {
			let mut file_pattern_found = false;
			let mut cmd = Command::new(arguments.next().unwrap());
			for arg in arguments {
				if arg.as_os_str() == "%" {
					file_pattern_found = true;
					cmd.arg(filepath);
				}
				else {
					cmd.arg(arg);
				}
			}
			if !file_pattern_found {
				cmd.arg(filepath);
			}
			cmd.status().map_err(|e| anyhow!(e).context("Unable to run editor"))
		};
		view.end()?;
		let exit_status = callback();
		view.start()?;
		let exit_status = exit_status?;
		if !exit_status.success() {
			return Err(anyhow!("Editor returned a non-zero exit status"));
		}
		Ok(())
	}
}
