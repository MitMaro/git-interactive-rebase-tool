#[derive(Clone, Debug, PartialEq)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Edit,
	Exiting,
	ExternalEditor,
	List,
	ShowCommit,
	WindowSizeError(Box<State>),
}
