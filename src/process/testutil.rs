use crate::config::key_bindings::KeyBindings;
use crate::config::Config;
use crate::display::curses::{Curses, Input as PancursesInput};
use crate::display::Display;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::list::line::Line;
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::view::testutil::render_view_data;
use crate::view::View;
use anyhow::Error;
use std::env::set_var;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug)]
pub struct ProcessModuleTestState {
	pub position: (i32, i32),
	pub view_size: (i32, i32),
	pub state: Option<(State, State)>,
}

pub fn get_test_todo_path() -> PathBuf {
	Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("test")
		.join("git-rebase-todo-scratch")
}

pub fn panic_output_neq(expected: &str, actual: &str) {
	panic!(vec![
		"\n",
		"Unexpected output!",
		"==========",
		"Expected:",
		expected.replace(" ", "·").replace("\t", "   →").as_str(),
		"==========",
		"Actual:",
		actual.replace(" ", "·").replace("\t", "   →").as_str(),
		"==========\n"
	]
	.join("\n"));
}

pub fn _process_module_test<F, C>(
	lines: &[&str],
	module_state: ProcessModuleTestState,
	input: &Option<Vec<Input>>,
	expected_output: &[String],
	get_module: F,
	callback: C,
) where
	F: for<'p> FnOnce(&Config, &'p Display<'p>) -> Box<dyn ProcessModule + 'p>,
	C: for<'p> FnOnce(&'p mut (dyn ProcessModule + 'p), &'p mut GitInteractive),
{
	set_var(
		"GIT_DIR",
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple")
			.to_str()
			.unwrap(),
	);
	let config = Config::new().unwrap();
	let mut curses = Curses::new();
	curses.mv(module_state.position.1, module_state.position.0);
	curses.resize_term(module_state.view_size.1, module_state.view_size.0);
	if let Some(ref input) = *input {
		for i in input {
			curses.push_input(map_input_to_curses(&config.key_bindings, *i));
		}
	}
	let display = Display::new(&mut curses, &config.theme);
	let view = View::new(&display, &config);
	let mut git_interactive = GitInteractive::new(
		lines.iter().map(|l| Line::new(l).unwrap()).collect(),
		get_test_todo_path(),
		"#",
	)
	.unwrap();
	let mut module = get_module(&config, &display);
	if let Some((_, previous_state)) = module_state.state {
		module.activate(&git_interactive, previous_state);
	}
	if let Some(ref input) = *input {
		let input_handler = InputHandler::new(&display, &config.key_bindings);
		for _ in input {
			module.handle_input(&input_handler, &mut git_interactive, &view);
		}
	}
	callback(module.as_mut(), &mut git_interactive);
	let view_data = module.build_view_data(&view, &git_interactive);
	let expected = expected_output.join("\n");
	let output = render_view_data(view_data);
	if output != expected {
		panic_output_neq(expected.as_str(), output.as_str());
	}
}

#[macro_export]
macro_rules! process_module_state {
	(new_state = $new_state:expr, previous_state = $previous_state:expr) => {
		crate::process::testutil::ProcessModuleTestState {
			position: (0, 0),
			view_size: (50, 30),
			state: Some(($new_state, $previous_state)),
			}
	};
}

#[macro_export]
macro_rules! build_render_output {
	($($arg:expr),*) => {{
		let mut args = vec![];
		$( args.push(String::from($arg)); )*
		args
	}};
}

#[macro_export]
macro_rules! process_module_test {
	($name:ident, $lines:expr, $expected_output:expr, $get_module:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_test(
				&$lines,
				crate::process::testutil::ProcessModuleTestState {
					position: (0, 0),
					view_size: (50, 30),
					state: None,
				},
				&None,
				&$expected_output,
				$get_module,
				|_: &mut dyn ProcessModule, _: &mut GitInteractive| {},
			);
		}
	};
	($name:ident, $lines:expr, $state:expr, $input:expr, $expected_output:expr, $get_module:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_test(
				&$lines,
				$state,
				&Some($input),
				&$expected_output,
				$get_module,
				|_: &mut dyn ProcessModule, _: &mut GitInteractive| {},
			);
		}
	};
	($name:ident, $lines:expr, $state:expr, $input:expr, $expected_output:expr, $get_module:expr, $callback:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_test(
				&$lines,
				$state,
				&Some($input),
				&$expected_output,
				$get_module,
				$callback,
			);
		}
	};
}

