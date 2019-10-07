use crate::config::Config;
use crate::display::Display;
use crate::external_editor::argument_tolkenizer::tolkenize;
use crate::git_interactive::GitInteractive;
use crate::input::{Input, InputHandler};
use crate::process::{
	ExitStatus,
	HandleInputResult,
	HandleInputResultBuilder,
	ProcessModule,
	ProcessResult,
	ProcessResultBuilder,
	State,
};
use crate::view::View;
use std::ffi::OsString;
use std::process::Command;
use std::process::ExitStatus as ProcessExitStatus;

enum ExternalEditorState {
	Active,
	Error,
	Finish,
}

pub struct ExternalEditor<'e> {
	config: &'e Config,
	display: &'e Display<'e>,
	state: ExternalEditorState,
}

impl<'e> ProcessModule for ExternalEditor<'e> {
	fn activate(&mut self, _state: State, _git_interactive: &GitInteractive) {
		self.state = ExternalEditorState::Active;
	}

	fn process(&mut self, git_interactive: &mut GitInteractive, _view: &View) -> ProcessResult {
		match self.state {
			ExternalEditorState::Active => self.process_active(git_interactive),
			ExternalEditorState::Error => self.process_error(git_interactive),
			ExternalEditorState::Finish => self.process_finish(git_interactive),
		}
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
		_view: &View,
	) -> HandleInputResult
	{
		match self.state {
			ExternalEditorState::Active => self.handle_input_active(input_handler),
			_ => HandleInputResult::new(Input::Other),
		}
	}

	fn render(&self, _view: &View, _git_interactive: &GitInteractive) {}
}

impl<'e> ExternalEditor<'e> {
	pub fn new(display: &'e Display, config: &'e Config) -> Self {
		Self {
			config,
			display,
			state: ExternalEditorState::Active,
		}
	}

	pub fn run_editor(&mut self, git_interactive: &GitInteractive) -> Result<(), String> {
		let mut arguments = match tolkenize(self.config.editor.as_str()) {
			Some(args) => {
				if args.is_empty() {
					return Err(String::from("No editor configured"));
				}
				args.into_iter().map(OsString::from)
			},
			None => {
				return Err(format!("Invalid editor: {}", self.config.editor));
			},
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
				.map_err(|e| format!("Unable to run editor ({}):\n{}", self.config.editor, e.to_string()))
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
		if let Err(e) = git_interactive.reload_file(self.config.comment_char.as_str()) {
			result = result.error(e.as_str(), State::ExternalEditor);
			self.state = ExternalEditorState::Error;
		}
		else if git_interactive.get_lines().is_empty() {
			result = result.error("Rebase TODO list is empty", State::ExternalEditor);
			self.state = ExternalEditorState::Error;
		}
		else {
			result = result.state(State::List(false));
		}
		result.build()
	}

	fn process_error(&self, git_interactive: &GitInteractive) -> ProcessResult {
		let mut result = ProcessResultBuilder::new().state(State::Exiting);

		if git_interactive.get_lines().is_empty() {
			result = result.exit_status(ExitStatus::Good);
		}
		else {
			result = result.exit_status(ExitStatus::StateError);
		}
		result.build()
	}

	pub fn handle_input_active(&self, input_handler: &InputHandler) -> HandleInputResult {
		let input = input_handler.get_input();
		let mut result = HandleInputResultBuilder::new(input);
		match input {
			Input::Resize => {},
			_ => {
				result = result.state(State::List(false));
			},
		}
		result.build()
	}
}
