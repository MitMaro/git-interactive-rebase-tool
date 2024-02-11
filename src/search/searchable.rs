use crate::search::{Interrupter, SearchResult};

pub(crate) trait Searchable: Send {
	fn reset(&mut self);

	fn search(&mut self, interrupter: Interrupter, term: &str) -> SearchResult;
}
