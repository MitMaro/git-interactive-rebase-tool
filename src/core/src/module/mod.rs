mod exit_status;
mod module_handler;
mod module_provider;
mod modules;
mod state;

use anyhow::Error;
use input::InputOptions;
use lazy_static::lazy_static;
use todo_file::TodoFile;
use view::{RenderContext, ViewData, ViewSender};

pub(crate) use self::{
	exit_status::ExitStatus,
	module_handler::ModuleHandler,
	module_provider::ModuleProvider,
	modules::Modules,
	state::State,
};
use crate::{
	events::{Event, KeyBindings},
	process::Results,
};

lazy_static! {
	static ref DEFAULT_INPUT_OPTIONS: InputOptions = InputOptions::RESIZE;
	pub(crate) static ref DEFAULT_VIEW_DATA: ViewData = ViewData::new(|_| {});
}

pub(crate) trait Module: Send {
	fn activate(&mut self, _rebase_todo: &TodoFile, _previous_state: State) -> Results {
		Results::new()
	}

	fn deactivate(&mut self) -> Results {
		Results::new()
	}

	fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData {
		&DEFAULT_VIEW_DATA
	}

	fn input_options(&self) -> &InputOptions {
		&DEFAULT_INPUT_OPTIONS
	}

	fn read_event(&self, event: Event, _key_bindings: &KeyBindings) -> Event {
		event
	}

	fn handle_event(&mut self, _event: Event, _view_sender: &ViewSender, _rebase_todo: &mut TodoFile) -> Results {
		Results::new()
	}

	fn handle_error(&mut self, _error: &Error) {}
}
