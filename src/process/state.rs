#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Error,
	ExternalEditor,
	Help,
	List,
	ShowCommit,
	WindowSizeError,
}
