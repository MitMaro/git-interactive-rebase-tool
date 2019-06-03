use crate::exit_status::ExitStatus;
use crate::input::Input;
use crate::process::State;

pub struct HandleInputResult {
	pub exit_status: Option<ExitStatus>,
	pub input: Input,
	pub state: Option<State>,
}

impl HandleInputResult {
	pub fn new(input: Input) -> Self {
		Self {
			exit_status: None,
			input,
			state: None,
		}
	}
}

pub struct HandleInputResultBuilder {
	pub handle_input: HandleInputResult,
}

impl HandleInputResultBuilder {
	pub fn new(input: Input) -> Self {
		Self {
			handle_input: HandleInputResult {
				exit_status: None,
				input,
				state: None,
			},
		}
	}

	pub fn exit_status(mut self, status: ExitStatus) -> Self {
		self.handle_input.exit_status = Some(status);
		self
	}

	pub fn help(mut self, target_state: State) -> Self {
		self.handle_input.state = Some(State::Help(Box::new(target_state)));
		self
	}

	pub fn state(mut self, new_state: State) -> Self {
		self.handle_input.state = Some(new_state);
		self
	}

	pub fn build(self) -> HandleInputResult {
		self.handle_input
	}
}
