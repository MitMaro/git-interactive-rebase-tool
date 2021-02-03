use crate::config::Config;
use crate::display::{CrossTerm, Event, KeyCode, KeyEvent, KeyModifiers};
use crate::input::input_handler::InputHandler;
use crossterm::event::{MouseEvent, MouseEventKind};
use std::env::set_var;
use std::path::Path;

pub struct TestContext<'t> {
	pub config: &'t Config,
	pub crossterm: CrossTerm,
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
	let input_handler = InputHandler::new(&config.key_bindings);
	let crossterm = CrossTerm::new();
	callback(TestContext {
		config: &config,
		crossterm,
		input_handler,
	});
}

pub fn _create_key_event(c: KeyCode, modifiers: &[String]) -> Event {
	let mut key_modifiers = KeyModifiers::NONE;
	for modifier in modifiers {
		match modifier.as_str() {
			"Control" => key_modifiers.insert(KeyModifiers::CONTROL),
			"Shift" => key_modifiers.insert(KeyModifiers::SHIFT),
			"Alt" => key_modifiers.insert(KeyModifiers::ALT),
			_ => panic!("Invalid modifier: {}", modifier),
		}
	}
	Event::Key(KeyEvent::new(c, key_modifiers))
}

#[macro_export]
macro_rules! create_key_event {
	($char:expr) => {
		crate::display::testutil::_create_key_event(crate::display::KeyCode::Char($char), &[])
	};
	($char:expr, $($modifiers:expr),*) => {
		{
			let mut modifiers = vec![];
			$( modifiers.push(String::from($modifiers)); )*
			crate::display::testutil::_create_key_event(crate::display::KeyCode::Char($char), &modifiers)
		}
	};
	(code $code:expr) => {
		crate::display::testutil::_create_key_event($code, &[])
	};
	(code $code:expr, $($modifiers:expr),*) => {
		let mut modifiers = vec![];
		$( modifiers.push(String::from($modifiers)); )*
		crate::display::testutil::_create_key_event($code, &modifiers)
	};
}

pub fn _create_mouse_event(kind: MouseEventKind, column: u16, row: u16, modifiers: &[String]) -> Event {
	let mut key_modifiers = KeyModifiers::NONE;
	for modifier in modifiers {
		match modifier.as_str() {
			"Control" => key_modifiers.insert(KeyModifiers::CONTROL),
			"Shift" => key_modifiers.insert(KeyModifiers::SHIFT),
			"Alt" => key_modifiers.insert(KeyModifiers::ALT),
			_ => panic!("Invalid modifier: {}", modifier),
		}
	}
	Event::Mouse(MouseEvent {
		kind,
		column,
		row,
		modifiers: key_modifiers,
	})
}

#[macro_export]
macro_rules! create_mouse_event {
	($kind:expr) => {
		Event::Mouse(MouseEvent {
			kind: $kind,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::NONE,
		})
	};
}
