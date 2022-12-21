use std::sync::Arc;

use anyhow::Result;
use config::Config;
use display::Display;
use git::Repository;
use input::{Event, EventHandler, EventReaderFn};
use parking_lot::Mutex;
use runtime::{Runtime, Threadable};
use todo_file::TodoFile;
use view::View;

use crate::{
	events,
	events::{KeyBindings, MetaEvent},
	help::build_help,
	module::{self, ExitStatus, ModuleHandler},
	process::{self, Process},
	search,
	search::UpdateHandlerFn,
	Args,
	Exit,
};

pub(crate) struct Application<ModuleProvider>
where ModuleProvider: module::ModuleProvider + Send + 'static
{
	_config: Config,
	_repository: Repository,
	process: Process<ModuleProvider>,
	threads: Option<Vec<Box<dyn Threadable>>>,
}

impl<ModuleProvider> Application<ModuleProvider>
where ModuleProvider: module::ModuleProvider + Send + 'static
{
	pub(crate) fn new<EventProvider, Tui>(args: &Args, event_provider: EventProvider, tui: Tui) -> Result<Self, Exit>
	where
		EventProvider: EventReaderFn,
		Tui: display::Tui + Send + 'static,
	{
		let filepath = Self::filepath_from_args(args)?;
		let repository = Self::open_repository()?;
		let config = Self::load_config(&repository)?;
		let todo_file = Arc::new(Mutex::new(Self::load_todo_file(filepath.as_str(), &config)?));

		let module_handler = ModuleHandler::new(
			EventHandler::new(KeyBindings::new(&config.key_bindings)),
			ModuleProvider::new(&config, repository.clone(), &todo_file),
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
				.map_or_else(|| String::from("?"), String::from)
				.as_str(),
		);

		let mut threads: Vec<Box<dyn Threadable>> = vec![];
		let runtime = Runtime::new();

		let input_threads = events::Thread::new(event_provider);
		let input_state = input_threads.state();
		threads.push(Box::new(input_threads));

		let view_threads = view::Thread::new(view);
		let view_state = view_threads.state();
		threads.push(Box::new(view_threads));

		let search_update_handler = Self::create_search_update_handler(input_state.clone());
		let search_threads = search::Thread::new(search_update_handler);
		let search_state = search_threads.state();
		threads.push(Box::new(search_threads));

		let process = Process::new(
			initial_display_size,
			todo_file,
			module_handler,
			input_state,
			view_state,
			search_state,
			runtime.statuses(),
		);
		let process_threads = process::Thread::new(process.clone());
		threads.push(Box::new(process_threads));

		Ok(Self {
			_config: config,
			_repository: repository,
			process,
			threads: Some(threads),
		})
	}

	pub(crate) fn run_until_finished(&mut self) -> Result<(), Exit> {
		let Some(mut threads) = self.threads.take() else {
			return Err(Exit::new(ExitStatus::StateError, "Attempt made to run application a second time"));
		};

		let runtime = Runtime::new();

		for thread in &mut threads {
			runtime.register(thread.as_mut());
		}

		runtime.join().map_err(|err| {
			Exit::new(
				ExitStatus::StateError,
				format!("Failed to join runtime: {err}").as_str(),
			)
		})?;

		let exit_status = self.process.exit_status();
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
				format!("Unable to load Git repository: {err}").as_str(),
			);
		})
	}

	fn load_config(repo: &Repository) -> Result<Config, Exit> {
		Config::try_from(repo).map_err(|err| Exit::new(ExitStatus::ConfigError, format!("{err:#}").as_str()))
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

	fn create_search_update_handler(input_state: events::State) -> impl Fn() + Send + Sync {
		move || input_state.push_event(Event::MetaEvent(MetaEvent::SearchUpdate))
	}
}

#[cfg(all(unix, test))]
mod tests {
	use std::ffi::OsString;

	use claim::assert_ok;
	use display::{testutil::CrossTerm, Size};
	use input::{KeyCode, KeyEvent, KeyModifiers};
	use runtime::{Installer, RuntimeError};

