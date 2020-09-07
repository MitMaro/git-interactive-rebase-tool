use crate::config::Config;
use crate::display::color_mode::ColorMode;
use crate::display::curses::Curses;
use crate::display::curses::Input;
use crate::display::Display;
use crate::testutil::compare_trace;
use std::env::set_var;
use std::path::Path;

pub fn _display_module_test<F>(expected_trace: Vec<(String, Vec<String>)>, input: Option<Input>, callback: F)
where F: FnOnce(&mut Display<'_>) {
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
	if let Some(input) = input {
		curses.push_input(input);
	}
	curses.resize_term(66, 77);
	curses.mv(13, 17);
	curses.set_color_mode(ColorMode::TrueColor);
	let mut display = Display::new(&mut curses, &config.theme);
	callback(&mut display);
	let trace = curses.get_function_trace();
	compare_trace(&trace, &expected_trace);
}

// a lot of the testing here is just ensuring that the correct curses function are called
#[macro_export]
macro_rules! display_module_test {
		($name:ident, $expected_trace:expr, $fun:expr) => {
			concat_idents::concat_idents!(test_name = display_module_, $name {
				#[test]
				#[serial_test::serial]
				fn test_name() {
					crate::display::testutil::_display_module_test($expected_trace, None, $fun);
				}
			});
		};
		($name:ident, $input:expr, $expected_trace:expr, $fun:expr) => {
			concat_idents::concat_idents!(test_name = display_module_, $name {
				#[test]
				#[serial_test::serial]
				fn test_name() {
					crate::display::testutil::_display_module_test($expected_trace, Some($input), $fun);
				}
			});
		};
	}
