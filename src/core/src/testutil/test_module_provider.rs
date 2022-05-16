use input::EventHandler;

use crate::{
	module::{Module, ModuleHandler, ModuleProvider, State},
	testutil::create_test_keybindings,
};

pub(crate) struct TestModuleProvider<M: Module> {
	module: M,
}

impl<M: Module> TestModuleProvider<M> {
	pub(crate) fn new(module: M) -> Self {
		Self { module }
	}
}

impl<M: Module> ModuleProvider for TestModuleProvider<M> {
	fn get_mut_module(&mut self, _state: State) -> &mut dyn Module {
		&mut self.module
	}

	fn get_module(&self, _state: State) -> &dyn Module {
		&self.module
	}
}

pub(crate) struct DefaultTestModule {}

impl Module for DefaultTestModule {}

pub(crate) fn create_test_module_handler<M: Module>(module: M) -> ModuleHandler<TestModuleProvider<M>> {
	ModuleHandler::new(
		EventHandler::new(create_test_keybindings()),
		TestModuleProvider::new(module),
	)
}

pub(crate) fn create_default_test_module_handler() -> ModuleHandler<TestModuleProvider<DefaultTestModule>> {
	ModuleHandler::new(
		EventHandler::new(create_test_keybindings()),
		TestModuleProvider::<DefaultTestModule>::new(DefaultTestModule {}),
	)
}
