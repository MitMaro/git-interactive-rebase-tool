use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use super::StandardEvent;

/// An event, either from an input device, system change or action event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::exhaustive_enums)]
pub enum Event<CustomEvent: crate::CustomEvent> {
	/// A keyboard event.
	Key(KeyEvent),
	/// An action event.
	Standard(StandardEvent),
	/// A mouse event.
	Mouse(MouseEvent),
	/// An empty event.
	None,
	/// A terminal resize event.
	Resize(u16, u16),
	/// Custom application defined events
	MetaEvent(CustomEvent),
}

impl<CustomEvent: crate::CustomEvent> From<crossterm::event::Event> for Event<CustomEvent> {
	#[inline]
	fn from(event: crossterm::event::Event) -> Self {
		match event {
			crossterm::event::Event::Key(mut evt) => {
				if let KeyCode::Char(_) = evt.code {
					evt.modifiers.remove(KeyModifiers::SHIFT);
				}
				Self::Key(evt)
			},
			crossterm::event::Event::Mouse(evt) => Self::Mouse(evt),
			crossterm::event::Event::Resize(width, height) => Self::Resize(width, height),
		}
	}
}

impl<CustomEvent: crate::CustomEvent> From<StandardEvent> for Event<CustomEvent> {
	#[inline]
	fn from(event: StandardEvent) -> Self {
		Self::Standard(event)
	}
}

impl<CustomEvent: crate::CustomEvent> From<KeyCode> for Event<CustomEvent> {
	#[inline]
	fn from(code: KeyCode) -> Self {
		Self::Key(KeyEvent {
			code,
			modifiers: KeyModifiers::empty(),
		})
	}
}

impl<CustomEvent: crate::CustomEvent> From<char> for Event<CustomEvent> {
	#[inline]
	fn from(c: char) -> Self {
		Self::Key(KeyEvent {
			code: KeyCode::Char(c),
			modifiers: KeyModifiers::empty(),
		})
	}
}

#[cfg(test)]
mod tests {
	use crossterm::event::MouseEventKind;

	use super::*;
	use crate::testutil::local::Event;

	#[test]
	fn from_crossterm_key_event() {
		let key_event = KeyEvent::new(KeyCode::Null, KeyModifiers::empty());
		let event = Event::from(crossterm::event::Event::Key(key_event));
		assert_eq!(event, Event::Key(key_event));
	}

	#[test]
	fn from_crossterm_key_event_char_with_shift_modifier() {
		let key_event = KeyEvent::new(KeyCode::Char('?'), KeyModifiers::SHIFT);
		let event = Event::from(crossterm::event::Event::Key(key_event));
		assert_eq!(event, Event::Key(KeyEvent::from(KeyCode::Char('?'))));
	}

	#[test]
	fn from_crossterm_mouse_event() {
		let mouse_event = MouseEvent {
			kind: MouseEventKind::Moved,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::empty(),
		};
		let event = Event::from(crossterm::event::Event::Mouse(mouse_event));
		assert_eq!(event, Event::Mouse(mouse_event));
	}

	#[test]
	fn from_crossterm_resize_event() {
		let event = Event::from(crossterm::event::Event::Resize(100, 100));
		assert_eq!(event, Event::Resize(100, 100));
	}

	#[test]
	fn from_meta_event() {
		let event = Event::from(StandardEvent::Kill);
		assert_eq!(event, Event::Standard(StandardEvent::Kill));
	}

	#[test]
	fn from_key_code() {
		let event = Event::from(KeyCode::Null);
		assert_eq!(event, Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::empty())));
	}

	#[test]
	fn from_char() {
		let event = Event::from('a');
		assert_eq!(
			event,
			Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty()))
		);
	}
}
