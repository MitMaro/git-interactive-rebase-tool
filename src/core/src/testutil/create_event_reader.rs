use anyhow::Result;
use input::{Event, EventReaderFn};

use crate::events::MetaEvent;

pub(crate) fn create_event_reader<EventGeneratorFunction>(
	event_generator: EventGeneratorFunction,
) -> impl EventReaderFn
where EventGeneratorFunction: Fn() -> Result<Option<Event<MetaEvent>>> + Sync + Send + 'static {
	input::testutil::create_event_reader(event_generator)
}
