use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
	config::Config,
	git::Repository,
	module::{Module, State},
	todo_file::TodoFile,
};

pub(crate) trait ModuleProvider {
	fn new(config: &Config, repository: Repository, todo_file: &Arc<Mutex<TodoFile>>) -> Self;

	fn get_mut_module(&mut self, _state: State) -> &mut dyn Module;

	fn get_module(&self, _state: State) -> &dyn Module;
}
