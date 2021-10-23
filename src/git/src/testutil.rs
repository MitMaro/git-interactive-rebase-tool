//! Utilities for writing tests that interact with Git.
use std::path::Path;

use tempfile::tempdir;

use crate::Repository;

/// Create a bare repository for testing.
#[must_use]
#[inline]
pub fn create_bare_repository(path: &Path) -> Repository {
	let repo = git2::Repository::init_bare(path).expect("Unable to init a bare repository");
	Repository::from(repo)
}

/// Provide a bare repository for testing in a temporary directory.
#[inline]
pub fn with_temp_bare_repository<F>(callback: F)
where F: FnOnce(Repository) {
	let temp_repository_directory = tempdir().expect("Unable to init a bare repository");
	let path = temp_repository_directory.into_path();
	let repo = create_bare_repository(&path);
	callback(repo);
}
