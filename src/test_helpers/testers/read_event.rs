use crate::{config::Config, input::Event, test_helpers::testers};

pub(crate) fn read_event<C>(event: Event, config: Option<Config>, callback: C)
where C: FnOnce(testers::ModuleTestContext) {
	testers::module(&[], &[event], config, callback);
}
