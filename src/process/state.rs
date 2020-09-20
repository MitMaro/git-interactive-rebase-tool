#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Edit,
	Error,
	ExternalEditor,
	Help,
	List,
	ShowCommit,
	WindowSizeError,
}
