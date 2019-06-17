#[derive(Clone, Debug, PartialEq)]
pub enum State {
	ConfirmAbort,
	ConfirmRebase,
	Edit,
	Error { return_state: Box<State>, message: String },
	Exiting,
	ExternalEditor(Box<State>),
	ExternalEditorError,
	ExternalEditorFinish(Box<State>),
	Help(Box<State>),
	List,
	ShowCommit,
	VisualMode,
	WindowSizeError(Box<State>),
}
