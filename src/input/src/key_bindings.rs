use super::{Event, KeyCode, KeyEvent, KeyModifiers};

/// Represents a mapping between an input event and an action.
#[derive(Debug)]
#[non_exhaustive]
pub struct KeyBindings<CustomKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent> {
	/// Key bindings for redoing a change.
	pub redo: Vec<Event<CustomEvent>>,
	/// Key bindings for undoing a change.
	pub undo: Vec<Event<CustomEvent>>,
	/// Custom keybindings
	pub custom: CustomKeybinding,
}

/// Map a keybinding to a list of events.
#[must_use]
#[inline]
#[allow(clippy::string_slice, clippy::indexing_slicing)]
pub fn map_keybindings<CustomEvent: crate::CustomEvent>(bindings: &[String]) -> Vec<Event<CustomEvent>> {
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
				k if k.len() > 1 => {
					let key_number = k[1..].parse::<u8>().unwrap_or(1);
					KeyCode::F(key_number)
				},
				k => KeyCode::Char(k.chars().next().expect("Expected only one character from Char KeyCode")),
			};
			Event::Key(KeyEvent::new(code, modifiers))
		})
		.collect()
}

impl<CustomKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent>
	KeyBindings<CustomKeybinding, CustomEvent>
{
	/// Create a new instance from the configuration keybindings.
	#[inline]
	#[must_use]
	pub fn new(key_bindings: &config::KeyBindings) -> Self {
		Self {
			redo: map_keybindings(&key_bindings.redo),
			undo: map_keybindings(&key_bindings.undo),
			custom: CustomKeybinding::new(key_bindings),
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::testutil::local::{TestEvent, TestKeybinding};

	#[test]
	fn new() {
		let _key_bindings = KeyBindings::<TestKeybinding, TestEvent>::new(&config::KeyBindings::new());
	}

	#[test]
	fn map_keybindings_with_modifiers() {
		assert_eq!(map_keybindings::<TestEvent>(&[String::from("ControlAltShifta")]), vec![
			Event::Key(KeyEvent {
				code: KeyCode::Char('a'),
				modifiers: KeyModifiers::all()
			})
		]);
	}

	#[rstest]
	#[case::backspace("Backspace", KeyCode::Backspace)]
	#[case::back_tab("BackTab", KeyCode::BackTab)]
	#[case::delete("Delete", KeyCode::Delete)]
	#[case::down("Down", KeyCode::Down)]
	#[case::end("End", KeyCode::End)]
	#[case::enter("Enter", KeyCode::Enter)]
	#[case::esc("Esc", KeyCode::Esc)]
	#[case::home("Home", KeyCode::Home)]
	#[case::insert("Insert", KeyCode::Insert)]
	#[case::left("Left", KeyCode::Left)]
	#[case::page_down("PageDown", KeyCode::PageDown)]
	#[case::page_up("PageUp", KeyCode::PageUp)]
	#[case::right("Right", KeyCode::Right)]
	#[case::tab("Tab", KeyCode::Tab)]
	#[case::up("Up", KeyCode::Up)]
	#[case::function_in_range("F10", KeyCode::F(10))]
	#[case::function_out_of_range("F10000", KeyCode::F(1))]
	#[case::char("a", KeyCode::Char('a'))]
	fn map_keybindings_key_code(#[case] binding: &str, #[case] key_code: KeyCode) {
		assert_eq!(map_keybindings::<TestEvent>(&[String::from(binding)]), vec![
			Event::from(key_code)
		]);
	}
}
