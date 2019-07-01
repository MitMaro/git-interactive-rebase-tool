use crate::application::Application;
use crate::input::Input;
use crate::process::{ExitStatus, State};
use std::cell::RefCell;

pub struct Process<'r> {
	application: &'r mut Application<'r>,
	exit_status: Option<ExitStatus>,
	state: RefCell<State>,
}

impl<'r> Process<'r> {
	pub fn new(application: &'r mut Application<'r>) -> Self {
		Self {
			application,
			exit_status: None,
			state: RefCell::new(State::List(false)),
		}
	}

	pub fn run(&mut self) -> Result<Option<ExitStatus>, String> {
		self.check_window_size();
		while self.exit_status.is_none() {
			self.process();
			self.render();
			self.handle_input();
		}
		self.exit_end()?;
		Ok(self.exit_status)
	}

	fn process(&mut self) {
		let result = self.application.process(self.get_state());

		if let Some(exit_status) = result.exit_status {
			self.exit_status = Some(exit_status);
		}

		if let Some(new_state) = result.state {
			if new_state != self.get_state() {
				self.application.deactivate(self.get_state());
				self.set_state(new_state);
				self.application.activate(self.get_state());
			}
		}
	}

	fn render(&self) {
		self.application.render(self.get_state());
	}

	fn handle_input(&mut self) {
		let result = self.application.handle_input(self.get_state());

		if let Some(exit_status) = result.exit_status {
			self.exit_status = Some(exit_status);
		}

		if let Some(new_state) = result.state {
			if new_state != self.get_state() {
				self.application.deactivate(self.get_state());
				self.set_state(new_state);
				self.application.activate(self.get_state());
			}
		}

		if let Input::Resize = result.input {
			self.check_window_size();
		}
	}

	fn check_window_size(&self) {
		let check = self.application.check_window_size();
		let state = self.get_state();
		if let State::WindowSizeError(return_state) = state {
			if check {
				self.set_state(*return_state);
			}
		}
		else if !check {
			self.set_state(State::WindowSizeError(Box::new(self.get_state())));
		}
	}

	fn set_state(&self, new_state: State) {
		self.state.replace(new_state);
	}

	pub fn get_state(&self) -> State {
		self.state.borrow().clone()
	}

	fn exit_end(&mut self) -> Result<(), String> {
		match self.application.write_file() {
			Ok(_) => {},
			Err(msg) => {
				self.exit_status = Some(ExitStatus::FileWriteError);
				return Err(msg);
			},
		}
		Ok(())
	}
}
