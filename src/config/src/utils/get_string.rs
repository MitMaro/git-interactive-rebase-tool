use git::{Config, ErrorCode};

use crate::{ConfigError, ConfigErrorCause};

pub(crate) fn _get_string(config: Option<&Config>, name: &str) -> Result<Option<String>, ConfigError> {
	let cfg = match config {
		None => return Ok(None),
		Some(c) => c,
	};
	match cfg.get_string(name) {
		Ok(v) => Ok(Some(v)),
		Err(e) if e.code() == ErrorCode::NotFound => Ok(None),
		// detecting a UTF-8 error is tricky
		Err(e) if e.message() == "configuration value is not valid utf8" => {
			Err(ConfigError::new_read_error(name, ConfigErrorCause::InvalidUtf))
		},
		Err(e) => {
			Err(ConfigError::new_read_error(
				name,
				ConfigErrorCause::UnknownError(String::from(e.message())),
			))
		},
	}
}

pub(crate) fn get_string(config: Option<&Config>, name: &str, default: &str) -> Result<String, ConfigError> {
	Ok(_get_string(config, name)?.unwrap_or_else(|| String::from(default)))
}

#[cfg(test)]
mod tests {
	use claim::assert_ok_eq;
	use testutils::assert_err_eq;

	use super::*;
	use crate::testutils::{invalid_utf, with_git_config};

	#[test]
	fn read_value() {
		with_git_config(&["[test]", "value = foo"], |git_config| {
			assert_ok_eq!(
				get_string(Some(&git_config), "test.value", "default"),
				String::from("foo")
			);
		});
	}

	#[test]
	fn read_default() {
		with_git_config(&[], |git_config| {
			assert_ok_eq!(
				get_string(Some(&git_config), "test.value", "default"),
				String::from("default")
			);
		});
	}

	#[test]
	fn read_unexpected_error() {
		with_git_config(&["[test]", "value = invalid"], |git_config| {
			assert_err_eq!(
				get_string(Some(&git_config), "test", "default"),
				ConfigError::new_read_error(
					"test",
					ConfigErrorCause::UnknownError(String::from("invalid config item name 'test'"))
				)
			);
		});
	}

	#[test]
	fn read_invalid_non_utf() {
		with_git_config(
			&["[test]", format!("value = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_err_eq!(
					get_string(Some(&git_config), "test.value", "default"),
					ConfigError::new_read_error("test.value", ConfigErrorCause::InvalidUtf)
				);
			},
		);
	}
}
