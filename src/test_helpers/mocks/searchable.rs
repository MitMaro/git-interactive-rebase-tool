use crate::search::{Interrupter, SearchResult};

pub(crate) struct Searchable;

impl Searchable {
	pub(crate) const fn new() -> Self {
		Self {}
	}
}

impl crate::search::Searchable for Searchable {
	fn reset(&mut self) {}

	fn search(&mut self, _: Interrupter, _: &str) -> SearchResult {
		SearchResult::None
	}
}
