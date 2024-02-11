use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::view::{
	testutil::{render_view_line::render_view_data, AssertRenderOptions},
	ViewData,
};

const VISIBLE_SPACE_REPLACEMENT: &str = "\u{b7}"; // "·"
const VISIBLE_TAB_REPLACEMENT: &str = "   \u{2192}"; // "   →"

/// Replace invisible characters with visible counterparts
#[inline]
#[must_use]
pub(crate) fn replace_invisibles(line: &str) -> String {
	line.replace(' ', VISIBLE_SPACE_REPLACEMENT)
		.replace('\t', VISIBLE_TAB_REPLACEMENT)
}

/// A pattern matcher for a rendered line
pub(crate) trait LinePattern: Debug {
	/// Check if the rendered line matches the matchers pattern
	fn matches(&self, rendered: &str) -> bool;

	/// A formatted expected value for the matcher
	fn expected(&self) -> String;

	/// A formatted actual value for the matcher
	#[inline]
	#[must_use]
	fn actual(&self, rendered: &str) -> String {
		replace_invisibles(rendered)
	}

	/// Does this matcher use styles for matching
	#[inline]
	fn use_styles(&self) -> bool {
		true
	}
}

impl LinePattern for String {
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		rendered == self
	}

	#[inline]
	fn expected(&self) -> String {
		replace_invisibles(self.as_str())
	}
}

impl LinePattern for &str {
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		rendered == *self
	}

	#[inline]
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
	#[inline]
	#[must_use]
	pub(crate) fn new() -> Self {
		Self
	}
}

impl LinePattern for AnyLinePattern {
	#[inline]
	fn matches(&self, _: &str) -> bool {
		true
	}

	#[inline]
	fn expected(&self) -> String {
		String::from("{{Any}}")
	}

	#[inline]
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
	#[inline]
	#[must_use]
	pub(crate) fn new(pattern: &str) -> Self {
		Self(String::from(pattern))
	}
}

impl LinePattern for ExactPattern {
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		rendered == self.0
	}

	#[inline]
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
	#[inline]
	#[must_use]
	pub(crate) fn new(pattern: &str) -> Self {
		Self(String::from(pattern))
	}
}

impl LinePattern for StartsWithPattern {
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		rendered.starts_with(self.0.as_str())
	}

	#[inline]
	fn expected(&self) -> String {
		format!("StartsWith {}", replace_invisibles(self.0.as_str()))
	}

	#[inline]
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
	#[inline]
	#[must_use]
	pub(crate) fn new(pattern: &str) -> Self {
		Self(String::from(pattern))
	}
}

impl LinePattern for EndsWithPattern {
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		rendered.ends_with(self.0.as_str())
	}

	#[inline]
	fn expected(&self) -> String {
		format!("EndsWith {}", replace_invisibles(self.0.as_str()))
	}

	#[allow(clippy::string_slice)]
	#[inline]
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
	#[inline]
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
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		rendered.contains(self.0.as_str())
	}

	#[inline]
	fn expected(&self) -> String {
		format!("Contains {}", replace_invisibles(self.0.as_str()))
	}

	#[allow(clippy::string_slice)]
	#[inline]
	fn actual(&self, rendered: &str) -> String {
		format!("         {}", replace_invisibles(rendered))
	}
}

impl NotPattern {
	/// Create a new matcher with a pattern
	#[inline]
	#[must_use]
	pub(crate) fn new(pattern: Box<dyn LinePattern>) -> Self {
		Self(pattern)
	}
}

impl LinePattern for NotPattern {
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		!self.0.matches(rendered)
	}

	#[inline]
	fn expected(&self) -> String {
		format!("Not({})", self.0.expected())
	}

	#[inline]
	fn actual(&self, rendered: &str) -> String {
		format!("Not({})", self.0.actual(rendered))
	}
}

/// A pattern that matches that a rendered line matches all of a set of patterns
#[non_exhaustive]
pub(crate) struct AllPattern(Vec<Box<dyn LinePattern>>);

