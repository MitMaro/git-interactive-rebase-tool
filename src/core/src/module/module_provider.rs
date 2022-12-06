use std::sync::Arc;

use config::Config;
use git::Repository;
use parking_lot::Mutex;
use todo_file::TodoFile;

use super::{Module, State};

pub(crate) trait ModuleProvider {
	fn new(config: &Config, repository: Repository, todo_file: &Arc<Mutex<TodoFile>>) -> Self;

	fn get_mut_module(&mut self, _state: State) -> &mut dyn Module;

	fn get_module(&self, _state: State) -> &dyn Module;
}
