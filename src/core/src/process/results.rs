use std::collections::VecDeque;

use anyhow::Error;

use crate::{
	events::Event,
	module::{ExitStatus, State},
	process::artifact::Artifact,
};

#[derive(Debug)]
pub(crate) struct Results {
	pub(crate) artifacts: VecDeque<Artifact>,
}

impl Results {
	pub(crate) fn new() -> Self {
		Self {
			artifacts: VecDeque::new(),
		}
	}

	pub(crate) fn event(&mut self, event: Event) {
		self.artifacts.push_back(Artifact::Event(event));
	}

	pub(crate) fn error(&mut self, error: Error) {
		self.artifacts.push_back(Artifact::Error(error, None));
	}

	pub(crate) fn error_with_return(&mut self, error: Error, state: State) {
		self.artifacts.push_back(Artifact::Error(error, Some(state)));
	}

	pub(crate) fn exit_status(&mut self, status: ExitStatus) {
		self.artifacts.push_back(Artifact::ExitStatus(status));
	}

	pub(crate) fn state(&mut self, new_state: State) {
		self.artifacts.push_back(Artifact::ChangeState(new_state));
	}

	pub(crate) fn external_command(&mut self, command: String, arguments: Vec<String>) {
		self.artifacts
			.push_back(Artifact::ExternalCommand((command, arguments)));
	}

	pub(crate) fn enqueue_resize(&mut self) {
		self.artifacts.push_back(Artifact::EnqueueResize);
	}

	pub(crate) fn append(&mut self, other: Self) {
		self.artifacts.extend(other.artifacts);
	}

	pub(crate) fn artifact(&mut self) -> Option<Artifact> {
		self.artifacts.pop_front()
	}
}

impl From<Event> for Results {
	fn from(event: Event) -> Self {
		Self {
			artifacts: VecDeque::from(vec![Artifact::Event(event)]),
		}
	}
}

impl From<Error> for Results {
	fn from(error: Error) -> Self {
		Self {
			artifacts: VecDeque::from(vec![Artifact::Error(error, None)]),
		}
	}
}

impl From<ExitStatus> for Results {
	fn from(status: ExitStatus) -> Self {
		Self {
			artifacts: VecDeque::from(vec![Artifact::ExitStatus(status)]),
		}
	}
}

impl From<State> for Results {
	fn from(state: State) -> Self {
		Self {
			artifacts: VecDeque::from(vec![Artifact::ChangeState(state)]),
		}
	}
}

#[cfg(test)]
mod tests {
	use anyhow::anyhow;

	use super::*;

	#[test]
	fn empty() {
		let mut results = Results::new();
		assert!(results.artifact().is_none());
	}

	#[test]
	fn event() {
		let mut results = Results::from(Event::from('a'));
		assert!(matches!(results.artifact(), Some(Artifact::Event(_))));
	}

	#[test]
	fn error() {
		let mut results = Results::new();
		results.error(anyhow!("Test Error"));
		assert!(matches!(results.artifact(), Some(Artifact::Error(_, None))));
	}

	#[test]
	fn error_from() {
		let mut results = Results::from(anyhow!("Test Error"));
		assert!(matches!(results.artifact(), Some(Artifact::Error(_, None))));
	}

	#[test]
	fn error_with_return() {
		let mut results = Results::new();
		results.error_with_return(anyhow!("Test Error"), State::List);
		assert!(matches!(
			results.artifact(),
			Some(Artifact::Error(_, Some(State::List)))
		));
	}

	#[test]
	fn exit_status() {
		let mut results = Results::from(ExitStatus::Good);
		assert!(matches!(
			results.artifact(),
			Some(Artifact::ExitStatus(ExitStatus::Good))
		));
	}

	#[test]
	fn state() {
		let mut results = Results::from(State::List);
		assert!(matches!(results.artifact(), Some(Artifact::ChangeState(State::List))));
	}

	#[test]
	fn external_command() {
		let mut results = Results::new();
		results.external_command(String::from("editor"), vec![String::from("arg1"), String::from("arg2")]);
		assert!(matches!(results.artifact(), Some(Artifact::ExternalCommand(_))));
	}

	#[test]
	fn enqueue_resize() {
		let mut results = Results::new();
		results.enqueue_resize();
		assert!(matches!(results.artifact(), Some(Artifact::EnqueueResize)));
	}

	#[test]
	fn append() {
		let mut results_1 = Results::new();
		results_1.enqueue_resize();
		let mut results_2 = Results::new();
		results_2.state(State::List);
		results_1.append(results_2);
		assert!(matches!(results_1.artifact(), Some(Artifact::EnqueueResize)));
		assert!(matches!(results_1.artifact(), Some(Artifact::ChangeState(State::List))));
	}
}
