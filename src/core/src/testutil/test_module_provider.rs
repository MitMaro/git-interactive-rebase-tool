use config::Config;
use git::Repository;
use input::EventHandler;

use crate::{
	module::{Module, ModuleHandler, ModuleProvider, State},
	testutil::create_test_keybindings,
};

pub(crate) struct TestModuleProvider<M: Module> {
	module: M,
}

impl<M: Module> From<M> for TestModuleProvider<M> {
	fn from(module: M) -> Self {
		Self { module }
	}
}

impl<M: Module> ModuleProvider for TestModuleProvider<M> {
	fn new(_: &Config, _: Repository) -> Self {
		unimplemented!("Not implemented for the TestModuleProvider");
	}

	fn get_mut_module(&mut self, _state: State) -> &mut dyn Module {
		&mut self.module
	}

	fn get_module(&self, _state: State) -> &dyn Module {
		&self.module
	}
}

pub(crate) struct DefaultTestModule;

impl Module for DefaultTestModule {}

pub(crate) fn create_test_module_handler<M: Module>(module: M) -> ModuleHandler<TestModuleProvider<M>> {
	ModuleHandler::new(
		EventHandler::new(create_test_keybindings()),
		TestModuleProvider::from(module),
	)
}

pub(crate) fn create_default_test_module_handler() -> ModuleHandler<TestModuleProvider<DefaultTestModule>> {
	ModuleHandler::new(
		EventHandler::new(create_test_keybindings()),
		TestModuleProvider::<DefaultTestModule>::from(DefaultTestModule {}),
	)
}