	use super::*;
	use crate::{
		events::Event,
		module::Modules,
		testutil::{create_event_reader, set_git_directory, DefaultTestModule, TestModuleProvider},
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
		let event_provider = create_event_reader(|| Ok(None));
		let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> =
			Application::new(&args(&[]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::StateError);
		assert!(
			exit.get_message()
				.as_ref()
				.unwrap()
				.contains("A todo file path must be provided")
		);
	}

	#[test]
	#[serial_test::serial]
	fn load_repository_failure() {
		let _ = set_git_directory("fixtures/not-a-repository");
		let event_provider = create_event_reader(|| Ok(None));
		let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> =
			Application::new(&args(&["todofile"]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::StateError);
		assert!(
			exit.get_message()
				.as_ref()
				.unwrap()
				.contains("Unable to load Git repository: ")
		);
	}

	#[test]
	#[serial_test::serial]
	fn load_config_failure() {
		let _ = set_git_directory("fixtures/invalid-config");
		let event_provider = create_event_reader(|| Ok(None));
		let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> =
			Application::new(&args(&["rebase-todo"]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::ConfigError);
	}

	#[test]
	#[serial_test::serial]
	fn load_todo_file_load_error() {
		let _ = set_git_directory("fixtures/simple");
		let event_provider = create_event_reader(|| Ok(None));
		let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> =
			Application::new(&args(&["does-not-exist"]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::FileReadError);
	}

	#[test]
	#[serial_test::serial]
	fn load_todo_file_noop() {
		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{git_dir}/rebase-todo-noop");
		let event_provider = create_event_reader(|| Ok(None));
		let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> = Application::new(
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
		let rebase_todo = format!("{git_dir}/rebase-todo-empty");
		let event_provider = create_event_reader(|| Ok(None));
		let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		);
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::Good);
		assert!(
			exit.get_message()
				.as_ref()
				.unwrap()
				.contains("An empty rebase was provided, nothing to edit")
		);
	}

	#[test]
	#[serial_test::serial]
	fn search_update_handler_handles_update() {
		let event_provider = create_event_reader(|| Ok(None));
		let input_threads = events::Thread::new(event_provider);
		let input_state = input_threads.state();
		let update_handler =
			Application::<TestModuleProvider<DefaultTestModule>>::create_search_update_handler(input_state.clone());
		update_handler();

		assert_eq!(input_state.read_event(), Event::MetaEvent(MetaEvent::SearchUpdate));
	}

	#[test]
	#[serial_test::serial]
	fn run_until_finished_success() {
		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{git_dir}/rebase-todo");
		let event_provider = create_event_reader(|| Ok(Some(Event::Key(KeyEvent::from(KeyCode::Char('W'))))));
		let mut application: Application<Modules> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		)
		.unwrap();
		assert_ok!(application.run_until_finished());
	}

	#[test]
	#[serial_test::serial]
	fn run_join_error() {
		struct FailingThread;
		impl Threadable for FailingThread {
			fn install(&self, installer: &Installer) {
				installer.spawn("THREAD", |notifier| {
					move || {
						notifier.error(RuntimeError::ThreadSpawnError(String::from("Error")));
					}
				});
			}
		}

		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{git_dir}/rebase-todo");
		let event_provider = create_event_reader(|| Ok(Some(Event::Key(KeyEvent::from(KeyCode::Char('W'))))));
		let mut application: Application<Modules> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		)
		.unwrap();

		application.threads = Some(vec![Box::new(FailingThread {})]);

		let exit = application.run_until_finished().unwrap_err();
		assert_eq!(exit.get_status(), &ExitStatus::StateError);
		assert!(
			exit.get_message()
				.as_ref()
				.unwrap()
				.starts_with("Failed to join runtime:")
		);
	}

	#[test]
	#[serial_test::serial]
	fn run_until_finished_kill() {
		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{git_dir}/rebase-todo");
		let event_provider = create_event_reader(|| {
			Ok(Some(Event::Key(KeyEvent::new(
				KeyCode::Char('c'),
				KeyModifiers::CONTROL,
			))))
		});
		let mut application: Application<Modules> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		)
		.unwrap();
		let exit = application.run_until_finished().unwrap_err();
		assert_eq!(exit.get_status(), &ExitStatus::Kill);
	}

	#[test]
	#[serial_test::serial]
	fn run_error_on_second_attempt() {
		let git_dir = set_git_directory("fixtures/simple");
		let rebase_todo = format!("{git_dir}/rebase-todo");
		let event_provider = create_event_reader(|| Ok(Some(Event::Key(KeyEvent::from(KeyCode::Char('W'))))));
		let mut application: Application<Modules> = Application::new(
			&args(&[rebase_todo.as_str()]),
			event_provider,
			create_mocked_crossterm(),
		)
		.unwrap();
		assert_ok!(application.run_until_finished());
		let exit = application.run_until_finished().unwrap_err();
		assert_eq!(exit.get_status(), &ExitStatus::StateError);
		assert_eq!(
			exit.get_message().as_ref().unwrap(),
			"Attempt made to run application a second time"
		);
	}
}
