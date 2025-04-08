mod app_data;

use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;

pub(crate) use crate::application::app_data::AppData;
use crate::{
	Args,
	Exit,
	config::{Config, ConfigLoader, DiffIgnoreWhitespaceSetting},
	diff::{self, CommitDiffLoader, CommitDiffLoaderOptions},
	display::Display,
	git::open_repository_from_env,
	help::build_help,
	input::{Event, EventHandler, EventReaderFn, KeyBindings, StandardEvent},
	module::{self, ExitStatus, ModuleHandler, State},
	process::{self, Process},
	runtime::{Runtime, ThreadStatuses, Threadable},
	search,
	todo_file::{TodoFile, TodoFileOptions},
	view::View,
};

pub(crate) struct Application<ModuleProvider>
where ModuleProvider: module::ModuleProvider + Send + 'static
{
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
		let config_loader = ConfigLoader::from(repository);
		let config = Self::load_config(&config_loader)?;
		let todo_file = Arc::new(Mutex::new(Self::load_todo_file(filepath.as_str(), &config)?));

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

		let input_threads = crate::input::Thread::new(event_provider);
		let input_state = input_threads.state();
		threads.push(Box::new(input_threads));

		let view_threads = crate::view::Thread::new(view);
		let view_state = view_threads.state();
		threads.push(Box::new(view_threads));

		let search_update_handler = Self::create_search_update_handler(input_state.clone());
		let search_threads = search::Thread::new(search_update_handler);
		let search_state = search_threads.state();
		threads.push(Box::new(search_threads));

		let commit_diff_loader_options = CommitDiffLoaderOptions::new()
			.context_lines(config.git.diff_context)
			.copies(config.git.diff_copies)
			.ignore_whitespace(config.diff_ignore_whitespace == DiffIgnoreWhitespaceSetting::All)
			.ignore_whitespace_change(config.diff_ignore_whitespace == DiffIgnoreWhitespaceSetting::Change)
			.ignore_blank_lines(config.diff_ignore_blank_lines)
			.interhunk_context(config.git.diff_interhunk_lines)
			.renames(config.git.diff_renames, config.git.diff_rename_limit);
		let commit_diff_loader = CommitDiffLoader::new(config_loader.eject_repository(), commit_diff_loader_options);

		let diff_update_handler = Self::create_diff_update_handler(input_state.clone());
		let diff_thread = diff::thread::Thread::new(commit_diff_loader, diff_update_handler);
		let diff_state = diff_thread.state();
		threads.push(Box::new(diff_thread));

		let keybindings = KeyBindings::new(&config.key_bindings);

		let app_data = AppData::new(
			config,
			State::WindowSizeError,
			Arc::clone(&todo_file),
			diff_state.clone(),
			view_state.clone(),
			input_state.clone(),
			search_state.clone(),
		);

		let module_handler = ModuleHandler::new(EventHandler::new(keybindings), ModuleProvider::new(&app_data));

		let process = Process::new(&app_data, initial_display_size, module_handler, thread_statuses.clone());
		let process_threads = process::Thread::new(process.clone());
		threads.push(Box::new(process_threads));

		Ok(Self {
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
		args.todo_file_path().map(String::from).ok_or_else(|| {
			Exit::new(
				ExitStatus::StateError,
				build_help(Some(String::from("A todo file path must be provided."))).as_str(),
			)
		})
	}

	fn open_repository() -> Result<git2::Repository, Exit> {
		open_repository_from_env().map_err(|err| {
			Exit::new(
				ExitStatus::StateError,
				format!("Unable to load Git repository: {err}").as_str(),
			)
		})
	}

	fn load_config(config_loader: &ConfigLoader) -> Result<Config, Exit> {
		Config::try_from(config_loader).map_err(|err| Exit::new(ExitStatus::ConfigError, format!("{err:#}").as_str()))
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

	fn create_search_update_handler(input_state: crate::input::State) -> impl Fn() + Send + Sync {
		move || input_state.push_event(Event::Standard(StandardEvent::SearchUpdate))
	}

	fn create_diff_update_handler(input_state: crate::input::State) -> impl Fn() + Send + Sync {
		move || input_state.push_event(Event::Standard(StandardEvent::DiffUpdate))
	}
}

#[cfg(all(unix, test))]
mod tests {
	use std::ffi::OsString;

	use claims::assert_ok;

	use super::*;
	use crate::{
		display::Size,
		input::{KeyCode, KeyEvent, KeyModifiers},
		module::Modules,
		runtime::{Installer, RuntimeError},
		test_helpers::{
			DefaultTestModule,
			TestModuleProvider,
			create_config,
			create_event_reader,
			mocks,
			with_git_directory,
		},
	};

	fn args(args: &[&str]) -> Args {
		Args::try_from(args.iter().map(OsString::from).collect::<Vec<OsString>>()).unwrap()
	}

	fn create_mocked_crossterm() -> mocks::CrossTerm {
		let mut crossterm = mocks::CrossTerm::new();
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
				.unwrap()
				.contains("A todo file path must be provided")
		);
	}

	#[test]
	fn load_repository_failure() {
		with_git_directory("fixtures/not-a-repository", |_| {
			let event_provider = create_event_reader(|| Ok(None));
			let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> =
				Application::new(&args(&["todofile"]), event_provider, create_mocked_crossterm());
			let exit = application_error!(application);
			assert_eq!(exit.get_status(), &ExitStatus::StateError);
			assert!(exit.get_message().unwrap().contains("Unable to load Git repository: "));
		});
	}

	#[test]
	fn load_config_failure() {
		with_git_directory("fixtures/invalid-config", |_| {
			let event_provider = create_event_reader(|| Ok(None));
			let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> =
				Application::new(&args(&["rebase-todo"]), event_provider, create_mocked_crossterm());
			let exit = application_error!(application);
			assert_eq!(exit.get_status(), &ExitStatus::ConfigError);
		});
	}

	#[test]
	fn todo_file_options_without_command() {
		let mut config = create_config();
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
		let mut config = create_config();
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
	fn load_todo_file_load_error() {
		with_git_directory("fixtures/simple", |_| {
			let event_provider = create_event_reader(|| Ok(None));
			let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> =
				Application::new(&args(&["does-not-exist"]), event_provider, create_mocked_crossterm());
			let exit = application_error!(application);
			assert_eq!(exit.get_status(), &ExitStatus::FileReadError);
		});
	}

	#[test]
	fn load_todo_file_noop() {
		with_git_directory("fixtures/simple", |git_dir| {
			let rebase_todo = format!("{git_dir}/rebase-todo-noop");
			let event_provider = create_event_reader(|| Ok(None));
			let application: Result<Application<TestModuleProvider<DefaultTestModule>>, Exit> = Application::new(
				&args(&[rebase_todo.as_str()]),
				event_provider,
				create_mocked_crossterm(),
			);
			let exit = application_error!(application);
			assert_eq!(exit.get_status(), &ExitStatus::Good);
		});
	}

	#[test]
	fn load_todo_file_empty() {
		with_git_directory("fixtures/simple", |git_dir| {
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
					.unwrap()
					.contains("An empty rebase was provided, nothing to edit")
			);
		});
	}

	#[test]
	#[serial_test::serial]
	fn search_update_handler_handles_update() {
		let event_provider = create_event_reader(|| Ok(None));
		let input_threads = crate::input::Thread::new(event_provider);
		let input_state = input_threads.state();
		let update_handler =
			Application::<TestModuleProvider<DefaultTestModule>>::create_search_update_handler(input_state.clone());
		update_handler();

		assert_eq!(input_state.read_event(), Event::Standard(StandardEvent::SearchUpdate));
	}

	#[test]
	fn run_until_finished_success() {
		with_git_directory("fixtures/simple", |git_dir| {
			let rebase_todo = format!("{git_dir}/rebase-todo");
			let event_provider = create_event_reader(|| Ok(Some(Event::Key(KeyEvent::from(KeyCode::Char('W'))))));
			let mut application: Application<Modules> = Application::new(
				&args(&[rebase_todo.as_str()]),
				event_provider,
				create_mocked_crossterm(),
			)
			.unwrap();
			assert_ok!(application.run_until_finished());
		});
	}

	#[test]
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

		with_git_directory("fixtures/simple", |git_dir| {
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
			assert!(exit.get_message().unwrap().starts_with("Failed to join runtime:"));
		});
	}

	#[test]
	fn run_until_finished_kill() {
		with_git_directory("fixtures/simple", |git_dir| {
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
		});
	}

	#[test]
	fn run_error_on_second_attempt() {
		with_git_directory("fixtures/simple", |git_dir| {
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
				exit.get_message().unwrap(),
				"Attempt made to run application a second time"
			);
		});
	}
}
