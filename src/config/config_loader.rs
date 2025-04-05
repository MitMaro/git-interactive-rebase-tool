use std::fmt::{Debug, Formatter};

use git2::Repository;

use crate::git::{Config, GitError};

pub(crate) struct ConfigLoader {
	repository: Repository,
}

impl ConfigLoader {
	/// Load the git configuration for the repository.
	///
	/// # Errors
	/// Will result in an error if the configuration is invalid.
	pub(crate) fn load_config(&self) -> Result<Config, GitError> {
		self.repository.config().map_err(|e| GitError::ConfigLoad { cause: e })
	}

	pub(crate) fn eject_repository(self) -> Repository {
		self.repository
	}
}

impl From<Repository> for ConfigLoader {
	fn from(repository: Repository) -> Self {
		Self { repository }
	}
}

impl Debug for ConfigLoader {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.debug_struct("ConfigLoader")
			.field("[path]", &self.repository.path())
			.finish()
	}
}

// Paths in Windows make these tests difficult, so disable
#[cfg(all(unix, test))]
mod unix_tests {
	use claims::assert_ok;

	use crate::{
		config::ConfigLoader,
		test_helpers::{with_temp_bare_repository, with_temp_repository},
	};

	#[test]
	fn load_config() {
		with_temp_bare_repository(|repository| {
			let config = ConfigLoader::from(repository);
			assert_ok!(config.load_config());
		});
	}

	#[test]
	fn fmt() {
		with_temp_repository(|repository| {
			let path = repository.path().canonicalize().unwrap();
			let loader = ConfigLoader::from(repository);
			let formatted = format!("{loader:?}");
			assert_eq!(
				formatted,
				format!("ConfigLoader {{ [path]: \"{}/\" }}", path.to_str().unwrap())
			);
		});
	}
}
