#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Action {
	AbortRebase,
	EditRebase,
	RestoreAndAbortEdit,
	UndoAndEdit,
}
