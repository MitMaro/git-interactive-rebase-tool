/// An action to send to the thread handling updates to the view.
#[derive(Debug, Copy, Clone)]
pub(crate) enum ViewAction {
	/// Stop processing actions.
	Stop,
	/// Force a refresh of the view.
	Refresh,
	/// Render the latest `ViewData`.
	Render,
	/// Start processing actions.
	Start,
	/// End the thread and the processing of actions.
	End,
}
