use crate::{
	input::{Event, EventHandler, State},
	test_helpers::create_test_keybindings,
};

/// Context for a `EventHandler` based test.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct EventHandlerTestContext {
	/// The `EventHandler` instance.
	pub(crate) event_handler: EventHandler,
	/// The sender instance.
	pub(crate) state: State,
	/// The number of known available events.
	pub(crate) number_events: usize,
}

/// Provide an `EventHandler` instance for use within a test.
pub(crate) fn with_event_handler<C>(events: &[Event], callback: C)
where C: FnOnce(EventHandlerTestContext) {
	let event_handler = EventHandler::new(create_test_keybindings());
	let state = State::new();

	for event in events {
		state.enqueue_event(*event);
	}

	callback(EventHandlerTestContext {
		event_handler,
		state,
		number_events: events.len(),
	});
}
