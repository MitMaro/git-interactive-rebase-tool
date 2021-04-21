use std::{cell::Cell, path::Path};

use anyhow::Error;
use tempfile::{Builder, NamedTempFile};

use crate::{
	config::{testutil::create_config, Config},
	display::{size::Size, CrossTerm, Display},
	input::{input_handler::InputHandler, testutil::setup_mocked_inputs, Input},
	process::{exit_status::ExitStatus, process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::{line::Line, TodoFile},
	view::{view_data::ViewData, View},
	Exit,
};

pub struct TestContext<'t> {
	pub config: &'t Config,
	pub rebase_todo_file: TodoFile,
	todo_file: Cell<NamedTempFile>,
	pub view: View<'t>,
	pub input_handler: InputHandler<'t>,
	num_inputs: usize,
}

impl<'t> TestContext<'t> {
	fn get_build_data<'tc>(&self, module: &'tc mut dyn ProcessModule) -> &'tc mut ViewData {
		module.build_view_data(&self.view.get_render_context(), &self.rebase_todo_file)
	}

	pub fn activate(&self, module: &'_ mut dyn ProcessModule, state: State) -> ProcessResult {
		module.activate(&self.rebase_todo_file, state)
	}

	#[allow(clippy::unused_self)]
	pub fn deactivate(&mut self, module: &'_ mut dyn ProcessModule) {
		module.deactivate();
	}

	pub fn update_view_data_size(&self, module: &'_ mut dyn ProcessModule) {
		let view_data = self.get_build_data(module);
		let context = self.view.get_render_context();
		view_data.set_view_size(context.width(), context.height());
	}

	pub fn build_view_data<'tc>(&self, module: &'tc mut dyn ProcessModule) -> &'tc mut ViewData {
		let view_data = self.get_build_data(module);
		let context = self.view.get_render_context();
		view_data.set_view_size(context.width(), context.height());
		view_data
	}

	pub fn handle_input(&mut self, module: &'_ mut dyn ProcessModule) -> ProcessResult {
		module.handle_input(&self.input_handler, &mut self.view, &mut self.rebase_todo_file)
	}

	pub fn handle_n_inputs(&mut self, module: &'_ mut dyn ProcessModule, n: usize) -> Vec<ProcessResult> {
		let mut results = vec![];
		for _ in 0..n {
			results.push(module.handle_input(&self.input_handler, &mut self.view, &mut self.rebase_todo_file));
		}
		results
	}

	pub fn handle_all_inputs(&mut self, module: &'_ mut dyn ProcessModule) -> Vec<ProcessResult> {
		let mut results = vec![];
		for _ in 0..self.num_inputs {
			results.push(module.handle_input(&self.input_handler, &mut self.view, &mut self.rebase_todo_file));
		}
		results
	}

	pub fn new_todo_file(&self) -> TodoFile {
		TodoFile::new(self.get_todo_file_path().as_str(), 1, "#")
	}

	pub fn get_todo_file_path(&self) -> String {
		let t = self.todo_file.replace(NamedTempFile::new().unwrap());
		let path = t.path().to_str().unwrap().to_owned();
		self.todo_file.replace(t);
		path
	}

	pub fn delete_todo_file(&self) {
		self.todo_file
			.replace(Builder::new().tempfile().unwrap())
			.close()
			.unwrap();
	}

	pub fn set_todo_file_readonly(&self) {
		let t = self.todo_file.replace(NamedTempFile::new().unwrap());
		let todo_file = t.as_file();
		let mut permissions = todo_file.metadata().unwrap().permissions();
		permissions.set_readonly(true);
		todo_file.set_permissions(permissions).unwrap();
		self.todo_file.replace(t);
	}
}

#[derive(Copy, Clone, Debug)]
pub struct ViewState {
	pub size: Size,
}

impl Default for ViewState {
	fn default() -> Self {
		Self {
			size: Size::new(500, 30),
		}
	}
}

