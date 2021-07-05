use anyhow::Error;

#[derive(Debug)]
pub enum ExternalEditorState {
	Active,
	Empty,
	Error(Error),
}
