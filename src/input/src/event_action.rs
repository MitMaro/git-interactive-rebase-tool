use crate::Event;

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_enums)]
pub enum EventAction {
	End,
	EnqueueEvent(Event),
	PushEvent(Event),
}
