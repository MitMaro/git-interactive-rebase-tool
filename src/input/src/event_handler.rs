use super::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::{key_bindings::KeyBindings, InputOptions, StandardEvent};

/// A handler for reading and processing events.
#[allow(missing_debug_implementations)]
pub struct EventHandler<CustomKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent> {
	key_bindings: KeyBindings<CustomKeybinding, CustomEvent>,
}

impl<CustomKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent>
	EventHandler<CustomKeybinding, CustomEvent>
{
	/// Create a new instance of the `EventHandler`.
	#[inline]
	#[must_use]
	pub fn new(key_bindings: KeyBindings<CustomKeybinding, CustomEvent>) -> Self {
		Self { key_bindings }
	}

	/// Read and handle an event.
	#[inline]
	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn read_event<F>(
		&self,
		event: Event<CustomEvent>,
		input_options: &InputOptions,
		callback: F,
	) -> Event<CustomEvent>
	where
		F: FnOnce(Event<CustomEvent>, &KeyBindings<CustomKeybinding, CustomEvent>) -> Event<CustomEvent>,
	{
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

		if input_options.contains(InputOptions::UNDO_REDO) {
			if let Some(evt) = Self::handle_undo_redo(&self.key_bindings, event) {
				return evt;
			}
		}

		callback(event, &self.key_bindings)
	}

	#[allow(clippy::wildcard_enum_match_arm)]
	fn handle_standard_inputs(event: Event<CustomEvent>) -> Option<Event<CustomEvent>> {
		match event {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => Some(Event::from(StandardEvent::Kill)),
			Event::Key(KeyEvent {
				code: KeyCode::Char('d'),
				modifiers: KeyModifiers::CONTROL,
			}) => Some(Event::from(StandardEvent::Exit)),
			_ => None,
		}
	}

	#[allow(clippy::wildcard_enum_match_arm)]
	fn handle_movement_inputs(event: Event<CustomEvent>) -> Option<Event<CustomEvent>> {
		match event {
			Event::Key(KeyEvent {
				code: KeyCode::Up,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(StandardEvent::ScrollUp)),
			Event::Key(KeyEvent {
				code: KeyCode::Down,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(StandardEvent::ScrollDown)),
			Event::Key(KeyEvent {
				code: KeyCode::Left,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(StandardEvent::ScrollLeft)),
			Event::Key(KeyEvent {
				code: KeyCode::Right,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(StandardEvent::ScrollRight)),
			Event::Key(KeyEvent {
				code: KeyCode::PageUp,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(StandardEvent::ScrollJumpUp)),
			Event::Key(KeyEvent {
				code: KeyCode::PageDown,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(StandardEvent::ScrollJumpDown)),
			Event::Key(KeyEvent {
				code: KeyCode::Home,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(StandardEvent::ScrollTop)),
			Event::Key(KeyEvent {
				code: KeyCode::End,
				modifiers: KeyModifiers::NONE,
			}) => Some(Event::from(StandardEvent::ScrollBottom)),
			_ => None,
		}
	}

	fn handle_undo_redo(
		key_bindings: &KeyBindings<CustomKeybinding, CustomEvent>,
		event: Event<CustomEvent>,
	) -> Option<Event<CustomEvent>> {
		if key_bindings.undo.contains(&event) {
			Some(Event::from(StandardEvent::Undo))
		}
		else if key_bindings.redo.contains(&event) {
			Some(Event::from(StandardEvent::Redo))
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
	use crate::testutil::local::{create_test_keybindings, Event, EventHandler};

	#[rstest]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('c'),
		modifiers: KeyModifiers::CONTROL,
	}), true)]
	#[case::resize(Event::Resize(100, 100), false)]
	#[case::movement(Event::from(KeyCode::Up), false)]
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
	}), Event::from(StandardEvent::Kill))]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('d'),
		modifiers: KeyModifiers::CONTROL,
	}), Event::from(StandardEvent::Exit))]
	#[case::other(Event::from('a'), Event::from(KeyCode::Null))]
	fn standard_inputs(#[case] event: Event, #[case] expected: Event) {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(event, &InputOptions::empty(), |_, _| Event::from(KeyCode::Null));
		assert_eq!(result, expected);
	}

	#[rstest]
	#[case::standard(Event::from(KeyCode::Up), Event::from(StandardEvent::ScrollUp))]
	#[case::standard(Event::from(KeyCode::Down), Event::from(StandardEvent::ScrollDown))]
	#[case::standard(Event::from(KeyCode::Left), Event::from(StandardEvent::ScrollLeft))]
	#[case::standard(Event::from(KeyCode::Right), Event::from(StandardEvent::ScrollRight))]
	#[case::standard(Event::from(KeyCode::PageUp), Event::from(StandardEvent::ScrollJumpUp))]
	#[case::standard(Event::from(KeyCode::PageDown), Event::from(StandardEvent::ScrollJumpDown))]
	#[case::standard(Event::from(KeyCode::Home), Event::from(StandardEvent::ScrollTop))]
	#[case::standard(Event::from(KeyCode::End), Event::from(StandardEvent::ScrollBottom))]
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
	}), Event::from(StandardEvent::Undo))]
	#[case::standard(Event::Key(KeyEvent {
		code: KeyCode::Char('y'),
		modifiers: KeyModifiers::CONTROL,
	}), Event::from(StandardEvent::Redo))]
	#[case::other(Event::from('a'), Event::from(KeyCode::Null))]
	fn undo_redo_inputs(#[case] event: Event, #[case] expected: Event) {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(event, &InputOptions::UNDO_REDO, |_, _| Event::from(KeyCode::Null));
		assert_eq!(result, expected);
	}
}
