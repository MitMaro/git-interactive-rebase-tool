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
use crate::view::View;
use std::env::set_var;
use std::path::Path;

#[macro_export]
macro_rules! build_trace {
	($name:expr) => {{
		let args: Vec<String> = vec![];
		(String::from($name), args)
	}};
	($name:expr, $($arg:expr),*) => {{
		let mut args = vec![];
		$( args.push(format!("{}", $arg)); )*
		(String::from($name), args)
	}};
}

pub fn panic_trace_error(e: &(String, Vec<String>), trace: &Vec<(String, Vec<String>)>) {
	panic!(vec![
		"\n==========",
		"Missing function call in trace",
		format!("Call: {}({})", e.0, e.1.join(", ")).as_str(),
		"Trace:",
		trace
			.clone()
			.iter()
			.map(|(f, args)| {
				format!(
					"\t{}({})",
					f,
					args.iter()
						.map(|v| v.replace("\n", "\\n"))
						.collect::<Vec<String>>()
						.join(", ")
				)
			})
			.collect::<Vec<String>>()
			.join("\n")
			.as_str(),
		"==========\n"
	]
	.join("\n"));
}

pub fn compare_trace(actual: &Vec<(String, Vec<String>)>, expected: &Vec<(String, Vec<String>)>) {
	let mut e_iter = expected.iter();
	let mut a_iter = actual.iter();
	'trace: loop {
		if let Some(e) = e_iter.next() {
			loop {
				if let Some(a) = a_iter.next() {
					// function name and argument length must match
					if !a.0.eq(&e.0) || a.1.len() != e.1.len() {
						continue;
					}
					if a.1.iter().zip(&e.1).all(|(a, e)| e.eq("*") || a.eq(e)) {
						continue 'trace;
					}
				}
				else {
					panic_trace_error(&e, &actual);
				}
			}
		}
		else {
			break;
		}
	}

	if let Some(e) = e_iter.next() {
		panic_trace_error(&e, &actual);
	}
}

pub fn _process_module_build_view_data_test<F>(
	lines: Vec<&str>,
	view_size: (i32, i32),
	position: (i32, i32),
	expected_trace: Vec<(String, Vec<String>)>,
	get_module: F,
) where
	F: FnOnce(&Config, &Display<'_>) -> Box<dyn ProcessModule>,
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
	view.render(view_data);
	let trace = curses.get_function_trace();
	compare_trace(&trace, &expected_trace);
}

#[macro_export]
macro_rules! process_module_build_view_data_test {
	($name:ident, $lines:expr, $view_size:expr, $position:expr, $expected_trace:expr, $get_module:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::testutil::_process_module_build_view_data_test(
				$lines,
				$view_size,
				$position,
				$expected_trace,
				$get_module,
			);
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
			crate::testutil::_process_module_handle_input_test($lines, $input, $fun);
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
	($actual:expr,input = $input:expr) => {
		crate::testutil::_assert_handle_input_result($actual, $input, None, None);
	};
	($actual:expr,input = $input:expr,state = $state:expr) => {
		crate::testutil::_assert_handle_input_result($actual, $input, Some($state), None);
	};
	($actual:expr,input = $input:expr,exit_status = $exit_status:expr) => {
		crate::testutil::_assert_handle_input_result($actual, $input, None, Some($exit_status));
	};
	($actual:expr,input = $input:expr,state = $state:expr,exit_status = $exit_status:expr) => {
		crate::testutil::_assert_handle_input_result($actual, $input, Some($state), Some($exit_status));
	};
}
