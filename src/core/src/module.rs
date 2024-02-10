mod exit_status;
mod module_handler;
mod module_provider;
mod modules;
mod state;
#[cfg(test)]
mod tests;

use anyhow::Error;
use lazy_static::lazy_static;

pub(crate) use self::{
	exit_status::ExitStatus,
	module_handler::ModuleHandler,
	module_provider::ModuleProvider,
	modules::Modules,
	state::State,
};
use crate::{
	events::{Event, KeyBindings},
	input::InputOptions,
	process::Results,
	view::{RenderContext, ViewData},
};

lazy_static! {
	pub(crate) static ref DEFAULT_INPUT_OPTIONS: InputOptions = InputOptions::RESIZE;
	pub(crate) static ref DEFAULT_VIEW_DATA: ViewData = ViewData::new(|_| {});
}

pub(crate) trait Module: Send {
	fn activate(&mut self, _previous_state: State) -> Results {
		Results::new()
	}

	fn deactivate(&mut self) -> Results {
		Results::new()
	}

	fn build_view_data(&mut self, _render_context: &RenderContext) -> &ViewData {
		&DEFAULT_VIEW_DATA
	}

	fn input_options(&self) -> &InputOptions {
		&DEFAULT_INPUT_OPTIONS
	}

	fn read_event(&self, event: Event, _key_bindings: &KeyBindings) -> Event {
		event
	}

	fn handle_event(&mut self, _event: Event, _view_state: &crate::view::State) -> Results {
		Results::new()
	}

	fn handle_error(&mut self, _error: &Error) -> Results {
		Results::new()
	}
}