fn map_input_str_to_curses(input: &str) -> PancursesInput {
	match input {
		"Backspace" => PancursesInput::KeyBackspace,
		"Delete" => PancursesInput::KeyDC,
		"Down" => PancursesInput::KeyDown,
		"End" => PancursesInput::KeyEnd,
		"Enter" => PancursesInput::KeyEnter,
		"F0" => PancursesInput::KeyF0,
		"F1" => PancursesInput::KeyF1,
		"F2" => PancursesInput::KeyF2,
		"F3" => PancursesInput::KeyF3,
		"F4" => PancursesInput::KeyF4,
		"F5" => PancursesInput::KeyF5,
		"F6" => PancursesInput::KeyF6,
		"F7" => PancursesInput::KeyF7,
		"F8" => PancursesInput::KeyF8,
		"F9" => PancursesInput::KeyF9,
		"F10" => PancursesInput::KeyF10,
		"F11" => PancursesInput::KeyF11,
		"F12" => PancursesInput::KeyF12,
		"F13" => PancursesInput::KeyF13,
		"F14" => PancursesInput::KeyF14,
		"F15" => PancursesInput::KeyF15,
		"Home" => PancursesInput::KeyHome,
		"Insert" => PancursesInput::KeyIC,
		"Left" => PancursesInput::KeyLeft,
		"PageDown" => PancursesInput::KeyNPage,
		"PageUp" => PancursesInput::KeyPPage,
		"Resize" => PancursesInput::KeyResize,
		"Right" => PancursesInput::KeyRight,
		"ShiftDelete" => PancursesInput::KeySDC,
		"ShiftDown" => PancursesInput::KeySF,
		"ShiftEnd" => PancursesInput::KeySEnd,
		"ShiftHome" => PancursesInput::KeySHome,
		"ShiftLeft" => PancursesInput::KeySLeft,
		"ShiftRight" => PancursesInput::KeySRight,
		"ShiftTab" => PancursesInput::KeySTab,
		"ShiftUp" => PancursesInput::KeySR,
		"Tab" => PancursesInput::Character('\t'),
		"Up" => PancursesInput::KeyUp,
		"Other" => PancursesInput::KeyEOL, // emulate other with EOL
		_ => PancursesInput::Character(input.chars().next().unwrap()),
	}
}

