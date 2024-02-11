use crate::{
	config::{utils::get_string, ConfigError, ConfigErrorCause, DiffIgnoreWhitespaceSetting},
	git::Config,
};

pub(crate) fn get_diff_ignore_whitespace(
	git_config: Option<&Config>,
	name: &str,
) -> Result<DiffIgnoreWhitespaceSetting, ConfigError> {
	match get_string(git_config, name, "none")?.to_lowercase().as_str() {
		"true" | "on" | "all" => Ok(DiffIgnoreWhitespaceSetting::All),
		"change" => Ok(DiffIgnoreWhitespaceSetting::Change),
		"false" | "off" | "none" => Ok(DiffIgnoreWhitespaceSetting::None),
		input => {
			Err(ConfigError::new(
				name,
				input,
				ConfigErrorCause::InvalidDiffIgnoreWhitespace,
			))
		},
	}
}

#[cfg(test)]
mod tests {
	use claims::assert_ok_eq;
	use rstest::rstest;
	use testutils::assert_err_eq;

	use super::*;
	use crate::test_helpers::{invalid_utf, with_git_config};

	#[rstest]
	#[case::true_str("true", DiffIgnoreWhitespaceSetting::All)]
	#[case::on("on", DiffIgnoreWhitespaceSetting::All)]
	#[case::all("all", DiffIgnoreWhitespaceSetting::All)]
	#[case::change("change", DiffIgnoreWhitespaceSetting::Change)]
	#[case::false_str("false", DiffIgnoreWhitespaceSetting::None)]
	#[case::off("off", DiffIgnoreWhitespaceSetting::None)]
	#[case::none("none", DiffIgnoreWhitespaceSetting::None)]
	#[case::mixed_case("ChAnGe", DiffIgnoreWhitespaceSetting::Change)]
	fn read_ok(#[case] value: &str, #[case] expected: DiffIgnoreWhitespaceSetting) {
		with_git_config(&["[test]", format!("value = \"{value}\"").as_str()], |git_config| {
			assert_ok_eq!(get_diff_ignore_whitespace(Some(&git_config), "test.value"), expected);
		});
	}

	#[test]
	fn read_default() {
		with_git_config(&[], |git_config| {
			assert_ok_eq!(
				get_diff_ignore_whitespace(Some(&git_config), "test.value"),
				DiffIgnoreWhitespaceSetting::None
			);
		});
	}

	#[test]
	fn read_invalid_value() {
		with_git_config(&["[test]", "value = invalid"], |git_config| {
			assert_err_eq!(
				get_diff_ignore_whitespace(Some(&git_config), "test.value"),
				ConfigError::new("test.value", "invalid", ConfigErrorCause::InvalidDiffIgnoreWhitespace)
			);
		});
	}

	#[test]
	fn read_invalid_non_utf() {
		with_git_config(
			&["[test]", format!("value = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_err_eq!(
					get_diff_ignore_whitespace(Some(&git_config), "test.value"),
					ConfigError::new_read_error("test.value", ConfigErrorCause::InvalidUtf)
				);
			},
		);
	}
}
