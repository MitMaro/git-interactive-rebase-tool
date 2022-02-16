use input::Event;
use todo_file::TodoFile;
use view::{RenderContext, ViewData, ViewSender};

use crate::module::{Module, ProcessResult};

pub(crate) struct TestModule {
	pub(crate) event_callback: Box<dyn Fn(Event, &ViewSender, &mut TodoFile) -> ProcessResult>,
	pub(crate) view_data: ViewData,
	pub(crate) view_data_callback: Box<dyn Fn(&mut ViewData)>,
}

impl TestModule {
	pub(crate) fn new() -> Self {
		Self {
			event_callback: Box::new(|event, _, _| ProcessResult::from(event)),
			view_data: ViewData::new(|_| {}),
			view_data_callback: Box::new(|_| {}),
		}
	}
}

impl Module for TestModule {
	fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData {
		(self.view_data_callback)(&mut self.view_data);
		&self.view_data
	}

	fn handle_event(&mut self, event: Event, view_sender: &ViewSender, todo_file: &mut TodoFile) -> ProcessResult {
		(self.event_callback)(event, view_sender, todo_file)
	}
}
