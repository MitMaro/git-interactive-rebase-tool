use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
	test_helpers::shared::replace_invisibles,
	todo_file::{Line, ParseError},
};

/// A pattern matcher for a rendered line
pub(crate) trait LinePattern: Debug {
	/// Check if the rendered line matches the matchers pattern
	fn matches(&self, rendered: &str) -> bool;

	/// A formatted expected value for the matcher
	fn expected(&self) -> String;

	/// A formatted actual value for the matcher
	#[must_use]
	fn actual(&self, rendered: &str) -> String {
		replace_invisibles(rendered)
	}

	/// Does this matcher use styles for matching
	fn use_styles(&self) -> bool {
		true
	}
}

impl LinePattern for String {
	fn matches(&self, rendered: &str) -> bool {
		rendered == self
	}

	fn expected(&self) -> String {
		replace_invisibles(self.as_str())
	}
}

impl LinePattern for &str {
	fn matches(&self, rendered: &str) -> bool {
		rendered == *self
	}

	fn expected(&self) -> String {
		replace_invisibles(self)
	}
}

/// A pattern matcher that will match any line
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub(crate) struct AnyLinePattern;

impl AnyLinePattern {
	/// Create a new instance
	#[must_use]
	pub(crate) fn new() -> Self {
		Self
	}
}

impl LinePattern for AnyLinePattern {
	fn matches(&self, _: &str) -> bool {
		true
	}

	fn expected(&self) -> String {
		String::from("{{Any}}")
	}

	fn actual(&self, _: &str) -> String {
		String::from("{{Any}}")
	}
}

/// A pattern matcher that matches that a rendered line is an exact match
#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) struct ExactPattern(String);

impl ExactPattern {
	/// Create a new matcher against a line pattern
	#[must_use]
	pub(crate) fn new(pattern: &str) -> Self {
		Self(String::from(pattern))
	}
}

impl LinePattern for ExactPattern {
	fn matches(&self, rendered: &str) -> bool {
		rendered == self.0
	}

	fn expected(&self) -> String {
		replace_invisibles(self.0.as_str())
	}
}

/// A pattern that matches that a rendered line starts with a pattern
#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) struct StartsWithPattern(String);

impl StartsWithPattern {
	/// Create a new matcher with a pattern
	#[must_use]
	pub(crate) fn new(pattern: &str) -> Self {
		Self(String::from(pattern))
	}
}

impl LinePattern for StartsWithPattern {
	fn matches(&self, rendered: &str) -> bool {
		rendered.starts_with(self.0.as_str())
	}

	fn expected(&self) -> String {
		format!("StartsWith {}", replace_invisibles(self.0.as_str()))
	}

	fn actual(&self, rendered: &str) -> String {
		format!(
			"           {}",
			replace_invisibles(rendered.chars().take(self.0.len()).collect::<String>().as_str())
		)
	}
}

/// A pattern that matches that a rendered line ends with a pattern
#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) struct EndsWithPattern(String);

impl EndsWithPattern {
	/// Create a new matcher with a pattern
	#[must_use]
	pub(crate) fn new(pattern: &str) -> Self {
		Self(String::from(pattern))
	}
}

impl LinePattern for EndsWithPattern {
	fn matches(&self, rendered: &str) -> bool {
		rendered.ends_with(self.0.as_str())
	}

	fn expected(&self) -> String {
		format!("EndsWith {}", replace_invisibles(self.0.as_str()))
	}

	#[allow(clippy::string_slice)]
	fn actual(&self, rendered: &str) -> String {
		format!(
			"         {}",
			replace_invisibles(&rendered[rendered.len() - self.0.len() + 2..])
		)
	}
}

/// A pattern that matches that a rendered line contains a pattern
#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) struct ContainsPattern(String);

impl ContainsPattern {
	/// Create a new matcher with a pattern
	#[must_use]
	pub(crate) fn new(pattern: &str) -> Self {
		Self(String::from(pattern))
	}
}

/// A pattern that matches that a rendered line matches all patterns
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct NotPattern(Box<dyn LinePattern>);

impl LinePattern for ContainsPattern {
	fn matches(&self, rendered: &str) -> bool {
		rendered.contains(self.0.as_str())
	}

	fn expected(&self) -> String {
		format!("Contains {}", replace_invisibles(self.0.as_str()))
	}

	#[allow(clippy::string_slice)]
	fn actual(&self, rendered: &str) -> String {
		format!("         {}", replace_invisibles(rendered))
	}
}

impl NotPattern {
	/// Create a new matcher with a pattern
	#[must_use]
	pub(crate) fn new(pattern: Box<dyn LinePattern>) -> Self {
		Self(pattern)
	}
}

impl LinePattern for NotPattern {
	fn matches(&self, rendered: &str) -> bool {
		!self.0.matches(rendered)
	}

	fn expected(&self) -> String {
		format!("Not({})", self.0.expected())
	}

	fn actual(&self, rendered: &str) -> String {
		format!("Not({})", self.0.actual(rendered))
	}
}

/// A pattern that matches that a rendered line matches all of a set of patterns
#[non_exhaustive]
pub(crate) struct AllPattern(Vec<Box<dyn LinePattern>>);

impl AllPattern {
	/// Create a new matcher with patterns
	#[must_use]
	pub(crate) fn new(patterns: Vec<Box<dyn LinePattern>>) -> Self {
		Self(patterns)
	}
}

