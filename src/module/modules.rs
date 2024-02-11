use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
	config::Config,
	git::Repository,
	module::{Module, ModuleProvider, State},
	modules::{ConfirmAbort, ConfirmRebase, Error, ExternalEditor, Insert, List, ShowCommit, WindowSizeError},
	todo_file::TodoFile,
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
	fn new(config: &Config, repository: Repository, todo_file: &Arc<Mutex<TodoFile>>) -> Self {
		Self {
			error: Error::new(),
			list: List::new(config, Arc::clone(todo_file)),
			show_commit: ShowCommit::new(config, repository, Arc::clone(todo_file)),
			window_size_error: WindowSizeError::new(),
			confirm_abort: ConfirmAbort::new(
				&config.key_bindings.confirm_yes,
				&config.key_bindings.confirm_no,
				Arc::clone(todo_file),
			),
			confirm_rebase: ConfirmRebase::new(&config.key_bindings.confirm_yes, &config.key_bindings.confirm_no),
			external_editor: ExternalEditor::new(config.git.editor.as_str(), Arc::clone(todo_file)),
			insert: Insert::new(Arc::clone(todo_file)),
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
	use super::*;
	use crate::{git::testutil::with_temp_repository, todo_file::testutil::with_todo_file};

	pub(crate) fn modules_test<C>(callback: C)
	where C: FnOnce(Modules) {
		with_temp_repository(|repository| {
			with_todo_file(&[], |todo_file_context| {
				let (_todo_file_path, todo_file) = todo_file_context.to_owned();
				let config = Config::new();
				let modules = Modules::new(&config, repository, &Arc::new(Mutex::new(todo_file)));
				callback(modules);
			});
		});
	}

	// someday I would like to test the returned types for these
	#[test]
	fn get_mut_module() {
		modules_test(|mut modules| {
			_ = modules.get_mut_module(State::ConfirmAbort);
			_ = modules.get_mut_module(State::ConfirmRebase);
			_ = modules.get_mut_module(State::Error);
			_ = modules.get_mut_module(State::ExternalEditor);
			_ = modules.get_mut_module(State::Insert);
			_ = modules.get_mut_module(State::List);
			_ = modules.get_mut_module(State::ShowCommit);
			_ = modules.get_mut_module(State::WindowSizeError);
		});
	}

	#[test]
	fn get_module() {
		modules_test(|modules| {
			_ = modules.get_module(State::ConfirmAbort);
			_ = modules.get_module(State::ConfirmRebase);
			_ = modules.get_module(State::Error);
			_ = modules.get_module(State::ExternalEditor);
			_ = modules.get_module(State::Insert);
			_ = modules.get_module(State::List);
			_ = modules.get_module(State::ShowCommit);
			_ = modules.get_module(State::WindowSizeError);
		});
	}
}
