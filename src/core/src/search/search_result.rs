#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum SearchResult {
	None,
	Complete,
	Updated,
}
