mod argument_tolkenizer;

use crate::display::Display;
use crate::external_editor::argument_tolkenizer::tolkenize;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::view::view_data::ViewData;
use crate::view::View;
use anyhow::{anyhow, Result};
use std::ffi::OsString;
use std::process::Command;
use std::process::ExitStatus as ProcessExitStatus;

#[derive(Clone, Debug, PartialEq)]
enum ExternalEditorState {
	Active,
	Empty,
}

pub struct ExternalEditor<'e> {
	editor: String,
	display: &'e Display<'e>,
	state: ExternalEditorState,
	view_data_external: ViewData,
	view_data_error: ViewData,
}

impl<'e> ProcessModule for ExternalEditor<'e> {
	fn activate(&mut self, _: &GitInteractive, _: State) -> ProcessResult {
		if self.state != ExternalEditorState::Empty {
			self.state = ExternalEditorState::Active;
		}
		ProcessResult::new()
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &GitInteractive) -> &ViewData {
		let (window_width, window_height) = view.get_view_size();
		let view_data = if let ExternalEditorState::Empty = self.state {
			&mut self.view_data_error
		}
		else {
			&mut self.view_data_external
		};
		view_data.set_view_size(window_width, window_height);
		view_data.rebuild();
		view_data
	}

	fn process(&mut self, git_interactive: &mut GitInteractive) -> ProcessResult {
		let mut result = ProcessResult::new();
		if self.state == ExternalEditorState::Active {
			if let Err(e) = self.run_editor(git_interactive) {
				result = result.error(e).exit_status(ExitStatus::StateError);
			}
			else if let Err(e) = git_interactive.reload_file() {
				result = result.error(e).exit_status(ExitStatus::StateError);
			}
			else if git_interactive.get_lines().is_empty() {
				self.state = ExternalEditorState::Empty;
			}
			else {
				result = result.state(State::List);
			}
		}
		result
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		_git_interactive: &mut GitInteractive,
		_view: &View<'_>,
	) -> ProcessResult
	{
		let input = input_handler.get_input(InputMode::Confirm);
		let mut result = ProcessResult::new().input(input);
		if self.state == ExternalEditorState::Empty {
			match input {
				Input::Yes => {
					result = result.exit_status(ExitStatus::Good);
				},
				Input::No => {
					self.state = ExternalEditorState::Active;
				},
				_ => {},
			}
		}
		result
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

	fn run_editor(&mut self, git_interactive: &GitInteractive) -> Result<()> {
		let mut arguments =
			tolkenize(self.editor.as_str()).map_or(Err(anyhow!("Invalid editor: {}", self.editor)), |args| {
				if args.is_empty() {
					Err(anyhow!("No editor configured"))
				}
				else {
					Ok(args.into_iter().map(OsString::from))
				}
			})?;

		git_interactive.write_file()?;
		let filepath = git_interactive.get_filepath();
		let callback = || -> Result<ProcessExitStatus> {
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
				.map_err(|e| anyhow!(e).context(anyhow!("Unable to run editor ({})", self.editor)))
		};
		let exit_status: ProcessExitStatus = self.display.leave_temporarily(callback)?;

		if !exit_status.success() {
			return Err(anyhow!("Editor returned non-zero exit status."));
		}

		Ok(())
	}
}

#[cfg(all(unix, test))]
mod tests {
	use crate::assert_process_result;
	use crate::assert_rendered_output;
	use crate::external_editor::{ExternalEditor, ExternalEditorState};
	use crate::input::Input;
	use crate::list::action::Action;
	use crate::process::exit_status::ExitStatus;
	use crate::process::state::State;
	use crate::process::testutil::{process_module_test, TestContext, ViewState};
	use anyhow::anyhow;
	use std::path::Path;

	fn get_external_editor(content: &str, exit_code: &str) -> String {
		format!(
			"{} \"{}\" % \"{}\"",
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("test")
				.join("write-content.sh")
				.to_str()
				.unwrap(),
			content,
			exit_code
		)
	}

	#[test]
	#[serial_test::serial]
	fn success() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::MoveCursorLeft],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("drop aaa comment", "0").as_str(),
				);
				test_context.activate(&mut module, State::List);
				assert_process_result!(test_context.process(&mut module), state = State::List);
				assert_eq!(test_context.git_interactive.get_selected_line_action(), &Action::Drop);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn empty_edit_success() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				test_context.activate(&mut module, State::List);
				assert_process_result!(test_context.process(&mut module));
				assert_process_result!(test_context.process(&mut module));
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{PROMPT}",
					"Empty rebase todo file. Do you wish to exit"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn noop() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("noop", "0").as_str());
				test_context.activate(&mut module, State::List);
				assert_process_result!(test_context.process(&mut module), state = State::List);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn fail_exit_code() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("drop aaa comment", "1").as_str(),
				);
				test_context.activate(&mut module, State::List);
				assert_process_result!(
					test_context.process(&mut module),
					error = anyhow!("Editor returned non-zero exit status."),
					exit_status = ExitStatus::StateError
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn fail_general() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, "does-not-exist-xxxx");
				test_context.activate(&mut module, State::List);
				assert_process_result!(
					test_context.process(&mut module),
					error = anyhow!("No such file or directory (os error 2)")
						.context("Unable to run editor (does-not-exist-xxxx)"),
					exit_status = ExitStatus::StateError
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn fail_invalid_edit() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, "\"");
				test_context.activate(&mut module, State::List);
				assert_process_result!(
					test_context.process(&mut module),
					error = anyhow!("Invalid editor: \""),
					exit_status = ExitStatus::StateError
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn fail_empty_editor() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, "");
				test_context.activate(&mut module, State::List);
				assert_process_result!(
					test_context.process(&mut module),
					error = anyhow!("No editor configured"),
					exit_status = ExitStatus::StateError
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn confirm_empty() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::No],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, "editor");
				test_context.activate(&mut module, State::List);
				module.state = ExternalEditorState::Empty;
				assert_process_result!(test_context.handle_input(&mut module), input = Input::No);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn not_confirm_other_key() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, "editor");
				test_context.activate(&mut module, State::List);
				module.state = ExternalEditorState::Empty;
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Resize);
			},
		);
	}
}
