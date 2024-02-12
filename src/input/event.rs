use crossterm::event::{KeyCode, MouseEvent};

use crate::input::{KeyEvent, StandardEvent};

/// An event, either from an input device, system change or action event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum Event {
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
}

impl From<crossterm::event::Event> for Event {
	fn from(event: crossterm::event::Event) -> Self {
		match event {
			crossterm::event::Event::Key(evt) => Self::Key(KeyEvent::from(evt)),
			crossterm::event::Event::Mouse(evt) => Self::Mouse(evt),
			crossterm::event::Event::Resize(width, height) => Self::Resize(width, height),
			// ignore these events for now, as we don't need them
			crossterm::event::Event::FocusGained
			| crossterm::event::Event::FocusLost
			| crossterm::event::Event::Paste(_) => Self::None,
		}
	}
}

impl From<KeyEvent> for Event {
	fn from(key_event: KeyEvent) -> Self {
		Self::Key(key_event)
	}
}

impl From<MouseEvent> for Event {
	fn from(mouse_event: MouseEvent) -> Self {
		Self::Mouse(mouse_event)
	}
}

impl From<StandardEvent> for Event {
	fn from(event: StandardEvent) -> Self {
		Self::Standard(event)
	}
}

impl From<KeyCode> for Event {
	fn from(code: KeyCode) -> Self {
		Self::Key(KeyEvent::from(code))
	}
}

impl From<char> for Event {
	fn from(c: char) -> Self {
		Self::Key(KeyEvent::from(KeyCode::Char(c)))
	}
}

#[cfg(test)]
mod tests {
	use crossterm::{
		event as ct_event,
		event::{KeyModifiers, MouseEventKind},
	};

	use super::*;

	#[test]
	fn from_crossterm_key_event() {
		let event = Event::from(ct_event::Event::Key(ct_event::KeyEvent::new(
			KeyCode::Null,
			KeyModifiers::empty(),
		)));
		assert_eq!(event, Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::empty())));
	}

	#[test]
	fn from_crossterm_key_event_char_with_modifier() {
		let event = Event::from(ct_event::Event::Key(ct_event::KeyEvent::new(
			KeyCode::Char('?'),
			KeyModifiers::ALT,
		)));

		assert_eq!(event, Event::Key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::ALT)));
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
	fn from_crossterm_focused_gained_event() {
		let event = Event::from(crossterm::event::Event::FocusGained);
		assert_eq!(event, Event::None);
	}

	#[test]
	fn from_crossterm_focused_lost_event() {
		let event = Event::from(crossterm::event::Event::FocusLost);
		assert_eq!(event, Event::None);
	}

	#[test]
	fn from_crossterm_paste_event() {
		let event = Event::from(crossterm::event::Event::Paste(String::from("test")));
		assert_eq!(event, Event::None);
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
	fn from_mouse_event() {
		let event = Event::from(MouseEvent {
			kind: MouseEventKind::Moved,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::empty(),
		});
		assert_eq!(
			event,
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::Moved,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			})
		);
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
