use crate::Event;

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum EventAction<CustomEvent: crate::CustomEvent> {
	End,
	EnqueueEvent(Event<CustomEvent>),
	PushEvent(Event<CustomEvent>),
}