fn map_input_to_curses(key_bindings: &KeyBindings, input: Input) -> PancursesInput {
	match input {
		Input::Abort => map_input_str_to_curses(key_bindings.abort.as_str()),
		Input::ActionBreak => map_input_str_to_curses(key_bindings.action_break.as_str()),
		Input::ActionDrop => map_input_str_to_curses(key_bindings.action_drop.as_str()),
		Input::ActionEdit => map_input_str_to_curses(key_bindings.action_edit.as_str()),
		Input::ActionFixup => map_input_str_to_curses(key_bindings.action_fixup.as_str()),
		Input::ActionPick => map_input_str_to_curses(key_bindings.action_pick.as_str()),
		Input::ActionReword => map_input_str_to_curses(key_bindings.action_reword.as_str()),
		Input::ActionSquash => map_input_str_to_curses(key_bindings.action_squash.as_str()),
		Input::Backspace => map_input_str_to_curses("Backspace"),
		Input::Character(c) => map_input_str_to_curses(c.to_string().as_str()),
		Input::Delete => map_input_str_to_curses("Delete"),
		Input::Down => map_input_str_to_curses("Down"),
		Input::Edit => map_input_str_to_curses("Edit"),
		Input::End => map_input_str_to_curses("End"),
		Input::Enter => map_input_str_to_curses("Enter"),
		Input::F0 => map_input_str_to_curses("F0"),
		Input::F1 => map_input_str_to_curses("F1"),
		Input::F2 => map_input_str_to_curses("F2"),
		Input::F3 => map_input_str_to_curses("F3"),
		Input::F4 => map_input_str_to_curses("F4"),
		Input::F5 => map_input_str_to_curses("F5"),
		Input::F6 => map_input_str_to_curses("F6"),
		Input::F7 => map_input_str_to_curses("F7"),
		Input::F8 => map_input_str_to_curses("F8"),
		Input::F9 => map_input_str_to_curses("F9"),
		Input::F10 => map_input_str_to_curses("F10"),
		Input::F11 => map_input_str_to_curses("F11"),
		Input::F12 => map_input_str_to_curses("F12"),
		Input::F13 => map_input_str_to_curses("F13"),
		Input::F14 => map_input_str_to_curses("F14"),
		Input::F15 => map_input_str_to_curses("F15"),
		Input::ForceAbort => map_input_str_to_curses(key_bindings.force_abort.as_str()),
		Input::ForceRebase => map_input_str_to_curses(key_bindings.force_rebase.as_str()),
		Input::Help => map_input_str_to_curses(key_bindings.help.as_str()),
		Input::Home => map_input_str_to_curses("Home"),
		Input::Insert => map_input_str_to_curses("Insert"),
		Input::KeypadCenter => map_input_str_to_curses("KeypadCenter"),
		Input::KeypadLowerLeft => map_input_str_to_curses("KeypadLowerLeft"),
		Input::KeypadLowerRight => map_input_str_to_curses("KeypadLowerRight"),
		Input::KeypadUpperLeft => map_input_str_to_curses("KeypadUpperLeft"),
		Input::KeypadUpperRight => map_input_str_to_curses("KeypadUpperRight"),
		Input::Left => map_input_str_to_curses("Left"),
		Input::MoveCursorDown => map_input_str_to_curses(key_bindings.move_down.as_str()),
		Input::MoveCursorLeft => map_input_str_to_curses(key_bindings.move_left.as_str()),
		Input::MoveCursorPageDown => map_input_str_to_curses(key_bindings.move_down_step.as_str()),
		Input::MoveCursorPageUp => map_input_str_to_curses(key_bindings.move_up_step.as_str()),
		Input::MoveCursorRight => map_input_str_to_curses(key_bindings.move_right.as_str()),
		Input::MoveCursorUp => map_input_str_to_curses(key_bindings.move_up.as_str()),
		Input::No => map_input_str_to_curses(key_bindings.confirm_no.as_str()),
		Input::OpenInEditor => map_input_str_to_curses(key_bindings.open_in_external_editor.as_str()),
		Input::Other => map_input_str_to_curses("Other"),
		Input::PageDown => map_input_str_to_curses("PageDown"),
		Input::PageUp => map_input_str_to_curses("PageUp"),
		Input::Print => map_input_str_to_curses("Print"),
		Input::Rebase => map_input_str_to_curses(key_bindings.rebase.as_str()),
		Input::Resize => map_input_str_to_curses("Resize"),
		Input::Right => map_input_str_to_curses("Right"),
		Input::ScrollBottom => map_input_str_to_curses("ScrollBottom"),
		Input::ScrollDown => map_input_str_to_curses("ScrollDown"),
		Input::ScrollJumpDown => map_input_str_to_curses("ScrollJumpDown"),
		Input::ScrollJumpUp => map_input_str_to_curses("ScrollJumpUp"),
		Input::ScrollLeft => map_input_str_to_curses("ScrollLeft"),
		Input::ScrollRight => map_input_str_to_curses("ScrollRight"),
		Input::ScrollTop => map_input_str_to_curses("ScrollTop"),
		Input::ScrollUp => map_input_str_to_curses("ScrollUp"),
		Input::ShiftDelete => map_input_str_to_curses("ShiftDelete"),
		Input::ShiftDown => map_input_str_to_curses("ShiftDown"),
		Input::ShiftEnd => map_input_str_to_curses("ShiftEnd"),
		Input::ShiftHome => map_input_str_to_curses("ShiftHome"),
		Input::ShiftLeft => map_input_str_to_curses("ShiftLeft"),
		Input::ShiftPageDown => map_input_str_to_curses("ShiftPageDown"),
		Input::ShiftPageUp => map_input_str_to_curses("ShiftPageUp"),
		Input::ShiftPrint => map_input_str_to_curses("ShiftPrint"),
		Input::ShiftRight => map_input_str_to_curses("ShiftRight"),
		Input::ShiftTab => map_input_str_to_curses("ShiftTab"),
		Input::ShiftUp => map_input_str_to_curses("ShiftUp"),
		Input::ShowCommit => map_input_str_to_curses(key_bindings.show_commit.as_str()),
		Input::ShowDiff => map_input_str_to_curses(key_bindings.show_diff.as_str()),
		Input::SwapSelectedDown => map_input_str_to_curses(key_bindings.move_selection_down.as_str()),
		Input::SwapSelectedUp => map_input_str_to_curses(key_bindings.move_selection_up.as_str()),
		Input::Tab => map_input_str_to_curses("Tab"),
		Input::ToggleVisualMode => map_input_str_to_curses(key_bindings.toggle_visual_mode.as_str()),
		Input::Up => map_input_str_to_curses("Up"),
		Input::Yes => map_input_str_to_curses(key_bindings.confirm_yes.as_str()),
	}
}

