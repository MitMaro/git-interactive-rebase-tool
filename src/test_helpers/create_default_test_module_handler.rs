use crate::{
	input::EventHandler,
	module::{Module, ModuleHandler},
	test_helpers::{TestModuleProvider, create_test_keybindings},
};

pub(crate) struct DefaultTestModule;

impl Module for DefaultTestModule {}

pub(crate) fn create_default_test_module_handler() -> ModuleHandler<TestModuleProvider<DefaultTestModule>> {
	ModuleHandler::new(
		EventHandler::new(create_test_keybindings()),
		TestModuleProvider::<DefaultTestModule>::from(DefaultTestModule {}),
	)
}
