use std::io;

use thiserror::Error;

/// A display error.
#[derive(Error, Debug)]
#[non_exhaustive]
pub(crate) enum DisplayError {
	/// An unexpected error occurred.
	#[error("Unexpected error")]
	Unexpected(io::Error),
}

impl PartialEq for DisplayError {
	#[allow(clippy::pattern_type_mismatch)]
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Unexpected(self_io_error), Self::Unexpected(other_io_error)) => {
				self_io_error.kind() == other_io_error.kind()
			},
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn partial_eq_io_error_same() {
		assert_eq!(
			DisplayError::Unexpected(io::Error::from(io::ErrorKind::Other)),
			DisplayError::Unexpected(io::Error::from(io::ErrorKind::Other))
		);
	}

	#[test]
	fn partial_eq_io_error_different() {
		assert_ne!(
			DisplayError::Unexpected(io::Error::from(io::ErrorKind::Other)),
			DisplayError::Unexpected(io::Error::from(io::ErrorKind::NotFound))
		);
	}
}
