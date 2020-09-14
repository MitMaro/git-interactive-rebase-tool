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

pub fn _process_module_handle_input_test<F>(lines: Vec<&str>, input: PancursesInput, callback: F)
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
	curses.push_input(input);
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
