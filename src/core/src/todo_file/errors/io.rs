use std::{io, path::PathBuf};

use thiserror::Error;

use crate::todo_file::errors::ParseError;

/// The cause of a `FileRead` error
#[derive(Error, Debug)]
#[non_exhaustive]
#[allow(variant_size_differences)]
pub(crate) enum FileReadErrorCause {
	/// Caused by an io error
	#[error(transparent)]
	IoError(#[from] io::Error),
	/// Caused by a parse error
	#[error(transparent)]
	ParseError(#[from] ParseError),
}

impl PartialEq for FileReadErrorCause {
	#[inline]
	#[allow(clippy::pattern_type_mismatch)]
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::IoError(self_err), Self::IoError(other_err)) => self_err.kind() == other_err.kind(),
			(Self::ParseError(self_err), Self::ParseError(other_err)) => self_err == other_err,
			_ => false,
		}
	}
}

/// IO baser errors
#[derive(Error, Debug, PartialEq)]
#[non_exhaustive]
pub(crate) enum IoError {
	/// The file could not be read
	#[error("Unable to read file `{file}`")]
	FileRead {
		/// The file path that failed to read
		file: PathBuf,
		/// The reason for the read error
		cause: FileReadErrorCause,
	},
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn partial_eq_file_read_error_cause_different_cause() {
		assert_ne!(
			FileReadErrorCause::IoError(io::Error::from(io::ErrorKind::Other)),
			FileReadErrorCause::ParseError(ParseError::InvalidAction(String::from("action")))
		);
	}

	#[test]
	fn partial_eq_file_read_error_cause_io_error_same_kind() {
		assert_eq!(
			FileReadErrorCause::IoError(io::Error::from(io::ErrorKind::Other)),
			FileReadErrorCause::IoError(io::Error::from(io::ErrorKind::Other))
		);
	}

	#[test]
	fn partial_eq_file_read_error_cause_io_error_different_kind() {
		assert_ne!(
			FileReadErrorCause::IoError(io::Error::from(io::ErrorKind::Other)),
			FileReadErrorCause::IoError(io::Error::from(io::ErrorKind::NotFound))
		);
	}

	#[test]
	fn partial_eq_file_read_error_cause_different_parse_error() {
		assert_ne!(
			FileReadErrorCause::ParseError(ParseError::InvalidAction(String::from("action"))),
			FileReadErrorCause::ParseError(ParseError::InvalidLine(String::from("line"))),
		);
	}

	#[test]
	fn partial_eq_file_read_error_cause_same_parse_error() {
		assert_eq!(
			FileReadErrorCause::ParseError(ParseError::InvalidAction(String::from("action"))),
			FileReadErrorCause::ParseError(ParseError::InvalidAction(String::from("action"))),
		);
	}
}
