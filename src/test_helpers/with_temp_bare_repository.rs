use crate::{
	git::Repository,
	test_helpers::shared::{create_repository, with_temporary_path},
};

/// Provide a bare repository for testing in a temporary directory.
///
/// # Panics
///
/// If the repository cannot be created for any reason, this function will panic.
pub(crate) fn with_temp_bare_repository<F>(callback: F)
where F: FnOnce(Repository) {
	with_temporary_path(|path| {
		let repo = create_repository(git2::Repository::init_bare(path).unwrap());
		callback(repo);
	});
}
