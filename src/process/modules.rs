use crate::{
	config::Config,
	confirm_abort::ConfirmAbort,
	confirm_rebase::ConfirmRebase,
	external_editor::ExternalEditor,
	input::EventHandler,
	insert::Insert,
	list::List,
	process::{
		error::Error,
		process_module::ProcessModule,
		process_result::ProcessResult,
		state::State,
		window_size_error::WindowSizeError,
	},
	show_commit::ShowCommit,
	todo_file::TodoFile,
	view::{RenderContext, ViewData, ViewSender},
};

pub struct Modules<'m> {
	pub confirm_abort: ConfirmAbort,
	pub confirm_rebase: ConfirmRebase,
	pub error: Error,
	pub external_editor: ExternalEditor,
	pub insert: Insert,
	pub list: List<'m>,
	pub show_commit: ShowCommit<'m>,
	pub window_size_error: WindowSizeError,
}

impl<'m> Modules<'m> {
	pub fn new(config: &'m Config) -> Self {
		Modules {
			confirm_abort: ConfirmAbort::new(&config.key_bindings.confirm_yes, &config.key_bindings.confirm_no),
			confirm_rebase: ConfirmRebase::new(&config.key_bindings.confirm_yes, &config.key_bindings.confirm_no),
			error: Error::new(),
			external_editor: ExternalEditor::new(config.git.editor.as_str()),
			insert: Insert::new(),
			list: List::new(config),
			show_commit: ShowCommit::new(config),
			window_size_error: WindowSizeError::new(),
		}
	}

	fn get_mut_module(&mut self, state: State) -> &mut dyn ProcessModule {
		match state {
			State::ConfirmAbort => &mut self.confirm_abort as &mut dyn ProcessModule,
			State::ConfirmRebase => &mut self.confirm_rebase as &mut dyn ProcessModule,
			State::Error => &mut self.error as &mut dyn ProcessModule,
			State::ExternalEditor => &mut self.external_editor as &mut dyn ProcessModule,
			State::Insert => &mut self.insert as &mut dyn ProcessModule,
			State::List => &mut self.list as &mut dyn ProcessModule,
			State::ShowCommit => &mut self.show_commit as &mut dyn ProcessModule,
			State::WindowSizeError => &mut self.window_size_error as &mut dyn ProcessModule,
		}
	}

	pub fn activate(&mut self, state: State, rebase_todo: &TodoFile, previous_state: State) -> ProcessResult {
		self.get_mut_module(state).activate(rebase_todo, previous_state)
	}

	pub fn deactivate(&mut self, state: State) {
		self.get_mut_module(state).deactivate();
	}

	pub fn build_view_data(
		&mut self,
		state: State,
		render_context: &RenderContext,
		rebase_todo: &TodoFile,
	) -> &ViewData {
		self.get_mut_module(state).build_view_data(render_context, rebase_todo)
	}

	pub fn handle_input(
		&mut self,
		state: State,
		event_handler: &EventHandler,
		view_sender: &ViewSender,
		rebase_todo: &mut TodoFile,
	) -> ProcessResult {
		self.get_mut_module(state)
			.handle_events(event_handler, view_sender, rebase_todo)
	}

	pub fn set_error_message(&mut self, error: &anyhow::Error) {
		self.error.set_error_message(error);
	}
}

#[cfg(test)]
mod tests {
	// these tests just ensure that nothing panics

	use anyhow::anyhow;
	use rstest::rstest;

	use super::*;
	use crate::{
		input::Event,
		process::testutil::{process_module_test, TestContext},
	};

	#[rstest(
		state,
		case::confirm_abort(State::ConfirmAbort),
		case::confirm_rabase(State::ConfirmRebase),
		case::error(State::Error),
		case::external_editor(State::ExternalEditor),
		case::insert(State::Insert),
		case::list(State::List),
		case::show_commit(State::ShowCommit),
		case::window_size_error(State::WindowSizeError)
	)]
	#[serial_test::serial]
	fn module_lifecycle(state: State) {
		process_module_test(
			&["pick 18d82dcc4c36cade807d7cf79700b6bbad8080b9 comment"],
			&[Event::Resize(100, 100)],
			|mut test_context: TestContext<'_>| {
				test_context.set_git_directory_environment();
				let config = test_context.config.clone();
				let mut modules = Modules::new(&config);
				modules.activate(state, &test_context.rebase_todo_file, State::List);
				modules.handle_input(
					state,
					&test_context.event_handler_context.event_handler,
					&test_context.event_handler_context.view_sender,
					&mut test_context.rebase_todo_file,
				);
				modules.build_view_data(state, &test_context.render_context, &test_context.rebase_todo_file);
				modules.deactivate(state);
			},
		);
	}

	#[test]
	fn set_error_message() {
		process_module_test(&["pick aaa comment"], &[], |test_context: TestContext<'_>| {
			let mut modules = Modules::new(test_context.config);
			modules.set_error_message(&anyhow!("Test Error"));
		});
	}
}
