use config::Config;
#[cfg(test)]
use display::testutil::CrossTerm;
#[cfg(not(test))]
use display::CrossTerm;
use display::{Display, Tui};
use git::Repository;
use input::{EventHandler, KeyBindings};
use todo_file::TodoFile;
use view::View;

use crate::{
	arguments::Args,
	exit::Exit,
	help::build_help,
	module::{ExitStatus, Modules, State},
	modules::{ConfirmAbort, ConfirmRebase, Error, ExternalEditor, Insert, List, ShowCommit, WindowSizeError},
	process::Process,
};

pub(super) fn load_config(repo: &Repository) -> Result<Config, Exit> {
	Config::try_from(repo).map_err(|err| Exit::new(ExitStatus::ConfigError, format!("{:#}", err).as_str()))
}

pub(super) fn load_todo_file(filepath: &str, config: &Config) -> Result<TodoFile, Exit> {
	let mut todo_file = TodoFile::new(filepath, config.undo_limit, config.git.comment_char.as_str());
	if let Err(err) = todo_file.load_file() {
		return Err(Exit::new(ExitStatus::FileReadError, err.to_string().as_str()));
	}

	if todo_file.is_noop() {
		return Err(Exit::new(
			ExitStatus::Good,
			"A noop rebase was provided, skipping editing",
		));
	}

	if todo_file.is_empty() {
		return Err(Exit::new(
			ExitStatus::Good,
			"An empty rebase was provided, nothing to edit",
		));
	}

	Ok(todo_file)
}

pub(super) fn run_process(todo_file: TodoFile, event_handler: EventHandler, config: &Config) -> Exit {
	let mut modules = Modules::new();
	modules.register_module(State::Error, Error::new());
	modules.register_module(State::List, List::new(config));
	modules.register_module(State::ShowCommit, ShowCommit::new(config));
	modules.register_module(State::WindowSizeError, WindowSizeError::new());
	modules.register_module(
		State::ConfirmAbort,
		ConfirmAbort::new(&config.key_bindings.confirm_yes, &config.key_bindings.confirm_no),
	);
	modules.register_module(
		State::ConfirmRebase,
		ConfirmRebase::new(&config.key_bindings.confirm_yes, &config.key_bindings.confirm_no),
	);
	modules.register_module(State::ExternalEditor, ExternalEditor::new(config.git.editor.as_str()));
	modules.register_module(State::Insert, Insert::new());

	let display = Display::new(CrossTerm::new(), &config.theme);
	let mut process = Process::new(
		todo_file,
		event_handler,
		View::new(
			display,
			config.theme.character_vertical_spacing.as_str(),
			config
				.key_bindings
				.help
				.first()
				.map_or(String::from("?"), String::from)
				.as_str(),
		),
	);
	match process.run(modules) {
		Ok(status) => Exit::from(status),
		Err(err) => Exit::new(ExitStatus::FileWriteError, err.to_string().as_str()),
	}
}

pub(crate) fn run(args: &Args) -> Exit {
	if let Some(filepath) = args.todo_file_path().as_ref() {
		let repo = match Repository::open_from_env() {
			Ok(repo) => repo,
			Err(err) => {
				return Exit::new(
					ExitStatus::StateError,
					format!("Unable to load Git repository: {}", err).as_str(),
				);
			},
		};
		let config = match load_config(&repo) {
			Ok(config) => config,
			Err(exit) => return exit,
		};
		let todo_file = match load_todo_file(filepath, &config) {
			Ok(todo_file) => todo_file,
			Err(exit) => return exit,
		};
		let event_handler = EventHandler::new(CrossTerm::read_event, KeyBindings::new(&config.key_bindings));
		run_process(todo_file, event_handler, &config)
	}
	else {
		Exit::new(
			ExitStatus::StateError,
			build_help(Some(String::from("A todo file path must be provided."))).as_str(),
		)
	}
}
