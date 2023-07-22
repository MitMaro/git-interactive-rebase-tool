use git::Config;

use crate::{get_string, ConfigError, ConfigErrorCause};

pub(crate) fn git_diff_renames(git_config: Option<&Config>, name: &str) -> Result<(bool, bool), ConfigError> {
	match get_string(git_config, name, "true")?.to_lowercase().as_str() {
		"true" => Ok((true, false)),
		"false" => Ok((false, false)),
		"copy" | "copies" => Ok((true, true)),
		input => Err(ConfigError::new(name, input, ConfigErrorCause::InvalidDiffRenames)),
	}
}

#[cfg(test)]
mod tests {
	use claims::assert_ok_eq;
	use rstest::rstest;
	use testutils::assert_err_eq;

	use super::*;
	use crate::testutils::{invalid_utf, with_git_config};

	#[rstest]
	#[case::true_str("true", (true, false))]
	#[case::false_str("false", (false, false))]
	#[case::off("copy", (true, true))]
	#[case::none("copies", (true, true))]
	#[case::mixed_case("CoPiEs", (true, true))]
	fn read_ok(#[case] value: &str, #[case] expected: (bool, bool)) {
		with_git_config(&["[test]", format!("value = \"{value}\"").as_str()], |git_config| {
			assert_ok_eq!(git_diff_renames(Some(&git_config), "test.value"), expected);
		});
	}

	#[test]
	fn read_default() {
		with_git_config(&[], |git_config| {
			assert_ok_eq!(git_diff_renames(Some(&git_config), "test.value"), (true, false));
		});
	}

	#[test]
	fn read_invalid_value() {
		with_git_config(&["[test]", "value = invalid"], |git_config| {
			assert_err_eq!(
				git_diff_renames(Some(&git_config), "test.value"),
				ConfigError::new("test.value", "invalid", ConfigErrorCause::InvalidDiffRenames)
			);
		});
	}

	#[test]
	fn read_invalid_non_utf() {
		with_git_config(
			&["[test]", format!("value = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_err_eq!(
					git_diff_renames(Some(&git_config), "test.value"),
					ConfigError::new_read_error("test.value", ConfigErrorCause::InvalidUtf)
				);
			},
		);
	}
}
