use crate::{
	application::AppData,
	git::Repository,
	module::{Module, State},
};

pub(crate) trait ModuleProvider {
	fn new(repository: Repository, app_data: &AppData) -> Self;

	fn get_mut_module(&mut self, _state: State) -> &mut dyn Module;

	fn get_module(&self, _state: State) -> &dyn Module;
}
