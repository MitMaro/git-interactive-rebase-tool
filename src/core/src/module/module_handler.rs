use input::EventHandler;
use todo_file::TodoFile;
use view::{RenderContext, ViewData};

use super::State;
use crate::{
	events,
	events::{AppKeyBindings, Event, MetaEvent},
	process::Results,
};

pub(crate) struct ModuleHandler<ModuleProvider: crate::module::ModuleProvider> {
	event_handler: EventHandler<AppKeyBindings, MetaEvent>,
	module_provider: ModuleProvider,
}

impl<ModuleProvider: crate::module::ModuleProvider> ModuleHandler<ModuleProvider> {
	pub(crate) const fn new(
		event_handler: EventHandler<AppKeyBindings, MetaEvent>,
		module_provider: ModuleProvider,
	) -> Self {
		Self {
			event_handler,
			module_provider,
		}
	}

	pub(crate) fn activate(&mut self, state: State, rebase_todo: &TodoFile, previous_state: State) -> Results {
		self.module_provider
			.get_mut_module(state)
			.activate(rebase_todo, previous_state)
	}

	pub(crate) fn deactivate(&mut self, state: State) -> Results {
		self.module_provider.get_mut_module(state).deactivate()
	}

	pub(crate) fn build_view_data(
		&mut self,
		state: State,
		render_context: &RenderContext,
		rebase_todo: &TodoFile,
	) -> &ViewData {
		self.module_provider
			.get_mut_module(state)
			.build_view_data(render_context, rebase_todo)
	}

	pub(crate) fn handle_event(
		&mut self,
		state: State,
		input_state: &events::State,
		view_state: &view::State,
		rebase_todo: &mut TodoFile,
	) -> Option<Results> {
		let module = self.module_provider.get_module(state);
		let input_options = module.input_options();
		let event = self
			.event_handler
			.read_event(input_state.read_event(), input_options, |event, key_bindings| {
				module.read_event(event, key_bindings)
			});
		(event != Event::None).then(|| {
			let mut results = Results::new();
			results.event(event);
			results.append(
				self.module_provider
					.get_mut_module(state)
					.handle_event(event, view_state, rebase_todo),
			);
			results
		})
	}

	pub(crate) fn error(&mut self, state: State, error: &anyhow::Error) -> Results {
		self.module_provider.get_mut_module(state).handle_error(error)
	}
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use anyhow::{anyhow, Error};
	use input::StandardEvent;
	use parking_lot::Mutex;

	use super::*;
	use crate::{
		module::Module,
		testutil::{module_test, TestModuleProvider},
	};

	#[derive(Clone)]
	struct TestModule {
		view_data: Arc<ViewData>,
		trace: Arc<Mutex<Vec<String>>>,
	}

	impl TestModule {
		fn new() -> Self {
			Self {
				view_data: Arc::new(ViewData::new(|_| {})),
				trace: Arc::new(Mutex::new(vec![])),
			}
		}

		fn trace(&self) -> String {
			self.trace.lock().join(",")
		}
	}

	impl Module for TestModule {
		fn activate(&mut self, _rebase_todo: &TodoFile, _previous_state: State) -> Results {
			self.trace.lock().push(String::from("Activate"));
			Results::new()
		}

		fn deactivate(&mut self) -> Results {
			self.trace.lock().push(String::from("Deactivate"));
			Results::new()
		}

		fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData {
			self.trace.lock().push(String::from("Build View Data"));
			&self.view_data
		}

		fn handle_event(&mut self, _: Event, _: &view::State, _: &mut TodoFile) -> Results {
			self.trace.lock().push(String::from("Handle Events"));
			Results::new()
		}

		fn handle_error(&mut self, error: &Error) -> Results {
			self.trace.lock().push(error.to_string());
			Results::new()
		}
	}

	#[test]
	fn module_lifecycle() {
		module_test(
			&["pick aaa comment"],
			&[Event::Standard(StandardEvent::Exit)],
			|mut context| {
				let test_module = TestModule::new();
				let mut module_handler = ModuleHandler::new(
					context.event_handler_context.event_handler,
					TestModuleProvider::from(test_module.clone()),
				);
				let _ = module_handler.activate(State::List, context.todo_file_context.todo_file(), State::Insert);
				let _ = module_handler.handle_event(
					State::List,
					&context.event_handler_context.state,
					&context.view_context.state,
					context.todo_file_context.todo_file_mut(),
				);

				let _ = module_handler.build_view_data(
					State::List,
					&RenderContext::new(100, 100),
					context.todo_file_context.todo_file(),
				);
				let _ = module_handler.deactivate(State::List);
				assert_eq!(test_module.trace(), "Activate,Handle Events,Build View Data,Deactivate");
			},
		);
	}

	#[test]
	fn error() {
		module_test(
			&["pick aaa comment"],
			&[Event::Standard(StandardEvent::Exit)],
			|context| {
				let test_module = TestModule::new();
				let mut module_handler = ModuleHandler::new(
					context.event_handler_context.event_handler,
					TestModuleProvider::from(test_module.clone()),
				);
				let _ = module_handler.error(State::Error, &anyhow!("Test Error"));
				assert_eq!(test_module.trace(), "Test Error");
			},
		);
	}
}
