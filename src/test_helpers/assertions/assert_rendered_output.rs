mod patterns;
mod render_style;
mod render_view_data;
mod render_view_line;

use std::fmt::Debug;

use bitflags::bitflags;

#[allow(unused_imports)]
pub(crate) use self::{
	patterns::{
		ActionPattern,
		AllPattern,
		AnyLinePattern,
		AnyPattern,
		ContainsPattern,
		EndsWithPattern,
		ExactPattern,
		LinePattern,
		NotPattern,
	},
	render_style::render_style,
	render_view_data::render_view_data,
	render_view_line::render_view_line,
};
use crate::{test_helpers::shared::replace_invisibles, view::ViewData};

bitflags! {
	/// Options for the `assert_rendered_output!` macro
	#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
	pub(crate) struct AssertRenderOptions: u8 {
		/// Ignore trailing whitespace
		const INCLUDE_TRAILING_WHITESPACE = 0b0000_0001;
		/// Ignore pinned indicator
		const INCLUDE_PINNED = 0b0000_0010;
		/// Don't include style information
		const INCLUDE_STYLE = 0b0000_0100;
		/// Only render the body, in this mode {BODY} is also not rendered
		const BODY_ONLY = 0b0000_1000;
	}
}

#[allow(clippy::string_slice)]
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
	($line:expr) => {{
		$crate::test_helpers::assertions::assert_rendered_output::ExactPattern::new($line)
	}};
	(Line) => {{
		$crate::test_helpers::assertions::assert_rendered_output::AnyLinePattern::new()
	}};
	(StartsWith $line:expr) => {{
		$crate::test_helpers::assertions::assert_rendered_output::StartsWithPattern::new($line)
	}};
	(Not StartsWith $line:expr) => {{
		$crate::test_helpers::assertions::assert_rendered_output::NotPattern::new(
			Box::new(
				$crate::test_helpers::assertions::assert_rendered_output::StartsWithPattern::new($line)
			)
		)
	}};
	(EndsWith $line:expr) => {{
		$crate::test_helpers::assertions::assert_rendered_output::EndsWithPattern::new($line)
	}};
	(Not EndsWith $line:expr) => {{
		$crate::test_helpers::assertions::assert_rendered_output::NotPattern::new(
			Box::new($crate::test_helpers::assertions::assert_rendered_output::EndsWithPattern::new($line))
		)
	}};
	(Contains $line:expr) => {{
		$crate::test_helpers::assertions::assert_rendered_output::ContainsPattern::new($line)
	}};
	(Not Contains $line:expr) => {{
		$crate::test_helpers::assertions::assert_rendered_output::NotPattern::new(
			Box::new(
				$crate::test_helpers::assertions::assert_rendered_output::ContainsPattern::new($line)
			)
		)
	}};
	(Not $pattern:expr) => {{
		$crate::test_helpers::assertions::assert_rendered_output::NotPattern::new(Box::new($pattern))
	}};
	(All $($patterns:expr),*) => {{
		let patterns: Vec<Box<dyn LinePattern>> = vec![$( Box::new($patterns), )*];
		$crate::test_helpers::assertions::assert_rendered_output::AllPattern::new(patterns)
	}};
	(Any $($patterns:expr),*) => {{
		let patterns: Vec<Box<dyn LinePattern>> = vec![$( Box::new($patterns), )*];
		$crate::test_helpers::assertions::assert_rendered_output::AnyPattern::new(patterns)
	}};
}

#[macro_export]
macro_rules! action_line {
	(Break) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_break(false)
	}};
	(Selected Break) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_break(true)
	}};
	(Drop $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_drop($hash, $comment, false)
	}};
	(Selected Drop $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_drop($hash, $comment, true)
	}};
	(Edit $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_edit($hash, $comment, false)
	}};
	(Selected Edit $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_edit($hash, $comment, true)
	}};
	(Fixup $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_fixup($hash, $comment, false)
	}};
	(Selected Fixup $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_fixup($hash, $comment, true)
	}};
	(Pick $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_pick($hash, $comment, false)
	}};
	(Selected Pick $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_pick($hash, $comment, true)
	}};
	(Reword $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_reword($hash, $comment, false)
	}};
	(Selected Reword $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_reword($hash, $comment, true)
	}};
	(Squash $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_squash($hash, $comment, false)
	}};
	(Selected Squash $hash:expr, $comment:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_squash($hash, $comment, true)
	}};
	(Exec $command:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_exec($command, false)
	}};
	(Selected Exec $command:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_exec($command, true)
	}};
	(Label $reference:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_label($reference, false)
	}};
	(Selected Label $reference:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_label($reference, true)
	}};
	(Reset $reference:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_reset($reference, false)
	}};
	(Selected Reset $reference:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_reset($reference, true)
	}};
	(Merge $reference:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_merge($reference, false)
	}};
	(Selected Merge $reference:expr) => {{
		use $crate::test_helpers::assertions::assert_rendered_output::ActionPattern;
		ActionPattern::new_merge($reference, true)
	}};
}

/// Assert the rendered output from a `ViewData`.
#[macro_export]
macro_rules! assert_rendered_output {
	($view_data:expr, $($arg:expr),*) => {
		use $crate::test_helpers::assertions::assert_rendered_output::AssertRenderOptions;
		assert_rendered_output!(
			@base AssertRenderOptions::default(), None, None, $view_data, $($arg),*
		)
	};
	(Body $view_data:expr, $($arg:expr),*) => {
		use $crate::test_helpers::assertions::assert_rendered_output::AssertRenderOptions;
		assert_rendered_output!(
			@base AssertRenderOptions::BODY_ONLY, None, None, $view_data, $($arg),*
		)
	};
	(Style $view_data:expr, $($arg:expr),*) => {
		use $crate::test_helpers::assertions::assert_rendered_output::AssertRenderOptions;
		assert_rendered_output!(
			@base AssertRenderOptions::INCLUDE_STYLE, None, None, $view_data, $($arg),*
		)
	};
	(Skip $start:expr, $view_data:expr, $($arg:expr),*) => {
		use $crate::test_helpers::assertions::assert_rendered_output::AssertRenderOptions;
		assert_rendered_output!(
			@base AssertRenderOptions::default(), Some($start), None, $view_data, $($arg),*
		)
	};
	(Skip $start:expr;$end:expr, $view_data:expr, $($arg:expr),*) => {
		use $crate::test_helpers::assertions::assert_rendered_output::AssertRenderOptions;
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
		use $crate::test_helpers::assertions::assert_rendered_output::AssertRenderOptions;
		assert_rendered_output!(
			@base AssertRenderOptions::BODY_ONLY, Some($start), None, $view_data, $($arg),*
		)
	};
	(Body, Skip $start:expr;$end:expr, $view_data:expr, $($arg:expr),*) => {
		use $crate::test_helpers::assertions::assert_rendered_output::AssertRenderOptions;
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
		use $crate::test_helpers::assertions::assert_rendered_output::{
			_assert_rendered_output_from_view_data,
			LinePattern,
		};
		let expected: Vec<Box<dyn LinePattern>> = vec![$( Box::new($arg), )*];
		_assert_rendered_output_from_view_data($view_data, &expected, $options, $start, $end);
	};
}
