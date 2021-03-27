#[derive(Clone, Copy, Debug, PartialEq)]
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
