use crate::search::{Interrupter, SearchResult, Searchable};

pub(crate) struct MockedSearchable;

impl MockedSearchable {
	pub(crate) const fn new() -> Self {
		Self {}
	}
}

impl Searchable for MockedSearchable {
	fn reset(&mut self) {}

	fn search(&mut self, _: Interrupter, _: &str) -> SearchResult {
		SearchResult::None
	}
}
