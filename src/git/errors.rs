//! Git Interactive Rebase Tool - Git crate errors
//!
//! # Description
//! This module contains error types used in the Git crate.

use std::fmt::{Display, Formatter};

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
#[allow(clippy::enum_variant_names)]
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
	/// The configuration could not be loaded
	#[cfg(test)]
	#[error("Could not load configuration")]
	ReferenceNotFound {
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
}
