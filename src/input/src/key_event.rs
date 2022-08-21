use crossterm::event::{KeyCode, KeyModifiers};

/// Represents a key event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::exhaustive_structs)]
pub struct KeyEvent {
	/// The key itself.
	pub code: KeyCode,
	/// Additional key modifiers.
	pub modifiers: KeyModifiers,
}

impl KeyEvent {
	/// Creates a new `KeyEvent` with `code` and `modifiers`.
	#[must_use]
	#[inline]
	pub fn new(mut code: KeyCode, mut modifiers: KeyModifiers) -> Self {
		// ensure that uppercase characters always have SHIFT
		if let KeyCode::Char(c) = code {
			if c.is_ascii_uppercase() {
				modifiers.insert(KeyModifiers::SHIFT);
			}
			else if modifiers.contains(KeyModifiers::SHIFT) {
				code = KeyCode::Char(c.to_ascii_uppercase());
			}
		}
		Self { code, modifiers }
	}
}

impl From<crossterm::event::KeyEvent> for KeyEvent {
	#[inline]
	fn from(key_event: crossterm::event::KeyEvent) -> Self {
		Self::new(key_event.code, key_event.modifiers)
	}
}

impl From<KeyCode> for KeyEvent {
	#[inline]
	fn from(code: KeyCode) -> Self {
		Self::new(code, KeyModifiers::empty())
	}
}
