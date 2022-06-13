use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use anyhow::{anyhow, Error, Result};
use git2::{Oid, Signature};
use parking_lot::Mutex;

use crate::{commit_diff_loader::CommitDiffLoader, Commit, CommitDiff, CommitDiffLoaderOptions, Config, Reference};

/// A light cloneable, simple wrapper around the `git2::Repository` struct
#[derive(Clone)]
pub struct Repository {
	repository: Arc<Mutex<git2::Repository>>,
}

impl Repository {
	/// Find and open an existing repository, respecting git environment variables. This will check
	/// for and use `$GIT_DIR`, and if unset will search for a repository starting in the current
	/// directory, walking to the root.
	///
	/// # Errors
	/// Will result in an error if the repository cannot be opened.
	#[inline]
	pub fn open_from_env() -> Result<Self> {
		let repository = git2::Repository::open_from_env()
			.map_err(|e| anyhow!(String::from(e.message())).context("Could not open repository from environment"))?;
		Ok(Self {
			repository: Arc::new(Mutex::new(repository)),
		})
	}

	/// Attempt to open an already-existing repository at `path`.
	///
	/// # Errors
	/// Will result in an error if the repository cannot be opened.
	#[inline]
	pub fn open_from_path(path: &Path) -> Result<Self> {
		let repository = git2::Repository::open(path)
			.map_err(|e| anyhow!(String::from(e.message())).context("Could not open repository from path"))?;
		Ok(Self {
			repository: Arc::new(Mutex::new(repository)),
		})
	}

	/// Load the git configuration for the repository.
	///
	/// # Errors
	/// Will result in an error if the configuration is invalid.
	#[inline]
	pub fn load_config(&self) -> Result<Config> {
		self.repository
			.lock()
			.config()
			.map_err(|e| anyhow!(String::from(e.message())))
	}

	/// Load a diff for a commit hash
	///
	/// # Errors
	/// Will result in an error if the commit cannot be loaded.
	#[inline]
	pub fn load_commit_diff(&self, hash: &str, config: &CommitDiffLoaderOptions) -> Result<CommitDiff> {
		let oid = self.repository.lock().revparse_single(hash)?.id();
		let diff_loader_repository = Arc::clone(&self.repository);
		let loader = CommitDiffLoader::new(diff_loader_repository, config);
		// TODO this is ugly because it assumes one parent
		Ok(loader.load_from_hash(oid).map_err(|e| anyhow!("{}", e))?.remove(0))
	}

	/// Find a reference by the reference name.
	///
	/// # Errors
	/// Will result in an error if the reference cannot be found.
	#[inline]
	pub fn find_reference(&self, reference: &str) -> Result<Reference> {
		let repo = self.repository.lock();
		let git2_reference = repo.find_reference(reference)?;
		Ok(Reference::from(&git2_reference))
	}

	/// Find a commit by a reference name.
	///
	/// # Errors
	/// Will result in an error if the reference cannot be found or is not a commit.
	#[inline]
	pub fn find_commit(&self, reference: &str) -> Result<Commit> {
		let repo = self.repository.lock();
		let git2_reference = repo.find_reference(reference)?;
		Commit::try_from(&git2_reference)
	}

	pub(crate) fn repo_path(&self) -> PathBuf {
		self.repository.lock().path().to_path_buf()
	}

	pub(crate) fn head_id(&self, head_name: &str) -> Result<Oid> {
		let repo = self.repository.lock();
		let ref_name = format!("refs/heads/{}", head_name);
		let revision = repo.revparse_single(ref_name.as_str())?;
		Ok(revision.id())
	}

	pub(crate) fn commit_id_from_ref(&self, reference: &str) -> Result<Oid> {
		let repo = self.repository.lock();
		let commit = repo.find_reference(reference)?.peel_to_commit()?;
		Ok(commit.id())
	}

	pub(crate) fn add_path_to_index(&self, path: &Path) -> Result<()> {
		let repo = self.repository.lock();
		let mut index = repo.index()?;
		index.add_path(path).map_err(Error::from)
	}

	pub(crate) fn remove_path_from_index(&self, path: &Path) -> Result<()> {
		let repo = self.repository.lock();
		let mut index = repo.index()?;
		index.remove_path(path).map_err(Error::from)
	}

	pub(crate) fn create_commit_on_index(
		&self,
		reference: &str,
		author: &Signature<'_>,
		committer: &Signature<'_>,
		message: &str,
	) -> Result<()> {
		let repo = self.repository.lock();
		let tree = repo.find_tree(repo.index()?.write_tree()?)?;
		let head = repo.find_reference(reference)?.peel_to_commit()?;
		let _ = repo.commit(Some("HEAD"), author, committer, message, &tree, &[&head])?;
		Ok(())
	}

	#[cfg(test)]
	pub(crate) fn repository(&self) -> Arc<Mutex<git2::Repository>> {
		self.repository.clone()
	}
}

impl From<git2::Repository> for Repository {
	#[inline]
	fn from(repository: git2::Repository) -> Self {
		Self {
			repository: Arc::new(Mutex::new(repository)),
		}
	}
}

impl ::std::fmt::Debug for Repository {
	#[inline]
	fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
		f.debug_struct("Repository")
			.field("[path]", &self.repository.lock().path())
			.finish()
	}
}

// Paths in Windows makes these tests difficult, so disable
#[cfg(all(unix, test))]
mod tests {
	use std::env::set_var;

	use claim::assert_ok;

	use super::*;
	use crate::testutil::{commit_id_from_ref, create_commit, with_temp_bare_repository, with_temp_repository};

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
		assert_eq!(
			format!("{:#}", Repository::open_from_env().err().unwrap()),
			format!(
				"Could not open repository from environment: failed to resolve path '{}': No such file or directory",
				path.to_str().unwrap()
			)
		);
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
			.join("test")
			.join("fixtures")
			.join("does-not-exist");
		assert_eq!(
			format!("{:#}", Repository::open_from_path(&path).err().unwrap()),
			format!(
				"Could not open repository from path: failed to resolve path '{}': No such file or directory",
				path.to_str().unwrap()
			)
		);
	}

	#[test]
	fn load_config() {
		with_temp_bare_repository(|repo| {
			assert_ok!(repo.load_config());
			Ok(())
		});
	}

	#[test]
	fn load_commit_diff() {
		with_temp_repository(|repository| {
			create_commit(&repository, None);
			let id = commit_id_from_ref(&repository, "refs/heads/main");
			assert_ok!(repository.load_commit_diff(id.to_string().as_str(), &CommitDiffLoaderOptions::new()));
			Ok(())
		});
	}

	#[test]
	fn fmt() {
		with_temp_bare_repository(|repository| {
			let formatted = format!("{:?}", repository);
			let path = repository.repo_path().canonicalize().unwrap();
			assert_eq!(
				formatted,
				format!("Repository {{ [path]: \"{}/\" }}", path.to_str().unwrap())
			);
			Ok(())
		});
	}
}
