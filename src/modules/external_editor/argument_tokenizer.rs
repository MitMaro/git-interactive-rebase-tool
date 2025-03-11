#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
	Normal,
	Escape,
	DoubleQuote,
	SingleQuote,
	WhiteSpace,
}

#[expect(
	clippy::string_slice,
	reason = "Slices are safe because the slices are always on specific boundaries"
)]
pub(super) fn tokenize(input: &str) -> Option<Vec<String>> {
	let mut previous_state = State::Normal;
	let mut state = State::Normal;
	let mut token_start: usize = 0;
	let mut value = String::new();
	let mut force_value = false;

	let mut tokens = vec![];
	for (i, c) in input.chars().enumerate() {
		match state {
			State::Normal => {
				if c == '\\' {
					previous_state = State::Normal;
					state = State::Escape;
				}
				else if c == '"' {
					value.push_str(&input[token_start..i]);
					token_start = i + 1;
					state = State::DoubleQuote;
				}
				else if c == '\'' {
					value.push_str(&input[token_start..i]);
					token_start = i + 1;
					state = State::SingleQuote;
				}
				else if c.is_ascii_whitespace() {
					state = State::WhiteSpace;
					if token_start != i || !value.is_empty() || force_value {
						tokens.push(format!("{value}{}", &input[token_start..i]));
						value.clear();
					}
				}
			},
			State::DoubleQuote => {
				if c == '\\' {
					previous_state = State::DoubleQuote;
					state = State::Escape;
				}
				else if c == '"' {
					let v = &input[token_start..i];
					if v.is_empty() {
						force_value = true;
					}
					value.push_str(&input[token_start..i]);
					token_start = i + 1;
					state = State::Normal;
				}
			},
			State::SingleQuote => {
				if c == '\'' {
					let v = &input[token_start..i];
					if v.is_empty() {
						force_value = true;
					}
					value.push_str(&input[token_start..i]);
					token_start = i + 1;
					state = State::Normal;
				}
			},
			State::WhiteSpace => {
				force_value = false;
				token_start = i;
				if c == '\\' {
					// this next character should be parsed in normal state
					previous_state = State::Normal;
					state = State::Escape;
				}
				else if c == '"' {
					value.push_str(&input[token_start..i]);
					token_start = i + 1;
					state = State::DoubleQuote;
				}
				else if c == '\'' {
					value.push_str(&input[token_start..i]);
					token_start = i + 1;
					state = State::SingleQuote;
				}
				else if !c.is_ascii_whitespace() {
					state = State::Normal;
				}
			},
			State::Escape => {
				value.push_str(&input[token_start..(i - 1)]);
				value.push_str(&input[i..=i]);
				state = previous_state;
				token_start = i + 1;
			},
		}
	}

	if state != State::Normal && state != State::WhiteSpace {
		return None;
	}

	if state == State::Normal && token_start < input.len() {
		value.push_str(&input[token_start..]);
	}

	if force_value || !value.is_empty() {
		tokens.push(value);
	}

	Some(tokens)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tokenize_empty_string() {
		assert_eq!(tokenize("").unwrap().len(), 0);
	}

	#[test]
	fn tokenize_single_spaces() {
		assert_eq!(tokenize(" ").unwrap().len(), 0);
	}

	#[test]
	fn tokenize_single_tab() {
		assert_eq!(tokenize("\t").unwrap().len(), 0);
	}

	#[test]
	fn tokenize_multiple_spaces() {
		assert_eq!(tokenize("    ").unwrap().len(), 0);
	}

	#[test]
	fn tokenize_multiple_tabs() {
		assert_eq!(tokenize("\t\t\t").unwrap().len(), 0);
	}

	#[test]
	fn tokenize_empty_double_quoted_string() {
		assert_eq!(tokenize("\"\"").unwrap(), vec![""]);
	}

	#[test]
	fn tokenize_empty_double_quoted_string_not_last() {
		assert_eq!(tokenize("\"\" bar").unwrap(), vec!["", "bar"]);
	}

	#[test]
	fn tokenize_empty_double_quoted_string_not_fist() {
		assert_eq!(tokenize("foo \"\"").unwrap(), vec!["foo", ""]);
	}

	#[test]
	fn tokenize_empty_double_quoted_string_middle() {
		assert_eq!(tokenize("foo \"\" bar").unwrap(), vec!["foo", "", "bar"]);
	}

	#[test]
	fn tokenize_empty_single_quoted_string() {
		assert_eq!(tokenize("''").unwrap(), vec![""]);
	}

	#[test]
	fn tokenize_single_character() {
		assert_eq!(tokenize("a").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_single_character_in_double_quoted_string() {
		assert_eq!(tokenize("\"a\"").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_single_character_in_single_quoted_string() {
		assert_eq!(tokenize("'a'").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_single_leading_spaces() {
		assert_eq!(tokenize(" a").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_multiple_leading_spaces() {
		assert_eq!(tokenize("     a").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_single_leading_tab() {
		assert_eq!(tokenize("\ta").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_multiple_leading_tabs() {
		assert_eq!(tokenize("\t\t\ta").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_single_trailing_spaces() {
		assert_eq!(tokenize("a ").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_multiple_trailing_spaces() {
		assert_eq!(tokenize("a     ").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_single_trailing_tab() {
		assert_eq!(tokenize("a\t").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_multiple_trailing_tabs() {
		assert_eq!(tokenize("a\t\t\t").unwrap(), vec!["a"]);
	}

	#[test]
	fn tokenize_escaped_space() {
		assert_eq!(tokenize("\\ ").unwrap(), vec![" "]);
	}

	#[test]
	fn tokenize_escaped_double_quote() {
		assert_eq!(tokenize("\\\"").unwrap(), vec!["\""]);
	}

	#[test]
	fn tokenize_escaped_single_quote() {
		assert_eq!(tokenize("\\'").unwrap(), vec!["'"]);
	}

	#[test]
	fn tokenize_escaped_slash() {
		assert_eq!(tokenize("\\\\").unwrap(), vec!["\\"]);
	}

	#[test]
	fn tokenize_escaped_space_before_parameter() {
		assert_eq!(tokenize("\\ foo").unwrap(), vec![" foo"]);
	}

	#[test]
	fn tokenize_escaped_space_with_space_before() {
		assert_eq!(tokenize(" \\ ").unwrap(), vec![" "]);
	}

	#[test]
	fn tokenize_escaped_space_after_parameter() {
		assert_eq!(tokenize("foo\\ ").unwrap(), vec!["foo "]);
	}

	#[test]
	fn tokenize_escaped_space_before_double_quotes() {
		assert_eq!(tokenize("\\ \"foo\"").unwrap(), vec![" foo"]);
	}

	#[test]
	fn tokenize_space_before_single_quotes() {
		assert_eq!(tokenize(" 'foo'").unwrap(), vec!["foo"]);
	}

	#[test]
	fn tokenize_escaped_space_before_single_quotes() {
		assert_eq!(tokenize("\\ 'foo'").unwrap(), vec![" foo"]);
	}

	#[test]
	fn tokenize_escaped_space_after_double_quotes() {
		assert_eq!(tokenize("\"foo\"\\ ").unwrap(), vec!["foo "]);
	}

	#[test]
	fn tokenize_escaped_space_after_single_quotes() {
		assert_eq!(tokenize("'foo'\\ ").unwrap(), vec!["foo "]);
	}

	#[test]
	fn tokenize_escaped_spaces_1() {
		assert_eq!(tokenize(" \\ aaa\\ bbb\\  ").unwrap(), vec![" aaa bbb "]);
	}

	#[test]
	fn tokenize_mixed_whitespace_1() {
		assert_eq!(tokenize("\t\taaa \t bbb\t \tccc \tddd\t eee  ").unwrap(), vec![
			"aaa", "bbb", "ccc", "ddd", "eee"
		]);
	}

	#[test]
	fn tokenize_mixed_whitespace_2() {
		assert_eq!(tokenize("\t\t\"aaa \t bbb\t \tccc\" \td\\\"dd\t eee  ").unwrap(), vec![
			"aaa \t bbb\t \tccc",
			"d\"dd",
			"eee"
		]);
	}

	#[test]
	fn tokenize_mixed_whitespace_3() {
		assert_eq!(tokenize("\t\"a\" e").unwrap(), vec!["a", "e"]);
	}

	#[test]
	fn tokenize_basic_string() {
		assert_eq!(tokenize("a simple arguments").unwrap(), vec![
			"a",
			"simple",
			"arguments"
		]);
	}

	#[test]
	fn tokenize_joined_double_quote() {
		assert_eq!(tokenize("foo\"bar\"").unwrap(), vec!["foobar"]);
	}

	#[test]
	fn tokenize_argument_with_space_in_quotes() {
		assert_eq!(tokenize("\"bar with space\"").unwrap(), vec!["bar with space"]);
	}

	#[test]
	fn tokenize_argument_with_escaped_double_quote() {
		assert_eq!(tokenize("\"bar \\\"with\\\" space\"").unwrap(), vec![
			"bar \"with\" space"
		]);
	}

	#[test]
	fn tokenize_argument_with_embedded_single_quote() {
		assert_eq!(tokenize("\"bar 'with' space\"").unwrap(), vec!["bar 'with' space"]);
	}

	#[test]
	fn tokenize_joined_double_quoted_arguments() {
		assert_eq!(tokenize("\"foo\"\"bar\"").unwrap(), vec!["foobar"]);
	}

	#[test]
	fn tokenize_joined_single_quoted_arguments() {
		assert_eq!(tokenize("'foo''bar'").unwrap(), vec!["foobar"]);
	}

	#[test]
	fn tokenize_mixed_joined_1() {
		assert_eq!(tokenize("'foo'bar").unwrap(), vec!["foobar"]);
	}

	#[test]
	fn tokenize_mixed_joined_2() {
		assert_eq!(tokenize("foo'bar'").unwrap(), vec!["foobar"]);
	}

	#[test]
	fn tokenize_just_escaped() {
		assert!(tokenize("\\").is_none());
	}

	#[test]
	fn tokenize_just_double_quote() {
		assert!(tokenize("\"").is_none());
	}

	#[test]
	fn tokenize_just_single_quote() {
		assert!(tokenize("'").is_none());
	}

	#[test]
	fn tokenize_double_quote_unmatched() {
		assert!(tokenize("\"   ").is_none());
	}

	#[test]
	fn tokenize_single_quote_unmatched() {
		assert!(tokenize("'   ").is_none());
	}
}
