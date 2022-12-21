use crate::search::State;

pub(crate) struct TestContext {
	pub(crate) state: State,
}

pub(crate) fn with_search<C>(callback: C)
where C: FnOnce(TestContext) {
	callback(TestContext { state: State::new() });
}
