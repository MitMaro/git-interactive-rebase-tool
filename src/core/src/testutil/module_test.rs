use std::{cell::Cell, path::Path};

use captur::capture;
use tempfile::{Builder, NamedTempFile};
use todo_file::{Line, TodoFile};
use view::{
	testutil::{with_view_state, TestContext as ViewContext},
	RenderContext,
	ViewData,
};

use crate::{
	events::Event,
	module::{Module, State},
	process::Results,
	testutil::{with_event_handler, EventHandlerTestContext},
};

pub(crate) struct TestContext {
	pub(crate) event_handler_context: EventHandlerTestContext,
	pub(crate) rebase_todo_file: TodoFile,
	pub(crate) render_context: RenderContext,
	pub(crate) view_context: ViewContext,
	todo_file: Cell<NamedTempFile>,
}

impl TestContext {
	fn get_build_data<'tc>(&self, module: &'tc mut dyn Module) -> &'tc ViewData {
		module.build_view_data(&self.render_context, &self.rebase_todo_file)
	}

	pub(crate) fn activate(&self, module: &'_ mut dyn Module, state: State) -> Results {
		module.activate(&self.rebase_todo_file, state)
	}

	#[allow(clippy::unused_self)]
	pub(crate) fn deactivate(&mut self, module: &'_ mut dyn Module) -> Results {
		module.deactivate()
	}

	pub(crate) fn build_view_data<'tc>(&self, module: &'tc mut dyn Module) -> &'tc ViewData {
		self.get_build_data(module)
	}

	pub(crate) fn handle_event(&mut self, module: &'_ mut dyn Module) -> Results {
		let input_options = module.input_options();
		let event = self.event_handler_context.event_handler.read_event(
			self.event_handler_context.state.read_event(),
			input_options,
			|event, key_bindings| module.read_event(event, key_bindings),
		);
		let mut results = Results::new();
		results.event(event);
		results.append(module.handle_event(event, &self.view_context.state, &mut self.rebase_todo_file));
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

	pub(crate) fn get_todo_file_path(&self) -> String {
		let t = self.todo_file.replace(NamedTempFile::new().unwrap());
		let path = t.path().to_str().unwrap().to_owned();
		let _ = self.todo_file.replace(t);
		path
	}

	pub(crate) fn delete_todo_file(&self) {
		self.todo_file
			.replace(Builder::new().tempfile().unwrap())
			.close()
			.unwrap();
	}

	pub(crate) fn set_todo_file_readonly(&self) {
		let t = self.todo_file.replace(NamedTempFile::new().unwrap());
		let todo_file = t.as_file();
		let mut permissions = todo_file.metadata().unwrap().permissions();
		permissions.set_readonly(true);
		todo_file.set_permissions(permissions).unwrap();
		let _ = self.todo_file.replace(t);
	}
}

pub(crate) fn module_test<C>(lines: &[&str], events: &[Event], callback: C)
where C: FnOnce(TestContext) {
	with_event_handler(events, |event_handler_context| {
		with_view_state(|view_context| {
			capture!(lines);
			let git_repo_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("..")
				.join("..")
				.join("test")
				.join("fixtures")
				.join("simple");
			let todo_file = Builder::new()
				.prefix("git-rebase-todo-scratch")
				.suffix("")
				.tempfile_in(git_repo_dir.as_path())
				.unwrap();

			let mut rebase_todo_file = TodoFile::new(todo_file.path().to_str().unwrap(), 1, "#");
			rebase_todo_file.set_lines(lines.iter().map(|l| Line::new(l).unwrap()).collect());
			callback(TestContext {
				event_handler_context,
				rebase_todo_file,
				view_context,
				todo_file: Cell::new(todo_file),
				render_context: RenderContext::new(300, 120),
			});
		});
	});
}