fn format_process_result(
	input: Option<Input>,
	state: Option<State>,
	exit_status: Option<ExitStatus>,
	error: &Option<Error>,
) -> String {
	format!(
		"ExitStatus({}), State({}), Input({}), Error({})",
		exit_status.map_or("None", |exit_status| {
			match exit_status {
				ExitStatus::Abort => "Abort",
				ExitStatus::ConfigError => "ConfigError",
				ExitStatus::FileReadError => "FileReadError",
				ExitStatus::FileWriteError => "FileWriteError",
				ExitStatus::Good => "Good",
				ExitStatus::StateError => "StateError",
				ExitStatus::Kill => "Kill",
			}
		}),
		state.map_or("None", |state| {
			match state {
				State::ConfirmAbort => "ConfirmAbort",
				State::ConfirmRebase => "ConfirmRebase",
				State::Error => "Error",
				State::ExternalEditor => "ExternalEditor",
				State::Insert => "Insert",
				State::List => "List",
				State::ShowCommit => "ShowCommit",
				State::WindowSizeError => "WindowSizeError",
			}
		}),
		input.map_or(String::from("None"), |input| {
			match input {
				Input::Abort => String::from("Abort"),
				Input::ActionBreak => String::from("ActionBreak"),
				Input::ActionDrop => String::from("ActionDrop"),
				Input::ActionEdit => String::from("ActionEdit"),
				Input::ActionFixup => String::from("ActionFixup"),
				Input::ActionPick => String::from("ActionPick"),
				Input::ActionReword => String::from("ActionReword"),
				Input::ActionSquash => String::from("ActionSquash"),
				Input::Backspace => String::from("Backspace"),
				Input::BackTab => String::from("BackTab"),
				Input::Character(char) => String::from(char),
				Input::Delete => String::from("Delete"),
				Input::Down => String::from("Down"),
				Input::Edit => String::from("Edit"),
				Input::End => String::from("End"),
				Input::Enter => String::from("Enter"),
				Input::Escape => String::from("Escape"),
				Input::Exit => String::from("Exit"),
				Input::ForceAbort => String::from("ForceAbort"),
				Input::ForceRebase => String::from("ForceRebase"),
				Input::Help => String::from("Help"),
				Input::Home => String::from("Home"),
				Input::Insert => String::from("Insert"),
				Input::InsertLine => String::from("InsertLine"),
				Input::Kill => String::from("Kill"),
				Input::Left => String::from("Left"),
				Input::MoveCursorDown => String::from("MoveCursorDown"),
				Input::MoveCursorEnd => String::from("MoveCursorEnd"),
				Input::MoveCursorHome => String::from("MoveCursorHome"),
				Input::MoveCursorLeft => String::from("MoveCursorLeft"),
				Input::MoveCursorPageDown => String::from("MoveCursorPageDown"),
				Input::MoveCursorPageUp => String::from("MoveCursorPageUp"),
				Input::MoveCursorRight => String::from("MoveCursorRight"),
				Input::MoveCursorUp => String::from("MoveCursorUp"),
				Input::No => String::from("No"),
				Input::OpenInEditor => String::from("OpenInEditor"),
				Input::Other => String::from("Other"),
				Input::PageDown => String::from("PageDown"),
				Input::PageUp => String::from("PageUp"),
				Input::Rebase => String::from("Rebase"),
				Input::Redo => String::from("Redo"),
				Input::Resize => String::from("Resize"),
				Input::Right => String::from("Right"),
				Input::ScrollBottom => String::from("ScrollBottom"),
				Input::ScrollDown => String::from("ScrollDown"),
				Input::ScrollJumpDown => String::from("ScrollJumpDown"),
				Input::ScrollJumpUp => String::from("ScrollJumpUp"),
				Input::ScrollLeft => String::from("ScrollLeft"),
				Input::ScrollRight => String::from("ScrollRight"),
				Input::ScrollTop => String::from("ScrollTop"),
				Input::ScrollUp => String::from("ScrollUp"),
				Input::ShowCommit => String::from("ShowCommit"),
				Input::ShowDiff => String::from("ShowDiff"),
				Input::SwapSelectedDown => String::from("SwapSelectedDown"),
				Input::SwapSelectedUp => String::from("SwapSelectedUp"),
				Input::Tab => String::from("Tab"),
				Input::ToggleVisualMode => String::from("ToggleVisualMode"),
				Input::Undo => String::from("Undo"),
				Input::Up => String::from("Up"),
				Input::Yes => String::from("Yes"),
			}
		}),
		error
			.as_ref()
			.map_or(String::from("None"), |error| { format!("{:#}", error) })
	)
}

