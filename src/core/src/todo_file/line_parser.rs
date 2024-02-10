use crate::todo_file::errors::ParseError;

pub(crate) struct LineParser<'line> {
	input: &'line str,
	index: usize,
}

impl<'line> LineParser<'line> {
	pub(crate) fn new(input: &'line str) -> Self {
		let mut index = 0;
		while input.get(index..=index).unwrap_or("") == " " {
			index += 1;
		}
		Self { input, index }
	}

	pub(crate) const fn has_more(&self) -> bool {
		self.index < self.input.len()
	}

	#[allow(clippy::unwrap_in_result)]
	fn scan(&mut self) -> Result<(&'line str, usize), ParseError> {
		let mut new_index = self.index;
		if !self.has_more() {
			return Err(self.parse_error());
		}

		loop {
			if self.input.get(new_index..=new_index).unwrap_or(" ") == " " {
				let slice = self.input.get(self.index..new_index).unwrap();
				// skip whitespace
				new_index += 1;
				while self.input.get(new_index..=new_index).unwrap_or("") == " " {
					new_index += 1;
				}
				return Ok((slice, new_index));
			}
			new_index += 1;
		}
	}

	pub(crate) fn next(&mut self) -> Result<&'line str, ParseError> {
		let (slice, new_index) = self.scan()?;
		self.index = new_index;
		Ok(slice)
	}

	pub(crate) fn take_remaining(self) -> &'line str {
		self.input.get(self.index..self.input.len()).unwrap_or("")
	}

	pub(crate) fn parse_error(&self) -> ParseError {
		ParseError::InvalidLine(String::from(self.input))
	}
}

#[cfg(test)]
mod tests {
	use testutils::assert_err_eq;

	use super::*;

	fn collect_all_tokens<'parser>(parser: &'parser mut LineParser<'parser>) -> Vec<&'parser str> {
		let mut tokens = vec![];
		while let Ok(t) = parser.next() {
			tokens.push(t);
		}
		tokens
	}

	#[test]
	fn has_more_new() {
		let parser = LineParser::new("foo");
		assert!(parser.has_more());
	}

	#[test]
	fn has_more_after_next() {
		let mut parser = LineParser::new("foo");
		_ = parser.next();
		assert!(!parser.has_more());
	}

	#[test]
	fn has_more_after_next_with_trailing_spaces() {
		let mut parser = LineParser::new("foo ");
		_ = parser.next();
		assert!(!parser.has_more());
	}

	#[test]
	fn next_single_token() {
		let mut parser = LineParser::new("foo");
		assert_eq!(collect_all_tokens(&mut parser), vec!["foo"]);
	}

	#[test]
	fn next_single_token_leading_space() {
		let mut parser = LineParser::new(" foo");
		assert_eq!(collect_all_tokens(&mut parser), vec!["foo"]);
	}

	#[test]
	fn next_single_token_trailing_space() {
		let mut parser = LineParser::new("foo ");
		assert_eq!(collect_all_tokens(&mut parser), vec!["foo"]);
	}

	#[test]
	fn next_multiple_tokens() {
		let mut parser = LineParser::new("foo bar foobar");
		assert_eq!(collect_all_tokens(&mut parser), vec!["foo", "bar", "foobar"]);
	}

	#[test]
	fn next_tokens_with_multiple_spaces_between() {
		let mut parser = LineParser::new("foo   bar  foobar");
		assert_eq!(collect_all_tokens(&mut parser), vec!["foo", "bar", "foobar"]);
	}

	#[test]
	fn next_tokens_trailing_spaces() {
		let mut parser = LineParser::new("foo bar foobar ");
		assert_eq!(collect_all_tokens(&mut parser), vec!["foo", "bar", "foobar"]);
	}

	#[test]
	fn next_empty_str() {
		let mut parser = LineParser::new("");
		assert_err_eq!(parser.next(), ParseError::InvalidLine(String::new()));
	}

	#[test]
	fn next_space_only_str() {
		let mut parser = LineParser::new(" ");
		assert_err_eq!(parser.next(), ParseError::InvalidLine(String::from(" ")));
	}

	#[test]
	fn next_end_of_tokens() {
		let mut parser = LineParser::new("foo");
		_ = parser.next();
		assert_err_eq!(parser.next(), ParseError::InvalidLine(String::from("foo")));
	}

	#[test]
	fn next_end_of_tokens_with_trailing_space() {
		let mut parser = LineParser::new("foo ");
		_ = parser.next();
		assert_err_eq!(parser.next(), ParseError::InvalidLine(String::from("foo ")));
	}

	#[test]
	fn take_remaining_new() {
		let parser = LineParser::new("foo");
		assert_eq!(parser.take_remaining(), "foo");
	}

	#[test]
	fn take_remaining_empty_str() {
		let parser = LineParser::new("");
		assert_eq!(parser.take_remaining(), "");
	}

	#[test]
	fn take_remaining_space_str() {
		let parser = LineParser::new(" ");
		assert_eq!(parser.take_remaining(), "");
	}

	#[test]
	fn take_remaining_after_next() {
		let mut parser = LineParser::new("foo bar");
		_ = parser.next();
		assert_eq!(parser.take_remaining(), "bar");
	}

	#[test]
	fn take_remaining_end_of_tokens() {
		let mut parser = LineParser::new("foo");
		_ = parser.next();
		assert_eq!(parser.take_remaining(), "");
	}

	#[test]
	fn take_remaining_end_of_tokens_trailing_space() {
		let mut parser = LineParser::new("foo ");
		_ = parser.next();
		assert_eq!(parser.take_remaining(), "");
	}
}
