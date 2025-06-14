use crate::{
	config::{ConfigError, ConfigErrorCause, DiffShowWhitespaceSetting, utils::get_optional_string},
	git::Config,
};

pub(crate) fn get_diff_show_whitespace(
	git_config: &Config,
	name: &str,
) -> Result<DiffShowWhitespaceSetting, ConfigError> {
	if let Some(config_value) = get_optional_string(git_config, name)? {
		DiffShowWhitespaceSetting::parse(&config_value)
			.ok_or_else(|| ConfigError::new(name, &config_value, ConfigErrorCause::InvalidShowWhitespace))
	} else {
		Ok(DiffShowWhitespaceSetting::Both)
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_err_eq, assert_ok_eq};
	use rstest::rstest;

	use super::*;
	use crate::test_helpers::{invalid_utf, with_git_config};

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
			assert_ok_eq!(get_diff_show_whitespace(&git_config, "test.value"), expected);
		});
	}

	#[test]
	fn read_default() {
		with_git_config(&[], |git_config| {
			assert_ok_eq!(
				get_diff_show_whitespace(&git_config, "test.value"),
				DiffShowWhitespaceSetting::Both
			);
		});
	}

	#[test]
	fn read_invalid_value() {
		with_git_config(&["[test]", "value = invalid"], |git_config| {
			assert_err_eq!(
				get_diff_show_whitespace(&git_config, "test.value"),
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
					get_diff_show_whitespace(&git_config, "test.value"),
					ConfigError::new_read_error("test.value", ConfigErrorCause::InvalidUtf)
				);
			},
		);
	}
}
