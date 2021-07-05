use anyhow::Error;
use input::Event;

use crate::module::{ExitStatus, ProcessResult, State};

fn format_process_result(
	event: Option<Event>,
	state: Option<State>,
	exit_status: Option<ExitStatus>,
	error: &Option<Error>,
	external_command: &Option<(String, Vec<String>)>,
) -> String {
	format!(
		"ExitStatus({}), State({}), Event({}), Error({}), ExternalCommand({})",
		exit_status.map_or("None", |exit_status| {
			match exit_status {
				ExitStatus::Abort => "Abort",
				ExitStatus::ConfigError => "ConfigError",
				ExitStatus::FileReadError => "FileReadError",
				ExitStatus::FileWriteError => "FileWriteError",
				ExitStatus::Good => "Good",
				ExitStatus::StateError => "StateError",
				ExitStatus::Kill => "Kill",
			}
		}),
		state.map_or("None", |state| {
			match state {
				State::ConfirmAbort => "ConfirmAbort",
				State::ConfirmRebase => "ConfirmRebase",
				State::Error => "Error",
				State::ExternalEditor => "ExternalEditor",
				State::Insert => "Insert",
				State::List => "List",
				State::ShowCommit => "ShowCommit",
				State::WindowSizeError => "WindowSizeError",
			}
		}),
		event.map_or(String::from("None"), |evt| format!("{:?}", evt)),
		error
			.as_ref()
			.map_or(String::from("None"), |error| format!("{:#}", error)),
		external_command.as_ref().map_or(String::from("None"), |command| {
			format!("{} {}", command.0, command.1.join(","))
		})
	)
}

pub(crate) fn _assert_process_result(
	actual: &ProcessResult,
	event: Option<Event>,
	state: Option<State>,
	exit_status: Option<ExitStatus>,
	error: &Option<Error>,
	external_command: &Option<(String, Vec<String>)>,
) {
	let error_compare_fn = |expected| {
		actual
			.error
			.as_ref()
			.map_or(false, |actual| format!("{:#}", expected) == format!("{:#}", actual))
	};
	let external_command_compare_fn = |expected| {
		actual
			.external_command
			.as_ref()
			.map_or(false, |actual| actual == expected)
	};

	if !(exit_status.map_or(actual.exit_status.is_none(), |expected| {
		actual.exit_status.map_or(false, |actual| expected == actual)
	}) && state.map_or(actual.state.is_none(), |expected| {
		actual.state.map_or(false, |actual| expected == actual)
	}) && event.map_or(actual.event.is_none(), |expected| {
		actual.event.map_or(false, |actual| expected == actual)
	}) && error.as_ref().map_or(actual.error.is_none(), error_compare_fn)
		&& external_command
			.as_ref()
			.map_or(actual.external_command.is_none(), external_command_compare_fn))
	{
		panic!(
			"{}",
			vec![
				"\n",
				"ProcessResult does not match",
				"==========",
				"Expected State:",
				format_process_result(event, state, exit_status, error, external_command).as_str(),
				"Actual:",
				format_process_result(
					actual.event,
					actual.state,
					actual.exit_status,
					&actual.error,
					&actual.external_command
				)
				.as_str(),
				"==========\n"
			]
			.join("\n")
		);
	}
}

#[macro_export]
macro_rules! assert_process_result {
	($actual:expr) => {
		crate::testutil::_assert_process_result(&$actual, None, None, None, &None, &None)
	};
	($actual:expr, error = $error:expr, exit_status = $exit_status:expr) => {
		crate::testutil::_assert_process_result(&$actual, None, None, Some($exit_status), &Some($error), &None)
	};
	($actual:expr, state = $state:expr) => {
		crate::testutil::_assert_process_result(&$actual, None, Some($state), None, &None, &None)
	};
	($actual:expr, state = $state:expr, external_command = $external_command:expr) => {
		crate::testutil::_assert_process_result(&$actual, None, Some($state), None, &None, &Some($external_command))
	};
	($actual:expr, state = $state:expr, error = $error:expr) => {
		crate::testutil::_assert_process_result(&$actual, None, Some($state), None, &Some($error), &None)
	};
	($actual:expr, event = $event:expr) => {
		crate::testutil::_assert_process_result(&$actual, Some($event), None, None, &None, &None)
	};
	($actual:expr, event = $event:expr, state = $state:expr) => {
		crate::testutil::_assert_process_result(&$actual, Some($event), Some($state), None, &None, &None)
	};
	($actual:expr, event = $event:expr, exit_status = $exit_status:expr) => {
		crate::testutil::_assert_process_result(&$actual, Some($event), None, Some($exit_status), &None, &None)
	};
	($actual:expr, event = $event:expr, external_command = $external_command:expr) => {
		crate::testutil::_assert_process_result(&$actual, Some($event), None, None, &None, &Some($external_command))
	};

	($actual:expr, external_command = $external_command:expr) => {
		crate::testutil::_assert_process_result(&$actual, None, None, None, &None, &Some($external_command))
	};
}
