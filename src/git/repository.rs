use std::{
	fmt::{Debug, Formatter},
	path::{Path, PathBuf},
	sync::Arc,
};

use git2::{Oid, Signature};
use parking_lot::Mutex;

use crate::git::{
	Commit,
	CommitDiff,
	CommitDiffLoader,
	CommitDiffLoaderOptions,
	Config,
	GitError,
	Reference,
	RepositoryLoadKind,
};

/// A light cloneable, simple wrapper around the `git2::Repository` struct
#[derive(Clone)]
pub(crate) struct Repository {
	repository: Arc<Mutex<git2::Repository>>,
}

impl Repository {
	/// Find and open an existing repository, respecting git environment variables. This will check
	/// for and use `$GIT_DIR`, and if unset will search for a repository starting in the current
	/// directory, walking to the root.
	///
	/// # Errors
	/// Will result in an error if the repository cannot be opened.
	pub(crate) fn open_from_env() -> Result<Self, GitError> {
		let repository = git2::Repository::open_from_env().map_err(|e| {
			GitError::RepositoryLoad {
				kind: RepositoryLoadKind::Environment,
				cause: e,
			}
		})?;
		Ok(Self {
			repository: Arc::new(Mutex::new(repository)),
		})
	}

	/// Attempt to open an already-existing repository at `path`.
	///
	/// # Errors
	/// Will result in an error if the repository cannot be opened.
	pub(crate) fn open_from_path(path: &Path) -> Result<Self, GitError> {
		let repository = git2::Repository::open(path).map_err(|e| {
			GitError::RepositoryLoad {
				kind: RepositoryLoadKind::Path,
				cause: e,
			}
		})?;
		Ok(Self {
			repository: Arc::new(Mutex::new(repository)),
		})
	}

	/// Load the git configuration for the repository.
	///
	/// # Errors
	/// Will result in an error if the configuration is invalid.
	pub(crate) fn load_config(&self) -> Result<Config, GitError> {
		self.repository
			.lock()
			.config()
			.map_err(|e| GitError::ConfigLoad { cause: e })
	}

	/// Load a diff for a commit hash
	///
	/// # Errors
	/// Will result in an error if the commit cannot be loaded.
	pub(crate) fn load_commit_diff(
		&self,
		hash: &str,
		config: &CommitDiffLoaderOptions,
	) -> Result<CommitDiff, GitError> {
		let oid = self
			.repository
			.lock()
			.revparse_single(hash)
			.map_err(|e| GitError::CommitLoad { cause: e })?
			.id();
		let diff_loader_repository = Arc::clone(&self.repository);
		let loader = CommitDiffLoader::new(diff_loader_repository, config);
		// TODO this is ugly because it assumes one parent
		Ok(loader
			.load_from_hash(oid)
			.map_err(|e| GitError::CommitLoad { cause: e })?
			.remove(0))
	}

	/// Find a reference by the reference name.
	///
	/// # Errors
	/// Will result in an error if the reference cannot be found.
	pub(crate) fn find_reference(&self, reference: &str) -> Result<Reference, GitError> {
		let repo = self.repository.lock();
		let git2_reference = repo
			.find_reference(reference)
			.map_err(|e| GitError::ReferenceNotFound { cause: e })?;
		Ok(Reference::from(&git2_reference))
	}

	/// Find a commit by a reference name.
	///
	/// # Errors
	/// Will result in an error if the reference cannot be found or is not a commit.
	pub(crate) fn find_commit(&self, reference: &str) -> Result<Commit, GitError> {
		let repo = self.repository.lock();
		let git2_reference = repo
			.find_reference(reference)
			.map_err(|e| GitError::ReferenceNotFound { cause: e })?;
		Commit::try_from(&git2_reference)
	}

	pub(crate) fn repo_path(&self) -> PathBuf {
		self.repository.lock().path().to_path_buf()
	}

	pub(crate) fn head_id(&self, head_name: &str) -> Result<Oid, git2::Error> {
		let repo = self.repository.lock();
		let ref_name = format!("refs/heads/{head_name}");
		let revision = repo.revparse_single(ref_name.as_str())?;
		Ok(revision.id())
	}

	pub(crate) fn commit_id_from_ref(&self, reference: &str) -> Result<Oid, git2::Error> {
		let repo = self.repository.lock();
		let commit = repo.find_reference(reference)?.peel_to_commit()?;
		Ok(commit.id())
	}

	pub(crate) fn add_path_to_index(&self, path: &Path) -> Result<(), git2::Error> {
		let repo = self.repository.lock();
		let mut index = repo.index()?;
		index.add_path(path)
	}

	pub(crate) fn remove_path_from_index(&self, path: &Path) -> Result<(), git2::Error> {
		let repo = self.repository.lock();
		let mut index = repo.index()?;
		index.remove_path(path)
	}

	pub(crate) fn create_commit_on_index(
		&self,
		reference: &str,
		author: &Signature<'_>,
		committer: &Signature<'_>,
		message: &str,
	) -> Result<(), git2::Error> {
		let repo = self.repository.lock();
		let tree = repo.find_tree(repo.index()?.write_tree()?)?;
		let head = repo.find_reference(reference)?.peel_to_commit()?;
		_ = repo.commit(Some("HEAD"), author, committer, message, &tree, &[&head])?;
		Ok(())
	}

