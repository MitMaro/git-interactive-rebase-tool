//! Git Interactive Rebase Tool - Config crate errors
//!
//! # Description
//! This module contains error types used in the Config crate.

mod config_error_cause;
mod invalid_color;

use std::fmt::{Display, Formatter};

use thiserror::Error;

pub use crate::errors::{config_error_cause::ConfigErrorCause, invalid_color::InvalidColorError};

/// Config errors
#[derive(Error, Debug, PartialEq)]
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
pub struct ConfigError {
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
	#[inline]
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
