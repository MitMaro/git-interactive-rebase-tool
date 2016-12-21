#[derive(Clone, Copy, Debug)]
pub(crate) enum ExitStatus {
	ConfigError,
	FileReadError,
	FileWriteError,
	Good,
	StateError,
}

impl ExitStatus {
	pub(crate) fn to_code(self) -> i32 {
		match self {
			ExitStatus::ConfigError => 1,
			ExitStatus::FileReadError => 2,
			ExitStatus::FileWriteError => 3,
			ExitStatus::Good => 0,
			ExitStatus::StateError => 4,
		}
	}
}
