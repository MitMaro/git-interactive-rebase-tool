#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Action {
	AbortRebase,
	EditRebase,
	RestoreAndAbortEdit,
	UndoAndEdit,
}
