use anyhow::Result;

use crate::{
	events::MetaEvent,
	input::{Event, EventReaderFn},
};

pub(crate) fn create_event_reader<EventGeneratorFunction>(
	event_generator: EventGeneratorFunction,
) -> impl EventReaderFn
where EventGeneratorFunction: Fn() -> Result<Option<Event<MetaEvent>>> + Sync + Send + 'static {
	crate::input::testutil::create_event_reader(event_generator)
}
