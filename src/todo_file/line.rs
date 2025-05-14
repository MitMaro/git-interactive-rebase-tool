use crate::todo_file::{Action, LineParser, ParseError};

/// Represents a line in the rebase file.
#[derive(Clone, Debug, PartialEq, Eq)]
#[expect(clippy::struct_field_names, reason = "Clarity")]
pub(crate) struct Line {
	action: Action,
	content: String,
	hash: String,
	mutated: bool,
	option: Option<String>,
	original_line: Option<Box<Line>>,
}

impl Line {
	fn new(action: Action, hash: &str, content: &str, option: Option<&str>) -> Self {
		let original_action = action;
		let original_content = String::from(content);
		let original_option = option.map(String::from);

		Self {
			action,
			content: String::from(content),
			hash: String::from(hash),
			mutated: false,
			option: original_option.clone(),
			original_line: Some(Box::new(Line {
				action: original_action,
				content: original_content,
				hash: String::from(hash),
				mutated: false,
				option: original_option,
				original_line: None,
			})),
		}
	}

	/// Create a new noop line.
	#[must_use]
	fn new_noop() -> Self {
		Self::new(Action::Noop, "", "", None)
	}

	/// Create a new pick line.
	#[must_use]
	pub(crate) fn new_pick(hash: &str) -> Self {
		Self::new(Action::Pick, hash, "", None)
	}

	/// Create a new break line.
	#[must_use]
	pub(crate) fn new_break() -> Self {
		Self::new(Action::Break, "", "", None)
	}

	/// Create a new exec line.
	#[must_use]
	pub(crate) fn new_exec(command: &str) -> Self {
		Self::new(Action::Exec, "", command, None)
	}

	/// Create a new merge line.
	#[must_use]
	pub(crate) fn new_merge(label: &str) -> Self {
		Self::new(Action::Merge, "", label, None)
	}

	/// Create a new label line.
	#[must_use]
	pub(crate) fn new_label(label: &str) -> Self {
		Self::new(Action::Label, "", label, None)
	}

	/// Create a new reset line.
	#[must_use]
	pub(crate) fn new_reset(label: &str) -> Self {
		Self::new(Action::Reset, "", label, None)
	}

	/// Create a new update-ref line.
	#[must_use]
	pub(crate) fn new_update_ref(ref_name: &str) -> Self {
		Self::new(Action::UpdateRef, "", ref_name, None)
	}

	/// Create a new line from a rebase file line.
	///
	/// # Errors
	///
	/// Returns an error if an invalid line is provided.
	pub(crate) fn parse(input_line: &str) -> Result<Self, ParseError> {
		let mut line_parser = LineParser::new(input_line);

		let action = Action::try_from(line_parser.next()?)?;
		Ok(match action {
			Action::Noop => Self::new_noop(),
			Action::Break => Self::new_break(),
			Action::Pick | Action::Reword | Action::Edit | Action::Squash | Action::Drop => {
				Self::new(action, line_parser.next()?, line_parser.take_remaining(), None)
			},
			Action::Fixup => {
				let mut next = line_parser.next()?;

				let option = if next.starts_with('-') {
					let opt = String::from(next);
					next = line_parser.next()?;
					Some(opt)
				}
				else {
					None
				};

				Self::new(action, next, line_parser.take_remaining(), option.as_deref())
			},
			Action::Exec | Action::Merge | Action::Label | Action::Reset | Action::UpdateRef => {
				if !line_parser.has_more() {
					return Err(line_parser.parse_error());
				}
				Self::new(action, "", line_parser.take_remaining(), None)
			},
		})
	}

	/// Set the action of the line.
	pub(crate) fn set_action(&mut self, action: Action) {
		if !self.action.is_static() && self.action != action {
			self.mutated = true;
			self.action = action;
			self.option = None;
		}
	}

	/// Edit the content of the line, if it is editable.
	pub(crate) fn edit_content(&mut self, content: &str) {
		if self.is_editable() {
			self.content = String::from(content);
			self.mutated = true;
		}
	}

	/// Set the option on the line, toggling if the existing option matches.
	pub(crate) fn toggle_option(&mut self, option: &str) {
		// try toggle off first
		if let Some(current) = self.option.as_deref() {
			if current == option {
				self.option = None;
				return;
			}
		}
		self.option = Some(String::from(option));
	}

	/// Get the original line, before any modifications
	#[must_use]
	pub(crate) fn original(&self) -> Option<&Line> {
		self.original_line.as_deref()
	}

