use git2::Repository;

use crate::test_helpers::shared::{create_repository, with_temporary_path};

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
		let repo = Repository::init_opts(path, &opts).unwrap();
		create_repository(&repo);
		callback(repo);
	});
}