	#[cfg(test)]
	pub(crate) fn repository(&self) -> Arc<Mutex<git2::Repository>> {
		Arc::clone(&self.repository)
	}
}

impl From<git2::Repository> for Repository {
	fn from(repository: git2::Repository) -> Self {
		Self {
			repository: Arc::new(Mutex::new(repository)),
		}
	}
}

impl Debug for Repository {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.debug_struct("Repository")
			.field("[path]", &self.repository.lock().path())
			.finish()
	}
}

// Paths in Windows makes these tests difficult, so disable
#[cfg(all(unix, test))]
mod tests {
	use std::env::set_var;

	use claims::assert_ok;
	use git2::{ErrorClass, ErrorCode};
	use testutils::assert_err_eq;

	use super::*;
	use crate::{
		git::testutil::{with_temp_bare_repository, with_temp_repository},
		test_helpers::create_commit,
	};

	#[test]
	#[serial_test::serial]
	fn open_from_env() {
		let path = Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple");
		set_var("GIT_DIR", path.to_str().unwrap());
		assert_ok!(Repository::open_from_env());
	}

	#[test]
	#[serial_test::serial]
	fn open_from_env_error() {
		let path = Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("does-not-exist");
		set_var("GIT_DIR", path.to_str().unwrap());
		assert_err_eq!(Repository::open_from_env(), GitError::RepositoryLoad {
			kind: RepositoryLoadKind::Environment,
			cause: git2::Error::new(
				ErrorCode::NotFound,
				ErrorClass::Os,
				format!(
					"failed to resolve path '{}': No such file or directory",
					path.to_string_lossy()
				)
			),
		});
	}

	#[test]
	fn open_from_path() {
		let path = Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple");
		assert_ok!(Repository::open_from_path(&path));
	}

	#[test]
	fn open_from_path_error() {
		let path = Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("..")
			.join("..")
			.join("test")
			.join("fixtures")
			.join("does-not-exist");
		assert_err_eq!(Repository::open_from_path(&path), GitError::RepositoryLoad {
			kind: RepositoryLoadKind::Path,
			cause: git2::Error::new(
				ErrorCode::NotFound,
				ErrorClass::Os,
				format!(
					"failed to resolve path '{}': No such file or directory",
					path.to_string_lossy()
				)
			),
		});
	}

	#[test]
	fn load_config() {
		with_temp_bare_repository(|repo| {
			assert_ok!(repo.load_config());
		});
	}

	#[test]
	fn load_commit_diff() {
		with_temp_repository(|repository| {
			create_commit(&repository, None);
			let id = repository.commit_id_from_ref("refs/heads/main").unwrap();
			assert_ok!(repository.load_commit_diff(id.to_string().as_str(), &CommitDiffLoaderOptions::new()));
		});
	}

	#[test]
	fn load_commit_diff_with_non_commit() {
		with_temp_repository(|repository| {
			let blob_ref = {
				let git2_repository = repository.repository();
				let git2_lock = git2_repository.lock();
				let blob = git2_lock.blob(b"foo").unwrap();
				_ = git2_lock.reference("refs/blob", blob, false, "blob").unwrap();
				blob.to_string()
			};

			assert_err_eq!(
				repository.load_commit_diff(blob_ref.as_str(), &CommitDiffLoaderOptions::new()),
				GitError::CommitLoad {
					cause: git2::Error::new(
						ErrorCode::NotFound,
						ErrorClass::Invalid,
						"the requested type does not match the type in the ODB",
					),
				}
			);
		});
	}

	#[test]
	fn find_reference() {
		with_temp_repository(|repository| {
			assert_ok!(repository.find_reference("refs/heads/main"));
		});
	}

	#[test]
	fn find_reference_error() {
		with_temp_repository(|repository| {
			assert_err_eq!(
				repository.find_reference("refs/heads/invalid"),
				GitError::ReferenceNotFound {
					cause: git2::Error::new(
						ErrorCode::NotFound,
						ErrorClass::Reference,
						"reference 'refs/heads/invalid' not found",
					),
				}
			);
		});
	}

	#[test]
	fn find_commit() {
		with_temp_repository(|repository| {
			assert_ok!(repository.find_commit("refs/heads/main"));
		});
	}

	#[test]
	fn find_commit_error() {
		with_temp_repository(|repository| {
			assert_err_eq!(
				repository.find_commit("refs/heads/invalid"),
				GitError::ReferenceNotFound {
					cause: git2::Error::new(
						ErrorCode::NotFound,
						ErrorClass::Reference,
						"reference 'refs/heads/invalid' not found",
					),
				}
			);
		});
	}

	#[test]
	fn fmt() {
		with_temp_bare_repository(|repository| {
			let formatted = format!("{repository:?}");
			let path = repository.repo_path().canonicalize().unwrap();
			assert_eq!(
				formatted,
				format!("Repository {{ [path]: \"{}/\" }}", path.to_str().unwrap())
			);
		});
	}
}
