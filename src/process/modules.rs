use crate::{
	config::Config,
	confirm_abort::ConfirmAbort,
	confirm_rebase::ConfirmRebase,
	edit::Edit,
	external_editor::ExternalEditor,
	list::List,
	process::{
		error::Error,
		help::Help,
		process_module::ProcessModule,
		process_result::ProcessResult,
		state::State,
		window_size_error::WindowSizeError,
	},
	show_commit::ShowCommit,
	todo_file::TodoFile,
	view::{view_data::ViewData, View},
};

pub struct Modules<'m> {
	pub confirm_abort: ConfirmAbort,
	pub confirm_rebase: ConfirmRebase,
	pub edit: Edit,
	pub error: Error,
	pub external_editor: ExternalEditor,
	pub help: Help,
	pub list: List<'m>,
	pub show_commit: ShowCommit<'m>,
	pub window_size_error: WindowSizeError,
}

impl<'m> Modules<'m> {
	pub fn new(config: &'m Config) -> Self {
		Modules {
			confirm_abort: ConfirmAbort::new(),
			confirm_rebase: ConfirmRebase::new(),
			edit: Edit::new(),
			error: Error::new(),
			external_editor: ExternalEditor::new(config.git.editor.as_str()),
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
			State::ExternalEditor => &mut self.external_editor as &mut dyn ProcessModule,
			State::Help => &mut self.help as &mut dyn ProcessModule,
			State::List => &mut self.list as &mut dyn ProcessModule,
			State::ShowCommit => &mut self.show_commit as &mut dyn ProcessModule,
			State::WindowSizeError => &mut self.window_size_error as &mut dyn ProcessModule,
		}
	}

	pub fn activate(&mut self, state: State, rebase_todo: &TodoFile, previous_state: State) -> ProcessResult {
		self.get_mut_module(state).activate(rebase_todo, previous_state)
	}

	pub fn deactivate(&mut self, state: State) {
		self.get_mut_module(state).deactivate()
	}

	pub fn build_view_data(&mut self, state: State, view: &View<'_>, rebase_todo: &TodoFile) -> &ViewData {
		self.get_mut_module(state).build_view_data(view, rebase_todo)
	}

	pub fn handle_input(&mut self, state: State, view: &mut View<'_>, rebase_todo: &mut TodoFile) -> ProcessResult {
		self.get_mut_module(state).handle_input(view, rebase_todo)
	}

	pub fn set_error_message(&mut self, error: &anyhow::Error) {
		self.error.set_error_message(error);
	}

	pub fn update_help_data(&mut self, state: State) {
		if let Some(ref keybindings_descriptions) = self.get_module(state).get_help_keybindings_descriptions() {
			self.help.update_from_keybindings_descriptions(keybindings_descriptions);
		}
		else {
			self.help.clear();
		}
	}
}

#[cfg(test)]
mod tests {
	// these tests just ensure that nothing panics
	use anyhow::anyhow;
	use rstest::rstest;

	use super::*;
	use crate::{
		input::Input,
		process::testutil::{process_module_test, TestContext, ViewState},
	};

	#[rstest(
		state,
		case::confirm_abort(State::ConfirmAbort),
		case::confirm_rabase(State::ConfirmRebase),
		case::edit(State::Edit),
		case::error(State::Error),
		case::external_editor(State::ExternalEditor),
		case::help(State::Help),
		case::list(State::List),
		case::show_commit(State::ShowCommit),
		case::window_size_error(State::WindowSizeError)
	)]
	#[serial_test::serial]
	fn module_lifecycle(state: State) {
		process_module_test(
			&["pick 18d82dcc4c36cade807d7cf79700b6bbad8080b9 comment"],
			ViewState::default(),
			&[Input::Resize],
			|mut test_context: TestContext<'_>| {
				let mut config = test_context.config.clone();
				config.git.editor = String::from("true");
				let mut modules = Modules::new(&config);
				modules.activate(state, &test_context.rebase_todo_file, State::List);
				modules.handle_input(state, &mut test_context.view, &mut test_context.rebase_todo_file);
				modules.build_view_data(state, &test_context.view, &test_context.rebase_todo_file);
				modules.deactivate(state);
				modules.update_help_data(state);
			},
		);
	}

	#[test]
	#[serial_test::serial]
	fn set_error_message() {
		process_module_test(
			&["pick aaa comment"],
			ViewState::default(),
			&[],
			|test_context: TestContext<'_>| {
				let mut modules = Modules::new(test_context.config);
				modules.set_error_message(&anyhow!("Test Error"));
			},
		);
	}
}
