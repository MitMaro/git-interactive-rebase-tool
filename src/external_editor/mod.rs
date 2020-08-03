mod argument_tolkenizer;

use crate::display::Display;
use crate::external_editor::argument_tolkenizer::tolkenize;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::process_result::{ProcessResult, ProcessResultBuilder};
use crate::process::state::State;
use crate::view::view_data::ViewData;
use crate::view::View;
use std::ffi::OsString;
use std::process::Command;
use std::process::ExitStatus as ProcessExitStatus;

#[derive(Clone, Debug, PartialEq)]
enum ExternalEditorState {
	Active,
	Error,
	Empty,
	Finish,
}

pub struct ExternalEditor<'e> {
	editor: String,
	display: &'e Display<'e>,
	state: ExternalEditorState,
	view_data_external: ViewData,
	view_data_error: ViewData,
}

impl<'e> ProcessModule for ExternalEditor<'e> {
	fn activate(&mut self, _state: State, _git_interactive: &GitInteractive) {
		if self.state != ExternalEditorState::Empty {
			self.state = ExternalEditorState::Active;
		}
	}

	fn build_view_data(&mut self, _: &View<'_>, _: &GitInteractive) -> &ViewData {
		if let ExternalEditorState::Empty = self.state {
			&self.view_data_error
		}
		else {
			&self.view_data_external
		}
	}

	fn process(&mut self, git_interactive: &mut GitInteractive, _view: &View<'_>) -> ProcessResult {
		match self.state {
			ExternalEditorState::Active => self.process_active(git_interactive),
			ExternalEditorState::Error => Self::process_error(git_interactive),
			ExternalEditorState::Empty => ProcessResult::new(),
			ExternalEditorState::Finish => self.process_finish(git_interactive),
		}
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_git_interactive: &mut GitInteractive,
		_view: &View<'_>,
	) -> HandleInputResult
	{
		match self.state {
			ExternalEditorState::Active => Self::handle_input_active(input_handler),
			ExternalEditorState::Empty => self.handle_input_empty(input_handler),
			_ => HandleInputResult::new(Input::Other),
		}
	}
}

impl<'e> ExternalEditor<'e> {
	pub(crate) fn new(display: &'e Display<'_>, editor: &str) -> Self {
		Self {
			editor: String::from(editor),
			display,
			state: ExternalEditorState::Active,
			view_data_external: ViewData::new(),
			view_data_error: ViewData::new_confirm("Empty rebase todo file. Do you wish to exit"),
		}
	}

	fn run_editor(&mut self, git_interactive: &GitInteractive) -> Result<(), String> {
		let mut arguments = if let Some(args) = tolkenize(self.editor.as_str()) {
			if args.is_empty() {
				return Err(String::from("No editor configured"));
			}
			args.into_iter().map(OsString::from)
		}
		else {
			return Err(format!("Invalid editor: {}", self.editor));
		};

		git_interactive.write_file()?;
		let filepath = git_interactive.get_filepath();
		let callback = || -> Result<ProcessExitStatus, String> {
			let mut file_pattern_found = false;
			let mut cmd = Command::new(arguments.next().unwrap());
			for arg in arguments {
				if arg.as_os_str() == "%" {
					file_pattern_found = true;
					cmd.arg(filepath.as_os_str());
				}
				else {
					cmd.arg(arg);
				}
			}
			if !file_pattern_found {
				cmd.arg(filepath.as_os_str());
			}
			cmd.status()
				.map_err(|e| format!("Unable to run editor ({}):\n{}", self.editor, e.to_string()))
		};
		let exit_status: ProcessExitStatus = self.display.leave_temporarily(callback)?;

		if !exit_status.success() {
			return Err(String::from("Editor returned non-zero exit status."));
		}

		Ok(())
	}

	fn process_active(&mut self, git_interactive: &GitInteractive) -> ProcessResult {
		let mut result = ProcessResultBuilder::new();
		if let Err(e) = self.run_editor(git_interactive) {
			result = result.error(e.as_str(), State::ExternalEditor);
			self.state = ExternalEditorState::Error;
		}
		else {
			self.state = ExternalEditorState::Finish;
		}
		result.build()
	}

	fn process_finish(&mut self, git_interactive: &mut GitInteractive) -> ProcessResult {
		let mut result = ProcessResultBuilder::new();
		if let Err(e) = git_interactive.reload_file() {
			result = result.error(e.as_str(), State::ExternalEditor);
			self.state = ExternalEditorState::Error;
		}
		else if git_interactive.get_lines().is_empty() {
			self.state = ExternalEditorState::Empty;
		}
		else {
			result = result.state(State::List(false));
		}
		result.build()
	}

	fn process_error(git_interactive: &GitInteractive) -> ProcessResult {
		let mut result = ProcessResultBuilder::new().state(State::Exiting);

		if git_interactive.get_lines().is_empty() {
			result = result.exit_status(ExitStatus::Good);
		}
		else {
			result = result.exit_status(ExitStatus::StateError);
		}
		result.build()
	}

	fn handle_input_active(input_handler: &InputHandler<'_>) -> HandleInputResult {
		let input = input_handler.get_input(InputMode::Default);
		let mut result = HandleInputResultBuilder::new(input);
		if let Input::Resize = input {
		}
		else {
			result = result.state(State::List(false));
		}
		result.build()
	}

	fn handle_input_empty(&mut self, input_handler: &InputHandler<'_>) -> HandleInputResult {
		let input = input_handler.get_input(InputMode::Confirm);
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Yes => {
				result = result.exit_status(ExitStatus::Good).state(State::Exiting);
			},
			Input::No => {
				self.state = ExternalEditorState::Active;
				result = result.state(State::ExternalEditor);
			},
			_ => {},
		}
		result.build()
	}
}
