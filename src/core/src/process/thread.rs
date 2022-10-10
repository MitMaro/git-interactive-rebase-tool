use std::sync::Arc;

use captur::capture;
use runtime::{Installer, RuntimeError, Threadable};

use crate::{
	module,
	module::{ExitStatus, State},
	process::{Process, Results},
};

pub(crate) const THEAD_NAME: &str = "core_process";

pub(crate) struct Thread<ModuleProvider: module::ModuleProvider + Send + 'static> {
	process: Arc<Process<ModuleProvider>>,
}

impl<ModuleProvider: module::ModuleProvider + Send + 'static> Threadable for Thread<ModuleProvider> {
	fn install(&self, installer: &Installer) {
		let process = Arc::clone(&self.process);

		installer.spawn(THEAD_NAME, |notifier| {
			move || {
				capture!(notifier, process);
				notifier.busy();

				process.handle_results(Results::from(State::List));

				while !process.should_exit() {
					notifier.busy();
					process.render();

					while !process.should_exit() {
						notifier.wait();
						if let Some(results) = process.handle_event() {
							notifier.busy();
							process.handle_results(results);
							break;
						}
					}
				}

				if !process.is_exit_status_kill() {
					if let Err(err) = process.write_todo_file() {
						process.handle_results(Results::from(ExitStatus::FileWriteError));
						notifier.error(RuntimeError::ThreadError(err.to_string()));
						return;
					}
				}

				notifier.request_end();
				notifier.end();
			}
		});
	}

	fn end(&self) {
		self.process.end();
	}
}

impl<ModuleProvider: module::ModuleProvider + Send + 'static> Thread<ModuleProvider> {
	pub(crate) fn new(process: Process<ModuleProvider>) -> Self {
		Self {
			process: Arc::new(process),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::fs::File;

	use input::StandardEvent;
	use runtime::{testutils::ThreadableTester, Status};
	use todo_file::{Line, TodoFile};

	use super::*;
	use crate::{
		events::Event,
		module::Module,
		testutil::{create_default_test_module_handler, create_test_module_handler, process_test, ProcessTestContext},
	};

	#[test]
	fn end() {
		process_test(
			create_default_test_module_handler(),
			|ProcessTestContext { process, .. }| {
				let thread = Thread::new(process);
				thread.end();
				assert!(thread.process.is_ended());
			},
		);
	}

	#[test]
	fn start() {
		process_test(
			create_default_test_module_handler(),
			|ProcessTestContext { process, .. }| {
				let thread = Thread::new(process);
				thread.end();
				let tester = ThreadableTester::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_status(&Status::Ended);

				assert_eq!(thread.process.state(), State::List);
			},
		);
	}

	#[test]
	fn render() {
		process_test(
			create_default_test_module_handler(),
			|ProcessTestContext {
			     process,
			     event_handler_context,
			     ..
			 }| {
				event_handler_context
					.state
					.enqueue_event(Event::from(StandardEvent::Exit));
				let thread = Thread::new(process);
				let tester = ThreadableTester::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_status(&Status::Ended);
			},
		);
	}

	#[test]
	fn run_success() {
		struct TestModule {}

		impl Module for TestModule {
			fn handle_event(&mut self, _: Event, _: &view::State, rebase_todo: &mut TodoFile) -> Results {
				rebase_todo.add_line(0, Line::new("pick 123456789 comment").unwrap());
				Results::from(ExitStatus::Good)
			}
		}

		process_test(
			create_test_module_handler(TestModule {}),
			|ProcessTestContext {
			     process,
			     todo_file_path,
			     ..
			 }| {
				let thread = Thread::new(process);
				let tester = ThreadableTester::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_status(&Status::Ended);
				let mut todo_file = TodoFile::new(todo_file_path.to_str().unwrap(), 0, "#");
				todo_file.load_file().unwrap();
				assert_eq!(
					todo_file.get_line(0).unwrap(),
					&Line::new("pick 123456789 comment").unwrap()
				);
			},
		);
	}

	#[cfg(unix)]
	#[test]
	fn run_write_error() {
		struct TestModule {}

		impl Module for TestModule {
			fn handle_event(&mut self, _: Event, _: &view::State, rebase_todo: &mut TodoFile) -> Results {
				rebase_todo.add_line(0, Line::new("pick 123456789 comment").unwrap());
				Results::from(ExitStatus::Good)
			}
		}

		process_test(
			create_test_module_handler(TestModule {}),
			|ProcessTestContext {
			     process,
			     todo_file_path,
			     ..
			 }| {
				let todo_file = File::open(todo_file_path.as_path()).unwrap();
				let mut permissions = todo_file.metadata().unwrap().permissions();
				permissions.set_readonly(true);
				todo_file.set_permissions(permissions).unwrap();

				let thread = Thread::new(process.clone());
				let tester = ThreadableTester::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_error_status();
				assert_eq!(process.exit_status(), ExitStatus::FileWriteError);
			},
		);
	}

	#[test]
	fn run_kill() {
		struct TestModule {}

		impl Module for TestModule {
			fn handle_event(&mut self, _: Event, _: &view::State, rebase_todo: &mut TodoFile) -> Results {
				rebase_todo.add_line(0, Line::new("pick 123456789 comment").unwrap());
				Results::from(ExitStatus::Kill)
			}
		}

		process_test(
			create_test_module_handler(TestModule {}),
			|ProcessTestContext {
			     process,
			     todo_file_path,
			     ..
			 }| {
				let thread = Thread::new(process);
				let tester = ThreadableTester::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_status(&Status::Ended);
				let mut todo_file = TodoFile::new(todo_file_path.to_str().unwrap(), 0, "#");
				todo_file.load_file().unwrap();
				assert!(todo_file.get_line(0).is_none());
			},
		);
	}
}
