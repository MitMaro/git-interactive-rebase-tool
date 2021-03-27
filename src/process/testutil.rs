use std::{cell::Cell, env::set_var, path::Path};

use anyhow::Error;
use crossterm::event::{KeyCode, KeyModifiers};
use tempfile::{Builder, NamedTempFile};

use crate::{
	config::{key_bindings::KeyBindings, Config},
	create_key_event,
	display::{size::Size, CrossTerm, Display, Event, KeyEvent},
	input::{input_handler::InputHandler, Input},
	process::{exit_status::ExitStatus, process_module::ProcessModule, process_result::ProcessResult, state::State},
	todo_file::{line::Line, TodoFile},
	view::{view_data::ViewData, View},
};

pub struct TestContext<'t> {
	pub config: &'t Config,
	pub rebase_todo_file: TodoFile,
	todo_file: Cell<NamedTempFile>,
	pub view: View<'t>,
	num_inputs: usize,
}

impl<'t> TestContext<'t> {
	pub fn activate(&self, module: &'_ mut dyn ProcessModule, state: State) -> ProcessResult {
		module.activate(&self.rebase_todo_file, state)
	}

	#[allow(clippy::unused_self)]
	pub fn deactivate(&mut self, module: &'_ mut dyn ProcessModule) {
		module.deactivate();
	}

	pub fn build_view_data<'tc>(&self, module: &'tc mut dyn ProcessModule) -> &'tc ViewData {
		module.build_view_data(&self.view, &self.rebase_todo_file)
	}

	pub fn handle_input(&mut self, module: &'_ mut dyn ProcessModule) -> ProcessResult {
		module.handle_input(&mut self.view, &mut self.rebase_todo_file)
	}

	pub fn handle_n_inputs(&mut self, module: &'_ mut dyn ProcessModule, n: usize) -> Vec<ProcessResult> {
		let mut results = vec![];
		for _ in 0..n {
			results.push(module.handle_input(&mut self.view, &mut self.rebase_todo_file));
		}
		results
	}

	pub fn handle_all_inputs(&mut self, module: &'_ mut dyn ProcessModule) -> Vec<ProcessResult> {
		let mut results = vec![];
		for _ in 0..self.num_inputs {
			results.push(module.handle_input(&mut self.view, &mut self.rebase_todo_file));
		}
		results
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
	pub position: (u16, u16),
	pub size: Size,
}

impl Default for ViewState {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: Size::new(500, 30),
		}
	}
}

fn map_str_to_event(input: &str) -> Event {
	match input {
		"Backspace" => create_key_event!(code KeyCode::Backspace),
		"Enter" => create_key_event!(code KeyCode::Enter),
		"Delete" => create_key_event!(code KeyCode::Delete),
		"End" => create_key_event!(code KeyCode::End),
		"Home" => create_key_event!(code KeyCode::Home),
		"Other" => create_key_event!(code KeyCode::Null),
		"Left" => create_key_event!(code KeyCode::Left),
		"PageUp" | "ScrollJumpUp" => create_key_event!(code KeyCode::PageUp),
		"PageDown" | "ScrollJumpDown" => create_key_event!(code KeyCode::PageDown),
		"Up" | "ScrollUp" => create_key_event!(code KeyCode::Up),
		"Right" | "ScrollRight" => create_key_event!(code KeyCode::Right),
		"Down" | "ScrollDown" => create_key_event!(code KeyCode::Down),
		"Controlz" => create_key_event!('z', "Control"),
		"Controly" => create_key_event!('y', "Control"),
		"Exit" => create_key_event!('d', "Control"),
		"Resize" => Event::Resize(0, 0),
		_ => {
			if input.len() > 1 {
				panic!("Unexpected input: {}", input);
			}
			Event::Key(KeyEvent::new(
				KeyCode::Char(input.chars().next().unwrap()),
				KeyModifiers::NONE,
			))
		},
	}
}

