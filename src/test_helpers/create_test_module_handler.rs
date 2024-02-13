use crate::{
	input::EventHandler,
	module::{Module, ModuleHandler},
	test_helpers::{create_test_keybindings, shared::TestModuleProvider},
};

pub(crate) fn create_test_module_handler<M: Module>(module: M) -> ModuleHandler<TestModuleProvider<M>> {
	ModuleHandler::new(
		EventHandler::new(create_test_keybindings()),
		TestModuleProvider::from(module),
	)
}
