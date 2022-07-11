use git::{Config, ErrorCode};

use crate::{utils::_get_string, ConfigError, ConfigErrorCause};

#[allow(clippy::map_err_ignore)]
pub(crate) fn get_unsigned_integer(config: Option<&Config>, name: &str, default: u32) -> Result<u32, ConfigError> {
	if let Some(cfg) = config {
		match cfg.get_i32(name) {
			Ok(v) => {
				v.try_into().map_err(|_| {
					ConfigError::new_with_optional_input(
						name,
						_get_string(config, name).ok().flatten(),
						ConfigErrorCause::InvalidUnsignedInteger,
					)
				})
			},
			Err(e) if e.code() == ErrorCode::NotFound => Ok(default),
			Err(e) if e.message().contains("failed to parse") => {
				Err(ConfigError::new_with_optional_input(
					name,
					_get_string(config, name).ok().flatten(),
					ConfigErrorCause::InvalidUnsignedInteger,
				))
			},
			Err(e) => {
				Err(ConfigError::new_with_optional_input(
					name,
					_get_string(config, name).ok().flatten(),
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
	use claim::assert_ok_eq;
	use testutils::assert_err_eq;

	use super::*;
	use crate::testutils::{invalid_utf, with_git_config};

	#[test]
	fn read_value() {
		with_git_config(&["[test]", "value = 42"], |git_config| {
			assert_ok_eq!(get_unsigned_integer(Some(&git_config), "test.value", 24), 42);
		});
	}

	#[test]
	fn read_value_min() {
		with_git_config(&["[test]", "value = 0"], |git_config| {
			assert_ok_eq!(get_unsigned_integer(Some(&git_config), "test.value", 24), 0);
		});
	}
	#[test]
	fn read_value_max() {
		with_git_config(&["[test]", format!("value = {}", i32::MAX).as_str()], |git_config| {
			assert_ok_eq!(
				get_unsigned_integer(Some(&git_config), "test.value", 24),
				i32::MAX as u32
			);
		});
	}

	#[test]
	fn read_value_too_small() {
		with_git_config(&["[test]", "value = -1"], |git_config| {
			assert_err_eq!(
				get_unsigned_integer(Some(&git_config), "test.value", 24),
				ConfigError::new("test.value", "-1", ConfigErrorCause::InvalidUnsignedInteger)
			);
		});
	}

	#[test]
	fn read_value_too_small_i32() {
		with_git_config(&["[test]", format!("value = {}", i64::MIN).as_str()], |git_config| {
			assert_err_eq!(
				get_unsigned_integer(Some(&git_config), "test.value", 24),
				ConfigError::new(
					"test.value",
					i64::MIN.to_string().as_str(),
					ConfigErrorCause::InvalidUnsignedInteger
				)
			);
		});
	}

	#[test]
	fn read_value_too_large() {
		with_git_config(&["[test]", format!("value = {}", u64::MAX).as_str()], |git_config| {
			assert_err_eq!(
				get_unsigned_integer(Some(&git_config), "test.value", 24),
				ConfigError::new(
					"test.value",
					u64::MAX.to_string().as_str(),
					ConfigErrorCause::InvalidUnsignedInteger
				)
			);
		});
	}

	#[test]
	fn read_default() {
		with_git_config(&[], |git_config| {
			assert_ok_eq!(get_unsigned_integer(Some(&git_config), "test.value", 24), 24);
		});
	}

	#[test]
	fn read_invalid() {
		with_git_config(&["[test]", "value = invalid"], |git_config| {
			assert_err_eq!(
				get_unsigned_integer(Some(&git_config), "test.value", 24),
				ConfigError::new("test.value", "invalid", ConfigErrorCause::InvalidUnsignedInteger)
			);
		});
	}

	#[test]
	fn read_unexpected_error() {
		with_git_config(&["[test]", "value = invalid"], |git_config| {
			assert_err_eq!(
				get_unsigned_integer(Some(&git_config), "test", 24),
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
					get_unsigned_integer(Some(&git_config), "test.value", 24),
					ConfigError::new_read_error("test.value", ConfigErrorCause::InvalidUnsignedInteger)
				);
			},
		);
	}
}
