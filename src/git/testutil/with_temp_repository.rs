#![cfg(not(tarpaulin_include))]

use std::path::Path;

use tempfile::Builder;

use crate::{git::Repository, test_helpers::JAN_2021_EPOCH};

fn with_temporary_path<F>(callback: F)
where F: FnOnce(&Path) {
	let temp_repository_directory = Builder::new().prefix("interactive-rebase-tool").tempdir().unwrap();
	let path = temp_repository_directory.path();
	callback(path);
	temp_repository_directory.close().unwrap();
}

fn create_repository_from_git2_repo(repo: git2::Repository) -> Repository {
	{
		let id = repo.index().unwrap().write_tree().unwrap();
		let tree = repo.find_tree(id).unwrap();
		let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(JAN_2021_EPOCH, 0)).unwrap();
		_ = repo
			.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
			.unwrap();
	};
	Repository::from(repo)
}

/// Provides a new repository instance in a temporary directory for testing that contains an initial
/// empty commit.
///
/// # Panics
///
/// If the repository cannot be created for any reason, this function will panic.
pub(crate) fn with_temp_repository<F>(callback: F)
where F: FnOnce(Repository) {
	with_temporary_path(|path| {
		let mut opts = git2::RepositoryInitOptions::new();
		_ = opts.initial_head("main");
		let repo = create_repository_from_git2_repo(git2::Repository::init_opts(path, &opts).unwrap());
		callback(repo);
	});
}

/// Provide a bare repository for testing in a temporary directory.
///
/// # Panics
///
/// If the repository cannot be created for any reason, this function will panic.
#[allow(clippy::panic)]
pub(crate) fn with_temp_bare_repository<F>(callback: F)
where F: FnOnce(Repository) {
	with_temporary_path(|path| {
		let repo = create_repository_from_git2_repo(git2::Repository::init_bare(path).unwrap());
		callback(repo);
	});
}
