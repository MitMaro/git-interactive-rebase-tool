//! Git Interactive Rebase Tool - Git Module
//!
//! # Description
//! This module is used to handle working with external Git systems.
//!
//! ## Test Utilities
//! To facilitate testing the usages of this crate, a set of testing utilities are provided. Since
//! these utilities are not tested, and often are optimized for developer experience than
//! performance, they should only be used in test code.

mod errors;
mod repository;

pub(crate) use git2::{Config, ErrorCode};

pub(crate) use self::{
	errors::{GitError, RepositoryLoadKind},
	repository::Repository,
};

/// Find and open an existing repository, respecting git environment variables. This will check
/// for and use `$GIT_DIR`, and if unset will search for a repository starting in the current
/// directory, walking to the root.
///
/// # Errors
/// Will result in an error if the repository cannot be opened.
pub(crate) fn open_repository_from_env() -> Result<git2::Repository, GitError> {
	git2::Repository::open_from_env().map_err(|e| {
		GitError::RepositoryLoad {
			kind: RepositoryLoadKind::Environment,
			cause: e,
		}
	})
}

// Paths in Windows make these tests difficult, so disable
#[cfg(all(unix, test))]
mod tests {
	use claims::assert_ok;
	use git2::ErrorClass;

	use super::*;
	use crate::test_helpers::with_git_directory;

	#[test]
	fn open_repository_from_env_success() {
		with_git_directory("fixtures/simple", |_| {
			assert_ok!(open_repository_from_env());
		});
	}

	#[test]
	fn open_repository_from_env_error() {
		with_git_directory("fixtures/does-not-exist", |path| {
			match open_repository_from_env() {
				Ok(_) => {
					panic!("open_repository_from_env should return error")
				},
				Err(err) => {
					assert_eq!(err, GitError::RepositoryLoad {
						kind: RepositoryLoadKind::Environment,
						cause: git2::Error::new(
							ErrorCode::NotFound,
							ErrorClass::Os,
							format!("failed to resolve path '{path}': No such file or directory")
						),
					});
				},
			}
		});
	}
}
