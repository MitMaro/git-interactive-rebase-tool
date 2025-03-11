use crossterm::event::{KeyCode, KeyModifiers};

use crate::input::{Event, KeyEvent};

/// Map a keybinding to a list of events.
#[must_use]
#[expect(clippy::string_slice, reason = "Slice usage is guarded.")]
pub(crate) fn map_keybindings(bindings: &[String]) -> Vec<Event> {
	bindings
		.iter()
		.map(|b| {
			let mut key = String::from(b);
			let mut modifiers = KeyModifiers::empty();
			if key.contains("Control") {
				key = key.replace("Control", "");
				modifiers.insert(KeyModifiers::CONTROL);
			}
			if key.contains("Alt") {
				key = key.replace("Alt", "");
				modifiers.insert(KeyModifiers::ALT);
			}
			if key.contains("Shift") {
				key = key.replace("Shift", "");
				modifiers.insert(KeyModifiers::SHIFT);
			}

			let code = match key.as_str() {
				"Backspace" => KeyCode::Backspace,
				"BackTab" => KeyCode::BackTab,
				"Delete" => KeyCode::Delete,
				"Down" => KeyCode::Down,
				"End" => KeyCode::End,
				"Enter" => KeyCode::Enter,
				"Esc" => KeyCode::Esc,
				"Home" => KeyCode::Home,
				"Insert" => KeyCode::Insert,
				"Left" => KeyCode::Left,
				"PageDown" => KeyCode::PageDown,
				"PageUp" => KeyCode::PageUp,
				"Right" => KeyCode::Right,
				"Tab" => KeyCode::Tab,
				"Up" => KeyCode::Up,
				// assume that this is an F key
				k if k.len() > 1 && k.to_ascii_lowercase().starts_with('f') => {
					let key_number = k[1..].parse::<u8>().unwrap_or(1);
					KeyCode::F(key_number)
				},
				k => KeyCode::Char(k.chars().next().expect("Expected only one character from Char KeyCode")),
			};
			Event::Key(KeyEvent::new(code, modifiers))
		})
		.collect()
}
