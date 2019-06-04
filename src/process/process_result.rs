use crate::process::{ExitStatus, State};

#[derive(Debug)]
pub struct ProcessResult {
	pub exit_status: Option<ExitStatus>,
	pub state: Option<State>,
}

impl ProcessResult {
	pub fn new() -> Self {
		Self {
			exit_status: None,
			state: None,
		}
	}
}

pub struct ProcessResultBuilder {
	process_result: ProcessResult,
}

impl ProcessResultBuilder {
	pub fn new() -> Self {
		Self {
			process_result: ProcessResult {
				exit_status: None,
				state: None,
			},
		}
	}

	pub fn error(mut self, message: &str, return_state: State) -> Self {
		self.process_result.state = Some(State::Error {
			return_state: Box::new(return_state),
			message: String::from(message),
		});
		self
	}

	pub fn exit_status(mut self, status: ExitStatus) -> Self {
		self.process_result.exit_status = Some(status);
		self
	}

	pub fn state(mut self, new_state: State) -> Self {
		self.process_result.state = Some(new_state);
		self
	}

	pub fn build(self) -> ProcessResult {
		self.process_result
	}
}
