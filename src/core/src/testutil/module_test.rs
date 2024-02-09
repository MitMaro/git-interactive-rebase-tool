use captur::capture;
use todo_file::{testutil::with_todo_file, TodoFile};

use crate::{
	events::Event,
	module::{Module, State},
	process::Results,
	testutil::{with_event_handler, EventHandlerTestContext},
	view::{
		testutil::{with_view_state, TestContext as ViewContext},
		RenderContext,
		ViewData,
	},
};

pub(crate) struct TestContext {
	pub(crate) event_handler_context: EventHandlerTestContext,
	pub(crate) render_context: RenderContext,
	pub(crate) view_context: ViewContext,
	todo_file: Option<TodoFile>,
}

impl TestContext {
	fn get_build_data<'tc>(&self, module: &'tc mut dyn Module) -> &'tc ViewData {
		module.build_view_data(&self.render_context)
	}

	#[allow(clippy::unused_self)]
	pub(crate) fn activate(&self, module: &'_ mut dyn Module, state: State) -> Results {
		module.activate(state)
	}

	#[allow(clippy::unused_self)]
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
where C: FnOnce(TestContext) {
	with_event_handler(events, |event_handler_context| {
		with_view_state(|view_context| {
			capture!(lines);
			with_todo_file(lines, |todo_file_context| {
				let (_git_todo_file, todo_file) = todo_file_context.to_owned();
				callback(TestContext {
					event_handler_context,
					render_context: RenderContext::new(300, 120),
					todo_file: Some(todo_file),
					view_context,
				});
			});
		});
	});
}
