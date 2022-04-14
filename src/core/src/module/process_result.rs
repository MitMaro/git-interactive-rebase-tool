use anyhow::Error;

use crate::{
	events::Event,
	module::{ExitStatus, State},
};

#[derive(Debug)]
pub(crate) struct ProcessResult {
	pub(crate) error: Option<Error>,
	pub(crate) exit_status: Option<ExitStatus>,
	pub(crate) event: Option<Event>,
	pub(crate) state: Option<State>,
	pub(crate) external_command: Option<(String, Vec<String>)>,
}

impl ProcessResult {
	pub(crate) const fn new() -> Self {
		Self {
			error: None,
			exit_status: None,
			event: None,
			state: None,
			external_command: None,
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

	pub(crate) fn external_command(mut self, command: String, arguments: Vec<String>) -> Self {
		self.external_command = Some((command, arguments));
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
			external_command: None,
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
		assert_eq!(result.external_command, None);
	}

	#[test]
	fn with_event() {
		let result = ProcessResult::new().event(Event::from('a'));
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.event, Some(Event::from('a')));
		assert_eq!(result.state, None);
		assert_eq!(result.external_command, None);
	}

	#[test]
	fn with_error() {
		let result = ProcessResult::new().error(anyhow!("Test Error"));
		assert_eq!(result.error.unwrap().to_string(), String::from("Test Error"));
		assert_eq!(result.exit_status, None);
		assert_eq!(result.event, None);
		assert_eq!(result.state, None);
		assert_eq!(result.external_command, None);
	}

	#[test]
	fn exit_status() {
		let result = ProcessResult::new().exit_status(ExitStatus::Good);
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, Some(ExitStatus::Good));
		assert_eq!(result.event, None);
		assert_eq!(result.state, None);
		assert_eq!(result.external_command, None);
	}

	#[test]
	fn state() {
		let result = ProcessResult::new().state(State::List);
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.event, None);
		assert_eq!(result.state, Some(State::List));
		assert_eq!(result.external_command, None);
	}

	#[test]
	fn external_command() {
		let result = ProcessResult::new()
			.external_command(String::from("editor"), vec![String::from("arg1"), String::from("arg2")]);
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.event, None);
		assert_eq!(result.state, None);
		assert_eq!(
			result.external_command,
			Some((String::from("editor"), vec![String::from("arg1"), String::from("arg2")]))
		);
	}

	#[test]
	fn everything() {
		let result = ProcessResult::new()
			.error(anyhow!("Test Error"))
			.state(State::List)
			.exit_status(ExitStatus::Good)
			.event(Event::from('a'))
			.external_command(String::from("editor"), vec![String::from("arg1"), String::from("arg2")]);
		assert_eq!(result.error.unwrap().to_string(), String::from("Test Error"));
		assert_eq!(result.exit_status, Some(ExitStatus::Good));
		assert_eq!(result.event, Some(Event::from('a')));
		assert_eq!(result.state, Some(State::List));
		assert_eq!(
			result.external_command,
			Some((String::from("editor"), vec![String::from("arg1"), String::from("arg2")]))
		);
	}
}
