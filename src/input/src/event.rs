use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use super::MetaEvent;

/// An event, either from an input device, system change or action event.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::exhaustive_enums)]
pub enum Event {
	/// A keyboard event.
	Key(KeyEvent),
	/// An action event.
	Meta(MetaEvent),
	/// A mouse event.
	Mouse(MouseEvent),
	/// An empty event.
	None,
	/// A terminal resize event.
	Resize(u16, u16),
}

impl From<crossterm::event::Event> for Event {
	#[inline]
	fn from(event: crossterm::event::Event) -> Self {
		match event {
			crossterm::event::Event::Key(evt) => Self::Key(evt),
			crossterm::event::Event::Mouse(evt) => Self::Mouse(evt),
			crossterm::event::Event::Resize(width, height) => Self::Resize(width, height),
		}
	}
}

impl From<MetaEvent> for Event {
	#[inline]
	fn from(event: MetaEvent) -> Self {
		Self::Meta(event)
	}
}

impl From<KeyCode> for Event {
	#[inline]
	fn from(code: KeyCode) -> Self {
		Self::Key(KeyEvent {
			code,
			modifiers: KeyModifiers::empty(),
		})
	}
}

impl From<char> for Event {
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

	#[test]
	fn from_crossterm_key_event() {
		let key_event = KeyEvent::new(KeyCode::Null, KeyModifiers::empty());
		let event = Event::from(crossterm::event::Event::Key(key_event));
		assert_eq!(event, Event::Key(key_event));
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
		let event = Event::from(MetaEvent::Kill);
		assert_eq!(event, Event::Meta(MetaEvent::Kill));
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
