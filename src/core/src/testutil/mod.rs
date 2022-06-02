mod assert_results;
mod create_test_keybindings;
mod module_test;
mod process_test;
mod set_git_directory;
mod test_module_provider;
mod with_event_handler;

pub(crate) use self::{
	assert_results::_assert_results,
	create_test_keybindings::{create_test_custom_keybindings, create_test_keybindings},
	module_test::module_test,
	process_test::{process_test, TestContext as ProcessTestContext},
	set_git_directory::set_git_directory,
	test_module_provider::{
		create_default_test_module_handler,
		create_test_module_handler,
		DefaultTestModule,
		TestModuleProvider,
	},
	with_event_handler::{with_event_handler, EventHandlerTestContext},
};
