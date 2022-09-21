use super::{Event, KeyCode, KeyModifiers};
use crate::{key_bindings::KeyBindings, InputOptions, KeyEvent, StandardEvent};

/// A handler for reading and processing events.
#[derive(Debug)]
pub struct EventHandler<CustomKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent> {
	key_bindings: KeyBindings<CustomKeybinding, CustomEvent>,
}

impl<CustomKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent>
	EventHandler<CustomKeybinding, CustomEvent>
{
	/// Create a new instance of the `EventHandler`.
	#[inline]
	#[must_use]
	pub const fn new(key_bindings: KeyBindings<CustomKeybinding, CustomEvent>) -> Self {
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
			if let Some(evt) = Self::handle_movement_inputs(&self.key_bindings, event) {
				return evt;
			}
		}

		if input_options.contains(InputOptions::HELP) && self.key_bindings.help.contains(&event) {
			return Event::from(StandardEvent::Help);
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
			_ => None,
		}
	}

	#[allow(clippy::wildcard_enum_match_arm)]
	fn handle_movement_inputs(
		key_bindings: &KeyBindings<CustomKeybinding, CustomEvent>,
		event: Event<CustomEvent>,
	) -> Option<Event<CustomEvent>> {
		Some(match event {
			e if key_bindings.scroll_down.contains(&e) => Event::from(StandardEvent::ScrollDown),
			e if key_bindings.scroll_end.contains(&e) => Event::from(StandardEvent::ScrollBottom),
			e if key_bindings.scroll_home.contains(&e) => Event::from(StandardEvent::ScrollTop),
			e if key_bindings.scroll_left.contains(&e) => Event::from(StandardEvent::ScrollLeft),
			e if key_bindings.scroll_right.contains(&e) => Event::from(StandardEvent::ScrollRight),
			e if key_bindings.scroll_up.contains(&e) => Event::from(StandardEvent::ScrollUp),
			e if key_bindings.scroll_step_down.contains(&e) => Event::from(StandardEvent::ScrollJumpDown),
			e if key_bindings.scroll_step_up.contains(&e) => Event::from(StandardEvent::ScrollJumpUp),
			// these are required, since in some contexts (like editing), other keybindings will not work
			Event::Key(KeyEvent {
				code: KeyCode::Up,
				modifiers: KeyModifiers::NONE,
			}) => Event::from(StandardEvent::ScrollUp),
			Event::Key(KeyEvent {
				code: KeyCode::Down,
				modifiers: KeyModifiers::NONE,
			}) => Event::from(StandardEvent::ScrollDown),
			Event::Key(KeyEvent {
				code: KeyCode::Left,
				modifiers: KeyModifiers::NONE,
			}) => Event::from(StandardEvent::ScrollLeft),
			Event::Key(KeyEvent {
				code: KeyCode::Right,
				modifiers: KeyModifiers::NONE,
			}) => Event::from(StandardEvent::ScrollRight),
			Event::Key(KeyEvent {
				code: KeyCode::PageUp,
				modifiers: KeyModifiers::NONE,
			}) => Event::from(StandardEvent::ScrollJumpUp),
			Event::Key(KeyEvent {
				code: KeyCode::PageDown,
				modifiers: KeyModifiers::NONE,
			}) => Event::from(StandardEvent::ScrollJumpDown),
			Event::Key(KeyEvent {
				code: KeyCode::Home,
				modifiers: KeyModifiers::NONE,
			}) => Event::from(StandardEvent::ScrollTop),
			Event::Key(KeyEvent {
				code: KeyCode::End,
				modifiers: KeyModifiers::NONE,
			}) => Event::from(StandardEvent::ScrollBottom),
			_ => return None,
		})
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
	use crate::{
		map_keybindings,
		testutil::local::{create_test_keybindings, Event, EventHandler},
	};

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
	#[case::standard(Event::from(KeyCode::Up), Event::from(StandardEvent::ScrollUp))]
	#[case::standard(Event::from(KeyCode::Down), Event::from(StandardEvent::ScrollDown))]
	#[case::standard(Event::from(KeyCode::Left), Event::from(StandardEvent::ScrollLeft))]
	#[case::standard(Event::from(KeyCode::Right), Event::from(StandardEvent::ScrollRight))]
	#[case::standard(Event::from(KeyCode::PageUp), Event::from(StandardEvent::ScrollJumpUp))]
	#[case::standard(Event::from(KeyCode::PageDown), Event::from(StandardEvent::ScrollJumpDown))]
	#[case::standard(Event::from(KeyCode::Home), Event::from(StandardEvent::ScrollTop))]
	#[case::standard(Event::from(KeyCode::End), Event::from(StandardEvent::ScrollBottom))]
	#[case::other(Event::from('a'), Event::from(KeyCode::Null))]
	fn default_movement_inputs(#[case] event: Event, #[case] expected: Event) {
		let mut bindings = create_test_keybindings();
		bindings.scroll_down = map_keybindings(&[String::from("x")]);
		bindings.scroll_end = map_keybindings(&[String::from("x")]);
		bindings.scroll_home = map_keybindings(&[String::from("x")]);
		bindings.scroll_left = map_keybindings(&[String::from("x")]);
		bindings.scroll_right = map_keybindings(&[String::from("x")]);
		bindings.scroll_up = map_keybindings(&[String::from("x")]);
		bindings.scroll_step_down = map_keybindings(&[String::from("x")]);
		bindings.scroll_step_up = map_keybindings(&[String::from("x")]);
		let event_handler = EventHandler::new(bindings);
		let result = event_handler.read_event(event, &InputOptions::MOVEMENT, |_, _| Event::from(KeyCode::Null));
		assert_eq!(result, expected);
	}

	#[test]
	fn help_event() {
		let event_handler = EventHandler::new(create_test_keybindings());
		let result = event_handler.read_event(Event::from('?'), &InputOptions::HELP, |_, _| Event::from(KeyCode::Null));
		assert_eq!(result, Event::from(StandardEvent::Help));
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
