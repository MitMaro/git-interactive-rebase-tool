use std::time::Duration;

use crate::view::{RenderAction, State, ViewAction};

fn assert_view_state_actions(state: &State, expected_actions: &[String]) {
	let actions = state
		.render_slice()
		.lock()
		.get_actions()
		.iter()
		.map(|a| {
			match *a {
				RenderAction::ScrollDown => String::from("ScrollDown"),
				RenderAction::ScrollUp => String::from("ScrollUp"),
				RenderAction::ScrollRight => String::from("ScrollRight"),
				RenderAction::ScrollLeft => String::from("ScrollLeft"),
				RenderAction::ScrollTop => String::from("ScrollTop"),
				RenderAction::ScrollBottom => String::from("ScrollBottom"),
				RenderAction::PageUp => String::from("PageUp"),
				RenderAction::PageDown => String::from("PageDown"),
				RenderAction::Resize(width, height) => format!("Resize({width}, {height})"),
			}
		})
		.collect::<Vec<String>>();

	let mut mismatch = false;
	let mut error_output = vec![
		String::from("\nUnexpected actions!"),
		String::from("--- Expected"),
		String::from("+++ Actual"),
		String::from("=========="),
	];

	for (expected_action, actual_action) in expected_actions.iter().zip(actions.iter()) {
		if expected_action == actual_action {
			error_output.push(format!(" {expected_action}"));
		}
		else {
			mismatch = true;
			error_output.push(format!("-{expected_action}"));
			error_output.push(format!("+{actual_action}"));
		}
	}

	match expected_actions.len() {
		a if a > actions.len() => {
			mismatch = true;
			for action in expected_actions.iter().skip(actions.len()) {
				error_output.push(format!("-{action}"));
			}
		},
		a if a < actions.len() => {
			mismatch = true;
			for action in actions.iter().skip(expected_actions.len()) {
				error_output.push(format!("+{action}"));
			}
		},
		_ => {},
	}

	if mismatch {
		error_output.push(String::from("==========\n"));
		panic!("{}", error_output.join("\n"));
	}
}

fn action_to_string(action: ViewAction) -> String {
	String::from(match action {
		ViewAction::Stop => "Stop",
		ViewAction::Refresh => "Refresh",
		ViewAction::Render => "Render",
		ViewAction::Start => "Start",
		ViewAction::End => "End",
	})
}

/// Context for a view state test.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct ViewStateTestContext {
	/// The state instance.
	pub(crate) state: State,
}

impl ViewStateTestContext {
	/// Assert that render actions were sent.
	pub(crate) fn assert_render_action(&self, actions: &[&str]) {
		assert_view_state_actions(
			&self.state,
			actions
				.iter()
				.map(|s| String::from(*s))
				.collect::<Vec<String>>()
				.as_slice(),
		);
	}

	/// Assert that certain messages were sent by the `State`.
	pub(crate) fn assert_sent_messages(&self, messages: Vec<&str>) {
		let mut mismatch = false;
		let mut error_output = vec![
			String::from("\nUnexpected messages!"),
			String::from("--- Expected"),
			String::from("+++ Actual"),
			String::from("=========="),
		];

		let update_receiver = self.state.update_receiver();
		for message in messages {
			if let Ok(action) = update_receiver.recv_timeout(Duration::new(1, 0)) {
				let action_name = action_to_string(action);
				if message == action_name {
					error_output.push(format!(" {message}"));
				}
				else {
					mismatch = true;
					error_output.push(format!("-{message}"));
					error_output.push(format!("+{action_name}"));
				}
			}
			else {
				error_output.push(format!("-{message}"));
			}
		}

		// wait some time for any other actions that were sent that should have not been
		while let Ok(action) = update_receiver.recv_timeout(Duration::new(0, 10000)) {
			mismatch = true;
			error_output.push(format!("+{}", action_to_string(action)));
		}

		if mismatch {
			error_output.push(String::from("==========\n"));
			panic!("{}", error_output.join("\n"));
		}
	}
}

/// Provide a `State` instance for use within a view test.
pub(crate) fn with_view_state<C>(callback: C)
where C: FnOnce(ViewStateTestContext) {
	callback(ViewStateTestContext { state: State::new() });
}
