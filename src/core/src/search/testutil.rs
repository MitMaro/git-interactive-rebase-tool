use std::time::Duration;

use crate::search::{Interrupter, SearchResult, Searchable};

const SEARCH_INTERRUPT_TIME: Duration = Duration::from_secs(1);

pub(crate) struct SearchableRunner<S: Searchable + Clone> {
	searchable: S,
}

impl<S: Searchable + Clone> SearchableRunner<S> {
	pub(crate) fn new(searchable: &S) -> Self {
		Self {
			searchable: searchable.clone(),
		}
	}

	pub(crate) fn run_search(&mut self, search_term: &str) -> SearchResult {
		self.searchable
			.search(Interrupter::new(SEARCH_INTERRUPT_TIME), search_term)
	}

	pub(crate) fn run_search_with_time(&mut self, search_term: &str, millis: u64) -> SearchResult {
		self.searchable
			.search(Interrupter::new(Duration::from_millis(millis)), search_term)
	}
}
