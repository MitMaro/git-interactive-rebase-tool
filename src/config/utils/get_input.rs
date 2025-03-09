use crate::{
	config::{ConfigError, ConfigErrorCause, utils::get_string},
	git::Config,
};

#[allow(clippy::string_slice)]
pub(crate) fn get_input(config: Option<&Config>, name: &str, default: &str) -> Result<Vec<String>, ConfigError> {
	let mut values = vec![];
	let input = get_string(config, name, default)?;
	for mut value in input.split_whitespace().map(String::from) {
		let mut modifiers = vec![];

		let shift_index = value.to_lowercase().find("shift+");
		if let Some(index) = shift_index {
			value.replace_range(index..index + 6, "");
		}
		if let Some(index) = value.to_lowercase().find("control+") {
			modifiers.push("Control");
			value.replace_range(index..index + 8, "");
		}
		if let Some(index) = value.to_lowercase().find("alt+") {
			modifiers.push("Alt");
			value.replace_range(index..index + 4, "");
		}

		let mut key = match value.to_lowercase().as_ref() {
			"backspace" => String::from("Backspace"),
			"backtab" => String::from("BackTab"),
			"delete" => String::from("Delete"),
			"down" => String::from("Down"),
			"end" => String::from("End"),
			"enter" => String::from("Enter"),
			"esc" => String::from("Esc"),
			"home" => String::from("Home"),
			"insert" => String::from("Insert"),
			"left" => String::from("Left"),
			"pagedown" => String::from("PageDown"),
			"pageup" => String::from("PageUp"),
			"right" => String::from("Right"),
			"tab" => String::from("Tab"),
			"up" => String::from("Up"),
			v => {
				let v_len = v.chars().count();
				// allow F{number} values
				if v_len > 1 && v.starts_with('f') && v[1..].parse::<u8>().is_ok() {
					v.to_uppercase()
				}
				else if v_len == 1 {
					value
				}
				else {
					return Err(ConfigError::new(
						name,
						input.as_str(),
						ConfigErrorCause::InvalidKeyBinding,
					));
				}
			},
		};

		// Shift support was partially removed, due to Shift not being universally reported, but still maintain
		// some backwards compatibility with printable characters
		if shift_index.is_some() {
			if key.len() == 1 {
				key = key.to_uppercase();
			}
			else {
				modifiers.push("Shift");
			}
		}

		values.push(format!("{}{}", modifiers.join(""), key));
	}
	Ok(values)
}

#[cfg(test)]
mod tests {
	use claims::{assert_err_eq, assert_ok_eq};
	use rstest::rstest;

	use super::*;
	use crate::test_helpers::{invalid_utf, with_git_config};

	#[rstest]
	#[case::single_lower("a", "a")]
	#[case::single_upper("A", "A")]
	#[case::single_non_ascii("ẞ", "ẞ")]
	#[case::backspace("backspace", "Backspace")]
	#[case::backtab("backtab", "BackTab")]
	#[case::delete("delete", "Delete")]
	#[case::down("down", "Down")]
	#[case::end("end", "End")]
	#[case::end("enter", "Enter")]
	#[case::end("esc", "Esc")]
	#[case::home("home", "Home")]
	#[case::insert("insert", "Insert")]
	#[case::left("left", "Left")]
	#[case::pagedown("pagedown", "PageDown")]
	#[case::pageup("pageup", "PageUp")]
	#[case::right("right", "Right")]
	#[case::tab("tab", "Tab")]
	#[case::up("up", "Up")]
	#[case::f1("f1", "F1")]
	#[case::f255("f255", "F255")]
	#[case::modifier_character_lowercase("Control+a", "Controla")]
	#[case::modifier_character_uppercase("Control+A", "ControlA")]
	#[case::modifier_character_number("Control+1", "Control1")]
	#[case::modifier_character_special("Control++", "Control+")]
	#[case::modifier_character_non_ascii("Control+ẞ", "Controlẞ")]
	#[case::modifier_character("Control+a", "Controla")]
	#[case::modifier_special("Control+End", "ControlEnd")]
	#[case::modifier_function("Control+F32", "ControlF32")]
	#[case::modifier_control_alt_shift_lowercase("alt+shift+control+end", "ControlAltShiftEnd")]
	#[case::modifier_control_alt_shift_mixedcase("aLt+shIft+conTrol+eNd", "ControlAltShiftEnd")]
	#[case::modifier_control_alt_shift_out_of_order_1("Alt+Shift+Control+End", "ControlAltShiftEnd")]
	#[case::modifier_control_alt_shift_out_of_order_2("Shift+Control+Alt+End", "ControlAltShiftEnd")]
	#[case::modifier_only_shift("Shift+End", "ShiftEnd")]
	#[case::modifier_only_control("Control+End", "ControlEnd")]
	#[case::shift_with_printable_lower("Shift+a", "A")]
	#[case::shift_with_printable_upper("Shift+A", "A")]
	#[case::multiple("a b ẞ c d", "a,b,ẞ,c,d")]
	#[case::multiple_with_modifiers("Control+End Control+A", "ControlEnd,ControlA")]
	fn read_value(#[case] binding: &str, #[case] expected: &str) {
		with_git_config(&["[test]", format!("value = {binding}").as_str()], |git_config| {
			assert_ok_eq!(
				get_input(Some(&git_config), "test.value", "x"),
				expected.split(',').map(String::from).collect::<Vec<_>>()
			);
		});
	}

	#[test]
	fn read_value_default() {
		with_git_config(&[], |git_config| {
			assert_ok_eq!(get_input(Some(&git_config), "test.value", "x"), vec![String::from("x")]);
		});
	}

	#[rstest]
	#[case::multiple_characters("abcd")]
	#[case::function_key_index("F256")]
	#[case::multiple_bindings_one_invalid("f foo")]
	fn read_value_invalid(#[case] binding: &str) {
		with_git_config(&["[test]", format!("value = {binding}").as_str()], |git_config| {
			assert_err_eq!(
				get_input(Some(&git_config), "test.value", "x"),
				ConfigError::new("test.value", binding, ConfigErrorCause::InvalidKeyBinding)
			);
		});
	}

	#[test]
	fn read_invalid_non_utf() {
		with_git_config(
			&["[test]", format!("value = {}", invalid_utf()).as_str()],
			|git_config| {
				assert_err_eq!(
					get_input(Some(&git_config), "test.value", "x"),
					ConfigError::new_read_error("test.value", ConfigErrorCause::InvalidUtf)
				);
			},
		);
	}
}
