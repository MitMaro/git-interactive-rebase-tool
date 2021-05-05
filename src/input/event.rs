pub use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

use super::MetaEvent;

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Event {
	Key(KeyEvent),
	Meta(MetaEvent),
	Mouse(MouseEvent),
	None,
	Resize(u16, u16),
}

impl From<crossterm::event::Event> for Event {
	fn from(event: crossterm::event::Event) -> Self {
		match event {
			crossterm::event::Event::Key(event) => Self::Key(event),
			crossterm::event::Event::Mouse(event) => Self::Mouse(event),
			crossterm::event::Event::Resize(width, height) => Self::Resize(width, height),
		}
	}
}

impl From<MetaEvent> for Event {
	fn from(event: MetaEvent) -> Self {
		Self::Meta(event)
	}
}

impl From<KeyCode> for Event {
	fn from(code: KeyCode) -> Self {
		Self::Key(KeyEvent {
			code,
			modifiers: KeyModifiers::empty(),
		})
	}
}

impl From<char> for Event {
	fn from(c: char) -> Self {
		Self::Key(KeyEvent {
			code: KeyCode::Char(c),
			modifiers: KeyModifiers::empty(),
		})
	}
}

#[cfg(test)]
mod tests {
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