pub fn _assert_process_result(
	actual: &ProcessResult,
	input: Option<Input>,
	state: Option<State>,
	exit_status: Option<ExitStatus>,
	error: &Option<Error>,
) {
	if !(exit_status.map_or(actual.exit_status.is_none(), |expected| {
		actual.exit_status.map_or(false, |actual| expected == actual)
	}) && state.map_or(actual.state.is_none(), |expected| {
		actual.state.map_or(false, |actual| expected == actual)
	}) && input.map_or(actual.input.is_none(), |expected| {
		actual.input.map_or(false, |actual| expected == actual)
	}) && error.as_ref().map_or(actual.error.is_none(), |expected| {
		actual
			.error
			.as_ref()
			.map_or(false, |actual| format!("{:#}", expected) == format!("{:#}", actual))
	})) {
		panic!(
			"{}",
			vec![
				"\n",
				"ProcessResult does not match",
				"==========",
				"Expected State:",
				format_process_result(input, state, exit_status, error).as_str(),
				"Actual:",
				format_process_result(actual.input, actual.state, actual.exit_status, &actual.error).as_str(),
				"==========\n"
			]
			.join("\n")
		);
	}
}

#[macro_export]
macro_rules! assert_process_result {
	($actual:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, None, None, None, &None)
	};
	($actual:expr, error = $error:expr, exit_status = $exit_status:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, None, None, Some($exit_status), &Some($error))
	};
	($actual:expr, state = $state:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, None, Some($state), None, &None)
	};
	($actual:expr, state = $state:expr, error = $error:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, None, Some($state), None, &Some($error))
	};
	($actual:expr, input = $input:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), None, None, &None)
	};
	($actual:expr, input = $input:expr, state = $state:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), Some($state), None, &None)
	};
	($actual:expr, input = $input:expr, exit_status = $exit_status:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), None, Some($exit_status), &None)
	};
}

pub fn process_module_test<C>(lines: &[&str], view_state: ViewState, inputs: &[Input], callback: C)
where C: for<'p> FnOnce(TestContext<'p>) {
	let git_repo_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("test")
		.join("fixtures")
		.join("simple")
		.to_str()
		.unwrap()
		.to_owned();

	let mut config = create_config();
	config.git.editor = String::from("true");
	let mut crossterm = CrossTerm::new();
	crossterm.set_size(view_state.size);
	setup_mocked_inputs(inputs, &config.key_bindings);
	let input_handler = InputHandler::new(&config.key_bindings);
	let display = Display::new(&mut crossterm, &config.theme);
	let view = View::new(display, &config);
	let todo_file = Builder::new()
		.prefix("git-rebase-todo-scratch")
		.suffix("")
		.tempfile_in(git_repo_dir.as_str())
		.unwrap();

	let mut rebase_todo_file = TodoFile::new(todo_file.path().to_str().unwrap(), 1, "#");
	rebase_todo_file.set_lines(lines.iter().map(|l| Line::new(l).unwrap()).collect());

	callback(TestContext {
		config: &config,
		input_handler,
		num_inputs: inputs.len(),
		rebase_todo_file,
		todo_file: Cell::new(todo_file),
		view,
	});
}

fn format_exit_status(exit: &Result<ExitStatus, Exit>) -> String {
	format!(
		"Result({}, {})",
		exit.as_ref()
			.map_or_else(|_| String::from("None"), |e| { format!("{:?}", e) }),
		exit.as_ref().map_or_else(
			|e| { format!("Exit {{ Message({}), Status({:?}) }}", e.message, e.status) },
			|_| String::from("None")
		)
	)
}

pub fn _assert_exit_status(actual: &Result<ExitStatus, Exit>, expected: &Result<ExitStatus, Exit>) {
	if !match actual.as_ref() {
		Ok(actual_exit_status) => {
			if let Ok(expected_exit_status) = expected.as_ref() {
				actual_exit_status == expected_exit_status
			}
			else {
				false
			}
		},
		Err(actual_exit) => {
			if let Err(expected_exit) = expected.as_ref() {
				actual_exit.status == expected_exit.status && actual_exit.message == expected_exit.message
			}
			else {
				false
			}
		},
	} {
		panic!(
			"{}",
			vec![
				"\n",
				"Exit result does not match",
				"==========",
				"Expected:",
				format_exit_status(expected).as_str(),
				"Actual:",
				format_exit_status(actual).as_str(),
				"==========\n"
			]
			.join("\n")
		);
	}
}

#[macro_export]
macro_rules! assert_exit_status {
	($actual:expr, status = $status:expr) => {
		crate::process::testutil::_assert_exit_status(&$actual, &Ok($status))
	};
	($actual:expr, message = $message:expr, status = $status:expr) => {
		crate::process::testutil::_assert_exit_status(
			&$actual,
			&Err(Exit {
				message: String::from($message),
				status: $status,
			}),
		)
	};
}
