use crate::input::Input;
use crate::process::exit_status::ExitStatus;
use crate::process::state::State;

pub(crate) struct HandleInputResult {
	pub(super) exit_status: Option<ExitStatus>,
	pub(super) input: Input,
	pub(super) state: Option<State>,
}

impl HandleInputResult {
	pub(crate) fn new(input: Input) -> Self {
		Self {
			exit_status: None,
			input,
			state: None,
		}
	}
}

pub(crate) struct HandleInputResultBuilder {
	handle_input: HandleInputResult,
}

impl HandleInputResultBuilder {
	pub(crate) fn new(input: Input) -> Self {
		Self {
			handle_input: HandleInputResult {
				exit_status: None,
				input,
				state: None,
			},
		}
	}

	pub(crate) fn exit_status(mut self, status: ExitStatus) -> Self {
		self.handle_input.exit_status = Some(status);
		self
	}

	pub(crate) fn help(mut self, target_state: State) -> Self {
		self.handle_input.state = Some(State::Help(Box::new(target_state)));
		self
	}

	pub(crate) fn state(mut self, new_state: State) -> Self {
		self.handle_input.state = Some(new_state);
		self
	}

	pub(crate) fn build(self) -> HandleInputResult {
		self.handle_input
	}
}
