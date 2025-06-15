use std::fmt::{Debug, Formatter};

use git2::Repository;

use crate::git::{Config, GitError};

pub(crate) struct ConfigLoader {
	repository: Repository,
	overrides: Vec<(String, String)>,
}

impl ConfigLoader {
	#[cfg(test)]
	pub(crate) fn new(repository: Repository) -> Self {
		let overrides = Vec::new();
		Self { repository, overrides }
	}

	pub(crate) fn with_overrides(repository: Repository, overrides: Vec<(String, String)>) -> Self {
		Self { repository, overrides }
	}

	/// Load the git configuration for the repository,
	/// with any overrides taking priority over the values defined in the repositroy
	///
	/// # Errors
	/// Will result in an error if the configuration is invalid.
	pub(crate) fn load_config(&self) -> Result<Config, GitError> {
		let into_git_error = |cause| GitError::ConfigLoad { cause };

		let mut config = self.repository.config().map_err(into_git_error)?;
		for (name, value) in &self.overrides {
			if value.is_empty() {
				config.set_bool(name, true).map_err(into_git_error)?;
			} else {
				config.set_str(name, value).map_err(into_git_error)?;
			}
		}
		Ok(config)
	}

	pub(crate) fn eject_repository(self) -> Repository {
		self.repository
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
			let config = ConfigLoader::new(repository);
			assert_ok!(config.load_config());
		});
	}

	#[test]
	fn fmt() {
		with_temp_repository(|repository| {
			let path = repository.path().canonicalize().unwrap();
			let loader = ConfigLoader::new(repository);
			let formatted = format!("{loader:?}");
			assert_eq!(
				formatted,
				format!("ConfigLoader {{ [path]: \"{}/\" }}", path.to_str().unwrap())
			);
		});
	}
}
