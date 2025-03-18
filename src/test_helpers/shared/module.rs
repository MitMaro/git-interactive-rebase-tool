use crate::{
	application::AppData,
	git::Repository,
	module::{Module, ModuleProvider, State},
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
	fn new(_: Repository, _: &AppData) -> Self {
		unimplemented!("Not implemented for the TestModuleProvider");
	}

	fn get_mut_module(&mut self, _state: State) -> &mut dyn Module {
		&mut self.module
	}

	fn get_module(&self, _state: State) -> &dyn Module {
		&self.module
	}
}
