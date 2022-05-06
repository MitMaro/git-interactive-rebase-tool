use pretty_assertions::assert_eq;

use crate::process::{Artifact, Results};

fn _assert_results_format(artifacts: &[Artifact]) -> String {
	artifacts
		.iter()
		.map(|artifact| {
			match artifact {
				Artifact::Event(event) => format!("Event({:?})", event),
				Artifact::ChangeState(state) => format!("ChangeState({:?})", state),
				Artifact::Error(err, state) => {
					format!(
						"Error({:#}) State({})",
						err,
						state.map(|s| format!("{:?}", s)).unwrap_or(String::from("None"))
					)
				},
				Artifact::ExitStatus(status) => format!("ExitStatus({:?})", status),
				Artifact::ExternalCommand(command) => {
					format!("ExternalCommand({:?} {:?})", command.0, command.1.join(","))
				},
				Artifact::EnqueueResize => format!("EnqueueResize"),
			}
		})
		.collect::<Vec<String>>()
		.join("\n")
}

pub(crate) fn _assert_results(results: Results, expected_artifacts: &[Artifact]) {
	assert_eq!(
		_assert_results_format(expected_artifacts),
		_assert_results_format(Vec::from_iter(results.artifacts.into_iter()).as_slice())
	);
}

#[macro_export]
macro_rules! assert_results {
		($actual:expr) => {{
			use crate::testutil::_assert_results;
			_assert_results($actual, &[]);
		}};
		($actual:expr, $($arg:expr),*) => {{
			use crate::testutil::_assert_results;
			let expected = vec![$( $arg, )*];
			_assert_results($actual, &expected);
		}};
	}
