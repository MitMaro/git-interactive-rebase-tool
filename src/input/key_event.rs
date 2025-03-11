use crate::input::{KeyCode, KeyModifiers};

/// Represents a key event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub(crate) struct KeyEvent {
	/// The key itself.
	pub(crate) code: KeyCode,
	/// Additional key modifiers.
	pub(crate) modifiers: KeyModifiers,
}

impl KeyEvent {
	/// Creates a new `KeyEvent` with `code` and `modifiers`.
	#[must_use]
	pub(crate) fn new(mut code: KeyCode, mut modifiers: KeyModifiers) -> Self {
		// normalize keys with the SHIFT modifier
		if let KeyCode::Char(c) = code {
			if modifiers.contains(KeyModifiers::SHIFT) {
				code = KeyCode::Char(c.to_ascii_uppercase());
				modifiers.remove(KeyModifiers::SHIFT);
			}
		}
		Self { code, modifiers }
	}
}

impl From<crossterm::event::KeyEvent> for KeyEvent {
	fn from(key_event: crossterm::event::KeyEvent) -> Self {
		Self::new(key_event.code, key_event.modifiers)
	}
}

impl From<KeyCode> for KeyEvent {
	fn from(code: KeyCode) -> Self {
		Self::new(code, KeyModifiers::empty())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_non_character() {
		assert_eq!(KeyEvent::new(KeyCode::Backspace, KeyModifiers::ALT), KeyEvent {
			code: KeyCode::Backspace,
			modifiers: KeyModifiers::ALT
		});
	}

	#[test]
	fn new_lowercase_character_without_shift() {
		assert_eq!(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE), KeyEvent {
			code: KeyCode::Char('a'),
			modifiers: KeyModifiers::NONE
		});
	}

	#[test]
	fn new_uppercase_character_without_shift() {
		assert_eq!(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE), KeyEvent {
			code: KeyCode::Char('A'),
			modifiers: KeyModifiers::NONE
		});
	}

	#[test]
	fn new_lowercase_character_with_shift() {
		assert_eq!(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::SHIFT), KeyEvent {
			code: KeyCode::Char('A'),
			modifiers: KeyModifiers::NONE
		});
	}

	#[test]
	fn new_uppercase_character_with_shift() {
		assert_eq!(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT), KeyEvent {
			code: KeyCode::Char('A'),
			modifiers: KeyModifiers::NONE
		});
	}

	#[test]
	fn from_crossterm_key_event() {
		assert_eq!(
			KeyEvent::from(crossterm::event::KeyEvent::new(KeyCode::Char('a'), KeyModifiers::ALT)),
			KeyEvent {
				code: KeyCode::Char('a'),
				modifiers: KeyModifiers::ALT
			}
		);
	}

	#[test]
	fn from_keycode() {
		assert_eq!(KeyEvent::from(KeyCode::Char('a')), KeyEvent {
			code: KeyCode::Char('a'),
			modifiers: KeyModifiers::NONE
		});
	}
}
