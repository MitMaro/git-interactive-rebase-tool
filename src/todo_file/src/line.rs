use crate::{errors::ParseError, Action};

/// Represents a line in the rebase file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
	action: Action,
	content: String,
	hash: String,
	mutated: bool,
}

impl Line {
	/// Create a new noop line.
	#[must_use]
	const fn new_noop() -> Self {
		Self {
			action: Action::Noop,
			content: String::new(),
			hash: String::new(),
			mutated: false,
		}
	}

	/// Create a new pick line.
	#[must_use]
	#[inline]
	pub fn new_pick(hash: &str) -> Self {
		Self {
			action: Action::Pick,
			content: String::new(),
			hash: String::from(hash),
			mutated: false,
		}
	}

	/// Create a new break line.
	#[must_use]
	#[inline]
	pub const fn new_break() -> Self {
		Self {
			action: Action::Break,
			content: String::new(),
			hash: String::new(),
			mutated: false,
		}
	}

	/// Create a new exec line.
	#[must_use]
	#[inline]
	pub fn new_exec(command: &str) -> Self {
		Self {
			action: Action::Exec,
			content: String::from(command),
			hash: String::new(),
			mutated: false,
		}
	}

	/// Create a new merge line.
	#[must_use]
	#[inline]
	pub fn new_merge(command: &str) -> Self {
		Self {
			action: Action::Merge,
			content: String::from(command),
			hash: String::new(),
			mutated: false,
		}
	}

	/// Create a new label line.
	#[must_use]
	#[inline]
	pub fn new_label(label: &str) -> Self {
		Self {
			action: Action::Label,
			content: String::from(label),
			hash: String::new(),
			mutated: false,
		}
	}

	/// Create a new reset line.
	#[must_use]
	#[inline]
	pub fn new_reset(label: &str) -> Self {
		Self {
			action: Action::Reset,
			content: String::from(label),
			hash: String::new(),
			mutated: false,
		}
	}

	/// Create a new line from a rebase file line.
	///
	/// # Errors
	///
	/// Returns an error if an invalid line is provided.
	#[inline]
	pub fn new(input_line: &str) -> Result<Self, ParseError> {
		if input_line.starts_with("noop") {
			return Ok(Self::new_noop());
		}
		else if input_line.starts_with("break") || input_line.starts_with('b') {
			return Ok(Self::new_break());
		}
		else if input_line.starts_with("exec")
			|| input_line.starts_with('x')
			|| input_line.starts_with("merge")
			|| input_line.starts_with('m')
			|| input_line.starts_with("label")
			|| input_line.starts_with('l')
			|| input_line.starts_with("reset")
			|| input_line.starts_with('t')
		{
			let input: Vec<&str> = input_line.splitn(2, ' ').collect();
			if input.len() == 2 {
				return Ok(Self {
					action: Action::try_from(input[0])?,
					hash: String::new(),
					content: String::from(input[1]),
					mutated: false,
				});
			}
		}
		else {
			let input: Vec<&str> = input_line.splitn(3, ' ').collect();
			if input.len() >= 2 {
				return Ok(Self {
					action: Action::try_from(input[0])?,
					hash: String::from(input[1]),
					content: if input.len() == 3 {
						String::from(input[2])
					}
					else {
						String::new()
					},
					mutated: false,
				});
			}
		}

		Err(ParseError::InvalidLine(String::from(input_line)))
	}

	/// Set the action of the line.
	#[inline]
	pub fn set_action(&mut self, action: Action) {
		if !self.action.is_static() && self.action != action {
			self.mutated = true;
			self.action = action;
		}
	}

	/// Edit the content of the line, if it is editable.
	#[inline]
	pub fn edit_content(&mut self, content: &str) {
		if self.is_editable() {
			self.content = String::from(content);
		}
	}

	/// Get the action of the line.
	#[must_use]
	#[inline]
	pub const fn get_action(&self) -> &Action {
		&self.action
	}

	/// Get the content of the line.
	#[must_use]
	#[inline]
	pub fn get_content(&self) -> &str {
		self.content.as_str()
	}

	/// Get the commit hash for the line.
	#[must_use]
	#[inline]
	pub fn get_hash(&self) -> &str {
		self.hash.as_str()
	}

