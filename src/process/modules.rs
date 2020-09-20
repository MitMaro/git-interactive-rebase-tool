use crate::config::Config;
use crate::confirm_abort::ConfirmAbort;
use crate::confirm_rebase::ConfirmRebase;
use crate::display::Display;
use crate::edit::Edit;
use crate::exiting::Exiting;
use crate::external_editor::ExternalEditor;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::list::List;
use crate::process::error::Error;
use crate::process::help::Help;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::process::window_size_error::WindowSizeError;
use crate::show_commit::ShowCommit;
use crate::view::view_data::ViewData;
use crate::view::View;

pub struct Modules<'m> {
	pub confirm_abort: ConfirmAbort,
	pub confirm_rebase: ConfirmRebase,
	pub edit: Edit,
	pub error: Error,
	pub exiting: Exiting,
	pub external_editor: ExternalEditor<'m>,
	pub help: Help,
	pub list: List<'m>,
	pub show_commit: ShowCommit<'m>,
	pub window_size_error: WindowSizeError,
}

impl<'m> Modules<'m> {
	pub fn new(display: &'m Display<'m>, config: &'m Config) -> Self {
		Modules {
			confirm_abort: ConfirmAbort::new(),
			confirm_rebase: ConfirmRebase::new(),
			edit: Edit::new(),
			error: Error::new(),
			exiting: Exiting::new(),
			external_editor: ExternalEditor::new(display, config.git.editor.as_str()),
			help: Help::new(),
			list: List::new(config),
			show_commit: ShowCommit::new(config),
			window_size_error: WindowSizeError::new(),
		}
	}

	fn get_module(&self, state: State) -> &dyn ProcessModule {
		match state {
			State::ConfirmAbort => &self.confirm_abort as &dyn ProcessModule,
			State::ConfirmRebase => &self.confirm_rebase as &dyn ProcessModule,
			State::Edit => &self.edit as &dyn ProcessModule,
			State::Error => &self.error as &dyn ProcessModule,
			State::Exiting => &self.exiting as &dyn ProcessModule,
			State::ExternalEditor => &self.external_editor as &dyn ProcessModule,
			State::Help => &self.help as &dyn ProcessModule,
			State::List => &self.list as &dyn ProcessModule,
			State::ShowCommit => &self.show_commit as &dyn ProcessModule,
			State::WindowSizeError => &self.window_size_error as &dyn ProcessModule,
		}
	}

	fn get_mut_module(&mut self, state: State) -> &mut dyn ProcessModule {
		match state {
			State::ConfirmAbort => &mut self.confirm_abort as &mut dyn ProcessModule,
			State::ConfirmRebase => &mut self.confirm_rebase as &mut dyn ProcessModule,
			State::Edit => &mut self.edit as &mut dyn ProcessModule,
			State::Error => &mut self.error as &mut dyn ProcessModule,
			State::Exiting => &mut self.exiting as &mut dyn ProcessModule,
			State::ExternalEditor => &mut self.external_editor as &mut dyn ProcessModule,
			State::Help => &mut self.help as &mut dyn ProcessModule,
			State::List => &mut self.list as &mut dyn ProcessModule,
			State::ShowCommit => &mut self.show_commit as &mut dyn ProcessModule,
			State::WindowSizeError => &mut self.window_size_error as &mut dyn ProcessModule,
		}
	}

	pub fn activate(
		&mut self,
		state: State,
		git_interactive: &GitInteractive,
		previous_state: State,
	) -> Result<(), String>
	{
		self.get_mut_module(state).activate(git_interactive, previous_state)
	}

	pub fn deactivate(&mut self, state: State) {
		self.get_mut_module(state).deactivate()
	}

	pub fn build_view_data(&mut self, state: State, view: &View<'_>, git_interactive: &GitInteractive) -> &ViewData {
		self.get_mut_module(state).build_view_data(view, git_interactive)
	}

	pub fn process(&mut self, state: State, git_interactive: &mut GitInteractive, view: &View<'_>) -> ProcessResult {
		self.get_mut_module(state).process(git_interactive, view)
	}

	pub fn handle_input(
		&mut self,
		state: State,
		input_handler: &InputHandler<'_>,
		git_interactive: &mut GitInteractive,
		view: &View<'_>,
	) -> ProcessResult
	{
		self.get_mut_module(state)
			.handle_input(input_handler, git_interactive, view)
	}

	pub fn set_error_message(&mut self, error: &str) {
		self.error.set_error_message(error);
	}

	pub fn update_help_data(&mut self, state: State) {
		if let Some(ref keybindings_descriptions) = self.get_module(state).get_help_keybindings_descriptions() {
			self.help.update_from_keybindings_descriptions(keybindings_descriptions);
		}
		else if let Some(help_view_data) = self.get_module(state).get_help_view() {
			self.help.update_from_view_data(help_view_data);
		}
		else {
			self.help.clear_help();
		}
	}
}
