use std::fmt::{Debug, Formatter};

use crate::{
	diff::{CommitDiff, CommitDiffLoader, CommitDiffLoaderOptions},
	git::GitError,
};

/// A light cloneable, simple wrapper around the `git2::Repository` struct
pub(crate) struct Repository {
	repository: git2::Repository,
}

impl Repository {
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
			.revparse_single(hash)
			.map_err(|e| GitError::CommitLoad { cause: e })?
			.id();
		let loader = CommitDiffLoader::new(&self.repository, config);

		loader
			.load_from_hash(oid)
			.map_err(|e| GitError::CommitLoad { cause: e })
	}
}

impl From<git2::Repository> for Repository {
	fn from(repository: git2::Repository) -> Self {
		Self { repository }
	}
}

impl Debug for Repository {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.debug_struct("Repository")
			.field("[path]", &self.repository.path())
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use std::path::{Path, PathBuf};

	use git2::{Oid, Signature};

	use crate::{
		diff::{Commit, Reference},
		git::{GitError, Repository},
	};

	impl Repository {
		/// Find a reference by the reference name.
		///
		/// # Errors
		/// Will result in an error if the reference cannot be found.
		pub(crate) fn find_reference(&self, reference: &str) -> Result<Reference, GitError> {
			let git2_reference = self
				.repository
				.find_reference(reference)
				.map_err(|e| GitError::ReferenceNotFound { cause: e })?;
			Ok(Reference::from(&git2_reference))
		}

		/// Find a commit by a reference name.
		///
		/// # Errors
		/// Will result in an error if the reference cannot be found or is not a commit.
		pub(crate) fn find_commit(&self, reference: &str) -> Result<Commit, GitError> {
			let reference = self
				.repository
				.find_reference(reference)
				.map_err(|e| GitError::ReferenceNotFound { cause: e })?;
			Commit::try_from(&reference)
		}

		pub(crate) fn repo_path(&self) -> PathBuf {
			self.repository.path().to_path_buf()
		}

		pub(crate) fn head_id(&self, head_name: &str) -> Result<Oid, git2::Error> {
			let ref_name = format!("refs/heads/{head_name}");
			let revision = self.repository.revparse_single(ref_name.as_str())?;
			Ok(revision.id())
		}

		pub(crate) fn commit_id_from_ref(&self, reference: &str) -> Result<Oid, git2::Error> {
			let commit = self.repository.find_reference(reference)?.peel_to_commit()?;
			Ok(commit.id())
		}

		pub(crate) fn add_path_to_index(&self, path: &Path) -> Result<(), git2::Error> {
			let mut index = self.repository.index()?;
			index.add_path(path)
		}

		pub(crate) fn remove_path_from_index(&self, path: &Path) -> Result<(), git2::Error> {
			let mut index = self.repository.index()?;
			index.remove_path(path)
		}

		pub(crate) fn create_commit_on_index(
			&self,
			reference: &str,
			author: &Signature<'_>,
			committer: &Signature<'_>,
			message: &str,
		) -> Result<(), git2::Error> {
			let tree = self.repository.find_tree(self.repository.index()?.write_tree()?)?;
			let head = self.repository.find_reference(reference)?.peel_to_commit()?;
			_ = self
				.repository
				.commit(Some("HEAD"), author, committer, message, &tree, &[&head])?;
			Ok(())
		}

		pub(crate) fn repository(&self) -> &git2::Repository {
			&self.repository
		}
	}
}

// Paths in Windows make these tests difficult, so disable
#[cfg(all(unix, test))]
mod unix_tests {
	use claims::{assert_err_eq, assert_ok};
	use git2::{ErrorClass, ErrorCode};

	use super::*;
	use crate::test_helpers::{create_commit, with_temp_repository};

	#[test]
	fn load_commit_diff() {
		with_temp_repository(|repo| {
			let repository = Repository::from(repo);
			create_commit(&repository, None);
			let id = repository.commit_id_from_ref("refs/heads/main").unwrap();
			assert_ok!(repository.load_commit_diff(id.to_string().as_str(), &CommitDiffLoaderOptions::new()));
		});
	}

	#[test]
	fn load_commit_diff_with_non_commit() {
		with_temp_repository(|repo| {
			let blob_ref = {
				let blob = repo.blob(b"foo").unwrap();
				_ = repo.reference("refs/blob", blob, false, "blob").unwrap();
				blob.to_string()
			};
			let repository = Repository::from(repo);

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
	fn fmt() {
		with_temp_repository(|repo| {
			let repository = Repository::from(repo);
			let formatted = format!("{repository:?}");
			let path = repository.repo_path().canonicalize().unwrap();
			assert_eq!(
				formatted,
				format!("Repository {{ [path]: \"{}/\" }}", path.to_str().unwrap())
			);
		});
	}
}
