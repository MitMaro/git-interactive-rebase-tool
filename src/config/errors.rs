//! Git Interactive Rebase Tool - Config crate errors
//!
//! # Description
//! This module contains error types used in the Config crate.

mod config_error_cause;
mod invalid_color;

use std::fmt::{Display, Formatter};

use thiserror::Error;

pub(crate) use self::{config_error_cause::ConfigErrorCause, invalid_color::InvalidColorError};

/// Config errors
#[derive(Error, Debug, PartialEq)]
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
pub(crate) struct ConfigError {
	name: String,
	input: Option<String>,
	#[source]
	cause: ConfigErrorCause,
}

impl ConfigError {
	pub(crate) fn new(name: &str, input: &str, cause: ConfigErrorCause) -> Self {
		Self {
			name: String::from(name),
			input: Some(String::from(input)),
			cause,
		}
	}

	pub(crate) fn new_with_optional_input(name: &str, input: Option<String>, cause: ConfigErrorCause) -> Self {
		Self {
			name: String::from(name),
			input,
			cause,
		}
	}

	pub(crate) fn new_read_error(name: &str, cause: ConfigErrorCause) -> Self {
		Self {
			name: String::from(name),
			input: None,
			cause,
		}
	}
}

impl Display for ConfigError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(input) = self.input.as_deref() {
			write!(
				f,
				"Provided value '{input}' is invalid for '{}': {}.",
				self.name, self.cause
			)
		}
		else {
			write!(f, "Provided value is invalid for '{}': {}.", self.name, self.cause)
		}
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_none, assert_some_eq};

	use super::*;

	#[test]
	fn new() {
		let err = ConfigError::new("name", "input", ConfigErrorCause::InvalidBoolean);
		assert_eq!(err.name, "name");
		assert_some_eq!(err.input, "input");
		assert_eq!(err.cause, ConfigErrorCause::InvalidBoolean);
	}

	#[test]
	fn new_with_optional_input_with_input() {
		let err =
			ConfigError::new_with_optional_input("name", Some(String::from("input")), ConfigErrorCause::InvalidBoolean);
		assert_eq!(err.name, "name");
		assert_some_eq!(err.input, "input");
		assert_eq!(err.cause, ConfigErrorCause::InvalidBoolean);
	}

	#[test]
	fn new_with_optional_input_without_input() {
		let err = ConfigError::new_with_optional_input("name", None, ConfigErrorCause::InvalidBoolean);
		assert_eq!(err.name, "name");
		assert_none!(err.input);
		assert_eq!(err.cause, ConfigErrorCause::InvalidBoolean);
	}

	#[test]
	fn new_read_error() {
		let err = ConfigError::new_read_error("name", ConfigErrorCause::InvalidBoolean);
		assert_eq!(err.name, "name");
		assert_none!(err.input);
		assert_eq!(err.cause, ConfigErrorCause::InvalidBoolean);
	}

	#[test]
	fn display_valid_input_with_input() {
		let err = ConfigError::new("name", "input", ConfigErrorCause::InvalidBoolean);

		assert_eq!(
			format!("{err}"),
			"Provided value 'input' is invalid for 'name': The input provided is not a valid boolean value."
		);
	}
	#[test]
	fn display_valid_input_without_input() {
		let err = ConfigError::new_read_error("name", ConfigErrorCause::InvalidBoolean);

		assert_eq!(
			format!("{err}"),
			"Provided value is invalid for 'name': The input provided is not a valid boolean value."
		);
	}
}
