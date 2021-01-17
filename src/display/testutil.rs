use crate::config::Config;
use crate::display::curses::Curses;
use crate::input::input_handler::InputHandler;
use std::env::set_var;
use std::path::Path;

pub struct TestContext<'t> {
	pub config: &'t Config,
	pub curses: Curses,
	pub input_handler: InputHandler<'t>,
}

pub fn display_module_test<F>(callback: F)
where F: FnOnce(TestContext<'_>) {
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
	let curses = Curses::new();
	curses.erase();
	let input_handler = InputHandler::new(&config.key_bindings);
	callback(TestContext {
		config: &config,
		curses,
		input_handler,
	});
}
