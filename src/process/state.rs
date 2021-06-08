#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Error,
	ExternalEditor,
	List,
	Insert,
	ShowCommit,
	WindowSizeError,
}
