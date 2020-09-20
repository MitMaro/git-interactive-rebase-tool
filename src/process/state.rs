#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Edit,
	Error,
	Exiting,
	ExternalEditor,
	Help,
	List,
	ShowCommit,
	WindowSizeError,
}
