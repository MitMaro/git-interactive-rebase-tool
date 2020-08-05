#[derive(Clone, Debug, PartialEq)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Edit,
	Error { return_state: Box<State>, message: String },
	Exiting,
	ExternalEditor,
	List,
	ShowCommit,
	WindowSizeError(Box<State>),
}