	/// Get the action of the line.
	#[must_use]
	pub(crate) const fn get_action(&self) -> &Action {
		&self.action
	}

	/// Get the content of the line.
	#[must_use]
	pub(crate) fn get_content(&self) -> &str {
		self.content.as_str()
	}

	/// Get the commit hash for the line.
	#[must_use]
	pub(crate) fn get_hash(&self) -> &str {
		self.hash.as_str()
	}

	/// Get the commit hash for the line.
	#[must_use]
	pub(crate) fn option(&self) -> Option<&str> {
		self.option.as_deref()
	}

	/// Does this line contain a commit reference.
	#[must_use]
	pub(crate) fn has_reference(&self) -> bool {
		!self.hash.is_empty()
	}

	/// Can this line be edited.
	#[must_use]
	pub(crate) const fn is_editable(&self) -> bool {
		match self.action {
			Action::Exec | Action::Label | Action::Reset | Action::Merge | Action::UpdateRef => true,
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

	/// Can this line be duplicated.
	#[must_use]
	pub(crate) const fn is_duplicatable(&self) -> bool {
		match self.action {
			Action::Exec
			| Action::Label
			| Action::Reset
			| Action::Merge
			| Action::UpdateRef
			| Action::Drop
			| Action::Edit
			| Action::Fixup
			| Action::Pick
			| Action::Reword
			| Action::Squash => true,
			Action::Break | Action::Noop => false,
		}
	}

	/// Has this line been modified
	#[must_use]
	pub(crate) fn is_modified(&self) -> bool {
		self.mutated
	}

	/// Create a string containing a textual version of the line, as would be seen in the rebase file.
	#[must_use]
	pub(crate) fn to_text(&self) -> String {
		match self.action {
			Action::Drop | Action::Edit | Action::Fixup | Action::Pick | Action::Reword | Action::Squash => {
				if let Some(opt) = self.option.as_ref() {
					format!("{} {opt} {} {}", self.action, self.hash, self.content)
				}
				else {
					format!("{} {} {}", self.action, self.hash, self.content)
				}
			},
			Action::Exec | Action::Label | Action::Reset | Action::Merge | Action::UpdateRef => {
				format!("{} {}", self.action, self.content)
			},
			Action::Noop | Action::Break => self.action.to_string(),
		}
	}
}

#[cfg(test)]
mod tests {
	use claims::{assert_err_eq, assert_ok_eq};
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::pick_action("pick aaa comment", &Line::new(Action::Pick, "aaa", "comment", None))]
	#[case::reword_action("reword aaa comment", &Line::new(Action::Reword, "aaa", "comment", None))]
	#[case::edit_action("edit aaa comment", &Line::new(Action::Edit, "aaa", "comment", None))]
	#[case::squash_action("squash aaa comment", &Line::new(Action::Squash, "aaa", "comment", None))]
	#[case::fixup_action("fixup aaa comment", & Line::new(Action::Fixup, "aaa", "comment", None))]
	#[case::fixup_with_option_action("fixup -c aaa comment", &Line::new(Action::Fixup, "aaa", "comment", Some("-c")))]
	#[case::drop_action("drop aaa comment", &Line::new(Action::Drop, "aaa", "comment", None))]
	#[case::action_without_comment("pick aaa", &Line::new(Action::Pick, "aaa", "", None))]
	#[case::exec_action("exec command", &Line::new(Action::Exec, "", "command", None))]
	#[case::label_action("label ref", &Line::new(Action::Label, "", "ref", None))]
	#[case::reset_action("reset ref", &Line::new(Action::Reset, "", "ref", None))]
	#[case::reset_action("merge command", &Line::new(Action::Merge, "", "command", None))]
	#[case::update_ref_action("update-ref reference", &Line::new(Action::UpdateRef, "", "reference", None))]
	#[case::break_action("break", &Line::new(Action::Break, "", "", None))]
	#[case::noop( "noop", &Line::new(Action::Noop, "", "", None))]
	fn new(#[case] line: &str, #[case] expected: &Line) {
		assert_ok_eq!(&Line::parse(line), expected);
	}

	#[test]
	fn line_new_pick() {
		assert_eq!(Line::new_pick("abc123"), Line {
			action: Action::Pick,
			hash: String::from("abc123"),
			content: String::new(),
			mutated: false,
			option: None,
			original_line: Some(Box::new(Line {
				action: Action::Pick,
				hash: String::from("abc123"),
				content: String::new(),
				mutated: false,
				option: None,
				original_line: None,
			}))
		});
	}

	#[test]
	fn line_new_break() {
		assert_eq!(Line::new_break(), Line {
			action: Action::Break,
			hash: String::new(),
			content: String::new(),
			mutated: false,
			option: None,
			original_line: Some(Box::new(Line {
				action: Action::Break,
				hash: String::new(),
				content: String::new(),
				mutated: false,
				option: None,
				original_line: None,
			}))
		});
	}

	#[test]
	fn line_new_exec() {
		assert_eq!(Line::new_exec("command"), Line {
			action: Action::Exec,
			hash: String::new(),
			content: String::from("command"),
			mutated: false,
			option: None,
			original_line: Some(Box::new(Line {
				action: Action::Exec,
				hash: String::new(),
				content: String::from("command"),
				mutated: false,
				option: None,
				original_line: None,
			}))
		});
	}

	#[test]
	fn line_new_merge() {
		assert_eq!(Line::new_merge("command"), Line {
			action: Action::Merge,
			hash: String::new(),
			content: String::from("command"),
			mutated: false,
			option: None,
			original_line: Some(Box::new(Line {
				action: Action::Merge,
				hash: String::new(),
				content: String::from("command"),
				mutated: false,
				option: None,
				original_line: None,
			}))
		});
	}

	#[test]
	fn line_new_label() {
		assert_eq!(Line::new_label("label"), Line {
			action: Action::Label,
			hash: String::new(),
			content: String::from("label"),
			mutated: false,
			option: None,
			original_line: Some(Box::new(Line {
				action: Action::Label,
				hash: String::new(),
				content: String::from("label"),
				mutated: false,
				option: None,
				original_line: None,
			}))
		});
	}

	#[test]
	fn line_new_reset() {
		assert_eq!(Line::new_reset("label"), Line {
			action: Action::Reset,
			hash: String::new(),
			content: String::from("label"),
			mutated: false,
			option: None,
			original_line: Some(Box::new(Line {
				action: Action::Reset,
				hash: String::new(),
				content: String::from("label"),
				mutated: false,
				option: None,
				original_line: None,
			}))
		});
	}

	#[test]
	fn line_new_update_ref() {
		assert_eq!(Line::new_update_ref("reference"), Line {
			action: Action::UpdateRef,
			hash: String::new(),
			content: String::from("reference"),
			mutated: false,
			option: None,
			original_line: Some(Box::new(Line {
				action: Action::UpdateRef,
				hash: String::new(),
				content: String::from("reference"),
				mutated: false,
				option: None,
				original_line: None,
			}))
		});
	}

	#[test]
	fn new_err_invalid_action() {
		assert_err_eq!(
			Line::parse("invalid aaa comment"),
			ParseError::InvalidAction(String::from("invalid"))
		);
	}

	#[rstest]
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
	#[case::update_ref_line_only("update-ref")]
	fn new_err(#[case] line: &str) {
		assert_err_eq!(Line::parse(line), ParseError::InvalidLine(String::from(line)));
	}

	#[rstest]
	#[case::drop(Action::Drop, Action::Fixup)]
	#[case::edit(Action::Edit, Action::Fixup)]
	#[case::fixup(Action::Fixup, Action::Pick)]
	#[case::pick(Action::Pick, Action::Fixup)]
	#[case::reword(Action::Reword, Action::Fixup)]
	#[case::squash(Action::Squash, Action::Fixup)]
	fn set_action_non_static(#[case] from: Action, #[case] to: Action) {
		let mut line = Line::parse(format!("{from} aaa bbb").as_str()).unwrap();
		line.set_action(to);
		assert_eq!(line.action, to);
		assert!(line.is_modified());
	}

	#[rstest]
	#[case::break_action(Action::Break, Action::Fixup)]
	#[case::label_action(Action::Label, Action::Fixup)]
	#[case::reset_action(Action::Reset, Action::Fixup)]
	#[case::merge_action(Action::Merge, Action::Fixup)]
	#[case::exec(Action::Exec, Action::Fixup)]
	#[case::update_ref(Action::UpdateRef, Action::Fixup)]
	#[case::noop(Action::Noop, Action::Fixup)]
	fn set_action_static(#[case] from: Action, #[case] to: Action) {
		let mut line = Line::parse(format!("{from} comment").as_str()).unwrap();
		line.set_action(to);
		assert_eq!(line.action, from);
		assert!(!line.is_modified());
	}

	#[test]
	fn set_to_new_action_with_changed_action() {
		let mut line = Line::parse("pick aaa comment").unwrap();
		line.set_action(Action::Fixup);
		assert_eq!(line.action, Action::Fixup);
		assert!(line.is_modified());
	}

	#[test]
	fn set_to_new_action_with_unchanged_action() {
		let mut line = Line::parse("pick aaa comment").unwrap();
		line.set_action(Action::Pick);
		assert_eq!(line.action, Action::Pick);
		assert!(!line.is_modified());
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
	#[case::update_ref("update-ref reference", "new")]
	fn edit_content(#[case] line: &str, #[case] expected: &str) {
		let mut line = Line::parse(line).unwrap();
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
	#[case::label("label reference", "reference")]
	#[case::reset("reset reference", "reference")]
	#[case::merge("merge command", "command")]
	#[case::update_ref("update-ref reference", "reference")]
	fn get_content(#[case] line: &str, #[case] expected: &str) {
		assert_eq!(Line::parse(line).unwrap().get_content(), expected);
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
	#[case::label("label reference", Action::Label)]
	#[case::reset("reset reference", Action::Reset)]
	#[case::merge("merge command", Action::Merge)]
	#[case::update_ref("update-ref reference", Action::UpdateRef)]
	fn get_action(#[case] line: &str, #[case] expected: Action) {
		assert_eq!(Line::parse(line).unwrap().get_action(), &expected);
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
	#[case::label("label reference", "")]
	#[case::reset("reset reference", "")]
	#[case::merge("merge command", "")]
	#[case::update_ref("update-ref reference", "")]
	fn get_hash(#[case] line: &str, #[case] expected: &str) {
		assert_eq!(Line::parse(line).unwrap().get_hash(), expected);
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
	#[case::update_ref("update-ref reference", false)]
	fn has_reference(#[case] line: &str, #[case] expected: bool) {
		assert_eq!(Line::parse(line).unwrap().has_reference(), expected);
	}

	#[rstest]
	#[case::drop(Action::Break, false)]
	#[case::drop(Action::Drop, false)]
	#[case::edit(Action::Edit, false)]
	#[case::exec(Action::Exec, true)]
	#[case::fixup(Action::Fixup, false)]
	#[case::pick(Action::Noop, false)]
	#[case::pick(Action::Pick, false)]
	#[case::reword(Action::Reword, false)]
	#[case::squash(Action::Squash, false)]
	#[case::label(Action::Label, true)]
	#[case::reset(Action::Reset, true)]
	#[case::merge(Action::Merge, true)]
	#[case::update_ref(Action::UpdateRef, true)]
	fn is_editable(#[case] from: Action, #[case] editable: bool) {
		let line = Line::parse(format!("{from} aaa bbb").as_str()).unwrap();
		assert_eq!(line.is_editable(), editable);
	}

	#[rstest]
	#[case::drop(Action::Break, false)]
	#[case::drop(Action::Drop, true)]
	#[case::edit(Action::Edit, true)]
	#[case::exec(Action::Exec, true)]
	#[case::fixup(Action::Fixup, true)]
	#[case::pick(Action::Noop, false)]
	#[case::pick(Action::Pick, true)]
	#[case::reword(Action::Reword, true)]
	#[case::squash(Action::Squash, true)]
	#[case::label(Action::Label, true)]
	#[case::reset(Action::Reset, true)]
	#[case::merge(Action::Merge, true)]
	#[case::update_ref(Action::UpdateRef, true)]
	fn is_duplicatable(#[case] from: Action, #[case] duplicatable: bool) {
		let line = Line::parse(format!("{from} aaa bbb").as_str()).unwrap();
		assert_eq!(line.is_duplicatable(), duplicatable);
	}

	#[rstest]
	#[case::break_action("break")]
	#[case::drop("drop aaa comment")]
	#[case::edit("edit aaa comment")]
	#[case::exec("exec git commit --amend 'foo'")]
	#[case::fixup("fixup aaa comment")]
	#[case::fixup_with_options("fixup -c aaa comment")]
	#[case::pick("pick aaa comment")]
	#[case::reword("reword aaa comment")]
	#[case::squash("squash aaa comment")]
	#[case::label("label reference")]
	#[case::reset("reset reference")]
	#[case::merge("merge command")]
	#[case::update_ref("update-ref reference")]
	fn to_text(#[case] line: &str) {
		assert_eq!(Line::parse(line).unwrap().to_text(), line);
	}
}