pub fn _process_module_handle_input_test<F>(lines: &[&str], input: &[Input], callback: F)
where F: FnOnce(&InputHandler<'_>, &mut GitInteractive, &View<'_>, &Display<'_>) {
	set_var(
		"GIT_DIR",
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple")
			.to_str()
			.unwrap(),
	);
	let config = Config::new().unwrap();
	let mut curses = Curses::new();
	for i in input {
		curses.push_input(map_input_to_curses(&config.key_bindings, *i));
	}
	let display = Display::new(&mut curses, &config.theme);
	let input_handler = InputHandler::new(&display, &config.key_bindings);
	let view = View::new(&display, &config);
	let mut git_interactive = GitInteractive::new(
		lines.iter().map(|l| Line::new(l).unwrap()).collect(),
		get_test_todo_path(),
		"#",
	)
	.unwrap();
	callback(&input_handler, &mut git_interactive, &view, &display);
}

#[macro_export]
macro_rules! process_module_handle_input_test {
	($name:ident, $lines:expr, $input:expr, $fun:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_handle_input_test(&$lines, &$input, $fun);
		}
	};
}

fn format_process_result(
	input: Option<Input>,
	state: Option<State>,
	exit_status: Option<ExitStatus>,
	error: &Option<Error>,
) -> String
{
	format!(
		"ExitStatus({}), State({}), Input({}), Error({})",
		exit_status.map_or("None", |exit_status| {
			match exit_status {
				ExitStatus::ConfigError => "ConfigError",
				ExitStatus::FileReadError => "FileReadError",
				ExitStatus::FileWriteError => "FileWriteError",
				ExitStatus::Good => "Good",
				ExitStatus::StateError => "StateError",
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
				Input::Character(char) => char.to_string(),
				Input::Delete => "Delete".to_string(),
				Input::Down => "Down".to_string(),
				Input::Edit => "Edit".to_string(),
				Input::End => "End".to_string(),
				Input::Enter => "Enter".to_string(),
				Input::F0 => "F0".to_string(),
				Input::F1 => "F1".to_string(),
				Input::F2 => "F2".to_string(),
				Input::F3 => "F3".to_string(),
				Input::F4 => "F4".to_string(),
				Input::F5 => "F5".to_string(),
				Input::F6 => "F6".to_string(),
				Input::F7 => "F7".to_string(),
				Input::F8 => "F8".to_string(),
				Input::F9 => "F9".to_string(),
				Input::F10 => "F10".to_string(),
				Input::F11 => "F11".to_string(),
				Input::F12 => "F12".to_string(),
				Input::F13 => "F13".to_string(),
				Input::F14 => "F14".to_string(),
				Input::F15 => "F15".to_string(),
				Input::ForceAbort => "ForceAbort".to_string(),
				Input::ForceRebase => "ForceRebase".to_string(),
				Input::Help => "Help".to_string(),
				Input::Home => "Home".to_string(),
				Input::Insert => "Insert".to_string(),
				Input::KeypadCenter => "KeypadCenter".to_string(),
				Input::KeypadLowerLeft => "KeypadLowerLeft".to_string(),
				Input::KeypadLowerRight => "KeypadLowerRight".to_string(),
				Input::KeypadUpperLeft => "KeypadUpperLeft".to_string(),
				Input::KeypadUpperRight => "KeypadUpperRight".to_string(),
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
				Input::Print => "Print".to_string(),
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
				Input::ShiftDelete => "ShiftDelete".to_string(),
				Input::ShiftDown => "ShiftDown".to_string(),
				Input::ShiftEnd => "ShiftEnd".to_string(),
				Input::ShiftHome => "ShiftHome".to_string(),
				Input::ShiftLeft => "ShiftLeft".to_string(),
				Input::ShiftPageDown => "ShiftPageDown".to_string(),
				Input::ShiftPageUp => "ShiftPageUp".to_string(),
				Input::ShiftPrint => "ShiftPrint".to_string(),
				Input::ShiftRight => "ShiftRight".to_string(),
				Input::ShiftTab => "ShiftTab".to_string(),
				Input::ShiftUp => "ShiftUp".to_string(),
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
)
{
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
		crate::process::testutil::_assert_process_result(&$actual, None, None, None, &None);
	};
	($actual:expr, error = $error:expr, exit_status = $exit_status:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, None, None, Some($exit_status), &Some($error));
	};
	($actual:expr, state = $state:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, None, Some($state), None, &None);
	};
	($actual:expr, input = $input:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), None, None, &None);
	};
	($actual:expr, input = $input:expr, state = $state:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), Some($state), None, &None);
	};
	($actual:expr, input = $input:expr, exit_status = $exit_status:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), None, Some($exit_status), &None);
	};
}