	/// Does this line contain a commit reference.
	#[must_use]
	#[inline]
	pub fn has_reference(&self) -> bool {
		!self.hash.is_empty()
	}

	/// Can this line be edited.
	#[must_use]
	#[inline]
	pub const fn is_editable(&self) -> bool {
		match self.action {
			Action::Exec | Action::Label | Action::Reset | Action::Merge => true,
			Action::Break
			| Action::Drop
			| Action::Edit
			| Action::Fixup
			| Action::Noop
			| Action::Pick
			| Action::Reword
			| Action::Squash => false,
		}
	}

	/// Create a string containing a textual version of the line, as would be seen in the rebase file.
	#[must_use]
	#[inline]
	pub fn to_text(&self) -> String {
		match self.action {
			Action::Drop | Action::Edit | Action::Fixup | Action::Pick | Action::Reword | Action::Squash => {
				format!("{} {} {}", self.action, self.hash, self.content)
			},
			Action::Exec | Action::Label | Action::Reset | Action::Merge => {
				format!("{} {}", self.action, self.content)
			},
			Action::Noop | Action::Break => self.action.to_string(),
		}
	}
}

#[cfg(test)]
mod tests {
	use claim::assert_ok_eq;
	use rstest::rstest;
	use testutils::assert_err_eq;

	use super::*;

