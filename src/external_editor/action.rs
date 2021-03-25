#[derive(Clone, Debug, PartialEq)]
pub enum Action {
	AbortRebase,
	EditRebase,
	RestoreAndAbortEdit,
	UndoAndEdit,
}
