//! Utilities for writing tests that interact with input events.

use super::{Event, EventHandler, KeyBindings, KeyCode, KeyEvent, KeyModifiers};
use crate::{event_action::EventAction, map_keybindings, Sender};

#[cfg(test)]
pub(crate) mod local {
	use crate::{CustomEvent, CustomKeybinding};

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
	pub(crate) enum TestEvent {}
	impl CustomEvent for TestEvent {}

	pub(crate) struct TestKeybinding {}
	impl CustomKeybinding for TestKeybinding {
		fn new(_: &config::KeyBindings) -> Self {
			Self {}
		}
	}

	pub(crate) type Event = super::Event<TestEvent>;
	pub(crate) type EventHandler = super::EventHandler<TestKeybinding, TestEvent>;
	pub(crate) type KeyBindings = super::KeyBindings<TestKeybinding, TestEvent>;

	pub(crate) fn create_test_keybindings() -> KeyBindings {
		super::create_test_keybindings::<TestKeybinding, TestEvent>(TestKeybinding {})
	}
}

/// Create a mocked version of `KeyBindings`.
#[inline]
#[must_use]
pub fn create_test_keybindings<TestKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent>(
	custom_key_bindings: TestKeybinding,
) -> KeyBindings<TestKeybinding, CustomEvent> {
	KeyBindings {
		redo: vec![Event::Key({
			KeyEvent {
				code: KeyCode::Char('y'),
				modifiers: KeyModifiers::CONTROL,
			}
		})],
		undo: vec![Event::Key({
			KeyEvent {
				code: KeyCode::Char('z'),
				modifiers: KeyModifiers::CONTROL,
			}
		})],
		scroll_down: map_keybindings(&[String::from("Down")]),
		scroll_end: map_keybindings(&[String::from("End")]),
		scroll_home: map_keybindings(&[String::from("Home")]),
		scroll_left: map_keybindings(&[String::from("Left")]),
		scroll_right: map_keybindings(&[String::from("Right")]),
		scroll_up: map_keybindings(&[String::from("Up")]),
		scroll_step_down: map_keybindings(&[String::from("PageDown")]),
		scroll_step_up: map_keybindings(&[String::from("PageUp")]),
		custom: custom_key_bindings,
	}
}

/// Context for a `EventHandler` based test.
#[allow(missing_debug_implementations)]
#[non_exhaustive]
pub struct TestContext<TestKeybinding: crate::CustomKeybinding, CustomEvent: crate::CustomEvent> {
	/// The `EventHandler` instance.
	pub event_handler: EventHandler<TestKeybinding, CustomEvent>,
	/// The sender instance.
	pub sender: Sender<CustomEvent>,
	/// The receiver instance.
	pub receiver: crossbeam_channel::Receiver<EventAction<CustomEvent>>,
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
	let (sender, receiver) = crossbeam_channel::bounded(10);
	let (_, new_event_receiver) = crossbeam_channel::unbounded();
	let event_sender = Sender::new(sender, new_event_receiver);
	let event_queue = event_sender.clone_event_queue();

	for event in events {
		event_queue.lock().push_back(*event);
	}

	callback(TestContext {
		event_handler,
		sender: event_sender,
		receiver,
		number_events: events.len(),
	});
}
