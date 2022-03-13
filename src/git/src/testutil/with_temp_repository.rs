#![cfg(not(tarpaulin_include))]

use std::path::Path;

use anyhow::Result;
use tempfile::Builder;

use crate::{testutil::JAN_2021_EPOCH, Repository};

pub(crate) fn with_temporary_path<F>(callback: F)
where F: FnOnce(&Path) {
	let temp_repository_directory = Builder::new()
		.prefix("interactive-rebase-tool")
		.tempdir()
		.expect("Unable to init a bare repository");
	let path = temp_repository_directory.path();
	callback(path);
	temp_repository_directory
		.close()
		.expect("Unable to close temporary path");
}

/// Provides a new repository instance in a temporary directory for testing that contains an initial
/// empty commit.
///
/// # Panics
///
/// If the repository cannot be created for any reason, this function will panic.
#[allow(clippy::panic, clippy::unwrap_used)]
#[inline]
pub fn with_temp_repository<F>(callback: F)
where F: FnOnce(Repository) -> Result<()> {
	with_temporary_path(|path| {
		let mut opts = git2::RepositoryInitOptions::new();
		let _ = opts.initial_head("main");
		let repo = git2::Repository::init_opts(path, &opts).unwrap();

		{
			let id = repo.index().unwrap().write_tree().unwrap();
			let tree = repo.find_tree(id).unwrap();
			let sig = git2::Signature::new("name", "name@example.com", &git2::Time::new(JAN_2021_EPOCH, 0)).unwrap();
			let _ = repo
				.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
				.unwrap();
		}
		if let Err(e) = callback(Repository::from(repo)) {
			eprintln!("{:?}", e);
			panic!("{} failed with {}", stringify!(e), e)
		}
	});
}

/// Provide a bare repository for testing in a temporary directory.
///
/// # Panics
///
/// If the repository cannot be created for any reason, this function will panic.
#[inline]
#[allow(clippy::panic)]
pub fn with_temp_bare_repository<F>(callback: F)
where F: FnOnce(Repository) -> Result<()> {
	with_temporary_path(|path| {
		let git2_repository = git2::Repository::init_bare(path).expect("Unable to init a bare repository");
		let repository = Repository::from(git2_repository);
		if let Err(e) = callback(repository) {
			panic!("{} failed with {}", stringify!(e), e)
		};
	});
}
