use crate::{
	application::AppData,
	module::{Module, State},
};

pub(crate) trait ModuleProvider {
	fn new(app_data: &AppData) -> Self;

	fn get_mut_module(&mut self, _state: State) -> &mut dyn Module;

	fn get_module(&self, _state: State) -> &dyn Module;
}
