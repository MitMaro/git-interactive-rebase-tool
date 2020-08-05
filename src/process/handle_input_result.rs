use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::state::State;

pub struct HandleInputResult {
	pub(super) exit_status: Option<ExitStatus>,
	pub(super) input: Input,
	pub(super) state: Option<State>,
}

impl HandleInputResult {
	pub(crate) const fn new(input: Input) -> Self {
		Self {
			exit_status: None,
			input,
			state: None,
		}
	}
}

pub struct HandleInputResultBuilder {
	handle_input: HandleInputResult,
}

impl HandleInputResultBuilder {
	pub(crate) const fn new(input: Input) -> Self {
		Self {
			handle_input: HandleInputResult {
				exit_status: None,
				input,
				state: None,
			},
		}
	}

	pub(crate) const fn exit_status(mut self, status: ExitStatus) -> Self {
		self.handle_input.exit_status = Some(status);
		self
	}

	#[allow(clippy::missing_const_for_fn)]
	pub(crate) fn state(mut self, new_state: State) -> Self {
		self.handle_input.state = Some(new_state);
		self
	}

	#[allow(clippy::missing_const_for_fn)]
	pub(crate) fn build(self) -> HandleInputResult {
		self.handle_input
	}
}
