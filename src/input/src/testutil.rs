//! Utilities for writing tests that interact with input events.

use anyhow::Result;
use crossterm::event as c_event;

use crate::{
	map_keybindings,
	thread::State,
	Event,
	EventHandler,
	EventReaderFn,
	KeyBindings,
	KeyCode,
	KeyEvent,
	KeyModifiers,
};

#[cfg(test)]
pub(crate) mod local {
	use anyhow::Result;

	use crate::{CustomEvent, CustomKeybinding, EventReaderFn};

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
	pub(crate) enum TestEvent {}
	impl CustomEvent for TestEvent {}

	pub(crate) struct TestKeybinding;
	impl CustomKeybinding for TestKeybinding {
		fn new(_: &config::KeyBindings) -> Self {
			Self {}
		}
	}

	pub(crate) type Event = crate::Event<TestEvent>;
	pub(crate) type EventHandler = crate::EventHandler<TestKeybinding, TestEvent>;
	pub(crate) type KeyBindings = crate::KeyBindings<TestKeybinding, TestEvent>;

	pub(crate) fn create_test_keybindings() -> KeyBindings {
		super::create_test_keybindings::<TestKeybinding, TestEvent>(TestKeybinding {})
	}

	pub(crate) fn create_event_reader<EventGeneratorFunction>(
		event_generator: EventGeneratorFunction,
	) -> impl EventReaderFn
	where EventGeneratorFunction: Fn() -> Result<Option<Event>> + Sync + Send + 'static {
		super::create_event_reader(event_generator)
	}
}

/// Create a mocked version of `KeyBindings`.
#[inline]
#[must_use]
pub fn create_test_keybindings<TestKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent>(
	custom_key_bindings: TestKeybinding,
) -> KeyBindings<TestKeybinding, CustomEvent> {
	KeyBindings {
		redo: vec![Event::from(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CONTROL))],
		undo: vec![Event::from(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL))],
		scroll_down: map_keybindings(&[String::from("Down")]),
		scroll_end: map_keybindings(&[String::from("End")]),
		scroll_home: map_keybindings(&[String::from("Home")]),
		scroll_left: map_keybindings(&[String::from("Left")]),
		scroll_right: map_keybindings(&[String::from("Right")]),
		scroll_up: map_keybindings(&[String::from("Up")]),
		scroll_step_down: map_keybindings(&[String::from("PageDown")]),
		scroll_step_up: map_keybindings(&[String::from("PageUp")]),
		help: map_keybindings(&[String::from("?")]),
		search_start: map_keybindings(&[String::from("/")]),
		search_next: map_keybindings(&[String::from("n")]),
		search_previous: map_keybindings(&[String::from("N")]),
		custom: custom_key_bindings,
	}
}

/// Context for a `EventHandler` based test.
#[derive(Debug)]
#[non_exhaustive]
pub struct TestContext<TestKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent> {
	/// The `EventHandler` instance.
	pub event_handler: EventHandler<TestKeybinding, CustomEvent>,
	/// The sender instance.
	pub state: State<CustomEvent>,
	/// The number of known available events.
	pub number_events: usize,
}

/// Provide an `EventHandler` instance for use within a test.
#[inline]
#[allow(clippy::missing_panics_doc)]
pub fn with_event_handler<C, TestKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent>(
	custom_key_bindings: TestKeybinding,
	events: &[Event<CustomEvent>],
	callback: C,
) where
	C: FnOnce(TestContext<TestKeybinding, CustomEvent>),
{
	let event_handler = EventHandler::new(create_test_keybindings(custom_key_bindings));
	let state = State::new();

	for event in events {
		state.enqueue_event(*event);
	}

	callback(TestContext {
		event_handler,
		state,
		number_events: events.len(),
	});
}

/// Create an event reader that will map the provided events to the internal representation of the
/// events. This allows for mocking of event input when testing at the highest level of the application.
///
/// This function does not accept any `Event::MetaEvent` or `Event::StandardEvent` event types, instead
/// use other event types that will map to the expected value using the keybindings.
///
/// This function should be used sparingly, and instead `with_event_handler` should be used where possible.
///
/// # Panics
/// If provided an event generator that returns a `Event::MetaEvent` or `Event::StandardEvent` event type.
#[allow(clippy::panic)]
#[inline]
pub fn create_event_reader<EventGeneratorFunction, CustomEvent>(
	event_generator: EventGeneratorFunction,
) -> impl EventReaderFn
where
	EventGeneratorFunction: Fn() -> Result<Option<Event<CustomEvent>>> + Sync + Send + 'static,
	CustomEvent: crate::CustomEvent,
{
	move || {
		match event_generator()? {
			None => Ok(None),
			Some(event) => {
				match event {
					Event::Key(key) => {
						Ok(Some(c_event::Event::Key(c_event::KeyEvent::new(
							key.code,
							key.modifiers,
						))))
					},
					Event::Mouse(mouse_event) => Ok(Some(c_event::Event::Mouse(mouse_event))),
					Event::None => Ok(None),
					Event::Resize(width, height) => Ok(Some(c_event::Event::Resize(width, height))),
					Event::MetaEvent(_) | Event::Standard(_) => {
						panic!("MetaEvent and Standard are not supported, please use other event types")
					},
				}
			},
		}
	}
}
