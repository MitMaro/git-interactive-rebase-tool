use std::path::{Path, PathBuf};

use display::Size;
use runtime::ThreadStatuses;
use tempfile::Builder;
use todo_file::TodoFile;
use view::testutil::{with_view_state, TestContext as ViewContext};

use crate::{
	events::Event,
	module::{self, ModuleHandler},
	process::Process,
	testutil::{with_event_handler, EventHandlerTestContext},
};

pub(crate) struct TestContext<ModuleProvider: module::ModuleProvider + Send + 'static> {
	pub(crate) event_handler_context: EventHandlerTestContext,
	pub(crate) process: Process<ModuleProvider>,
	pub(crate) todo_file_path: PathBuf,
	pub(crate) view_context: ViewContext,
}

pub(crate) fn process_test<C, ModuleProvider: module::ModuleProvider + Send + 'static>(
	module_handler: ModuleHandler<ModuleProvider>,
	callback: C,
) where
	C: FnOnce(TestContext<ModuleProvider>),
{
	with_event_handler(&[Event::from('a')], |event_handler_context| {
		with_view_state(|view_context| {
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

			let rebase_todo_file = TodoFile::new(todo_file.path().to_str().unwrap(), 1, "#");
			let view_state = view_context.state.clone();
			let input_state = event_handler_context.state.clone();

			callback(TestContext {
				event_handler_context,
				process: Process::new(
					Size::new(300, 120),
					rebase_todo_file,
					module_handler,
					input_state,
					view_state,
					ThreadStatuses::new(),
				),
				todo_file_path: PathBuf::from(todo_file.path()),
				view_context,
			});
		});
	});
}
