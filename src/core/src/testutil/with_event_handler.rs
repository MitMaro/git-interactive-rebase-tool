use crate::{
	events::{AppKeyBindings, Event, MetaEvent},
	testutil::create_test_custom_keybindings,
};

pub(crate) type EventHandlerTestContext = input::testutil::TestContext<AppKeyBindings, MetaEvent>;

pub(crate) fn with_event_handler<C>(events: &[Event], callback: C)
where C: FnOnce(EventHandlerTestContext) {
	input::testutil::with_event_handler(create_test_custom_keybindings(), events, callback);
}
