use std::{path::PathBuf, sync::Arc};

use parking_lot::Mutex;

use crate::{
	display::Size,
	events::Event,
	module::{self, ModuleHandler},
	process::Process,
	runtime::ThreadStatuses,
	testutil::{with_event_handler, with_search, EventHandlerTestContext, SearchTestContext},
	todo_file::testutil::with_todo_file,
	view::testutil::{with_view_state, TestContext as ViewContext},
};

pub(crate) struct TestContext<ModuleProvider: module::ModuleProvider + Send + 'static> {
	pub(crate) event_handler_context: EventHandlerTestContext,
	pub(crate) process: Process<ModuleProvider>,
	pub(crate) search_context: SearchTestContext,
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
			with_todo_file(&[], |todo_file_context| {
				with_search(|search_context| {
					let (todo_file_tmp_path, todo_file) = todo_file_context.to_owned();
					let view_state = view_context.state.clone();
					let input_state = event_handler_context.state.clone();
					let todo_file_path = PathBuf::from(todo_file_tmp_path.path());

					callback(TestContext {
						event_handler_context,
						process: Process::new(
							Size::new(300, 120),
							Arc::new(Mutex::new(todo_file)),
							module_handler,
							input_state,
							view_state,
							search_context.state.clone(),
							ThreadStatuses::new(),
						),
						search_context,
						todo_file_path,
						view_context,
					});
				});
			});
		});
	});
}
