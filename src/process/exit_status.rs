#[derive(Clone, Copy, Debug)]
pub enum ExitStatus {
	ConfigError,
	FileReadError,
	FileWriteError,
	Good,
	StateError,
}

impl ExitStatus {
	pub fn to_code(self) -> i32 {
		match self {
			Self::ConfigError => 1,
			Self::FileReadError => 2,
			Self::FileWriteError => 3,
			Self::Good => 0,
			Self::StateError => 4,
		}
	}
}
