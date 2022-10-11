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
			}
		})
		.collect::<Vec<String>>()
		.join("\n")
}

pub(crate) fn _assert_results(results: Results, expected_artifacts: &[Artifact]) {
	assert_eq!(
		_assert_results_format(expected_artifacts),
		_assert_results_format(results.artifacts.into_iter().collect::<Vec<_>>().as_slice())
	);
}

#[macro_export]
macro_rules! assert_results {
		($actual:expr) => {{
			use $crate::testutil::_assert_results;
			_assert_results($actual, &[]);
		}};
		($actual:expr, $($arg:expr),*) => {{
			use $crate::testutil::_assert_results;
			let expected = vec![$( $arg, )*];
			_assert_results($actual, &expected);
		}};
	}
