use std::fmt::{Debug, Formatter};

use pretty_assertions::assert_eq;

use crate::process::{Artifact, Results};

fn _assert_results_format(artifacts: &[Artifact]) -> String {
	artifacts
		.iter()
		.map(|artifact| {
			match *artifact {
				Artifact::Event(event) => format!("Event({event:?})"),
				Artifact::ChangeState(state) => format!("ChangeState({state:?})"),
				Artifact::Error(ref err, state) => {
					format!(
						"Error({err:#}) State({})",
						state.map_or_else(|| String::from("None"), |s| format!("{s:?}"))
					)
				},
				Artifact::ExitStatus(status) => format!("ExitStatus({status:?})"),
				Artifact::ExternalCommand(ref command) => {
					format!("ExternalCommand({:?} {:?})", command.0, command.1.join(","))
				},
				Artifact::EnqueueResize => String::from("EnqueueResize"),
				Artifact::SearchCancel => String::from("SearchCancel"),
				Artifact::SearchTerm(ref term) => format!("SearchTerm({term})"),
				Artifact::Searchable(ref _searchable) => String::from("SearchCancel(_)"),
			}
		})
		.collect::<Vec<String>>()
		.join("\n")
}

fn compare_artifact(a: &Artifact, b: &Artifact) -> bool {
	match (a, b) {
		(Artifact::ChangeState(self_state), Artifact::ChangeState(other_state)) => self_state == other_state,
		(Artifact::Error(self_error, self_state), Artifact::Error(other_error, other_state)) => {
			self_state == other_state && format!("{self_error:#}") == format!("{other_error:#}")
		},
		(Artifact::Event(self_event), Artifact::Event(other_event)) => self_event == other_event,
		(Artifact::ExitStatus(self_exit_status), Artifact::ExitStatus(other_exit_status)) => {
			self_exit_status == other_exit_status
		},
		(Artifact::ExternalCommand(self_command), Artifact::ExternalCommand(other_command)) => {
			self_command == other_command
		},
		(Artifact::SearchTerm(self_term), Artifact::SearchTerm(other_term)) => self_term == other_term,
		(Artifact::SearchCancel, Artifact::SearchCancel)
		| (Artifact::EnqueueResize, Artifact::EnqueueResize)
		| (Artifact::Searchable(_), Artifact::Searchable(_)) => true,
		_ => false,
	}
}

pub(crate) struct AnyArtifact;

pub(crate) enum ArtifactCompareWrapper {
	Any,
	Artifact(Artifact),
}

impl PartialEq for ArtifactCompareWrapper {
	fn eq(&self, other: &Self) -> bool {
		match self {
			ArtifactCompareWrapper::Any => true,
			ArtifactCompareWrapper::Artifact(a) => {
				match other {
					ArtifactCompareWrapper::Any => true,
					ArtifactCompareWrapper::Artifact(o) => compare_artifact(a, o),
				}
			},
		}
	}
}

impl Debug for ArtifactCompareWrapper {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ArtifactCompareWrapper::Any => write!(f, "Any Result"),
			ArtifactCompareWrapper::Artifact(a) => a.fmt(f),
		}
	}
}

impl From<Artifact> for ArtifactCompareWrapper {
	fn from(value: Artifact) -> Self {
		Self::Artifact(value)
	}
}

impl From<AnyArtifact> for ArtifactCompareWrapper {
	fn from(_: AnyArtifact) -> Self {
		Self::Any
	}
}

pub(crate) fn _assert_results(results: Results, expected_artifacts: &[ArtifactCompareWrapper]) {
	assert_eq!(
		expected_artifacts,
		results
			.artifacts
			.into_iter()
			.map(ArtifactCompareWrapper::from)
			.collect::<Vec<_>>()
			.as_slice()
	);
}

#[macro_export]
macro_rules! assert_results {
	($actual:expr) => {{
		use $crate::test_helpers::assertions::_assert_results;
		_assert_results($actual, &[]);
	}};
	($actual:expr, $($arg:expr),*) => {{
		use $crate::test_helpers::assertions::{
			_assert_results,
			ArtifactCompareWrapper,
		};

		let expected= vec![$(
			ArtifactCompareWrapper::from($arg),
		)*];
		_assert_results($actual, &expected);
	}};
}
