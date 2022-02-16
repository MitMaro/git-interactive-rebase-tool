use super::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::{key_bindings::KeyBindings, InputOptions, MetaEvent};

/// A handler for reading and processing events.
#[allow(missing_debug_implementations)]
pub struct EventHandler {
	key_bindings: KeyBindings,
}

impl EventHandler {
	/// Create a new instance of the `EventHandler`.
	#[inline]
	#[must_use]
	pub const fn new(key_bindings: KeyBindings) -> Self {
		Self { key_bindings }
	}

	/// Read and handle an event.
	#[inline]
	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn read_event<F>(&self, event: Event, input_options: &InputOptions, callback: F) -> Event
	where F: FnOnce(Event, &KeyBindings) -> Event {
		if event == Event::None {
			return event;
		}

		if let Some(e) = Self::handle_standard_inputs(event) {
			return e;
		}

		if input_options.contains(InputOptions::RESIZE) {
			if let Event::Resize(..) = event {
				return event;
			}
		}

		if input_options.contains(InputOptions::MOVEMENT) {
			if let Some(evt) = Self::handle_movement_inputs(event) {
				return evt;
			}
		}

		if input_options.contains(InputOptions::HELP) && self.key_bindings.help.contains(&event) {
			return Event::from(MetaEvent::Help);
		}

		if input_options.contains(InputOptions::UNDO_REDO) {
			if let Some(evt) = Self::handle_undo_redo(&self.key_bindings, event) {
				return evt;
			}
		}

		callback(event, &self.key_bindings)
	}

	#[allow(clippy::wildcard_enum_match_arm)]
	fn handle_standard_inputs(event: Event) -> Option<Event> {
		match event {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => Some(Event::from(MetaEvent::Kill)),
			Event::Key(KeyEvent {
				code: KeyCode::Char('d'),
				modifiers: KeyModifiers::CONTROL,
			}) => Some(Event::from(MetaEvent::Exit)),
			_ => None,
		}
	}

	#[allow(clippy::wildcard_enum_match_arm)]
	fn handle_movement_inputs(event: Event) -> Option<Event> {
		match event {
			Event::Key(KeyEvent {
				code: KeyCode::Up,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollUp)),
			Event::Key(KeyEvent {
				code: KeyCode::Down,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollDown)),
			Event::Key(KeyEvent {
				code: KeyCode::Left,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollLeft)),
			Event::Key(KeyEvent {
				code: KeyCode::Right,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollRight)),
			Event::Key(KeyEvent {
				code: KeyCode::PageUp,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollJumpUp)),
			Event::Key(KeyEvent {
				code: KeyCode::PageDown,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollJumpDown)),
			Event::Key(KeyEvent {
				code: KeyCode::Home,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollTop)),
			Event::Key(KeyEvent {
				code: KeyCode::End,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(MetaEvent::ScrollBottom)),
			_ => None,
		}
	}

	fn handle_undo_redo(key_bindings: &KeyBindings, event: Event) -> Option<Event> {
		if key_bindings.undo.contains(&event) {
			Some(Event::from(MetaEvent::Undo))
		}
		else if key_bindings.redo.contains(&event) {
			Some(Event::from(MetaEvent::Redo))
		}
		else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::testutil::create_test_keybindings;
	#[rstest]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('c'),
		modifiers: KeyModifiers::CONTROL,
	}), true)]
	#[case::resize(Event::Resize(100, 100), false)]
	#[case::movement(Event::from(KeyCode::Up), false)]
	#[case::help(Event::from('?'), false)]
	#[case::undo_redo(Event::Key(KeyEvent {
		code: KeyCode::Char('z'),
		modifiers: KeyModifiers::CONTROL,
	}), false)]
	#[case::other(Event::from('a'), false)]
	fn read_event_options_disabled(#[case] event: Event, #[case] handled: bool) {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(event, &InputOptions::empty(), |_, _| Event::from(KeyCode::Null));

		if handled {
			assert_ne!(result, Event::from(KeyCode::Null));
		}
		else {
			assert_eq!(result, Event::from(KeyCode::Null));
		}
	}

	#[rstest]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('c'),
		modifiers: KeyModifiers::CONTROL,
	}), true)]
	#[case::resize(Event::Resize(100, 100), true)]
	#[case::movement(Event::from(KeyCode::Up), true)]
	#[case::help(Event::from('?'), true)]
	#[case::undo_redo(Event::Key(KeyEvent {
		code: KeyCode::Char('z'),
		modifiers: KeyModifiers::CONTROL,
	}), true)]
	#[case::other(Event::from('a'), false)]
	fn read_event_enabled(#[case] event: Event, #[case] handled: bool) {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(event, &InputOptions::all(), |_, _| Event::from(KeyCode::Null));

		if handled {
			assert_ne!(result, Event::from(KeyCode::Null));
		}
		else {
			assert_eq!(result, Event::from(KeyCode::Null));
		}
	}

	#[test]
	fn none_event() {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(Event::None, &InputOptions::empty(), |_, _| Event::from(KeyCode::Null));
		assert_eq!(result, Event::None);
	}

	#[rstest]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('c'),
		modifiers: KeyModifiers::CONTROL,
	}), Event::from(MetaEvent::Kill))]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('d'),
		modifiers: KeyModifiers::CONTROL,
	}), Event::from(MetaEvent::Exit))]
	#[case::other(Event::from('a'), Event::from(KeyCode::Null))]
	fn standard_inputs(#[case] event: Event, #[case] expected: Event) {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(event, &InputOptions::empty(), |_, _| Event::from(KeyCode::Null));
		assert_eq!(result, expected);
	}

	#[rstest]
	#[case::standard(Event::from(KeyCode::Up), Event::from(MetaEvent::ScrollUp))]
	#[case::standard(Event::from(KeyCode::Down), Event::from(MetaEvent::ScrollDown))]
	#[case::standard(Event::from(KeyCode::Left), Event::from(MetaEvent::ScrollLeft))]
	#[case::standard(Event::from(KeyCode::Right), Event::from(MetaEvent::ScrollRight))]
	#[case::standard(Event::from(KeyCode::PageUp), Event::from(MetaEvent::ScrollJumpUp))]
	#[case::standard(Event::from(KeyCode::PageDown), Event::from(MetaEvent::ScrollJumpDown))]
	#[case::standard(Event::from(KeyCode::Home), Event::from(MetaEvent::ScrollTop))]
	#[case::standard(Event::from(KeyCode::End), Event::from(MetaEvent::ScrollBottom))]
	#[case::other(Event::from('a'), Event::from(KeyCode::Null))]
	fn movement_inputs(#[case] event: Event, #[case] expected: Event) {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(event, &InputOptions::MOVEMENT, |_, _| Event::from(KeyCode::Null));
		assert_eq!(result, expected);
	}

	#[rstest]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('z'),
		modifiers: KeyModifiers::CONTROL,
	}), Event::from(MetaEvent::Undo))]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('y'),
		modifiers: KeyModifiers::CONTROL,
	}), Event::from(MetaEvent::Redo))]
	#[case::other(Event::from('a'), Event::from(KeyCode::Null))]
	fn undo_redo_inputs(#[case] event: Event, #[case] expected: Event) {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(event, &InputOptions::UNDO_REDO, |_, _| Event::from(KeyCode::Null));
		assert_eq!(result, expected);
	}
}
