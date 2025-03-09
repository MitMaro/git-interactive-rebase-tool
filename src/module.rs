mod exit_status;
mod module_handler;
mod module_provider;
mod modules;
mod state;
#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use anyhow::Error;

pub(crate) use self::{
	exit_status::ExitStatus,
	module_handler::ModuleHandler,
	module_provider::ModuleProvider,
	modules::Modules,
	state::State,
};
use crate::{
	input::{Event, InputOptions, KeyBindings},
	process::Results,
	view::{RenderContext, ViewData},
};

pub(crate) static DEFAULT_INPUT_OPTIONS: LazyLock<InputOptions> = LazyLock::new(|| InputOptions::RESIZE);
pub(crate) static DEFAULT_VIEW_DATA: LazyLock<ViewData> = LazyLock::new(|| ViewData::new(|_| {}));

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
