#[derive(Clone, Debug, PartialEq)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Edit,
	Error { return_state: Box<State>, message: String },
	Exiting,
	ExternalEditor,
	Help(Box<State>),
	List(bool), // TODO refactor help to not require visual mode boolean
	ShowCommit,
	WindowSizeError(Box<State>),
}
