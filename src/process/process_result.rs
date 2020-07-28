use crate::process::exit_status::ExitStatus;
use crate::process::state::State;

#[derive(Debug)]
pub struct ProcessResult {
	pub(super) exit_status: Option<ExitStatus>,
	pub(super) state: Option<State>,
}

impl ProcessResult {
	pub(crate) fn new() -> Self {
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
	pub(crate) fn new() -> Self {
		Self {
			process_result: ProcessResult {
				exit_status: None,
				state: None,
			},
		}
	}

	pub(crate) fn error(mut self, message: &str, return_state: State) -> Self {
		self.process_result.state = Some(State::Error {
			return_state: Box::new(return_state),
			message: String::from(message),
		});
		self
	}

	pub(crate) fn exit_status(mut self, status: ExitStatus) -> Self {
		self.process_result.exit_status = Some(status);
		self
	}

	pub(crate) fn state(mut self, new_state: State) -> Self {
		self.process_result.state = Some(new_state);
		self
	}

	pub(crate) fn build(self) -> ProcessResult {
		self.process_result
	}
}
