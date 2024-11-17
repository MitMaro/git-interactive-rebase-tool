use std::collections::VecDeque;

use anyhow::Error;

use crate::{
	input::Event,
	module::{ExitStatus, State},
	process::Artifact,
	search::Searchable,
};

#[derive(Debug)]
pub(crate) struct Results {
	pub artifacts: VecDeque<Artifact>,
}

impl Results {
	pub(crate) const fn new() -> Self {
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

	pub(crate) fn search_cancel(&mut self) {
		self.artifacts.push_back(Artifact::SearchCancel);
	}

	pub(crate) fn search_term(&mut self, term: &str) {
		self.artifacts.push_back(Artifact::SearchTerm(String::from(term)));
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

impl From<Box<dyn Searchable>> for Results {
	fn from(searchable: Box<dyn Searchable>) -> Self {
		Self {
			artifacts: VecDeque::from(vec![Artifact::Searchable(searchable)]),
		}
	}
}

#[cfg(test)]
mod tests {
	use anyhow::anyhow;

	use super::*;
	use crate::{assert_results, test_helpers::mocks};

	#[test]
	fn empty() {
		let results = Results::new();
		assert_results!(results);
	}

	#[test]
	fn event() {
		let results = Results::from(Event::from('a'));
		assert_results!(results, Artifact::Event(Event::from('a')));
	}

	#[test]
	fn error() {
		let mut results = Results::new();
		results.error(anyhow!("Test Error"));
		assert_results!(results, Artifact::Error(anyhow!("Test Error"), None));
	}

	#[test]
	fn error_from() {
		let results = Results::from(anyhow!("Test Error"));
		assert_results!(results, Artifact::Error(anyhow!("Test Error"), None));
	}

	#[test]
	fn error_with_return() {
		let mut results = Results::new();
		results.error_with_return(anyhow!("Test Error"), State::List);
		assert_results!(results, Artifact::Error(anyhow!("Test Error"), Some(State::List)));
	}

	#[test]
	fn exit_status() {
		let results = Results::from(ExitStatus::Good);
		assert_results!(results, Artifact::ExitStatus(ExitStatus::Good));
	}

	#[test]
	fn state() {
		let results = Results::from(State::List);
		assert_results!(results, Artifact::ChangeState(State::List));
	}

	#[test]
	fn search_cancel() {
		let mut results = Results::new();
		results.search_cancel();
		assert_results!(results, Artifact::SearchCancel);
	}

	#[test]
	fn search_term() {
		let mut results = Results::new();
		let search_term = String::from("foo");
		results.search_term(search_term.as_str());
		assert_results!(results, Artifact::SearchTerm(search_term));
	}

	#[test]
	fn searchable() {
		let mocked_searchable: Box<dyn Searchable> = Box::new(mocks::Searchable::new());
		let results = Results::from(mocked_searchable);
		assert_results!(results, Artifact::Searchable(Box::new(mocks::Searchable::new())));
	}

	#[test]
	fn external_command() {
		let mut results = Results::new();
		results.external_command(String::from("editor"), vec![String::from("arg1"), String::from("arg2")]);
		assert_results!(
			results,
			Artifact::ExternalCommand((String::from("editor"), vec![String::from("arg1"), String::from("arg2")]))
		);
	}

	#[test]
	fn enqueue_resize() {
		let mut results = Results::new();
		results.enqueue_resize();
		assert_results!(results, Artifact::EnqueueResize);
	}

	#[test]
	fn append() {
		let mut results_1 = Results::new();
		results_1.enqueue_resize();
		let mut results_2 = Results::new();
		results_2.state(State::List);
		results_1.append(results_2);
		assert_results!(results_1, Artifact::EnqueueResize, Artifact::ChangeState(State::List));
	}
}
