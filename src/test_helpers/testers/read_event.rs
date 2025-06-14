use crate::{input::Event, test_helpers::testers};

pub(crate) fn read_event<C>(event: Event, callback: C)
where C: FnOnce(testers::ModuleTestContext) {
	testers::module(&[], &[event], callback);
}
