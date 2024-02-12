use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
	config::Config,
	git::Repository,
	input::{testutil::create_test_keybindings, EventHandler},
	module::{Module, ModuleHandler, ModuleProvider, State},
	todo_file::TodoFile,
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
	fn new(_: &Config, _: Repository, _: &Arc<Mutex<TodoFile>>) -> Self {
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
