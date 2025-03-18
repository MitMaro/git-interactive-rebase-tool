use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
	application::AppData,
	display::Size,
	input::Event,
	module::{self, ModuleHandler, State},
	process::Process,
	runtime::ThreadStatuses,
	test_helpers::{
		ViewStateTestContext,
		create_config,
		with_event_handler,
		with_search,
		with_todo_file,
		with_view_state,
	},
};

pub(crate) struct ProcessTestContext<ModuleProvider: module::ModuleProvider + Send + 'static> {
	pub(crate) process: Process<ModuleProvider>,
	pub(crate) view_context: ViewStateTestContext,
	pub(crate) app_data: AppData,
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
					let (_todo_file_tmp_path, todo_file) = todo_file_context.to_owned();
					let view_state = view_context.state.clone();
					let input_state = event_handler_context.state.clone();
					let app_data = AppData::new(
						Arc::new(create_config()),
						State::WindowSizeError,
						Arc::new(Mutex::new(todo_file)),
						view_state,
						input_state,
						search_context.state.clone(),
					);

					callback(ProcessTestContext {
						process: Process::new(&app_data, Size::new(300, 120), module_handler, ThreadStatuses::new()),
						view_context,
						app_data,
					});
				});
			});
		});
	});
}