fn map_input_to_event(key_bindings: &KeyBindings, input: Input) -> Event {
	match input {
		Input::Abort => map_str_to_event(key_bindings.abort.first().unwrap().as_str()),
		Input::ActionBreak => map_str_to_event(key_bindings.action_break.first().unwrap().as_str()),
		Input::ActionDrop => map_str_to_event(key_bindings.action_drop.first().unwrap().as_str()),
		Input::ActionEdit => map_str_to_event(key_bindings.action_edit.first().unwrap().as_str()),
		Input::ActionFixup => map_str_to_event(key_bindings.action_fixup.first().unwrap().as_str()),
		Input::ActionPick => map_str_to_event(key_bindings.action_pick.first().unwrap().as_str()),
		Input::ActionReword => map_str_to_event(key_bindings.action_reword.first().unwrap().as_str()),
		Input::ActionSquash => map_str_to_event(key_bindings.action_squash.first().unwrap().as_str()),
		Input::Backspace => map_str_to_event("Backspace"),
		Input::Character(c) => map_str_to_event(String::from(c).as_str()),
		Input::Delete => map_str_to_event("Delete"),
		Input::Down | Input::ScrollDown => map_str_to_event("Down"),
		Input::Edit => map_str_to_event(key_bindings.edit.first().unwrap().as_str()),
		Input::End | Input::ScrollBottom => map_str_to_event("End"),
		Input::Enter => map_str_to_event("Enter"),
		Input::Exit => map_str_to_event("Exit"),
		Input::ForceAbort => map_str_to_event(key_bindings.force_abort.first().unwrap().as_str()),
		Input::ForceRebase => map_str_to_event(key_bindings.force_rebase.first().unwrap().as_str()),
		Input::Help => map_str_to_event(key_bindings.help.first().unwrap().as_str()),
		Input::InsertLine => map_str_to_event(key_bindings.insert_line.first().unwrap().as_str()),
		Input::Home | Input::ScrollTop => map_str_to_event("Home"),
		Input::Left | Input::ScrollLeft => map_str_to_event("Left"),
		Input::MoveCursorDown => map_str_to_event(key_bindings.move_down.first().unwrap().as_str()),
		Input::MoveCursorLeft => map_str_to_event(key_bindings.move_left.first().unwrap().as_str()),
		Input::MoveCursorPageDown => map_str_to_event(key_bindings.move_down_step.first().unwrap().as_str()),
		Input::MoveCursorPageUp => map_str_to_event(key_bindings.move_up_step.first().unwrap().as_str()),
		Input::MoveCursorRight => map_str_to_event(key_bindings.move_right.first().unwrap().as_str()),
		Input::MoveCursorUp => map_str_to_event(key_bindings.move_up.first().unwrap().as_str()),
		Input::No => map_str_to_event(key_bindings.confirm_no.first().unwrap().as_str()),
		Input::OpenInEditor => map_str_to_event(key_bindings.open_in_external_editor.first().unwrap().as_str()),
		Input::Other => map_str_to_event("Other"),
		Input::PageDown | Input::ScrollJumpDown => map_str_to_event("PageDown"),
		Input::PageUp | Input::ScrollJumpUp => map_str_to_event("PageUp"),
		Input::Rebase => map_str_to_event(key_bindings.rebase.first().unwrap().as_str()),
		Input::Redo => map_str_to_event(key_bindings.redo.first().unwrap().as_str()),
		Input::Resize => map_str_to_event("Resize"),
		Input::Right | Input::ScrollRight => map_str_to_event("Right"),
		Input::ShowCommit => map_str_to_event(key_bindings.show_commit.first().unwrap().as_str()),
		Input::ShowDiff => map_str_to_event(key_bindings.show_diff.first().unwrap().as_str()),
		Input::SwapSelectedDown => map_str_to_event(key_bindings.move_selection_down.first().unwrap().as_str()),
		Input::SwapSelectedUp => map_str_to_event(key_bindings.move_selection_up.first().unwrap().as_str()),
		Input::ToggleVisualMode => map_str_to_event(key_bindings.toggle_visual_mode.first().unwrap().as_str()),
		Input::Undo => map_str_to_event(key_bindings.undo.first().unwrap().as_str()),
		Input::Up | Input::ScrollUp => map_str_to_event("Up"),
		Input::Yes => map_str_to_event(key_bindings.confirm_yes.first().unwrap().as_str()),
		_ => {
			panic!("Unsupported input: {:?}", input);
		},
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
				Input::InsertLine => String::from("InsertLine"),
				Input::Home => String::from("Home"),
				Input::Ignore => String::from("Ignore"),
				Input::Insert => String::from("Insert"),
				Input::Kill => String::from("Kill"),
				Input::Left => String::from("Left"),
				Input::MoveCursorDown => String::from("MoveCursorDown"),
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
				Input::Up => String::from("Up"),
				Input::Yes => String::from("Yes"),
				Input::Redo => String::from("Redo"),
				Input::Undo => String::from("Undo"),
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

pub fn process_module_test<C>(lines: &[&str], view_state: ViewState, input: &[Input], callback: C)
where C: for<'p> FnOnce(TestContext<'p>) {
	let git_repo_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("test")
		.join("fixtures")
		.join("simple")
		.to_str()
		.unwrap()
		.to_owned();

	set_var("GIT_DIR", git_repo_dir.as_str());
	let mut config = Config::new().unwrap();
	config.git.editor = String::from("true");
	let mut crossterm = CrossTerm::new();
	crossterm.set_size(view_state.size);
	CrossTerm::set_inputs(
		input
			.iter()
			.map(|i| map_input_to_event(&config.key_bindings, *i))
			.collect(),
	);
	let input_handler = InputHandler::new(&config.key_bindings);
	let display = Display::new(input_handler, &mut crossterm, &config.theme);
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
		rebase_todo_file,
		todo_file: Cell::new(todo_file),
		view,
		num_inputs: input.len(),
	});
}