impl AllPattern {
	/// Create a new matcher with patterns
	#[inline]
	#[must_use]
	pub(crate) fn new(patterns: Vec<Box<dyn LinePattern>>) -> Self {
		Self(patterns)
	}
}

impl LinePattern for AllPattern {
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		self.0.iter().all(|pattern| pattern.matches(rendered))
	}

	#[inline]
	fn expected(&self) -> String {
		format!("All({})", self.0.iter().map(|p| { p.expected() }).join(", "))
	}
}

impl Debug for AllPattern {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "All({})", self.0.iter().map(|p| format!("{p:?}")).join(", "))
	}
}

/// A pattern that matches that a rendered line matches any of a set of patterns
#[non_exhaustive]
pub(crate) struct AnyPattern(Vec<Box<dyn LinePattern>>);

impl AnyPattern {
	/// Create a new matcher with patterns
	#[inline]
	#[must_use]
	pub(crate) fn new(patterns: Vec<Box<dyn LinePattern>>) -> Self {
		Self(patterns)
	}
}

impl LinePattern for AnyPattern {
	#[inline]
	fn matches(&self, rendered: &str) -> bool {
		self.0.iter().any(|pattern| pattern.matches(rendered))
	}

	#[inline]
	fn expected(&self) -> String {
		format!("Any({})", self.0.iter().map(|p| { p.expected() }).join(", "))
	}
}

impl Debug for AnyPattern {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Any({})", self.0.iter().map(|p| format!("{p:?}")).join(", "))
	}
}

#[allow(clippy::string_slice, clippy::panic)]
pub(crate) fn _assert_rendered_output(
	options: AssertRenderOptions,
	actual: &[String],
	expected_patterns: &[Box<dyn LinePattern>],
) {
	let mut mismatch = false;
	let mut error_output = vec![
		String::from("\nUnexpected output!"),
		String::from("--- Expected"),
		String::from("+++ Actual"),
		String::from("=========="),
	];

	for (expected_pattern, output_line) in expected_patterns.iter().zip(actual.iter()) {
		let output = if options.contains(AssertRenderOptions::INCLUDE_TRAILING_WHITESPACE) {
			output_line.as_str()
		}
		else {
			output_line.trim_end()
		};

		if expected_pattern.matches(output) {
			error_output.push(format!(" {}", expected_pattern.expected()));
		}
		else {
			mismatch = true;
			error_output.push(format!("-{}", expected_pattern.expected()));
			error_output.push(format!("+{}", expected_pattern.actual(output)));
		}
	}

	match expected_patterns.len() {
		a if a > actual.len() => {
			mismatch = true;
			for expected_pattern in expected_patterns.iter().skip(actual.len()) {
				error_output.push(format!("-{}", expected_pattern.expected().as_str()));
			}
		},
		a if a < actual.len() => {
			mismatch = true;
			for line in actual.iter().skip(expected_patterns.len()) {
				error_output.push(format!("+{}", replace_invisibles(line)));
			}
		},
		_ => {},
	}

	if mismatch {
		error_output.push(String::from("==========\n"));
		panic!("{}", error_output.join("\n"));
	}
}

/// Assert the rendered output from a `ViewData`. Generally this function is not used directly,
/// instead use the `assert_rendered_output!` macro.
#[inline]
pub(crate) fn _assert_rendered_output_from_view_data(
	view_data: &ViewData,
	expected: &[Box<dyn LinePattern>],
	options: AssertRenderOptions,
	skip_start: Option<usize>,
	skip_end: Option<usize>,
) {
	let rendered = render_view_data(view_data, options);
	let mut length = rendered.len();
	let mut output_iter: Box<dyn Iterator<Item = String>> = Box::new(rendered.into_iter());

	if let Some(skip) = skip_start {
		length = length.saturating_sub(skip);
		output_iter = Box::new(output_iter.skip(skip));
	}

	if let Some(skip) = skip_end {
		output_iter = Box::new(output_iter.take(length.saturating_sub(skip)));
	}

	_assert_rendered_output(options, &output_iter.collect::<Vec<String>>(), expected);
}

