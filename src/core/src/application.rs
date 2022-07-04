use anyhow::Result;
use config::Config;
use display::{Display, Size};
use git::Repository;
use input::{EventHandler, RawEvent};
use runtime::Runtime;
use todo_file::TodoFile;
use view::View;

use crate::{
	events,
	events::KeyBindings,
	help::build_help,
	module::{self, ExitStatus, ModuleHandler},
	process::{self, Process},
	Args,
	Exit,
};

pub(crate) struct Application<ModuleProvider, EventProvider, Tui>
where
	ModuleProvider: module::ModuleProvider + Send + 'static,
	EventProvider: Fn() -> Result<Option<RawEvent>> + Send + Sync + 'static,
	Tui: display::Tui + 'static,
{
	_config: Config,
	_repository: Repository,
	todo_file: TodoFile,
	view: View<Tui>,
	event_provider: EventProvider,
	initial_display_size: Size,
	module_handler: ModuleHandler<ModuleProvider>,
}

impl<ModuleProvider, EventProvider, Tui> Application<ModuleProvider, EventProvider, Tui>
where
	ModuleProvider: module::ModuleProvider + Send + 'static,
	EventProvider: Fn() -> Result<Option<RawEvent>> + Send + Sync + 'static,
	Tui: display::Tui + Send + 'static,
{
	pub(crate) fn new(args: &Args, event_provider: EventProvider, tui: Tui) -> Result<Self, Exit> {
		let filepath = Self::filepath_from_args(args)?;
		let repository = Self::open_repository()?;
		let config = Self::load_config(&repository)?;
		let todo_file = Self::load_todo_file(filepath.as_str(), &config)?;

		let module_handler = ModuleHandler::new(
			EventHandler::new(KeyBindings::new(&config.key_bindings)),
			ModuleProvider::new(&config, repository.clone()),
		);

		let display = Display::new(tui, &config.theme);
		let initial_display_size = display.get_window_size();
		let view = View::new(
			display,
			config.theme.character_vertical_spacing.as_str(),
			config
				.key_bindings
				.help
				.first()
				.map_or(String::from("?"), String::from)
				.as_str(),
		);

		Ok(Self {
			_config: config,
			_repository: repository,
			todo_file,
			view,
			event_provider,
			module_handler,
			initial_display_size,
		})
	}

	pub(crate) fn run_until_finished(self) -> Result<(), Exit> {
		let runtime = Runtime::new();

		let mut input_threads = events::Thread::new(self.event_provider);
		let input_state = input_threads.state();

		let mut view_threads = view::Thread::new(self.view);
		let view_state = view_threads.state();

		let process = Process::new(
			self.initial_display_size,
			self.todo_file,
			self.module_handler,
			input_state,
			view_state,
			runtime.statuses(),
		);
		let mut process_threads = process::Thread::new(process.clone());

		runtime.register(&mut input_threads);
		runtime.register(&mut view_threads);
		runtime.register(&mut process_threads);

		runtime.join().map_err(|err| {
			Exit::new(
				ExitStatus::StateError,
				format!("Failed to join runtime: {}", err).as_str(),
			)
		})?;

		let exit_status = process.exit_status();
		if exit_status != ExitStatus::Good {
			return Err(Exit::from(exit_status));
		}

		Ok(())
	}

	fn filepath_from_args(args: &Args) -> Result<String, Exit> {
		args.todo_file_path().as_ref().map(String::from).ok_or_else(|| {
			Exit::new(
				ExitStatus::StateError,
				build_help(Some(String::from("A todo file path must be provided."))).as_str(),
			)
		})
	}

	fn open_repository() -> Result<Repository, Exit> {
		Repository::open_from_env().map_err(|err| {
			return Exit::new(
				ExitStatus::StateError,
				format!("Unable to load Git repository: {}", err).as_str(),
			);
		})
	}

	fn load_config(repo: &Repository) -> Result<Config, Exit> {
		Config::try_from(repo).map_err(|err| Exit::new(ExitStatus::ConfigError, format!("{:#}", err).as_str()))
	}

	fn load_todo_file(filepath: &str, config: &Config) -> Result<TodoFile, Exit> {
		let mut todo_file = TodoFile::new(filepath, config.undo_limit, config.git.comment_char.as_str());
		todo_file
			.load_file()
			.map_err(|err| Exit::new(ExitStatus::FileReadError, err.to_string().as_str()))?;

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
}

#[cfg(all(unix, test))]
mod tests {
	use std::ffi::OsString;

	use display::testutil::CrossTerm;
	use input::{KeyCode, KeyEvent, KeyModifiers};

	use super::*;
	use crate::{
		module::Modules,
		testutil::{set_git_directory, DefaultTestModule, TestModuleProvider},
	};

	fn args(args: &[&str]) -> Args {
		Args::try_from(args.iter().map(OsString::from).collect::<Vec<OsString>>()).unwrap()
	}

	fn create_mocked_crossterm() -> CrossTerm {
		let mut crossterm = CrossTerm::new();
		crossterm.set_size(Size::new(300, 120));
		crossterm
	}

	macro_rules! application_error {
		($app:expr) => {
			if let Err(e) = $app {
				e
			}
			else {
				panic!("Application is not in an error state");
			}
		};
	}

	#[test]
	#[serial_test::serial]
	fn load_filepath_from_args_failure() {
		let event_provider = || Ok(None);
		let application: Result<Application<TestModuleProvider<DefaultTestModule>, _, _>, Exit> =
			Application::new(&args(&[]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::StateError);
		assert!(exit
			.get_message()
			.as_ref()
			.unwrap()
			.contains("A todo file path must be provided"));
	}

	#[test]
	#[serial_test::serial]
	fn load_repository_failure() {
		let _ = set_git_directory("fixtures/not-a-repository");
		let event_provider = || Ok(None);
		let application: Result<Application<TestModuleProvider<DefaultTestModule>, _, _>, Exit> =
			Application::new(&args(&["todofile"]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::StateError);
		assert!(exit
			.get_message()
			.as_ref()
			.unwrap()
			.contains("Unable to load Git repository: "));
	}

	#[test]
	#[serial_test::serial]
	fn load_config_failure() {
		let _ = set_git_directory("fixtures/invalid-config");
		let event_provider = || Ok(None);
		let application: Result<Application<TestModuleProvider<DefaultTestModule>, _, _>, Exit> =
			Application::new(&args(&["rebase-todo"]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::ConfigError);
	}

	#[test]
	#[serial_test::serial]
	fn load_todo_file_load_error() {
		let _ = set_git_directory("fixtures/simple");
		let event_provider = || Ok(None);
		let application: Result<Application<TestModuleProvider<DefaultTestModule>, _, _>, Exit> =
			Application::new(&args(&["does-not-exist"]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::FileReadError);
	}

	#[test]
	#[serial_test::serial]
	fn load_todo_file_noop() {
		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{}/rebase-todo-noop", git_dir);
		let event_provider = || Ok(None);
		let application: Result<Application<TestModuleProvider<DefaultTestModule>, _, _>, Exit> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		);
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::Good);
	}

	#[test]
	#[serial_test::serial]
	fn load_todo_file_empty() {
		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{}/rebase-todo-empty", git_dir);
		let event_provider = || Ok(None);
		let application: Result<Application<TestModuleProvider<DefaultTestModule>, _, _>, Exit> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		);
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::Good);
		assert!(exit
			.get_message()
			.as_ref()
			.unwrap()
			.contains("An empty rebase was provided, nothing to edit"));
	}

	#[test]
	#[serial_test::serial]
	fn run_until_finished_success() {
		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{}/rebase-todo", git_dir);
		let event_provider = || Ok(Some(RawEvent::Key(KeyEvent::from(KeyCode::Char('W')))));
		let application: Application<Modules, _, _> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		)
		.unwrap();
		assert!(application.run_until_finished().is_ok());
	}

	#[test]
	#[serial_test::serial]
	fn run_until_finished_kill() {
		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{}/rebase-todo", git_dir);
		let event_provider = || {
			Ok(Some(RawEvent::Key(KeyEvent::new(
				KeyCode::Char('c'),
				KeyModifiers::CONTROL,
			))))
		};
		let application: Application<Modules, _, _> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		)
		.unwrap();
		let exit = application.run_until_finished().unwrap_err();
		assert_eq!(exit.get_status(), &ExitStatus::Kill);
	}
}
