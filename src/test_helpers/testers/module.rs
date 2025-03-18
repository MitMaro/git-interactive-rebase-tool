use std::sync::Arc;

use captur::capture;
use parking_lot::Mutex;

use crate::{
	application::AppData,
	config::Config,
	input::Event,
	module::{Module, State},
	process::Results,
	test_helpers::{
		EventHandlerTestContext,
		ViewStateTestContext,
		create_config,
		with_event_handler,
		with_todo_file,
		with_view_state,
	},
	view::{RenderContext, ViewData},
};

pub(crate) struct ModuleTestContext {
	app_data: AppData,
	pub(crate) event_handler_context: EventHandlerTestContext,
	pub(crate) render_context: RenderContext,
	pub(crate) view_context: ViewStateTestContext,
}

impl ModuleTestContext {
	pub(crate) fn app_data(&self) -> AppData {
		self.app_data.clone()
	}

	pub(crate) fn config(&self) -> Arc<Config> {
		self.app_data.config()
	}

	fn get_build_data<'tc>(&self, module: &'tc mut dyn Module) -> &'tc ViewData {
		module.build_view_data(&self.render_context)
	}

	pub(crate) fn activate(&self, module: &'_ mut dyn Module, state: State) -> Results {
		module.activate(state)
	}

	pub(crate) fn deactivate(&mut self, module: &'_ mut dyn Module) -> Results {
		module.deactivate()
	}

	pub(crate) fn build_view_data<'tc>(&self, module: &'tc mut dyn Module) -> &'tc ViewData {
		self.get_build_data(module)
	}

	pub(crate) fn read_event(&mut self, module: &dyn Module) -> Event {
		let input_options = module.input_options();
		self.event_handler_context.event_handler.read_event(
			self.event_handler_context.state.read_event(),
			input_options,
			|event, key_bindings| module.read_event(event, key_bindings),
		)
	}

	pub(crate) fn handle_event(&mut self, module: &'_ mut dyn Module) -> Results {
		let event = self.read_event(module);
		let mut results = Results::new();
		results.event(event);
		results.append(module.handle_event(event));
		results
	}

	pub(crate) fn handle_n_events(&mut self, module: &'_ mut dyn Module, n: usize) -> Vec<Results> {
		let mut results = vec![];
		for _ in 0..n {
			results.push(self.handle_event(module));
		}
		results
	}

	pub(crate) fn handle_all_events(&mut self, module: &'_ mut dyn Module) -> Vec<Results> {
		self.handle_n_events(module, self.event_handler_context.number_events)
	}
}

pub(crate) fn module_test<C>(lines: &[&str], events: &[Event], config: Option<Config>, callback: C)
where C: FnOnce(ModuleTestContext) {
	with_event_handler(events, |event_handler_context| {
		with_view_state(|view_context| {
			capture!(lines);
			with_todo_file(lines, |todo_file_context| {
				let (_git_todo_file, todo_file) = todo_file_context.to_owned();
				let app_data = AppData::new(
					Arc::new(config.unwrap_or_else(create_config)),
					State::WindowSizeError,
					Arc::new(Mutex::new(todo_file)),
					view_context.state.clone(),
					event_handler_context.state.clone(),
					crate::search::State::new(),
				);

				callback(ModuleTestContext {
					app_data,
					event_handler_context,
					render_context: RenderContext::new(300, 120),
					view_context,
				});
			});
		});
	});
}
