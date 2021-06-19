use std::collections::HashMap;

use crate::{
	input::EventHandler,
	process::{module::Module, process_result::ProcessResult, state::State},
	todo_file::TodoFile,
	view::{RenderContext, ViewData, ViewSender},
};

pub struct Modules {
	pub modules: HashMap<State, Box<dyn Module>>,
}

impl Modules {
	pub fn new() -> Self {
		Self {
			modules: HashMap::new(),
		}
	}

	pub fn register_module<T: Module + 'static>(&mut self, state: State, module: T) {
		self.modules.insert(state, Box::new(module));
	}

	fn get_mut_module(&mut self, state: State) -> &mut Box<dyn Module> {
		self.modules
			.get_mut(&state)
			.unwrap_or_else(|| panic!("Invalid module for provided state: {:?}", state))
	}

	pub fn activate(&mut self, state: State, rebase_todo: &TodoFile, previous_state: State) -> ProcessResult {
		self.get_mut_module(state).activate(rebase_todo, previous_state)
	}

	pub fn deactivate(&mut self, state: State) {
		self.get_mut_module(state).deactivate();
	}

	pub fn build_view_data(
		&mut self,
		state: State,
		render_context: &RenderContext,
		rebase_todo: &TodoFile,
	) -> &ViewData {
		self.get_mut_module(state).build_view_data(render_context, rebase_todo)
	}

	pub fn handle_input(
		&mut self,
		state: State,
		event_handler: &EventHandler,
		view_sender: &ViewSender,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		self.get_mut_module(state)
			.handle_events(event_handler, view_sender, rebase_todo)
	}

	pub fn error(&mut self, state: State, error: &anyhow::Error) {
		self.get_mut_module(state).handle_error(error);
	}
}

#[cfg(test)]
mod tests {
	use std::{cell::RefCell, rc::Rc};

	use anyhow::{anyhow, Error};

	use super::*;
	use crate::{
		input::{Event, MetaEvent},
		process::testutil::process_module_test,
	};

	struct TestModule {
		view_data: ViewData,
		trace: Rc<RefCell<Vec<String>>>,
	}

	impl TestModule {
		fn new(trace: Rc<RefCell<Vec<String>>>) -> Self {
			Self {
				view_data: ViewData::new(|_| {}),
				trace,
			}
		}
	}

	impl Module for TestModule {
		fn activate(&mut self, _rebase_todo: &TodoFile, _previous_state: State) -> ProcessResult {
			self.trace.borrow_mut().push(String::from("Activate"));
			ProcessResult::new()
		}

		fn deactivate(&mut self) {
			self.trace.borrow_mut().push(String::from("Deactivate"));
		}

		fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData {
			self.trace.borrow_mut().push(String::from("Build View Data"));
			&self.view_data
		}

		fn handle_events(&mut self, _: &EventHandler, _: &ViewSender, _: &mut TodoFile) -> ProcessResult {
			self.trace.borrow_mut().push(String::from("Handle Events"));
			ProcessResult::new()
		}

		fn handle_error(&mut self, error: &Error) {
			self.trace.borrow_mut().push(error.to_string());
		}
	}

	#[test]
	fn module_lifecycle() {
		process_module_test(&["pick aaa comment"], &[Event::Meta(MetaEvent::Exit)], |mut context| {
			let mut modules = Modules::new();
			let trace = Rc::new(RefCell::new(Vec::new()));
			let test_module = TestModule::new(Rc::clone(&trace));
			modules.register_module(State::List, test_module);

			modules.activate(State::List, &context.rebase_todo_file, State::Insert);
			modules.handle_input(
				State::List,
				&context.event_handler_context.event_handler,
				&context.view_sender_context.sender,
				&mut context.rebase_todo_file,
			);
			modules.build_view_data(State::List, &RenderContext::new(100, 100), &context.rebase_todo_file);
			modules.deactivate(State::List);
			assert_eq!(
				(*trace).borrow().join(","),
				"Activate,Handle Events,Build View Data,Deactivate"
			);
		});
	}

	#[test]
	fn error() {
		let mut modules = Modules::new();
		let trace = Rc::new(RefCell::new(Vec::new()));
		let test_module = TestModule::new(Rc::clone(&trace));
		modules.register_module(State::Error, test_module);
		modules.error(State::Error, &anyhow!("Test Error"));
		assert_eq!((*trace).borrow().join(","), "Test Error");
	}
}
