use std::sync::Arc;

use parking_lot::Mutex;

use crate::{config::Config, input, module, search, todo_file::TodoFile, view};

#[derive(Clone, Debug)]
pub(crate) struct AppData {
	config: Arc<Config>,
	active_module: Arc<Mutex<module::State>>,
	todo_file: Arc<Mutex<TodoFile>>,
	view_state: view::State,
	input_state: input::State,
	search_state: search::State,
}

impl AppData {
	pub(crate) fn new(
		config: Arc<Config>,
		active_module: module::State,
		todo_file: Arc<Mutex<TodoFile>>,
		view_state: view::State,
		input_state: input::State,
		search_state: search::State,
	) -> Self {
		Self {
			config,
			active_module: Arc::new(Mutex::new(active_module)),
			todo_file,
			view_state,
			input_state,
			search_state,
		}
	}

	pub(crate) fn config(&self) -> Arc<Config> {
		Arc::clone(&self.config)
	}

	pub(crate) fn active_module(&self) -> Arc<Mutex<module::State>> {
		Arc::clone(&self.active_module)
	}

	pub(crate) fn todo_file(&self) -> Arc<Mutex<TodoFile>> {
		Arc::clone(&self.todo_file)
	}

	pub(crate) fn view_state(&self) -> view::State {
		self.view_state.clone()
	}

	pub(crate) fn input_state(&self) -> input::State {
		self.input_state.clone()
	}

	pub(crate) fn search_state(&self) -> search::State {
		self.search_state.clone()
	}
}
