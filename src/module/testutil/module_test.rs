use std::{cell::Cell, path::Path};

use input::{
	testutil::{with_event_handler, TestContext as EventHandlerTestContext},
	Event,
};
use tempfile::{Builder, NamedTempFile};
use todo_file::{Line, TodoFile};
use view::{
	testutil::{with_view_sender, TestContext as ViewSenderContext},
	RenderContext,
	ViewData,
};

use crate::module::{Module, ProcessResult, State};

pub struct TestContext {
	pub event_handler_context: EventHandlerTestContext,
	pub view_sender_context: ViewSenderContext,
	pub rebase_todo_file: TodoFile,
	pub render_context: RenderContext,
	todo_file: Cell<NamedTempFile>,
}

impl TestContext {
	fn get_build_data<'tc>(&self, module: &'tc mut dyn Module) -> &'tc ViewData {
		module.build_view_data(&self.render_context, &self.rebase_todo_file)
	}

	pub fn activate(&self, module: &'_ mut dyn Module, state: State) -> ProcessResult {
		module.activate(&self.rebase_todo_file, state)
	}

	#[allow(clippy::unused_self)]
	pub fn deactivate(&mut self, module: &'_ mut dyn Module) {
		module.deactivate();
	}

	pub fn build_view_data<'tc>(&self, module: &'tc mut dyn Module) -> &'tc ViewData {
		self.get_build_data(module)
	}

	pub fn handle_event(&mut self, module: &'_ mut dyn Module) -> ProcessResult {
		module.handle_events(
			&self.event_handler_context.event_handler,
			&self.view_sender_context.sender,
			&mut self.rebase_todo_file,
		)
	}

	pub fn handle_n_events(&mut self, module: &'_ mut dyn Module, n: usize) -> Vec<ProcessResult> {
		let mut results = vec![];
		for _ in 0..n {
			results.push(module.handle_events(
				&self.event_handler_context.event_handler,
				&self.view_sender_context.sender,
				&mut self.rebase_todo_file,
			));
		}
		results
	}

	pub fn handle_all_events(&mut self, module: &'_ mut dyn Module) -> Vec<ProcessResult> {
		let mut results = vec![];
		for _ in 0..self.event_handler_context.number_events {
			results.push(module.handle_events(
				&self.event_handler_context.event_handler,
				&self.view_sender_context.sender,
				&mut self.rebase_todo_file,
			));
		}
		results
	}

	pub fn new_todo_file(&self) -> TodoFile {
		TodoFile::new(self.get_todo_file_path().as_str(), 1, "#")
	}

	pub fn get_todo_file_path(&self) -> String {
		let t = self.todo_file.replace(NamedTempFile::new().unwrap());
		let path = t.path().to_str().unwrap().to_owned();
		self.todo_file.replace(t);
		path
	}

	pub fn delete_todo_file(&self) {
		self.todo_file
			.replace(Builder::new().tempfile().unwrap())
			.close()
			.unwrap();
	}

	pub fn set_todo_file_readonly(&self) {
		let t = self.todo_file.replace(NamedTempFile::new().unwrap());
		let todo_file = t.as_file();
		let mut permissions = todo_file.metadata().unwrap().permissions();
		permissions.set_readonly(true);
		todo_file.set_permissions(permissions).unwrap();
		self.todo_file.replace(t);
	}
}

pub fn module_test<C>(lines: &[&str], events: &[Event], callback: C)
where C: FnOnce(TestContext) {
	with_event_handler(events, |event_handler_context| {
		with_view_sender(|view_sender_context| {
			let git_repo_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("test")
				.join("fixtures")
				.join("simple")
				.to_str()
				.unwrap()
				.to_owned();
			let todo_file = Builder::new()
				.prefix("git-rebase-todo-scratch")
				.suffix("")
				.tempfile_in(git_repo_dir.as_str())
				.unwrap();

			let mut rebase_todo_file = TodoFile::new(todo_file.path().to_str().unwrap(), 1, "#");
			rebase_todo_file.set_lines(lines.iter().map(|l| Line::new(l).unwrap()).collect());
			callback(TestContext {
				event_handler_context,
				rebase_todo_file,
				view_sender_context,
				todo_file: Cell::new(todo_file),
				render_context: RenderContext::new(300, 120),
			});
		});
	});
}
