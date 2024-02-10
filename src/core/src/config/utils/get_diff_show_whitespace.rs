use git::Config;

use crate::config::{get_string, ConfigError, ConfigErrorCause, DiffShowWhitespaceSetting};

pub(crate) fn get_diff_show_whitespace(
	git_config: Option<&Config>,
	name: &str,
) -> Result<DiffShowWhitespaceSetting, ConfigError> {
	match get_string(git_config, name, "both")?.to_lowercase().as_str() {
		"true" | "on" | "both" => Ok(DiffShowWhitespaceSetting::Both),
		"trailing" => Ok(DiffShowWhitespaceSetting::Trailing),
		"leading" => Ok(DiffShowWhitespaceSetting::Leading),
		"false" | "off" | "none" => Ok(DiffShowWhitespaceSetting::None),
		input => Err(ConfigError::new(name, input, ConfigErrorCause::InvalidShowWhitespace)),
	}
}

#[cfg(test)]
mod tests {
	use claims::assert_ok_eq;
	use rstest::rstest;
	use testutils::assert_err_eq;

	use super::*;
	use crate::config::testutils::{invalid_utf, with_git_config};

	#[rstest]
	#[case::true_str("true", DiffShowWhitespaceSetting::Both)]
	#[case::on("on", DiffShowWhitespaceSetting::Both)]
	#[case::both("both", DiffShowWhitespaceSetting::Both)]
	#[case::trailing("trailing", DiffShowWhitespaceSetting::Trailing)]
	#[case::leading("leading", DiffShowWhitespaceSetting::Leading)]
	#[case::false_str("false", DiffShowWhitespaceSetting::None)]
	#[case::off("off", DiffShowWhitespaceSetting::None)]
	#[case::none("none", DiffShowWhitespaceSetting::None)]
	#[case::mixed_case("lEaDiNg", DiffShowWhitespaceSetting::Leading)]
	fn read_ok(#[case] value: &str, #[case] expected: DiffShowWhitespaceSetting) {
		with_git_config(&["[test]", format!("value = \"{value}\"").as_str()], |git_config| {
			assert_ok_eq!(get_diff_show_whitespace(Some(&git_config), "test.value"), expected);
		});
	}

	#[test]
	fn read_default() {
		with_git_config(&[], |git_config| {
			assert_ok_eq!(
				get_diff_show_whitespace(Some(&git_config), "test.value"),
				DiffShowWhitespaceSetting::Both
			);
		});
	}

	#[test]
	fn read_invalid_value() {
		with_git_config(&["[test]", "value = invalid"], |git_config| {
			assert_err_eq!(
				get_diff_show_whitespace(Some(&git_config), "test.value"),
				ConfigError::new("test.value", "invalid", ConfigErrorCause::InvalidShowWhitespace)
			);
		});
	}

	#[test]
	fn read_invalid_non_utf() {
		with_git_config(
			&["[test]", format!("value = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_err_eq!(
					get_diff_show_whitespace(Some(&git_config), "test.value"),
					ConfigError::new_read_error("test.value", ConfigErrorCause::InvalidUtf)
				);
			},
		);
	}
}
