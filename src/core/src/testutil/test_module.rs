use todo_file::TodoFile;
use view::{RenderContext, ViewData, ViewSender};

use crate::{
	events::Event,
	module::{Module, ProcessResult},
};

pub(crate) struct TestModule<'module> {
	event_callback: Box<dyn Fn(Event, &ViewSender, &mut TodoFile) -> ProcessResult + Send + 'module>,
	view_data: ViewData,
	view_data_callback: Box<dyn Fn(&mut ViewData) + Send + 'module>,
}

impl<'module> TestModule<'module> {
	pub(crate) fn new() -> Self {
		Self {
			event_callback: Box::new(|event, _, _| ProcessResult::from(event)),
			view_data: ViewData::new(|_| {}),
			view_data_callback: Box::new(|_| {}),
		}
	}

	pub(crate) fn event_callback<EC>(&mut self, callback: EC)
	where
		EC: Fn(Event, &ViewSender, &mut TodoFile) -> ProcessResult,
		EC: Send + 'module,
	{
		self.event_callback = Box::new(callback);
	}

	pub(crate) fn view_data_callback<VDC>(&mut self, callback: VDC)
	where
		VDC: Fn(&mut ViewData),
		VDC: Send + 'module,
	{
		self.view_data_callback = Box::new(callback);
	}
}

impl Module for TestModule<'_> {
	fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData {
		(self.view_data_callback)(&mut self.view_data);
		&self.view_data
	}

	fn handle_event(&mut self, event: Event, view_sender: &ViewSender, todo_file: &mut TodoFile) -> ProcessResult {
		(self.event_callback)(event, view_sender, todo_file)
	}
}
