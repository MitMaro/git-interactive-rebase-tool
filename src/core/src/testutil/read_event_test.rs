use crate::{
	events::Event,
	testutil::{module_test, module_test::TestContext},
};

pub(crate) fn read_event_test<C>(event: Event, callback: C)
where C: FnOnce(TestContext) {
	module_test(&[], &[event], callback);
}
