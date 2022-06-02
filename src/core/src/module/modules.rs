use config::Config;
use git::Repository;

use super::{Module, State};
use crate::{
	module::ModuleProvider,
	modules::{ConfirmAbort, ConfirmRebase, Error, ExternalEditor, Insert, List, ShowCommit, WindowSizeError},
};

pub(crate) struct Modules {
	confirm_abort: ConfirmAbort,
	confirm_rebase: ConfirmRebase,
	error: Error,
	external_editor: ExternalEditor,
	insert: Insert,
	list: List,
	show_commit: ShowCommit,
	window_size_error: WindowSizeError,
}

impl ModuleProvider for Modules {
	fn new(config: &Config, repository: Repository) -> Self {
		Self {
			error: Error::new(),
			list: List::new(config),
			show_commit: ShowCommit::new(config, repository),
			window_size_error: WindowSizeError::new(),
			confirm_abort: ConfirmAbort::new(&config.key_bindings.confirm_yes, &config.key_bindings.confirm_no),
			confirm_rebase: ConfirmRebase::new(&config.key_bindings.confirm_yes, &config.key_bindings.confirm_no),
			external_editor: ExternalEditor::new(config.git.editor.as_str()),
			insert: Insert::new(),
		}
	}

	fn get_mut_module(&mut self, state: State) -> &mut dyn Module {
		match state {
			State::ConfirmAbort => &mut self.confirm_abort,
			State::ConfirmRebase => &mut self.confirm_rebase,
			State::Error => &mut self.error,
			State::ExternalEditor => &mut self.external_editor,
			State::Insert => &mut self.insert,
			State::List => &mut self.list,
			State::ShowCommit => &mut self.show_commit,
			State::WindowSizeError => &mut self.window_size_error,
		}
	}

	fn get_module(&self, state: State) -> &dyn Module {
		match state {
			State::ConfirmAbort => &self.confirm_abort,
			State::ConfirmRebase => &self.confirm_rebase,
			State::Error => &self.error,
			State::ExternalEditor => &self.external_editor,
			State::Insert => &self.insert,
			State::List => &self.list,
			State::ShowCommit => &self.show_commit,
			State::WindowSizeError => &self.window_size_error,
		}
	}
}

#[cfg(test)]
mod tests {
	use git::testutil::with_temp_repository;

	use super::*;

	pub(crate) fn modules_test<C>(callback: C)
	where C: FnOnce(Modules) {
		with_temp_repository(|repository| {
			let config = Config::new();
			let modules = Modules::new(&config, repository);
			callback(modules);
			Ok(())
		});
	}

	// someday I would like to test the returned types for these
	#[test]
	fn get_mut_module() {
		modules_test(|mut modules| {
			let _ = modules.get_mut_module(State::ConfirmAbort);
			let _ = modules.get_mut_module(State::ConfirmRebase);
			let _ = modules.get_mut_module(State::Error);
			let _ = modules.get_mut_module(State::ExternalEditor);
			let _ = modules.get_mut_module(State::Insert);
			let _ = modules.get_mut_module(State::List);
			let _ = modules.get_mut_module(State::ShowCommit);
			let _ = modules.get_mut_module(State::WindowSizeError);
		});
	}

	#[test]
	fn get_module() {
		modules_test(|modules| {
			let _ = modules.get_module(State::ConfirmAbort);
			let _ = modules.get_module(State::ConfirmRebase);
			let _ = modules.get_module(State::Error);
			let _ = modules.get_module(State::ExternalEditor);
			let _ = modules.get_module(State::Insert);
			let _ = modules.get_module(State::List);
			let _ = modules.get_module(State::ShowCommit);
			let _ = modules.get_module(State::WindowSizeError);
		});
	}
}
