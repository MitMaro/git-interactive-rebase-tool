mod argument_tolkenizer;

use crate::display::display_color::DisplayColor;
use crate::display::Display;
use crate::input::input_handler::{InputHandler, InputMode};
use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::todo_file::line::Line;
use crate::todo_file::TodoFile;
use crate::view::line_segment::LineSegment;
use crate::view::view_data::ViewData;
use crate::view::view_line::ViewLine;
use crate::view::View;
use anyhow::{anyhow, Error, Result};
use argument_tolkenizer::tolkenize;
use std::ffi::OsString;
use std::process::Command;
use std::process::ExitStatus as ProcessExitStatus;

#[derive(Debug)]
enum ExternalEditorState {
	Active,
	Empty,
	Error(Error),
}

pub struct ExternalEditor<'e> {
	editor: String,
	display: &'e Display<'e>,
	state: ExternalEditorState,
	view_data: ViewData,
	invalid_selection: bool,
	lines: Vec<Line>,
}

impl<'e> ProcessModule for ExternalEditor<'e> {
	fn activate(&mut self, todo_file: &TodoFile, _: State) -> ProcessResult {
		let mut result = ProcessResult::new();
		self.state = ExternalEditorState::Active;
		if let Err(err) = todo_file.write_file() {
			result = result.error(err).state(State::List);
		}
		else if self.lines.is_empty() {
			self.lines = todo_file.get_lines().to_owned();
		}
		result
	}

	fn deactivate(&mut self) {
		self.lines.clear();
		self.invalid_selection = false;
		self.view_data.reset();
	}

	fn build_view_data(&mut self, view: &View<'_>, _: &TodoFile) -> &ViewData {
		let (window_width, window_height) = view.get_view_size();
		self.view_data.clear();

		match self.state {
			ExternalEditorState::Active => self.view_data.push_leading_line(ViewLine::from("Editing...")),
			ExternalEditorState::Empty => {
				self.view_data
					.push_leading_line(ViewLine::from("The rebase file is empty."));
				self.view_data.push_line(ViewLine::new_empty_line());
				self.view_data.push_line(ViewLine::from("1) Abort rebase"));
				self.view_data.push_line(ViewLine::from("2) Edit rebase file"));
				self.view_data
					.push_line(ViewLine::from("3) Undo modifications and edit rebase file"));
				self.view_data.push_line(ViewLine::new_empty_line());
			},
			ExternalEditorState::Error(ref error) => {
				for cause in error.chain() {
					self.view_data.push_line(ViewLine::from(format!("{:#}", cause)));
				}
				self.view_data.push_line(ViewLine::new_empty_line());
				self.view_data.push_line(ViewLine::from("1) Abort rebase"));
				self.view_data.push_line(ViewLine::from("2) Edit rebase file"));
				self.view_data
					.push_line(ViewLine::from("3) Restore rebase file and abort edit"));
				self.view_data
					.push_line(ViewLine::from("4) Undo modifications and edit rebase file"));
				self.view_data.push_line(ViewLine::new_empty_line());
			},
		}

		match &self.state {
			&ExternalEditorState::Active => {},
			&ExternalEditorState::Empty | &ExternalEditorState::Error(_) => {
				if self.invalid_selection {
					self.view_data.push_line(ViewLine::from(LineSegment::new_with_color(
						"Invalid option selected. Please choose an option.",
						DisplayColor::IndicatorColor,
					)));
				}
				else {
					self.view_data.push_line(ViewLine::from(LineSegment::new_with_color(
						"Please choose an option.",
						DisplayColor::IndicatorColor,
					)));
				}
			},
		}

		self.view_data.set_view_size(window_width, window_height);
		self.view_data.rebuild();
		&self.view_data
	}

