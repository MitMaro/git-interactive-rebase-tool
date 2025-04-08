//! Git Interactive Rebase Tool - Git crate errors
//!
//! # Description
//! This module contains error types used in the Git crate.

use std::fmt::{Display, Formatter};

use git2::ErrorCode;
use thiserror::Error;

/// The kind of repository load kind.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum RepositoryLoadKind {
	/// Repository was loaded from the path provided through an environment variable
	Environment,
	#[cfg(test)]
	/// Repository was loaded from a direct path
	Path,
}

impl Display for RepositoryLoadKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::Environment => write!(f, "environment"),
			#[cfg(test)]
			Self::Path => write!(f, "path"),
		}
	}
}

/// Git errors
#[derive(Error, Debug, PartialEq)]
#[non_exhaustive]
#[expect(clippy::enum_variant_names, reason = "'Load' postfix is by chance")]
pub(crate) enum GitError {
	/// The repository could not be loaded
	#[error("Could not open repository from {kind}")]
	RepositoryLoad {
		/// The kind of load error.
		kind: RepositoryLoadKind,
		/// The internal cause of the load error.
		#[source]
		cause: git2::Error,
	},
	/// The configuration could not be loaded
	#[error("Could not load configuration")]
	ConfigLoad {
		/// The internal cause of the load error.
		#[source]
		cause: git2::Error,
	},
	/// The commit could not be loaded
	#[error("Could not load commit")]
	CommitLoad {
		/// The internal cause of the load error.
		#[source]
		cause: git2::Error,
	},
	/// The diff could not be loaded
	#[error("Could not load diff")]
	DiffLoad {
		/// The internal cause of the load error.
		#[source]
		cause: git2::Error,
	},
}

impl GitError {
	pub(crate) fn code(&self) -> ErrorCode {
		match self {
			GitError::RepositoryLoad { cause, .. }
			| GitError::ConfigLoad { cause, .. }
			| GitError::CommitLoad { cause, .. }
			| GitError::DiffLoad { cause, .. } => cause.code(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn repository_load_kind_display_environment() {
		assert_eq!(format!("{}", RepositoryLoadKind::Environment), "environment");
	}

	#[test]
	fn repository_load_kind_display_path() {
		assert_eq!(format!("{}", RepositoryLoadKind::Path), "path");
	}

	#[test]
	fn code_repository_load() {
		let error = GitError::RepositoryLoad {
			kind: RepositoryLoadKind::Environment,
			cause: git2::Error::from_str("main"),
		};
		assert_eq!(error.code(), ErrorCode::GenericError);
	}

	#[test]
	fn code_config_load() {
		let error = GitError::ConfigLoad {
			cause: git2::Error::from_str("main"),
		};
		assert_eq!(error.code(), ErrorCode::GenericError);
	}

	#[test]
	fn code_commit_load() {
		let error = GitError::CommitLoad {
			cause: git2::Error::from_str("main"),
		};
		assert_eq!(error.code(), ErrorCode::GenericError);
	}

	#[test]
	fn code_diff_load() {
		let error = GitError::DiffLoad {
			cause: git2::Error::from_str("main"),
		};
		assert_eq!(error.code(), ErrorCode::GenericError);
	}
}
