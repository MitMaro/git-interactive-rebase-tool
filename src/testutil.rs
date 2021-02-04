use crate::{process::exit_status::ExitStatus, Exit};

fn format_exit_status(exit: &Result<ExitStatus, Exit>) -> String {
	format!(
		"Result({}, {})",
		exit.as_ref()
			.map_or_else(|_| String::from("None"), |e| { format!("{:?}", e) }),
		exit.as_ref().map_or_else(
			|e| { format!("Exit {{ Message({}), Status({:?}) }}", e.message, e.status) },
			|_| String::from("None")
		)
	)
}

pub fn _assert_exit_status(actual: &Result<ExitStatus, Exit>, expected: &Result<ExitStatus, Exit>) {
	if !match actual.as_ref() {
		Ok(actual_exit_status) => {
			if let Ok(expected_exit_status) = expected.as_ref() {
				actual_exit_status == expected_exit_status
			}
			else {
				false
			}
		},
		Err(actual_exit) => {
			if let Err(expected_exit) = expected.as_ref() {
				actual_exit.status == expected_exit.status && actual_exit.message == expected_exit.message
			}
			else {
				false
			}
		},
	} {
		panic!(vec![
			"\n",
			"Exit result does not match",
			"==========",
			"Expected:",
			format_exit_status(expected).as_str(),
			"Actual:",
			format_exit_status(actual).as_str(),
			"==========\n"
		]
		.join("\n"));
	}
}

#[macro_export]
macro_rules! assert_exit_status {
	($actual:expr, status = $status:expr) => {
		crate::testutil::_assert_exit_status(&$actual, &Ok($status))
	};
	($actual:expr, message = $message:expr, status = $status:expr) => {
		crate::testutil::_assert_exit_status(
			&$actual,
			&Err(Exit {
				message: String::from($message),
				status: $status,
			}),
		)
	};
}
