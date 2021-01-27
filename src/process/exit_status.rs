#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExitStatus {
	Abort,
	ConfigError,
	FileReadError,
	FileWriteError,
	Good,
	StateError,
	Kill,
}

impl ExitStatus {
	pub const fn to_code(self) -> i32 {
		match self {
			Self::Abort => 5,
			Self::ConfigError => 1,
			Self::FileReadError => 2,
			Self::FileWriteError => 3,
			Self::Good => 0,
			Self::StateError => 4,
			Self::Kill => 6,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[rstest(
		input,
		expected,
		case::abort(ExitStatus::Abort, 5),
		case::config_error(ExitStatus::ConfigError, 1),
		case::file_read_error(ExitStatus::FileReadError, 2),
		case::file_write_error(ExitStatus::FileWriteError, 3),
		case::good(ExitStatus::Good, 0),
		case::state_error(ExitStatus::StateError, 4),
		case::kill(ExitStatus::Kill, 6)
	)]
	fn to_code(input: ExitStatus, expected: i32) {
		assert_eq!(ExitStatus::to_code(input), expected);
	}
}
