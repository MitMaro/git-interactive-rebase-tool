use crate::input::Event;

pub(crate) struct Options {
	pub(crate) next_result_event: Vec<Event>,
	pub(crate) previous_result_event: Vec<Event>,
}

impl Options {
	pub(crate) fn new(next_result_event: Vec<Event>, previous_result_event: Vec<Event>) -> Self {
		Self {
			next_result_event,
			previous_result_event,
		}
	}
}