	#[rstest]
	#[case::pick_action("pick aaa comment", &Line {
		action: Action::Pick,
		hash: String::from("aaa"),
		content: String::from("comment"),
		mutated: false,
	})]
	#[case::reword_action("reword aaa comment", &Line {
		action: Action::Reword,
		hash: String::from("aaa"),
		content: String::from("comment"),
		mutated: false,
	})]
	#[case::edit_action("edit aaa comment", &Line {
		action: Action::Edit,
		hash: String::from("aaa"),
		content: String::from("comment"),
		mutated: false,
	})]
	#[case::squash_action("squash aaa comment", &Line {
		action: Action::Squash,
		hash: String::from("aaa"),
		content: String::from("comment"),
		mutated: false,
	})]
	#[case::fixup_action("fixup aaa comment", &Line {
		action: Action::Fixup,
		hash: String::from("aaa"),
		content: String::from("comment"),
		mutated: false,
	})]
	#[case::drop_action("drop aaa comment", &Line {
		action: Action::Drop,
		hash: String::from("aaa"),
		content: String::from("comment"),
		mutated: false,
	})]
	#[case::action_without_comment("pick aaa", &Line {
		action: Action::Pick,
		hash: String::from("aaa"),
		content: String::new(),
		mutated: false,
	})]
	#[case::exec_action("exec command", &Line {
		action: Action::Exec,
		hash: String::new(),
		content: String::from("command"),
		mutated: false,
	})]
	#[case::label_action("label ref", &Line {
		action: Action::Label,
		hash: String::new(),
		content: String::from("ref"),
		mutated: false,
	})]
	#[case::reset_action("reset ref", &Line {
		action: Action::Reset,
		hash: String::new(),
		content: String::from("ref"),
		mutated: false,
	})]
	#[case::reset_action("merge command", &Line {
		action: Action::Merge,
		hash: String::new(),
		content: String::from("command"),
		mutated: false,
	})]
	#[case::break_action("break", &Line {
		action: Action::Break,
		hash: String::new(),
		content: String::new(),
		mutated: false,
	})]
	#[case::nnop( "noop", &Line {
		action: Action::Noop,
		hash: String::new(),
		content: String::new(),
		mutated: false,
	})]
	fn new(#[case] line: &str, #[case] expected: &Line) {
		assert_ok_eq!(&Line::new(line), expected);
	}

	#[test]
	fn line_new_pick() {
		assert_eq!(Line::new_pick("abc123"), Line {
			action: Action::Pick,
			hash: String::from("abc123"),
			content: String::new(),
			mutated: false,
		});
	}

	#[test]
	fn line_new_break() {
		assert_eq!(Line::new_break(), Line {
			action: Action::Break,
			hash: String::new(),
			content: String::new(),
			mutated: false,
		});
	}

	#[test]
	fn line_new_exec() {
		assert_eq!(Line::new_exec("command"), Line {
			action: Action::Exec,
			hash: String::new(),
			content: String::from("command"),
			mutated: false,
		});
	}

	#[test]
	fn line_new_merge() {
		assert_eq!(Line::new_merge("command"), Line {
			action: Action::Merge,
			hash: String::new(),
			content: String::from("command"),
			mutated: false,
		});
	}

	#[test]
	fn line_new_label() {
		assert_eq!(Line::new_label("label"), Line {
			action: Action::Label,
			hash: String::new(),
			content: String::from("label"),
			mutated: false,
		});
	}

	#[test]
	fn line_new_reset() {
		assert_eq!(Line::new_reset("label"), Line {
			action: Action::Reset,
			hash: String::new(),
			content: String::from("label"),
			mutated: false,
		});
	}

	#[test]
	fn new_err_invalid_action() {
		assert_err_eq!(
			Line::new("invalid aaa comment"),
			ParseError::InvalidAction(String::from("invalid"))
		);
	}

	#[rstest]
	#[case::invalid_line_only("invalid")]
	#[case::pick_line_only("pick")]
	#[case::reword_line_only("reword")]
	#[case::edit_line_only("edit")]
	#[case::squash_line_only("squash")]
	#[case::fixup_line_only("fixup")]
	#[case::exec_line_only("exec")]
	#[case::drop_line_only("drop")]
	#[case::label_line_only("label")]
	#[case::reset_line_only("reset")]
	#[case::merge_line_only("merge")]
	fn new_err(#[case] line: &str) {
		assert_err_eq!(Line::new(line), ParseError::InvalidLine(String::from(line)));
	}

	#[rstest]
	#[case::drop(Action::Drop, Action::Fixup)]
	#[case::edit(Action::Edit, Action::Fixup)]
	#[case::fixup(Action::Fixup, Action::Pick)]
	#[case::pick(Action::Pick, Action::Fixup)]
	#[case::reword(Action::Reword, Action::Fixup)]
	#[case::squash(Action::Squash, Action::Fixup)]
	fn set_action_non_static(#[case] from: Action, #[case] to: Action) {
		let mut line = Line::new(format!("{from} aaa bbb").as_str()).unwrap();
		line.set_action(to);
		assert_eq!(line.action, to);
		assert!(line.mutated);
	}

	#[rstest]
	#[case::break_action(Action::Break, Action::Fixup)]
	#[case::label_action(Action::Label, Action::Fixup)]
	#[case::reset_action(Action::Reset, Action::Fixup)]
	#[case::merge_action(Action::Merge, Action::Fixup)]
	#[case::exec(Action::Exec, Action::Fixup)]
	#[case::noop(Action::Noop, Action::Fixup)]
	fn set_action_static(#[case] from: Action, #[case] to: Action) {
		let mut line = Line::new(format!("{from} comment").as_str()).unwrap();
		line.set_action(to);
		assert_eq!(line.action, from);
		assert!(!line.mutated);
	}

	#[test]
	fn set_to_new_action_with_changed_action() {
		let mut line = Line::new("pick aaa comment").unwrap();
		line.set_action(Action::Fixup);
		assert_eq!(line.action, Action::Fixup);
		assert!(line.mutated);
	}

	#[test]
	fn set_to_new_action_with_unchanged_action() {
		let mut line = Line::new("pick aaa comment").unwrap();
		line.set_action(Action::Pick);
		assert_eq!(line.action, Action::Pick);
		assert!(!line.mutated);
	}

	#[rstest]
	#[case::break_action("break", "")]
	#[case::drop("drop aaa comment", "comment")]
	#[case::edit("edit aaa comment", "comment")]
	#[case::exec("exec git commit --amend 'foo'", "new")]
	#[case::fixup("fixup aaa comment", "comment")]
	#[case::pick("pick aaa comment", "comment")]
	#[case::reword("reword aaa comment", "comment")]
	#[case::squash("squash aaa comment", "comment")]
	#[case::label("label ref", "new")]
	#[case::reset("reset ref", "new")]
	#[case::merge("merge command", "new")]
	fn edit_content(#[case] line: &str, #[case] expected: &str) {
		let mut line = Line::new(line).unwrap();
		line.edit_content("new");
		assert_eq!(line.get_content(), expected);
	}

	#[rstest]
	#[case::break_action("break", "")]
	#[case::drop("drop aaa comment", "comment")]
	#[case::edit("edit aaa comment", "comment")]
	#[case::exec("exec git commit --amend 'foo'", "git commit --amend 'foo'")]
	#[case::fixup("fixup aaa comment", "comment")]
	#[case::pick("pick aaa comment", "comment")]
	#[case::reword("reword aaa comment", "comment")]
	#[case::squash("squash aaa comment", "comment")]
	fn get_content(#[case] line: &str, #[case] expected: &str) {
		assert_eq!(Line::new(line).unwrap().get_content(), expected);
	}

	#[rstest]
	#[case::break_action("break", Action::Break)]
	#[case::drop("drop aaa comment", Action::Drop)]
	#[case::edit("edit aaa comment", Action::Edit)]
	#[case::exec("exec git commit --amend 'foo'", Action::Exec)]
	#[case::fixup("fixup aaa comment", Action::Fixup)]
	#[case::pick("pick aaa comment", Action::Pick)]
	#[case::reword("reword aaa comment", Action::Reword)]
	#[case::squash("squash aaa comment", Action::Squash)]
	fn get_action(#[case] line: &str, #[case] expected: Action) {
		assert_eq!(Line::new(line).unwrap().get_action(), &expected);
	}

	#[rstest]
	#[case::break_action("break", "")]
	#[case::drop("drop aaa comment", "aaa")]
	#[case::edit("edit aaa comment", "aaa")]
	#[case::exec("exec git commit --amend 'foo'", "")]
	#[case::fixup("fixup aaa comment", "aaa")]
	#[case::pick("pick aaa comment", "aaa")]
	#[case::reword("reword aaa comment", "aaa")]
	#[case::squash("squash aaa comment", "aaa")]
	fn get_hash(#[case] line: &str, #[case] expected: &str) {
		assert_eq!(Line::new(line).unwrap().get_hash(), expected);
	}

	#[rstest]
	#[case::break_action("break", false)]
	#[case::drop("drop aaa comment", true)]
	#[case::edit("edit aaa comment", true)]
	#[case::exec("exec git commit --amend 'foo'", false)]
	#[case::fixup("fixup aaa comment", true)]
	#[case::pick("pick aaa comment", true)]
	#[case::reword("reword aaa comment", true)]
	#[case::squash("squash aaa comment", true)]
	#[case::label("label ref", false)]
	#[case::reset("reset ref", false)]
	#[case::merge("merge command", false)]
	fn has_reference(#[case] line: &str, #[case] expected: bool) {
		assert_eq!(Line::new(line).unwrap().has_reference(), expected);
	}

	#[rstest]
	#[case::drop(Action::Break, false)]
	#[case::drop(Action::Drop, false)]
	#[case::edit(Action::Edit, false)]
	#[case::fixup(Action::Fixup, false)]
	#[case::pick(Action::Noop, false)]
	#[case::pick(Action::Pick, false)]
	#[case::reword(Action::Reword, false)]
	#[case::squash(Action::Squash, false)]
	#[case::squash(Action::Exec, true)]
	#[case::squash(Action::Label, true)]
	#[case::squash(Action::Reset, true)]
	#[case::squash(Action::Merge, true)]
	fn is_editable(#[case] from: Action, #[case] editable: bool) {
		let line = Line::new(format!("{from} aaa bbb").as_str()).unwrap();
		assert_eq!(line.is_editable(), editable);
	}

	#[rstest]
	#[case::break_action("break")]
	#[case::drop("drop aaa comment")]
	#[case::edit("edit aaa comment")]
	#[case::exec("exec git commit --amend 'foo'")]
	#[case::fixup("fixup aaa comment")]
	#[case::pick("pick aaa comment")]
	#[case::reword("reword aaa comment")]
	#[case::squash("squash aaa comment")]
	fn to_text(#[case] line: &str) {
		assert_eq!(Line::new(line).unwrap().to_text(), line);
	}
}
