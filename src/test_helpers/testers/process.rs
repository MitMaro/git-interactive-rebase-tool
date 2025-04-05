use std::{path::PathBuf, sync::Arc};

use parking_lot::Mutex;

use crate::{
	application::AppData,
	display::Size,
	input::Event,
	module::{self, ModuleHandler, State},
	process::Process,
	runtime::ThreadStatuses,
	test_helpers::{
		EventHandlerTestContext,
		SearchTestContext,
		ViewStateTestContext,
		create_config,
		with_event_handler,
		with_search,
		with_todo_file,
		with_view_state,
	},
};

pub(crate) struct ProcessTestContext<ModuleProvider: module::ModuleProvider + Send + 'static> {
	pub(crate) event_handler_context: EventHandlerTestContext,
	pub(crate) process: Process<ModuleProvider>,
	pub(crate) search_context: SearchTestContext,
	pub(crate) todo_file_path: PathBuf,
	pub(crate) view_context: ViewStateTestContext,
}

pub(crate) fn process<C, ModuleProvider: module::ModuleProvider + Send + 'static>(
	module_handler: ModuleHandler<ModuleProvider>,
	callback: C,
) where
	C: FnOnce(ProcessTestContext<ModuleProvider>),
{
	with_event_handler(&[Event::from('a')], |event_handler_context| {
		with_view_state(|view_context| {
			with_todo_file(&[], |todo_file_context| {
				with_search(|search_context| {
					let (todo_file_tmp_path, todo_file) = todo_file_context.to_owned();
					let view_state = view_context.state.clone();
					let input_state = event_handler_context.state.clone();
					let todo_file_path = PathBuf::from(todo_file_tmp_path.path());

					callback(ProcessTestContext {
						event_handler_context,
						process: Process::new(
							&AppData::new(
								create_config(),
								State::WindowSizeError,
								Arc::new(Mutex::new(todo_file)),
								view_state,
								input_state,
								search_context.state.clone(),
							),
							Size::new(300, 120),
							module_handler,
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
