use crate::{
	input::{EventHandler, KeyBindings},
	module::{Module, ModuleHandler},
	test_helpers::shared::TestModuleProvider,
};

pub(crate) fn create_test_module_handler<M: Module>(module: M) -> ModuleHandler<TestModuleProvider<M>> {
	ModuleHandler::new(
		EventHandler::new(KeyBindings::default()),
		TestModuleProvider::from(module),
	)
}
