use std::path::Path;

use tempfile::Builder;

pub(crate) fn with_temporary_path<F>(callback: F)
where F: FnOnce(&Path) {
	let temp_repository_directory = Builder::new().prefix("interactive-rebase-tool").tempdir().unwrap();
	let path = temp_repository_directory.path();
	callback(path);
	temp_repository_directory.close().unwrap();
}
