use crate::input::input_handler::{InputHandler, InputMode};
use crate::process::process_result::ProcessResult;
use crate::view::view_data::ViewData;
use crate::view::View;

pub struct Error {
	view_data: ViewData,
}

impl Error {
	pub fn new(message: &str) -> Self {
		Self {
			view_data: ViewData::new_error(message),
		}
	}

	pub fn get_view_data(&mut self, view: &View<'_>) -> &ViewData {
		let (view_width, view_height) = view.get_view_size();
		self.view_data.set_view_size(view_width, view_height);
		self.view_data.rebuild();
		&self.view_data
	}

	#[allow(clippy::unused_self)]
	pub fn handle_input(&mut self, input_handler: &InputHandler<'_>) -> ProcessResult {
		let input = input_handler.get_input(InputMode::Default);
		ProcessResult::new().input(input)
	}
}
