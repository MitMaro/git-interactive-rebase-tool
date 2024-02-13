use crate::search::State;

pub(crate) struct SearchTestContext {
	pub(crate) state: State,
}

pub(crate) fn with_search<C>(callback: C)
where C: FnOnce(SearchTestContext) {
	callback(SearchTestContext { state: State::new() });
}
