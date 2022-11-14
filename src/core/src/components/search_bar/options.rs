use input::Event;

pub struct Options {
	pub(crate) next_result_event: Vec<Event>,
	pub(crate) previous_result_event: Vec<Event>,
}

impl Options {
	pub fn new(next_result_event: Vec<Event>, previous_result_event: Vec<Event>) -> Self {
		Self {
			next_result_event,
			previous_result_event,
		}
	}
}
