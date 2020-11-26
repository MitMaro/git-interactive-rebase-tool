use crate::config::Config;
use crate::display::curses::Curses;
use std::env::set_var;
use std::path::Path;

pub struct TestContext {
	pub config: Config,
	pub curses: Curses,
}

pub fn display_module_test<F>(callback: F)
where F: FnOnce(TestContext) {
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
	callback(TestContext { config, curses });
}
