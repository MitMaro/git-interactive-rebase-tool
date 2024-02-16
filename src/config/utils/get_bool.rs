use crate::{
	config::{utils::get_optional_string, ConfigError, ConfigErrorCause},
	git::{Config, ErrorCode},
};

pub(crate) fn get_bool(config: Option<&Config>, name: &str, default: bool) -> Result<bool, ConfigError> {
	if let Some(cfg) = config {
		match cfg.get_bool(name) {
			Ok(v) => Ok(v),
			Err(e) if e.code() == ErrorCode::NotFound => Ok(default),
			Err(e) if e.message().contains("failed to parse") => {
				Err(ConfigError::new_with_optional_input(
					name,
					get_optional_string(config, name).ok().flatten(),
					ConfigErrorCause::InvalidBoolean,
				))
			},
			Err(e) => {
				Err(ConfigError::new_with_optional_input(
					name,
					get_optional_string(config, name).ok().flatten(),
					ConfigErrorCause::UnknownError(String::from(e.message())),
				))
			},
		}
	}
	else {
		Ok(default)
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_err_eq, assert_ok_eq};

	use super::*;
	use crate::test_helpers::{invalid_utf, with_git_config};

	#[test]
	fn read_true() {
		with_git_config(&["[test]", "bool = true"], |git_config| {
			assert_ok_eq!(get_bool(Some(&git_config), "test.bool", false), true);
		});
	}

	#[test]
	fn read_false() {
		with_git_config(&["[test]", "bool = false"], |git_config| {
			assert_ok_eq!(get_bool(Some(&git_config), "test.bool", true), false);
		});
	}

	#[test]
	fn read_default() {
		with_git_config(&[], |git_config| {
			assert_ok_eq!(get_bool(Some(&git_config), "test.bool", true), true);
		});
	}

	#[test]
	fn read_invalid_value() {
		with_git_config(&["[test]", "bool = invalid"], |git_config| {
			assert_err_eq!(
				get_bool(Some(&git_config), "test.bool", true),
				ConfigError::new("test.bool", "invalid", ConfigErrorCause::InvalidBoolean)
			);
		});
	}

	#[test]
	fn read_unexpected_error() {
		with_git_config(&["[test]", "bool = invalid"], |git_config| {
			assert_err_eq!(
				get_bool(Some(&git_config), "test", true),
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
			&["[test]", format!("bool = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_err_eq!(
					get_bool(Some(&git_config), "test.bool", true),
					ConfigError::new_read_error("test.bool", ConfigErrorCause::InvalidBoolean)
				);
			},
		);
	}
}
