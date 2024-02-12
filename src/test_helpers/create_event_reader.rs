use anyhow::Result;
use crossterm::event as c_event;

use crate::input::{Event, EventReaderFn};

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
