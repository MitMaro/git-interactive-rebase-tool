use crate::config::key_bindings::KeyBindings;
use crate::config::Config;
use crate::display::curses::{Curses, Input as PancursesInput};
use crate::display::Display;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::list::line::Line;
use crate::process::exit_status::ExitStatus;
use crate::process::handle_input_result::{HandleInputResult, HandleInputResultBuilder};
use crate::process::process_module::ProcessModule;
use crate::process::state::State;
use crate::view::testutil::render_view_data;
use crate::view::View;
use std::env::set_var;
use std::path::Path;

pub fn panic_output_neq(expected: &str, actual: &str) {
	panic!(vec![
		"\n==========",
		"Unexpected output!",
		"==========",
		"Expected:",
		expected,
		"==========",
		"Actual:",
		actual,
		"==========\n"
	]
	.join("\n"));
}

pub fn _process_module_test<F>(
	lines: Vec<&str>,
	state: ((i32, i32), (i32, i32)),
	expected_output: Vec<String>,
	get_module: F,
) where
	F: FnOnce(&Config, &Display<'_>) -> Box<dyn ProcessModule>,
{
	let (position, view_size) = state;
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
	curses.mv(position.1, position.0);
	curses.resize_term(view_size.1, view_size.0);
	let display = Display::new(&mut curses, &config.theme);
	let view = View::new(&display, &config);
	let git_interactive = GitInteractive::new(
		lines.iter().map(|l| Line::new(l).unwrap()).collect(),
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("git-rebase-todo-short"),
		"#",
	)
	.unwrap();
	let mut module = get_module(&config, &display);
	let view_data = module.build_view_data(&view, &git_interactive);
	let expected = expected_output.join("\n");
	let output = render_view_data(view_data);
	if output != expected {
		panic_output_neq(expected.as_str(), output.as_str());
	}
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
			crate::process::testutil::_process_module_test($lines, ((0, 0), (10, 10)), $expected_output, $get_module);
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
		Input::Edit => map_input_str_to_curses("Edit"),
		Input::Enter => map_input_str_to_curses("Enter"),
		Input::ForceAbort => map_input_str_to_curses(key_bindings.force_abort.as_str()),
		Input::ForceRebase => map_input_str_to_curses(key_bindings.force_rebase.as_str()),
		Input::Help => map_input_str_to_curses(key_bindings.help.as_str()),
		Input::MoveCursorDown => map_input_str_to_curses(key_bindings.move_down.as_str()),
		Input::MoveCursorLeft => map_input_str_to_curses(key_bindings.move_left.as_str()),
		Input::MoveCursorPageDown => map_input_str_to_curses(key_bindings.move_down_step.as_str()),
		Input::MoveCursorPageUp => map_input_str_to_curses(key_bindings.move_up_step.as_str()),
		Input::MoveCursorRight => map_input_str_to_curses(key_bindings.move_right.as_str()),
		Input::MoveCursorUp => map_input_str_to_curses(key_bindings.move_up.as_str()),
		Input::No => map_input_str_to_curses(key_bindings.confirm_no.as_str()),
		Input::OpenInEditor => map_input_str_to_curses(key_bindings.open_in_external_editor.as_str()),
		Input::Other => map_input_str_to_curses("Other"),
		Input::Rebase => map_input_str_to_curses(key_bindings.rebase.as_str()),
		Input::Resize => map_input_str_to_curses("Resize"),
		Input::ShowCommit => map_input_str_to_curses(key_bindings.show_commit.as_str()),
		Input::ShowDiff => map_input_str_to_curses(key_bindings.show_diff.as_str()),
		Input::SwapSelectedDown => map_input_str_to_curses(key_bindings.move_selection_down.as_str()),
		Input::SwapSelectedUp => map_input_str_to_curses(key_bindings.move_selection_up.as_str()),
		Input::ToggleVisualMode => map_input_str_to_curses(key_bindings.toggle_visual_mode.as_str()),
		Input::Yes => map_input_str_to_curses(key_bindings.confirm_yes.as_str()),
	}
}

pub fn _process_module_handle_input_test<F>(lines: Vec<&str>, input: Input, callback: F)
where F: FnOnce(&InputHandler<'_>, &mut GitInteractive, &View<'_>) {
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
	curses.push_input(map_input_to_curses(&config.key_bindings, input));
	let display = Display::new(&mut curses, &config.theme);
	let input_handler = InputHandler::new(&display, &config.key_bindings);
	let view = View::new(&display, &config);
	let mut git_interactive = GitInteractive::new(
		lines.iter().map(|l| Line::new(l).unwrap()).collect(),
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("git-rebase-todo-short"),
		"#",
	)
	.unwrap();
	callback(&input_handler, &mut git_interactive, &view);
}

#[macro_export]
macro_rules! process_module_handle_input_test {
	($name:ident, $lines:expr, $input:expr, $fun:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_handle_input_test($lines, $input, $fun);
		}
	};
}

pub fn _assert_handle_input_result(
	actual: HandleInputResult,
	input: Input,
	state: Option<State>,
	exit_status: Option<ExitStatus>,
)
{
	let mut expected = HandleInputResultBuilder::new(input);
	if let Some(state) = state {
		expected = expected.state(state);
	}
	if let Some(exit_status) = exit_status {
		expected = expected.exit_status(exit_status);
	}
	assert_eq!(actual, expected.build());
}

#[macro_export]
macro_rules! assert_handle_input_result {
	($actual:expr, input = $input:expr) => {
		crate::process::testutil::_assert_handle_input_result($actual, $input, None, None);
	};
	($actual:expr, input = $input:expr, state = $state:expr) => {
		crate::process::testutil::_assert_handle_input_result($actual, $input, Some($state), None);
	};
	($actual:expr, input = $input:expr, exit_status = $exit_status:expr) => {
		crate::process::testutil::_assert_handle_input_result($actual, $input, None, Some($exit_status));
	};
	($actual:expr, input = $input:expr, state = $state:expr, exit_status = $exit_status:expr) => {
		crate::process::testutil::_assert_handle_input_result($actual, $input, Some($state), Some($exit_status));
	};
}
