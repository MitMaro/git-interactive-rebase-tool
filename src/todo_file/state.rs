use std::{
	fmt::{Display, Formatter},
	path::{Path, PathBuf},
};

use crate::todo_file::{FileReadErrorCause, IoError};

/// Describes the state of rebase when editing the rebase todo file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)]
pub enum State {
	/// Editing todo at start of a rebase.
	Initial,
	/// Editing todo in the middle of a rebase with --edit.
	Edit,
	/// Editing the todo file for git-revise
	Revise,
}

pub(crate) fn detect_state(filepath: &Path) -> Result<State, IoError> {
	if filepath.ends_with("git-revise-todo") {
		return Ok(State::Revise);
	}
	if let Some(parent) = filepath.parent() {
		if parent.join("stopped-sha").try_exists().map_err(|err| {
			IoError::FileRead {
				file: PathBuf::from(parent),
				cause: FileReadErrorCause::from(err),
			}
		})? {
			return Ok(State::Edit);
		}
	}
	return Ok(State::Initial);
}

impl State {}

impl Display for State {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match *self {
			Self::Initial => "initial",
			Self::Edit => "edit",
			Self::Revise => "revise",
		})
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::testutil::{with_todo_revise_file, TodoFileTestDirContext, with_todo_rebase_edit_file, with_todo_rebase_edit_file_stopped};

	#[rstest]
	#[case::edit(State::Initial, "initial")]
	#[case::edit(State::Edit, "edit")]
	#[case::edit(State::Revise, "revise")]
	fn to_string(#[case] action: State, #[case] expected: &str) {
		assert_eq!(format!("{action}"), expected);
	}

	#[rstest]
	#[case::edit(State::Initial)]
	#[case::edit(State::Edit)]
	#[case::edit(State::Revise)]
	fn detect(#[case] expected: State) {
		let check = |context: TodoFileTestDirContext| {
			assert_eq!(
				detect_state(context.todo_file().filepath.as_path()).unwrap(),
				expected
			)
		};
		match expected {
			State::Initial => with_todo_rebase_edit_file(&[], check),
			State::Edit => with_todo_rebase_edit_file_stopped(&[], check),
			State::Revise => with_todo_revise_file(&[], check),
		}
	}
}
