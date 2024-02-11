//! Git Interactive Rebase Tool - Git crate errors
//!
//! # Description
//! This module contains error types used in the Git crate.

use thiserror::Error;

/// The kind of config error that occurred.
#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum RuntimeError {
	/// An error occurred while attempting to spawn a thread
	#[error("An error occurred while attempting to spawn thread: {0}")]
	ThreadSpawnError(String),
	/// No thread with the given name is registered
	#[error("No thread with name '{0}' is registered")]
	ThreadNotRegistered(String),
	/// A timeout occurred while attempting to wait for a thread
	#[error("A timeout occurred while waiting for thread: '{0}'")]
	ThreadWaitTimeout(String),
	/// An error occurred while sending a message
	#[error("Failed to send message")]
	SendError,
	/// The thread resulted in an error.
	#[error("An error occurred during ")]
	ThreadError(String),
}
