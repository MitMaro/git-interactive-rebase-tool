#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum SearchState {
	Inactive,
	Active,
	Complete,
}
