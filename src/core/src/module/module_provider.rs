use config::Config;
use git::Repository;

use super::{Module, State};

pub(crate) trait ModuleProvider {
	fn new(config: &Config, repository: Repository) -> Self;

	fn get_mut_module(&mut self, _state: State) -> &mut dyn Module;

	fn get_module(&self, _state: State) -> &dyn Module;
}
