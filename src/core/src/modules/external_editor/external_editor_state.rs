use anyhow::Error;

#[derive(Debug)]
pub(crate) enum ExternalEditorState {
	Active,
	Empty,
	Error(Error),
}
