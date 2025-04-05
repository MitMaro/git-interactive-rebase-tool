use git2::Repository;

use crate::test_helpers::shared::{create_repository, with_temporary_path};

/// Provide a bare repository for testing in a temporary directory.
///
/// # Panics
///
/// If the repository cannot be created for any reason, this function will panic.
pub(crate) fn with_temp_bare_repository<F>(callback: F)
where F: FnOnce(Repository) {
	with_temporary_path(|path| {
		let repo = Repository::init_bare(path).unwrap();
		create_repository(&repo);
		callback(repo);
	});
}
