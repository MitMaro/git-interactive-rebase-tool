use lazy_static::lazy_static;
use regex::Regex;

use crate::{
	todo_file::{errors::ParseError, Line},
	view::testutil::{replace_invisibles, LinePattern},
};

lazy_static! {
	pub static ref FORMAT_REGEX: Regex = Regex::new(r"\{.*?}").unwrap();
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
			replace_invisibles(format!("  {}", self.line.to_text()).as_str())
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

#[macro_export]
macro_rules! action_line {
	(Break) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_break(false)
	}};
	(Selected Break) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_break(true)
	}};
	(Drop $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_drop($hash, $comment, false)
	}};
	(Selected Drop $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_drop($hash, $comment, true)
	}};
	(Edit $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_edit($hash, $comment, false)
	}};
	(Selected Edit $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_edit($hash, $comment, true)
	}};
	(Fixup $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_fixup($hash, $comment, false)
	}};
	(Selected Fixup $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_fixup($hash, $comment, true)
	}};
	(Pick $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_pick($hash, $comment, false)
	}};
	(Selected Pick $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_pick($hash, $comment, true)
	}};
	(Reword $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_reword($hash, $comment, false)
	}};
	(Selected Reword $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_reword($hash, $comment, true)
	}};
	(Squash $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_squash($hash, $comment, false)
	}};
	(Selected Squash $hash:expr, $comment:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_squash($hash, $comment, true)
	}};
	(Exec $command:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_exec($command, false)
	}};
	(Selected Exec $command:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_exec($command, true)
	}};
	(Label $reference:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_label($reference, false)
	}};
	(Selected Label $reference:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_label($reference, true)
	}};
	(Reset $reference:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_reset($reference, false)
	}};
	(Selected Reset $reference:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_reset($reference, true)
	}};
	(Merge $reference:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_merge($reference, false)
	}};
	(Selected Merge $reference:expr) => {{
		use $crate::testutil::ActionPattern;
		ActionPattern::new_merge($reference, true)
	}};
}
