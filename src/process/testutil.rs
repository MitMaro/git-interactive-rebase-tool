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
		let path = t.path().to_str().unwrap().to_string();
		self.todo_file.replace(t);
		path
	}

	pub fn delete_todo_file(&self) {
		self.todo_file
			.replace(Builder::new().tempfile().unwrap())
			.close()
			.unwrap()
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
		Input::Abort => map_str_to_event(key_bindings.abort.as_str()),
		Input::ActionBreak => map_str_to_event(key_bindings.action_break.as_str()),
		Input::ActionDrop => map_str_to_event(key_bindings.action_drop.as_str()),
		Input::ActionEdit => map_str_to_event(key_bindings.action_edit.as_str()),
		Input::ActionFixup => map_str_to_event(key_bindings.action_fixup.as_str()),
		Input::ActionPick => map_str_to_event(key_bindings.action_pick.as_str()),
		Input::ActionReword => map_str_to_event(key_bindings.action_reword.as_str()),
		Input::ActionSquash => map_str_to_event(key_bindings.action_squash.as_str()),
		Input::Backspace => map_str_to_event("Backspace"),
		Input::Character(c) => map_str_to_event(c.to_string().as_str()),
		Input::Delete => map_str_to_event("Delete"),
		Input::Down | Input::ScrollDown => map_str_to_event("Down"),
		Input::Edit => map_str_to_event(key_bindings.edit.as_str()),
		Input::End | Input::ScrollBottom => map_str_to_event("End"),
		Input::Enter => map_str_to_event("Enter"),
		Input::Exit => map_str_to_event("Exit"),
		Input::ForceAbort => map_str_to_event(key_bindings.force_abort.as_str()),
		Input::ForceRebase => map_str_to_event(key_bindings.force_rebase.as_str()),
		Input::Help => map_str_to_event(key_bindings.help.as_str()),
		Input::Home | Input::ScrollTop => map_str_to_event("Home"),
		Input::Left | Input::ScrollLeft => map_str_to_event("Left"),
		Input::MoveCursorDown => map_str_to_event(key_bindings.move_down.as_str()),
		Input::MoveCursorLeft => map_str_to_event(key_bindings.move_left.as_str()),
		Input::MoveCursorPageDown => map_str_to_event(key_bindings.move_down_step.as_str()),
		Input::MoveCursorPageUp => map_str_to_event(key_bindings.move_up_step.as_str()),
		Input::MoveCursorRight => map_str_to_event(key_bindings.move_right.as_str()),
		Input::MoveCursorUp => map_str_to_event(key_bindings.move_up.as_str()),
		Input::No => map_str_to_event(key_bindings.confirm_no.as_str()),
		Input::OpenInEditor => map_str_to_event(key_bindings.open_in_external_editor.as_str()),
		Input::Other => map_str_to_event("Other"),
		Input::PageDown | Input::ScrollJumpDown => map_str_to_event("PageDown"),
		Input::PageUp | Input::ScrollJumpUp => map_str_to_event("PageUp"),
		Input::Rebase => map_str_to_event(key_bindings.rebase.as_str()),
		Input::Resize => map_str_to_event("Resize"),
		Input::Right | Input::ScrollRight => map_str_to_event("Right"),
		Input::ShowCommit => map_str_to_event(key_bindings.show_commit.as_str()),
		Input::ShowDiff => map_str_to_event(key_bindings.show_diff.as_str()),
		Input::SwapSelectedDown => map_str_to_event(key_bindings.move_selection_down.as_str()),
		Input::SwapSelectedUp => map_str_to_event(key_bindings.move_selection_up.as_str()),
		Input::ToggleVisualMode => map_str_to_event(key_bindings.toggle_visual_mode.as_str()),
		Input::Up | Input::ScrollUp => map_str_to_event("Up"),
		Input::Yes => map_str_to_event(key_bindings.confirm_yes.as_str()),
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
				State::Edit => "Edit",
				State::Error => "Error",
				State::ExternalEditor => "ExternalEditor",
				State::Help => "Help",
				State::List => "List",
				State::ShowCommit => "ShowCommit",
				State::WindowSizeError => "WindowSizeError",
			}
		}),
		input.map_or("None".to_string(), |input| {
			match input {
				Input::Abort => "Abort".to_string(),
				Input::ActionBreak => "ActionBreak".to_string(),
				Input::ActionDrop => "ActionDrop".to_string(),
				Input::ActionEdit => "ActionEdit".to_string(),
				Input::ActionFixup => "ActionFixup".to_string(),
				Input::ActionPick => "ActionPick".to_string(),
				Input::ActionReword => "ActionReword".to_string(),
				Input::ActionSquash => "ActionSquash".to_string(),
				Input::Backspace => "Backspace".to_string(),
				Input::BackTab => "BackTab".to_string(),
				Input::Character(char) => char.to_string(),
				Input::Delete => "Delete".to_string(),
				Input::Down => "Down".to_string(),
				Input::Edit => "Edit".to_string(),
				Input::End => "End".to_string(),
				Input::Enter => "Enter".to_string(),
				Input::Escape => "Escape".to_string(),
				Input::Exit => "Exit".to_string(),
				Input::ForceAbort => "ForceAbort".to_string(),
				Input::ForceRebase => "ForceRebase".to_string(),
				Input::Help => "Help".to_string(),
				Input::Home => "Home".to_string(),
				Input::Ignore => "Ignore".to_string(),
				Input::Insert => "Insert".to_string(),
				Input::Kill => "Kill".to_string(),
				Input::Left => "Left".to_string(),
				Input::MoveCursorDown => "MoveCursorDown".to_string(),
				Input::MoveCursorLeft => "MoveCursorLeft".to_string(),
				Input::MoveCursorPageDown => "MoveCursorPageDown".to_string(),
				Input::MoveCursorPageUp => "MoveCursorPageUp".to_string(),
				Input::MoveCursorRight => "MoveCursorRight".to_string(),
				Input::MoveCursorUp => "MoveCursorUp".to_string(),
				Input::No => "No".to_string(),
				Input::OpenInEditor => "OpenInEditor".to_string(),
				Input::Other => "Other".to_string(),
				Input::PageDown => "PageDown".to_string(),
				Input::PageUp => "PageUp".to_string(),
				Input::Rebase => "Rebase".to_string(),
				Input::Resize => "Resize".to_string(),
				Input::Right => "Right".to_string(),
				Input::ScrollBottom => "ScrollBottom".to_string(),
				Input::ScrollDown => "ScrollDown".to_string(),
				Input::ScrollJumpDown => "ScrollJumpDown".to_string(),
				Input::ScrollJumpUp => "ScrollJumpUp".to_string(),
				Input::ScrollLeft => "ScrollLeft".to_string(),
				Input::ScrollRight => "ScrollRight".to_string(),
				Input::ScrollTop => "ScrollTop".to_string(),
				Input::ScrollUp => "ScrollUp".to_string(),
				Input::ShowCommit => "ShowCommit".to_string(),
				Input::ShowDiff => "ShowDiff".to_string(),
				Input::SwapSelectedDown => "SwapSelectedDown".to_string(),
				Input::SwapSelectedUp => "SwapSelectedUp".to_string(),
				Input::Tab => "Tab".to_string(),
				Input::ToggleVisualMode => "ToggleVisualMode".to_string(),
				Input::Up => "Up".to_string(),
				Input::Yes => "Yes".to_string(),
			}
		}),
		error
			.as_ref()
			.map_or("None".to_string(), |error| { format!("{:#}", error) })
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
		panic!(vec![
			"\n",
			"ProcessResult does not match",
			"==========",
			"Expected State:",
			format_process_result(input, state, exit_status, error).as_str(),
			"Actual:",
			format_process_result(actual.input, actual.state, actual.exit_status, &actual.error).as_str(),
			"==========\n"
		]
		.join("\n"));
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
		.to_string();

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

	let mut rebase_todo_file = TodoFile::new(todo_file.path().to_str().unwrap(), "#");
	rebase_todo_file.set_lines(lines.iter().map(|l| Line::new(l).unwrap()).collect());

	callback(TestContext {
		config: &config,
		rebase_todo_file,
		todo_file: Cell::new(todo_file),
		view,
		num_inputs: input.len(),
	});
}
