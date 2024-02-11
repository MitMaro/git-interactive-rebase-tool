#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum State {
	ConfirmAbort,
	ConfirmRebase,
	Error,
	ExternalEditor,
	List,
	Insert,
	ShowCommit,
	WindowSizeError,
}