impl LinePattern for AllPattern {
	fn matches(&self, rendered: &str) -> bool {
		self.0.iter().all(|pattern| pattern.matches(rendered))
	}

	fn expected(&self) -> String {
		format!("All({})", self.0.iter().map(|p| { p.expected() }).join(", "))
	}
}

impl Debug for AllPattern {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "All({})", self.0.iter().map(|p| format!("{p:?}")).join(", "))
	}
}

/// A pattern that matches that a rendered line matches any of a set of patterns
#[non_exhaustive]
pub(crate) struct AnyPattern(Vec<Box<dyn LinePattern>>);

impl AnyPattern {
	/// Create a new matcher with patterns
	#[must_use]
	pub(crate) fn new(patterns: Vec<Box<dyn LinePattern>>) -> Self {
		Self(patterns)
	}
}

impl LinePattern for AnyPattern {
	fn matches(&self, rendered: &str) -> bool {
		self.0.iter().any(|pattern| pattern.matches(rendered))
	}

	fn expected(&self) -> String {
		format!("Any({})", self.0.iter().map(|p| { p.expected() }).join(", "))
	}
}

impl Debug for AnyPattern {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Any({})", self.0.iter().map(|p| format!("{p:?}")).join(", "))
	}
}

lazy_static! {
	pub(crate) static ref FORMAT_REGEX: Regex = Regex::new(r"\{.*?}").unwrap();
}

fn parse_rendered_action_line(rendered: &str) -> Result<Line, ParseError> {
	let cleaned_line = FORMAT_REGEX.replace_all(rendered, "").replace(" > ", "");
	Line::parse(cleaned_line.as_ref())
}

#[derive(Debug)]
pub(crate) struct ActionPattern {
	line: Line,
	selected: bool,
}

impl ActionPattern {
	fn new(line: &str, selected: bool) -> Self {
		Self {
			line: Line::parse(line).expect("Expected valid pick"),
			selected,
		}
	}

	pub(crate) fn new_break(selected: bool) -> Self {
		Self::new("break", selected)
	}

	pub(crate) fn new_drop(hash: &str, comment: &str, selected: bool) -> Self {
		Self::new(format!("drop {hash} {comment}").as_str(), selected)
	}

	pub(crate) fn new_edit(hash: &str, comment: &str, selected: bool) -> Self {
		Self::new(format!("edit {hash} {comment}").as_str(), selected)
	}

	pub(crate) fn new_fixup(hash: &str, comment: &str, selected: bool) -> Self {
		Self::new(format!("fixup {hash} {comment}").as_str(), selected)
	}

	pub(crate) fn new_pick(hash: &str, comment: &str, selected: bool) -> Self {
		Self::new(format!("pick {hash} {comment}").as_str(), selected)
	}

	pub(crate) fn new_reword(hash: &str, comment: &str, selected: bool) -> Self {
		Self::new(format!("reword {hash} {comment}").as_str(), selected)
	}

	pub(crate) fn new_squash(hash: &str, comment: &str, selected: bool) -> Self {
		Self::new(format!("squash {hash} {comment}").as_str(), selected)
	}

	pub(crate) fn new_exec(command: &str, selected: bool) -> Self {
		Self::new(format!("exec {command}").as_str(), selected)
	}

	pub(crate) fn new_label(reference: &str, selected: bool) -> Self {
		Self::new(format!("label {reference}").as_str(), selected)
	}

	pub(crate) fn new_reset(reference: &str, selected: bool) -> Self {
		Self::new(format!("reset {reference}").as_str(), selected)
	}

	pub(crate) fn new_merge(reference: &str, selected: bool) -> Self {
		Self::new(format!("merge {reference}").as_str(), selected)
	}
}

impl LinePattern for ActionPattern {
	fn matches(&self, rendered: &str) -> bool {
		if rendered.contains("{Selected}") {
			if !self.selected {
				return false;
			}
		}
		else if self.selected {
			return false;
		}

		let Ok(actual_line_parsed) = parse_rendered_action_line(rendered)
		else {
			return false;
		};

		if let Some(expected_option) = self.line.option() {
			let Some(actual_option) = actual_line_parsed.option()
			else {
				// options on expected, no options on actual
				return false;
			};
			if expected_option != actual_option {
				return false;
			}
		}

		// not using Eq on Line, since we want to ignore a few internals of Line
		self.line.get_action() == actual_line_parsed.get_action()
			&& self.line.get_hash() == actual_line_parsed.get_hash()
			&& self.line.get_content() == actual_line_parsed.get_content()
	}

	fn expected(&self) -> String {
		if self.selected {
			replace_invisibles(format!(">  {}", self.line.to_text()).as_str())
		}
		else {
			replace_invisibles(format!("   {}", self.line.to_text()).as_str())
		}
	}

	fn actual(&self, rendered: &str) -> String {
		let Ok(actual_line_parsed) = parse_rendered_action_line(rendered)
		else {
			return String::from(rendered);
		};
		if rendered.contains("{Selected}") {
			replace_invisibles(format!("> {}", actual_line_parsed.to_text()).as_str())
		}
		else {
			replace_invisibles(format!("  {}", actual_line_parsed.to_text()).as_str())
		}
	}
}
