use crate::{
	input::{EventHandler, KeyBindings},
	module::{Module, ModuleHandler},
	test_helpers::TestModuleProvider,
};

pub(crate) struct DefaultTestModule;

impl Module for DefaultTestModule {}

pub(crate) fn create_default_test_module_handler() -> ModuleHandler<TestModuleProvider<DefaultTestModule>> {
	ModuleHandler::new(
		EventHandler::new(KeyBindings::default()),
		TestModuleProvider::<DefaultTestModule>::from(DefaultTestModule {}),
	)
}
