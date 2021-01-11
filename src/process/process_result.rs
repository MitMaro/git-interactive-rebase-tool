use super::exit_status::ExitStatus;
use super::state::State;
use crate::input::Input;
use anyhow::Error;

#[derive(Debug)]
pub struct ProcessResult {
	pub(super) error: Option<Error>,
	pub(super) exit_status: Option<ExitStatus>,
	pub(super) input: Option<Input>,
	pub(super) state: Option<State>,
}

impl ProcessResult {
	pub(crate) const fn new() -> Self {
		Self {
			error: None,
			exit_status: None,
			input: None,
			state: None,
		}
	}

	pub(crate) const fn input(mut self, input: Input) -> Self {
		self.input = Some(input);
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

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::anyhow;

	#[test]
	fn empty() {
		let result = ProcessResult::new();
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.input, None);
		assert_eq!(result.state, None);
	}

	#[test]
	fn with_input() {
		let result = ProcessResult::new().input(Input::Character('a'));
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.input, Some(Input::Character('a')));
		assert_eq!(result.state, None);
	}

	#[test]
	fn with_error() {
		let result = ProcessResult::new().error(anyhow!("Test Error"));
		assert_eq!(result.error.unwrap().to_string(), String::from("Test Error"));
		assert_eq!(result.exit_status, None);
		assert_eq!(result.input, None);
		assert_eq!(result.state, None);
	}

	#[test]
	fn exit_status() {
		let result = ProcessResult::new().exit_status(ExitStatus::Good);
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, Some(ExitStatus::Good));
		assert_eq!(result.input, None);
		assert_eq!(result.state, None);
	}

	#[test]
	fn state() {
		let result = ProcessResult::new().state(State::List);
		assert!(result.error.is_none());
		assert_eq!(result.exit_status, None);
		assert_eq!(result.input, None);
		assert_eq!(result.state, Some(State::List));
	}

	#[test]
	fn everything() {
		let result = ProcessResult::new()
			.error(anyhow!("Test Error"))
			.state(State::List)
			.exit_status(ExitStatus::Good)
			.input(Input::Character('a'));
		assert_eq!(result.error.unwrap().to_string(), String::from("Test Error"));
		assert_eq!(result.exit_status, Some(ExitStatus::Good));
		assert_eq!(result.input, Some(Input::Character('a')));
		assert_eq!(result.state, Some(State::List));
	}
}
