#[derive(Debug, PartialEq, Eq)]
pub(crate) enum InsertState {
	Prompt,
	Edit,
}
