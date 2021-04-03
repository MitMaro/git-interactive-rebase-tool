use anyhow::Error;

use crate::{
	input::Event,
	process::{exit_status::ExitStatus, state::State},
};

#[derive(Debug)]
pub struct ProcessResult {
	pub(super) error: Option<Error>,
	pub(super) exit_status: Option<ExitStatus>,
	pub(super) event: Option<Event>,
	pub(super) state: Option<State>,
}

impl ProcessResult {
	pub(crate) const fn new() -> Self {
		Self {
			error: None,
			exit_status: None,
			event: None,
			state: None,
		}
	}

	pub(crate) const fn event(mut self, event: Event) -> Self {
		self.event = Some(event);
		self
	}

	pub(crate) fn error(mut self, error: Error) -> Self {
		self.error = Some(error);
		self
	}

	pub(crate) const fn exit_status(mut self, status: ExitStatus) -> Self {
		self.exit_status = Some(status);
		self
	}

	#[allow(clippy::missing_const_for_fn)] // false positive
	pub(crate) fn state(mut self, new_state: State) -> Self {
		self.state = Some(new_state);
		self
	}
}

impl From<Event> for ProcessResult {
	fn from(event: Event) -> Self {
		Self {
			error: None,
			exit_status: None,
			event: Some(event),
			state: None,
		}
	}
}

#[cfg(test)]
mod tests {
	use anyhow::anyhow;

	use super::*;

	#[test]
	fn empty() {
		let result = ProcessResult::new();
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.event, None);
		assert_eq!(result.state, None);
	}

	#[test]
	fn with_event() {
		let result = ProcessResult::new().event(Event::from('a'));
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.event, Some(Event::from('a')));
		assert_eq!(result.state, None);
	}

	#[test]
	fn with_error() {
		let result = ProcessResult::new().error(anyhow!("Test Error"));
		assert_eq!(result.error.unwrap().to_string(), String::from("Test Error"));
		assert_eq!(result.exit_status, None);
		assert_eq!(result.event, None);
		assert_eq!(result.state, None);
	}

	#[test]
	fn exit_status() {
		let result = ProcessResult::new().exit_status(ExitStatus::Good);
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, Some(ExitStatus::Good));
		assert_eq!(result.event, None);
		assert_eq!(result.state, None);
	}

	#[test]
	fn state() {
		let result = ProcessResult::new().state(State::List);
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.event, None);
		assert_eq!(result.state, Some(State::List));
	}

	#[test]
	fn everything() {
		let result = ProcessResult::new()
			.error(anyhow!("Test Error"))
			.state(State::List)
			.exit_status(ExitStatus::Good)
			.event(Event::from('a'));
		assert_eq!(result.error.unwrap().to_string(), String::from("Test Error"));
		assert_eq!(result.exit_status, Some(ExitStatus::Good));
		assert_eq!(result.event, Some(Event::from('a')));
		assert_eq!(result.state, Some(State::List));
	}
}
