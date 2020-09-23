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
	fn activate(&mut self, _: &GitInteractive, _: State) -> Result<()> {
		if self.state != ExternalEditorState::Empty {
			self.state = ExternalEditorState::Active;
		}
		Ok(())
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
	use crate::build_render_output;
	use crate::config::Config;
	use crate::display::Display;
	use crate::external_editor::{ExternalEditor, ExternalEditorState};
	use crate::git_interactive::GitInteractive;
	use crate::input::input_handler::InputHandler;
	use crate::input::Input;
	use crate::list::action::Action;
	use crate::process::exit_status::ExitStatus;
	use crate::process::process_module::ProcessModule;
	use crate::process::state::State;
	use crate::process::testutil::get_test_todo_path;
	use crate::process_module_handle_input_test;
	use crate::process_module_state;
	use crate::process_module_test;
	use crate::view::View;
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

	process_module_test!(
		external_edit_success,
		["pick aaa comment"],
		process_module_state!(new_state = State::ExternalEditor, previous_state = State::List),
		vec![],
		build_render_output!(""),
		|_: &Config, display: &Display<'_>| -> Box<dyn ProcessModule> {
			Box::new(ExternalEditor::new(
				display,
				get_external_editor("drop aaa comment", "0").as_str(),
			))
		},
		|module: &mut dyn ProcessModule, git_interactive: &mut GitInteractive| {
			assert_process_result!(module.process(git_interactive), state = State::List);
			assert_eq!(git_interactive.get_selected_line_action(), &Action::Drop)
		}
	);

	process_module_test!(
		external_edit_empty_edit_success,
		["pick aaa comment"],
		process_module_state!(new_state = State::ExternalEditor, previous_state = State::List),
		vec![],
		build_render_output!("{TITLE}", "{PROMPT}", "Empty rebase todo file. Do you wish to exit"),
		|_: &Config, display: &Display<'_>| -> Box<dyn ProcessModule> {
			Box::new(ExternalEditor::new(display, get_external_editor("", "0").as_str()))
		},
		|module: &mut dyn ProcessModule, git_interactive: &mut GitInteractive| {
			assert_process_result!(module.process(git_interactive));
			assert_process_result!(module.process(git_interactive));
		}
	);

	process_module_test!(
		external_edit_noop,
		["pick aaa comment"],
		process_module_state!(new_state = State::ExternalEditor, previous_state = State::List),
		vec![],
		build_render_output!(""),
		|_: &Config, display: &Display<'_>| -> Box<dyn ProcessModule> {
			Box::new(ExternalEditor::new(display, get_external_editor("noop", "0").as_str()))
		},
		|module: &mut dyn ProcessModule, git_interactive: &mut GitInteractive| {
			assert_process_result!(module.process(git_interactive), state = State::List);
		}
	);

	process_module_test!(
		external_edit_fail_exit_code,
		["pick aaa comment"],
		process_module_state!(new_state = State::ExternalEditor, previous_state = State::List),
		vec![],
		build_render_output!(""),
		|_: &Config, display: &Display<'_>| -> Box<dyn ProcessModule> {
			Box::new(ExternalEditor::new(
				display,
				get_external_editor("drop aaa comment", "1").as_str(),
			))
		},
		|module: &mut dyn ProcessModule, git_interactive: &mut GitInteractive| {
			assert_process_result!(
				module.process(git_interactive),
				error = anyhow!("Editor returned non-zero exit status."),
				exit_status = ExitStatus::StateError
			);
		}
	);

	process_module_test!(
		external_edit_fail_general,
		["pick aaa comment"],
		process_module_state!(new_state = State::ExternalEditor, previous_state = State::List),
		vec![],
		build_render_output!(""),
		|_: &Config, display: &Display<'_>| -> Box<dyn ProcessModule> {
			Box::new(ExternalEditor::new(display, "does-not-exist-xxxx"))
		},
		|module: &mut dyn ProcessModule, git_interactive: &mut GitInteractive| {
			assert_process_result!(
				module.process(git_interactive),
				error = anyhow!("No such file or directory (os error 2)")
					.context("Unable to run editor (does-not-exist-xxxx)"),
				exit_status = ExitStatus::StateError
			);
		}
	);

	process_module_test!(
		external_edit_fail_invalid_edit,
		["pick aaa comment"],
		process_module_state!(new_state = State::ExternalEditor, previous_state = State::List),
		vec![],
		build_render_output!(""),
		|_: &Config, display: &Display<'_>| -> Box<dyn ProcessModule> {
			Box::new(ExternalEditor::new(
				display,
				get_external_editor("this-is-invalid", "0").as_str(),
			))
		},
		|module: &mut dyn ProcessModule, git_interactive: &mut GitInteractive| {
			assert_process_result!(
				module.process(git_interactive),
				error = anyhow!("Error reading file: {}", get_test_todo_path().to_str().unwrap())
					.context("Invalid line: this-is-invalid"),
				exit_status = ExitStatus::StateError
			);
		}
	);

	process_module_test!(
		external_edit_fail_invalid_editor,
		["pick aaa comment"],
		process_module_state!(new_state = State::ExternalEditor, previous_state = State::List),
		vec![],
		build_render_output!(""),
		|_: &Config, display: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(ExternalEditor::new(display, "\"")) },
		|module: &mut dyn ProcessModule, git_interactive: &mut GitInteractive| {
			assert_process_result!(
				module.process(git_interactive),
				error = anyhow!("Invalid editor: \""),
				exit_status = ExitStatus::StateError
			);
		}
	);

	process_module_test!(
		external_edit_fail_empty_editor,
		["pick aaa comment"],
		process_module_state!(new_state = State::ExternalEditor, previous_state = State::List),
		vec![],
		build_render_output!(""),
		|_: &Config, display: &Display<'_>| -> Box<dyn ProcessModule> { Box::new(ExternalEditor::new(display, "")) },
		|module: &mut dyn ProcessModule, git_interactive: &mut GitInteractive| {
			assert_process_result!(
				module.process(git_interactive),
				error = anyhow!("No editor configured"),
				exit_status = ExitStatus::StateError
			);
		}
	);

	process_module_handle_input_test!(
		external_edit_confirm_empty,
		["pick aaa comment"],
		[Input::Yes],
		|input_handler: &InputHandler<'_>,
		 git_interactive: &mut GitInteractive,
		 view: &View<'_>,
		 display: &Display<'_>| {
			let mut external_edit = ExternalEditor::new(display, "editor");
			external_edit.state = ExternalEditorState::Empty;
			let result = external_edit.handle_input(input_handler, git_interactive, view);
			assert_process_result!(result, input = Input::Yes, exit_status = ExitStatus::Good);
		}
	);

	process_module_handle_input_test!(
		external_edit_not_confirm_empty,
		["pick aaa comment"],
		[Input::No],
		|input_handler: &InputHandler<'_>,
		 git_interactive: &mut GitInteractive,
		 view: &View<'_>,
		 display: &Display<'_>| {
			let mut external_edit = ExternalEditor::new(display, "editor");
			external_edit.state = ExternalEditorState::Empty;
			let result = external_edit.handle_input(input_handler, git_interactive, view);
			assert_process_result!(result, input = Input::No);
			assert_eq!(external_edit.state, ExternalEditorState::Active);
		}
	);

	process_module_handle_input_test!(
		external_edit_not_confirm_other_key,
		["pick aaa comment"],
		[Input::Resize],
		|input_handler: &InputHandler<'_>,
		 git_interactive: &mut GitInteractive,
		 view: &View<'_>,
		 display: &Display<'_>| {
			let mut external_edit = ExternalEditor::new(display, "editor");
			external_edit.state = ExternalEditorState::Empty;
			let result = external_edit.handle_input(input_handler, git_interactive, view);
			assert_process_result!(result, input = Input::Resize);
			assert_eq!(external_edit.state, ExternalEditorState::Empty);
		}
	);
}