/// Create an assertion on a line. For use in `assert_rendered_output` macro.
#[macro_export]
macro_rules! render_line {
	($line:expr) => {{ $crate::view::testutil::ExactPattern::new($line) }};
	(Line) => {{ $crate::view::testutil::AnyLinePattern::new() }};
	(StartsWith $line:expr) => {{ $crate::view::testutil::StartsWithPattern::new($line) }};
	(Not StartsWith $line:expr) => {{ $crate::view::testutil::NotPattern::new(
		Box::new($crate::view::testutil::StartsWithPattern::new($line))
	) }};
	(EndsWith $line:expr) => {{ $crate::view::testutil::EndsWithPattern::new($line) }};
	(Not EndsWith $line:expr) => {{ $crate::view::testutil::NotPattern::new(
		Box::new($crate::view::testutil::EndsWithPattern::new($line))
	) }};
	(Contains $line:expr) => {{ $crate::view::testutil::ContainsPattern::new($line) }};
	(Not Contains $line:expr) => {{ $crate::view::testutil::NotPattern::new(
		Box::new($crate::view::testutil::ContainsPattern::new($line))
	) }};
	(Not $pattern:expr) => {{ $crate::view::testutil::NotPattern::new(Box::new($pattern)) }};
	(All $($patterns:expr),*) => {{
		let patterns: Vec<Box<dyn LinePattern>> = vec![$( Box::new($patterns), )*];
		$crate::view::testutil::AllPattern::new(patterns)
	}};
	(Any $($patterns:expr),*) => {{
		let patterns: Vec<Box<dyn LinePattern>> = vec![$( Box::new($patterns), )*];
		$crate::view::testutil::AnyPattern::new(patterns)
	}};
}

/// Assert the rendered output from a `ViewData`.
#[macro_export]
macro_rules! assert_rendered_output {
	($view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base AssertRenderOptions::default(), None, None, $view_data, $($arg),*
		)
	};
	(Body $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base AssertRenderOptions::BODY_ONLY, None, None, $view_data, $($arg),*
		)
	};
	(Style $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base AssertRenderOptions::INCLUDE_STYLE, None, None, $view_data, $($arg),*
		)
	};
	(Skip $start:expr, $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base AssertRenderOptions::default(), Some($start), None, $view_data, $($arg),*
		)
	};
	(Skip $start:expr;$end:expr, $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base AssertRenderOptions::default(), Some($start), Some($end), $view_data, $($arg),*
		)
	};
	(Options $options:expr, $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base $options, None, None, $view_data, $($arg),*
		)
	};
	(Body, Skip $start:expr, $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base AssertRenderOptions::BODY_ONLY, Some($start), None, $view_data, $($arg),*
		)
	};
	(Body, Skip $start:expr;$end:expr, $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base AssertRenderOptions::BODY_ONLY, Some($start), Some($end), $view_data, $($arg),*
		)
	};
	(Options $options:expr, Skip $start:expr, $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base $options, Some($start), None, $view_data, $($arg),*
		)
	};
	(Options $options:expr, Skip $start:expr;$end:expr, $view_data:expr, $($arg:expr),*) => {
		assert_rendered_output!(
			@base $options, Some($start), Some($end), $view_data, $($arg),*
		)
	};
	(@base $options:expr, $start:expr, $end:expr, $view_data:expr, $($arg:expr),*) => {
		use $crate::view::testutil::{_assert_rendered_output_from_view_data, AssertRenderOptions, LinePattern};
		let expected: Vec<Box<dyn LinePattern>> = vec![$( Box::new($arg), )*];
		_assert_rendered_output_from_view_data($view_data, &expected, $options, $start, $end);
	};
}
