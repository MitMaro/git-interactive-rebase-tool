use captur::capture;

use crate::{
	input::Event,
	module::{Module, State},
	process::Results,
	test_helpers::{
		EventHandlerTestContext,
		ViewStateTestContext,
		with_event_handler,
		with_todo_file,
		with_view_state,
	},
	todo_file::TodoFile,
	view::{RenderContext, ViewData},
};

#[allow(clippy::partial_pub_fields)]
pub(crate) struct ModuleTestContext {
	pub event_handler_context: EventHandlerTestContext,
	pub render_context: RenderContext,
	pub view_context: ViewStateTestContext,
	todo_file: Option<TodoFile>,
}

impl ModuleTestContext {
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
		results.append(module.handle_event(event, &self.view_context.state));
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

	pub(crate) fn take_todo_file(&mut self) -> TodoFile {
		self.todo_file.take().expect("Cannot take the TodoFile more than once")
	}
}

pub(crate) fn module_test<C>(lines: &[&str], events: &[Event], callback: C)
where C: FnOnce(ModuleTestContext) {
	with_event_handler(events, |event_handler_context| {
		with_view_state(|view_context| {
			capture!(lines);
			with_todo_file(lines, |todo_file_context| {
				let (_git_todo_file, todo_file) = todo_file_context.to_owned();
				callback(ModuleTestContext {
					event_handler_context,
					render_context: RenderContext::new(300, 120),
					todo_file: Some(todo_file),
					view_context,
				});
			});
		});
	});
}
