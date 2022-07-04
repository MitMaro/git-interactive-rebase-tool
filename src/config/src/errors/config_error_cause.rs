use git::errors::GitError;
use thiserror::Error;

use crate::errors::InvalidColorError;

/// The kind of config error that occurred.
#[derive(Error, Debug, PartialEq)]
#[non_exhaustive]
#[allow(variant_size_differences)]
pub enum ConfigErrorCause {
	/// The input provided is not a valid color
	#[error(transparent)]
	InvalidColor(InvalidColorError),
	/// An error occurred reading the git config files.
	#[error(transparent)]
	GitError(GitError),
	/// The input provided is not a valid value for the show whitespace value.
	#[error("Must match one of 'true', 'on', 'both', 'trailing', 'leading', 'false', 'off' or 'none'.")]
	InvalidShowWhitespace,
	/// The input provided is not a valid value for the ignore whitespace value.
	#[error("Must match one of 'true', 'on', 'all', 'change', 'false', 'off' or 'none'")]
	InvalidDiffIgnoreWhitespace,
	/// The input provided is not a valid value for the diff renames.
	#[error("Must match one of 'true', 'false', 'copy', or 'copies'")]
	InvalidDiffRenames,
	/// The input provided is not a valid boolean value.
	#[error("The input provided is not a valid boolean value")]
	InvalidBoolean,
	/// The input provided is outside of valid range for an unsigned 32-bit integer.
	#[error("The input provided is outside of valid range for an unsigned 32-bit integer")]
	InvalidUnsignedInteger,
	/// The input provided is not a valid input keybinding.
	#[error("The input provided is not a valid input keybinding.")]
	InvalidKeyBinding,
	/// The input provided is not valid UTF.
	#[error("The input provided is not valid UTF")]
	InvalidUtf,
	/// The input provided resulted in an unknown error variant: {0}.
	#[error("The input provided resulted in an unknown error variant")]
	UnknownError(String),
}