	fn handle_input(
		&mut self,
		input_handler: &InputHandler<'_>,
		todo_file: &mut TodoFile,
		view: &View<'_>,
	) -> ProcessResult {
		let mut result = ProcessResult::new();
		match self.state {
			ExternalEditorState::Active => {
				if let Err(e) = self.run_editor(todo_file) {
					self.state = ExternalEditorState::Error(e);
				}
				else {
					match todo_file.load_file() {
						Ok(_) => {
							if todo_file.get_lines().is_empty() || todo_file.is_noop() {
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
				let input = input_handler.get_input(InputMode::Default);
				result = result.input(input);
				if let Some(input) = self.handle_standard_inputs(view, input) {
					self.invalid_selection = false;
					match input {
						Input::Character('1') => result = result.exit_status(ExitStatus::Good),
						Input::Character('2') => self.state = ExternalEditorState::Active,
						Input::Character('3') => {
							todo_file.set_lines(self.lines.to_vec());
							self.activate(todo_file, State::ExternalEditor);
						},
						_ => self.invalid_selection = true,
					}
				}
			},
			ExternalEditorState::Error(_) => {
				let input = input_handler.get_input(InputMode::Default);
				result = result.input(input);
				if let Some(input) = self.handle_standard_inputs(view, input) {
					self.invalid_selection = false;
					match input {
						Input::Character('1') => {
							todo_file.set_lines(vec![]);
							result = result.exit_status(ExitStatus::Good)
						},
						Input::Character('2') => self.state = ExternalEditorState::Active,
						Input::Character('3') => {
							todo_file.set_lines(self.lines.to_vec());
							result = result.state(State::List);
							if let Err(err) = todo_file.write_file() {
								result = result.error(err);
							}
						},
						Input::Character('4') => {
							todo_file.set_lines(self.lines.to_vec());
							self.activate(todo_file, State::ExternalEditor);
						},
						_ => self.invalid_selection = true,
					}
				}
			},
		}
		result
	}
}

impl<'e> ExternalEditor<'e> {
	pub(crate) fn new(display: &'e Display<'_>, editor: &str) -> Self {
		let mut view_data = ViewData::new();
		view_data.set_show_title(true);

		Self {
			editor: String::from(editor),
			display,
			state: ExternalEditorState::Active,
			view_data,
			invalid_selection: false,
			lines: vec![],
		}
	}

	fn handle_standard_inputs(&mut self, view: &View<'_>, input: Input) -> Option<Input> {
		match input {
			Input::ScrollLeft => self.view_data.scroll_left(),
			Input::ScrollRight => self.view_data.scroll_right(),
			Input::ScrollDown => self.view_data.scroll_down(),
			Input::ScrollUp => self.view_data.scroll_up(),
			Input::Resize => {
				let (window_width, window_height) = view.get_view_size();
				self.view_data.set_view_size(window_width, window_height);
			},
			_ => return Some(input),
		}
		None
	}

	fn run_editor(&mut self, todo_file: &TodoFile) -> Result<()> {
		let mut arguments = tolkenize(self.editor.as_str())
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
		let exit_status: ProcessExitStatus = self.display.leave_temporarily(callback)?;

		if !exit_status.success() {
			return Err(anyhow!("Editor returned a non-zero exit status"));
		}

		Ok(())
	}
}

#[cfg(all(unix, test))]
mod tests {
	use super::*;
	use crate::assert_process_result;
	use crate::assert_rendered_output;
	use crate::process::testutil::{process_module_test, TestContext, ViewState};
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

	#[macro_export]
	macro_rules! assert_external_editor_state_eq {
		($actual:expr, $expected:expr) => {
			assert_external_editor_state_eq(&$actual, &$expected);
		};
	}

	fn assert_external_editor_state_eq(actual: &ExternalEditorState, expected: &ExternalEditorState) {
		let actual_state = match *actual {
			ExternalEditorState::Active => String::from("Active"),
			ExternalEditorState::Empty => String::from("Empty"),
			ExternalEditorState::Error(ref err) => format!("Error({:#})", err),
		};

		let expected_state = match *expected {
			ExternalEditorState::Active => String::from("Active"),
			ExternalEditorState::Empty => String::from("Empty"),
			ExternalEditorState::Error(ref err) => format!("Error({:#})", err),
		};

		if actual_state != expected_state {
			panic!(vec![
				"\n",
				"ExternalEditorState does not match",
				"==========",
				"Expected:",
				expected_state.as_str(),
				"Actual:",
				actual_state.as_str(),
				"==========\n"
			]
			.join("\n"));
		}
	}

	#[test]
	#[serial_test::serial]
	fn activate() {
		process_module_test(
			&["pick aaa comment1", "drop bbb comment2"],
			ViewState::default(),
			&[Input::Up],
			|test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("pick aaa comment", "0").as_str(),
				);
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_eq!(test_context.rebase_todo_file.get_lines(), &vec![
					Line::new("pick aaa comment1").unwrap(),
					Line::new("drop bbb comment2").unwrap()
				]);
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn activate_write_file_fail() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Up],
			|test_context: TestContext<'_>| {
				let todo_path = test_context.get_todo_file_path();
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("pick aaa comment", "0").as_str(),
				);
				test_context.set_todo_file_readonly();
				assert_process_result!(
					test_context.activate(&mut module, State::List),
					state = State::List,
					error = anyhow!("Error opening file: {}: Permission denied (os error 13)", todo_path)
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn deactivate() {
		process_module_test(
			&["pick aaa comment", "drop bbb comment2"],
			ViewState::default(),
			&[Input::Up],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("pick aaa comment", "0").as_str(),
				);
				test_context.deactivate(&mut module);
				assert_eq!(module.lines, vec![]);
				assert_eq!(module.invalid_selection, false);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn edit_success() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Up],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("pick aaa comment", "0").as_str(),
				);
				assert_process_result!(test_context.activate(&mut module, State::List));
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "{TITLE}", "{LEADING}", "{Normal}Editing...");
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Other,
					state = State::List
				);
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn empty_edit_error() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('1')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{LEADING}",
					"{Normal}The rebase file is empty.",
					"{BODY}",
					"",
					"{Normal}1) Abort rebase",
					"{Normal}2) Edit rebase file",
					"{Normal}3) Undo modifications and edit rebase file",
					"",
					"{IndicatorColor}Please choose an option."
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn empty_edit_abort_rebase() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('1')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
				test_context.build_view_data(&mut module);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Character('1'),
					exit_status = ExitStatus::Good
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn empty_edit_re_edit_rebase_file() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('2')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
				test_context.build_view_data(&mut module);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Character('2'));
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn empty_edit_undo_and_edit() {
		process_module_test(
			&["pick aaa comment", "drop bbb comment"],
			ViewState::default(),
			&[Input::Character('3')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
				test_context.build_view_data(&mut module);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Character('3'));
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
				assert_eq!(test_context.rebase_todo_file.get_lines(), &vec![
					Line::new("pick aaa comment").unwrap(),
					Line::new("drop bbb comment").unwrap()
				]);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn empty_edit_invalid_selection() {
		process_module_test(
			&["pick aaa comment", "drop bbb comment"],
			ViewState::default(),
			&[Input::Character('4')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Character('4'));
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{LEADING}",
					"{Normal}The rebase file is empty.",
					"{BODY}",
					"",
					"{Normal}1) Abort rebase",
					"{Normal}2) Edit rebase file",
					"{Normal}3) Undo modifications and edit rebase file",
					"",
					"{IndicatorColor}Invalid option selected. Please choose an option."
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn empty_edit_noop() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('1')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("noop", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Empty);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{LEADING}",
					"{Normal}The rebase file is empty.",
					"{BODY}",
					"",
					"{Normal}1) Abort rebase",
					"{Normal}2) Edit rebase file",
					"{Normal}3) Undo modifications and edit rebase file",
					"",
					"{IndicatorColor}Please choose an option."
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn no_editor_set() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Up],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, "");
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(
					module.state,
					ExternalEditorState::Error(
						anyhow!("Please see the git \"core.editor\" configuration for details")
							.context(anyhow!("No editor configured"))
					)
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}No editor configured",
					"{Normal}Please see the git \"core.editor\" configuration for details",
					"",
					"{Normal}1) Abort rebase",
					"{Normal}2) Edit rebase file",
					"{Normal}3) Restore rebase file and abort edit",
					"{Normal}4) Undo modifications and edit rebase file",
					"",
					"{IndicatorColor}Please choose an option."
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn invalid_editor_set() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Up],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					Path::new(env!("CARGO_MANIFEST_DIR"))
						.join("test")
						.join("not-executable.sh")
						.to_str()
						.unwrap(),
				);
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(
					module.state,
					ExternalEditorState::Error(
						anyhow!("Permission denied (os error 13)").context(anyhow!("Unable to run editor"))
					)
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}Unable to run editor",
					"{Normal}Permission denied (os error 13)",
					"",
					"{Normal}1) Abort rebase",
					"{Normal}2) Edit rebase file",
					"{Normal}3) Restore rebase file and abort edit",
					"{Normal}4) Undo modifications and edit rebase file",
					"",
					"{IndicatorColor}Please choose an option."
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn editor_non_zero_exit() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Up],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("pick aaa comment", "1").as_str(),
				);
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(
					module.state,
					ExternalEditorState::Error(anyhow!("Editor returned a non-zero exit status"))
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}Editor returned a non-zero exit status",
					"",
					"{Normal}1) Abort rebase",
					"{Normal}2) Edit rebase file",
					"{Normal}3) Restore rebase file and abort edit",
					"{Normal}4) Undo modifications and edit rebase file",
					"",
					"{IndicatorColor}Please choose an option."
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn editor_reload_error() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Up],
			|mut test_context: TestContext<'_>| {
				let todo_path = test_context.get_todo_file_path();
				let mut module = ExternalEditor::new(test_context.display, "true");
				assert_process_result!(test_context.activate(&mut module, State::List));
				test_context.delete_todo_file();
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_external_editor_state_eq!(
					module.state,
					ExternalEditorState::Error(
						anyhow!("Error reading file: {}", todo_path).context("No such file or directory (os error 2)")
					)
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}No such file or directory (os error 2)",
					format!("{{Normal}}Error reading file: {}", todo_path),
					"",
					"{Normal}1) Abort rebase",
					"{Normal}2) Edit rebase file",
					"{Normal}3) Restore rebase file and abort edit",
					"{Normal}4) Undo modifications and edit rebase file",
					"",
					"{IndicatorColor}Please choose an option."
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_abort_rebase() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('1')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("pick aaa comment", "1").as_str(),
				);
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				test_context.build_view_data(&mut module);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Character('1'),
					exit_status = ExitStatus::Good
				);
				assert_eq!(test_context.rebase_todo_file.get_lines(), &vec![]);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_edit_rebase() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('2')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("pick aaa comment", "1").as_str(),
				);
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				test_context.build_view_data(&mut module);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Character('2'));
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_restore_and_abort() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('3')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("drop aaa comment", "1").as_str(),
				);
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				test_context.build_view_data(&mut module);
				assert_process_result!(
					test_context.handle_input(&mut module),
					input = Input::Character('3'),
					state = State::List
				);
				assert_eq!(test_context.rebase_todo_file.get_lines(), &vec![Line::new(
					"pick aaa comment"
				)
				.unwrap()]);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_undo_modifications_and_reedit() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('4')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(
					test_context.display,
					get_external_editor("rdop aaa comment", "1").as_str(),
				);
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				test_context.build_view_data(&mut module);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Character('4'));
				assert_external_editor_state_eq!(module.state, ExternalEditorState::Active);
				assert_eq!(test_context.rebase_todo_file.get_lines(), &vec![Line::new(
					"pick aaa comment"
				)
				.unwrap()]);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn error_invalid_selection() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[Input::Character('5')],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, "");
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Character('5'));
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{BODY}",
					"{Normal}No editor configured",
					"{Normal}Please see the git \"core.editor\" configuration for details",
					"",
					"{Normal}1) Abort rebase",
					"{Normal}2) Edit rebase file",
					"{Normal}3) Restore rebase file and abort edit",
					"{Normal}4) Undo modifications and edit rebase file",
					"",
					"{IndicatorColor}Invalid option selected. Please choose an option."
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn scroll_right() {
		process_module_test(
			&["pick aaa comment", "drop bbb comment"],
			ViewState {
				size: (10, 3),
				..ViewState::default()
			},
			&[Input::Right],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				test_context.build_view_data(&mut module);
				assert_process_result!(
					test_context.handle_all_inputs(&mut module).last().unwrap(),
					input = Input::ScrollRight
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "{TITLE}", "{LEADING}", "{Normal}he rebase ", "{BODY}", "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn scroll_left() {
		process_module_test(
			&["pick aaa comment", "drop bbb comment"],
			ViewState {
				size: (10, 3),
				..ViewState::default()
			},
			&[Input::Right, Input::Right, Input::Right, Input::Left],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				test_context.build_view_data(&mut module);
				assert_process_result!(
					test_context.handle_all_inputs(&mut module).last().unwrap(),
					input = Input::ScrollLeft
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "{TITLE}", "{LEADING}", "{Normal}e rebase f", "{BODY}", "");
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn scroll_down() {
		process_module_test(
			&["pick aaa comment", "drop bbb comment"],
			ViewState {
				size: (10, 3),
				..ViewState::default()
			},
			&[Input::Down],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				test_context.build_view_data(&mut module);
				assert_process_result!(
					test_context.handle_all_inputs(&mut module).last().unwrap(),
					input = Input::ScrollDown
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{LEADING}",
					"{Normal}The rebase",
					"{BODY}",
					"{Normal}1) Abort "
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn scroll_up() {
		process_module_test(
			&["pick aaa comment", "drop bbb comment"],
			ViewState {
				size: (10, 3),
				..ViewState::default()
			},
			&[Input::Down, Input::Down, Input::Down, Input::Up],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				test_context.build_view_data(&mut module);
				assert_process_result!(
					test_context.handle_all_inputs(&mut module).last().unwrap(),
					input = Input::ScrollUp
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(
					view_data,
					"{TITLE}",
					"{LEADING}",
					"{Normal}The rebase",
					"{BODY}",
					"{Normal}2) Edit r"
				);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn resize() {
		process_module_test(
			&["pick aaa comment", "drop bbb comment"],
			ViewState {
				size: (10, 3),
				..ViewState::default()
			},
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut module = ExternalEditor::new(test_context.display, get_external_editor("", "0").as_str());
				assert_process_result!(test_context.activate(&mut module, State::List));
				assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
				assert_process_result!(
					test_context.handle_all_inputs(&mut module).last().unwrap(),
					input = Input::Resize
				);
				let view_data = test_context.build_view_data(&mut module);
				assert_rendered_output!(view_data, "{TITLE}", "{LEADING}", "{Normal}The rebase", "{BODY}", "");
			},
		);
	}
}
