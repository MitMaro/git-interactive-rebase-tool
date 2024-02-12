use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;

use crate::{
	config::Config,
	display::Display,
	events,
	events::{KeyBindings, MetaEvent},
	git::Repository,
	help::build_help,
	input::{Event, EventHandler, EventReaderFn},
	module::{self, ExitStatus, ModuleHandler},
	process::{self, Process},
	runtime::{Runtime, ThreadStatuses, Threadable},
	search,
	todo_file::{TodoFile, TodoFileOptions},
	view::View,
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
	thread_statuses: ThreadStatuses,
}

impl<ModuleProvider> Application<ModuleProvider>
where ModuleProvider: module::ModuleProvider + Send + 'static
{
	pub(crate) fn new<EventProvider, Tui>(args: &Args, event_provider: EventProvider, tui: Tui) -> Result<Self, Exit>
	where
		EventProvider: EventReaderFn,
		Tui: crate::display::Tui + Send + 'static,
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

		let thread_statuses = ThreadStatuses::new();
		let mut threads: Vec<Box<dyn Threadable>> = vec![];

		let input_threads = events::Thread::new(event_provider);
		let input_state = input_threads.state();
		threads.push(Box::new(input_threads));

		let view_threads = crate::view::Thread::new(view);
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
			thread_statuses.clone(),
		);
		let process_threads = process::Thread::new(process.clone());
		threads.push(Box::new(process_threads));

		Ok(Self {
			_config: config,
			_repository: repository,
			process,
			threads: Some(threads),
			thread_statuses,
		})
	}

	pub(crate) fn run_until_finished(&mut self) -> Result<(), Exit> {
		let Some(mut threads) = self.threads.take()
		else {
			return Err(Exit::new(
				ExitStatus::StateError,
				"Attempt made to run application a second time",
			));
		};

		let runtime = Runtime::new(self.thread_statuses.clone());

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

	fn todo_file_options(config: &Config) -> TodoFileOptions {
		let mut todo_file_options = TodoFileOptions::new(config.undo_limit, config.git.comment_char.as_str());
		if let Some(command) = config.post_modified_line_exec_command.as_deref() {
			todo_file_options.line_changed_command(command);
		}
		todo_file_options
	}

	fn load_todo_file(filepath: &str, config: &Config) -> Result<TodoFile, Exit> {
		let mut todo_file = TodoFile::new(filepath, Self::todo_file_options(config));
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

	use claims::assert_ok;

	use super::*;
	use crate::{
		display::Size,
		events::Event,
		input::{KeyCode, KeyEvent, KeyModifiers},
		module::Modules,
		runtime::{Installer, RuntimeError},
		test_helpers::mocks::crossterm::CrossTerm,
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
		_ = set_git_directory("fixtures/not-a-repository");
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
		_ = set_git_directory("fixtures/invalid-config");
		let event_provider = create_event_reader(|| Ok(None));
		let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> =
			Application::new(&args(&["rebase-todo"]), event_provider, create_mocked_crossterm());
		let exit = application_error!(application);
		assert_eq!(exit.get_status(), &ExitStatus::ConfigError);
	}

	#[test]
	fn todo_file_options_without_command() {
		let mut config = Config::new();
		config.undo_limit = 10;
		config.git.comment_char = String::from("#");
		config.post_modified_line_exec_command = None;

		let expected = TodoFileOptions::new(10, "#");
		assert_eq!(
			Application::<TestModuleProvider<DefaultTestModule>>::todo_file_options(&config),
			expected
		);
	}

	#[test]
	fn todo_file_options_with_command() {
		let mut config = Config::new();
		config.undo_limit = 10;
		config.git.comment_char = String::from("#");
		config.post_modified_line_exec_command = Some(String::from("command"));

		let mut expected = TodoFileOptions::new(10, "#");
		expected.line_changed_command("command");

		assert_eq!(
			Application::<TestModuleProvider<DefaultTestModule>>::todo_file_options(&config),
			expected
		);
	}

	#[test]
	#[serial_test::serial]
	fn load_todo_file_load_error() {
		_ = set_git_directory("fixtures/simple");
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
