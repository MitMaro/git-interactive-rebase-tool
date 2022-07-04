use thiserror::Error;

/// Parsing errors
#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
	/// The provided action string is not one of the allowed values
	#[error("The action `{0}` is not valid")]
	InvalidAction(String),
	/// The provided line is not valid
	#[error("The line `{0}` is not valid")]
	InvalidLine(String),
}
