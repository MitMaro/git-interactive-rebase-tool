use config::Config;
#[cfg(test)]
use display::testutil::CrossTerm;
#[cfg(not(test))]
use display::CrossTerm;
use display::Display;
use git::Repository;
use input::EventHandler;
use todo_file::TodoFile;
use view::View;

use crate::{
	arguments::Args,
	events::KeyBindings,
	exit::Exit,
	help::build_help,
	module::{ExitStatus, ModuleHandler, ModuleProvider, Modules},
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

pub(super) fn create_module_handler(config: &Config, repo: Repository) -> ModuleHandler<Modules> {
	ModuleHandler::new(
		EventHandler::new(KeyBindings::new(&config.key_bindings)),
		Modules::new(config, repo),
	)
}

pub(super) fn create_process(todo_file: TodoFile, config: &Config) -> Process {
	let display = Display::new(CrossTerm::new(), &config.theme);
	Process::new(
		todo_file,
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
	)
}

pub(crate) fn run_process<T: ModuleProvider>(mut process: Process, module_handler: ModuleHandler<T>) -> Exit {
	match process.run(module_handler) {
		Ok(status) => Exit::from(status),
		Err(err) => Exit::new(ExitStatus::FileWriteError, err.to_string().as_str()),
	}
}

#[cfg(not(tarpaulin_include))]
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

		let process = create_process(todo_file, &config);
		run_process(process, create_module_handler(&config, repo))
	}
	else {
		Exit::new(
			ExitStatus::StateError,
			build_help(Some(String::from("A todo file path must be provided."))).as_str(),
		)
	}
}
