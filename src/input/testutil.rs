//! Utilities for writing tests that interact with input events.

use anyhow::Result;
use crossterm::event as c_event;

use crate::input::{
	map_keybindings,
	Event,
	EventHandler,
	EventReaderFn,
	KeyBindings,
	KeyCode,
	KeyEvent,
	KeyModifiers,
	State,
};

/// Create a mocked version of `KeyBindings`.
#[must_use]
pub(crate) fn create_test_keybindings() -> KeyBindings {
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
		abort: map_keybindings(&[String::from("q")]),
		action_break: map_keybindings(&[String::from("b")]),
		action_drop: map_keybindings(&[String::from("d")]),
		action_edit: map_keybindings(&[String::from("e")]),
		action_fixup: map_keybindings(&[String::from("f")]),
		action_pick: map_keybindings(&[String::from("p")]),
		action_reword: map_keybindings(&[String::from("r")]),
		action_squash: map_keybindings(&[String::from("s")]),
		confirm_yes: map_keybindings(&[String::from("y")]),
		edit: map_keybindings(&[String::from("E")]),
		force_abort: map_keybindings(&[String::from("Q")]),
		force_rebase: map_keybindings(&[String::from("W")]),
		insert_line: map_keybindings(&[String::from("I")]),
		move_down: map_keybindings(&[String::from("Down")]),
		move_down_step: map_keybindings(&[String::from("PageDown")]),
		move_end: map_keybindings(&[String::from("End")]),
		move_home: map_keybindings(&[String::from("Home")]),
		move_left: map_keybindings(&[String::from("Left")]),
		move_right: map_keybindings(&[String::from("Right")]),
		move_selection_down: map_keybindings(&[String::from("j")]),
		move_selection_up: map_keybindings(&[String::from("k")]),
		move_up: map_keybindings(&[String::from("Up")]),
		move_up_step: map_keybindings(&[String::from("PageUp")]),
		open_in_external_editor: map_keybindings(&[String::from('!')]),
		rebase: map_keybindings(&[String::from('w')]),
		remove_line: map_keybindings(&[String::from("Delete")]),
		show_commit: map_keybindings(&[String::from("c")]),
		show_diff: map_keybindings(&[String::from("d")]),
		toggle_visual_mode: map_keybindings(&[String::from("v")]),
		fixup_keep_message: map_keybindings(&[String::from("u")]),
		fixup_keep_message_with_editor: map_keybindings(&[String::from("U")]),
	}
}

/// Context for a `EventHandler` based test.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct TestContext {
	/// The `EventHandler` instance.
	pub(crate) event_handler: EventHandler,
	/// The sender instance.
	pub(crate) state: State,
	/// The number of known available events.
	pub(crate) number_events: usize,
}

/// Provide an `EventHandler` instance for use within a test.
#[allow(clippy::missing_panics_doc)]
pub(crate) fn with_event_handler<C>(events: &[Event], callback: C)
where C: FnOnce(TestContext) {
	let event_handler = EventHandler::new(create_test_keybindings());
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
pub(crate) fn create_event_reader<EventGeneratorFunction>(
	event_generator: EventGeneratorFunction,
) -> impl EventReaderFn
where EventGeneratorFunction: Fn() -> Result<Option<Event>> + Sync + Send + 'static {
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
					Event::Standard(_) => {
						panic!("MetaEvent and Standard are not supported, please use other event types")
					},
				}
			},
		}
	}
}
