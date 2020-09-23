use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::state::State;
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
		self.state = Some(State::Error);
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
