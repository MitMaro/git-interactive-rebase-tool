use input::{testutil::with_event_handler, Event, EventHandler};

use crate::view::{testutil::with_view_sender, ViewSender};

pub struct TestContext {
	pub event_handler: EventHandler,
	pub view_sender: ViewSender,
}

pub fn handle_event_test<C>(events: &[Event], callback: C)
where C: FnOnce(TestContext) {
	with_view_sender(|view_sender_context| {
		with_event_handler(events, |event_handler_context| {
			callback(TestContext {
				event_handler: event_handler_context.event_handler,
				view_sender: view_sender_context.sender,
			});
		});
	});
}
