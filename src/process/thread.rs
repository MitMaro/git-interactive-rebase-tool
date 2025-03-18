use std::sync::Arc;

use captur::capture;

use crate::{
	module,
	module::{ExitStatus, State},
	process::{Process, Results},
	runtime::{Installer, RuntimeError, Threadable},
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
	use std::{
		fs::File,
		sync::atomic::{AtomicBool, Ordering},
	};

	use super::*;
	use crate::{
		input::{Event, StandardEvent},
		module::Module,
		runtime::Status,
		test_helpers::{create_default_test_module_handler, create_test_module_handler, testers},
	};

	#[test]
	fn end() {
		testers::process(
			create_default_test_module_handler(),
			|testers::ProcessTestContext { process, .. }| {
				let thread = Thread::new(process);
				thread.end();
				assert!(thread.process.is_ended());
			},
		);
	}

	#[test]
	fn start() {
		testers::process(
			create_default_test_module_handler(),
			|testers::ProcessTestContext { process, .. }| {
				let thread = Thread::new(process);
				thread.end();
				let tester = testers::Threadable::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_status(&Status::Ended);

				assert_eq!(thread.process.state(), State::List);
			},
		);
	}

	#[test]
	fn render() {
		testers::process(
			create_default_test_module_handler(),
			|testers::ProcessTestContext { process, app_data, .. }| {
				app_data.input_state().enqueue_event(Event::from(StandardEvent::Exit));
				let thread = Thread::new(process);
				let tester = testers::Threadable::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_status(&Status::Ended);
			},
		);
	}

	#[test]
	fn run_success() {
		struct TestModule(Arc<AtomicBool>);

		impl Module for TestModule {
			fn handle_event(&mut self, _: Event) -> Results {
				self.0.store(true, Ordering::Release);
				Results::from(ExitStatus::Good)
			}
		}

		let handle_called = Arc::new(AtomicBool::new(false));

		testers::process(
			create_test_module_handler(TestModule(Arc::clone(&handle_called))),
			|testers::ProcessTestContext { process, .. }| {
				let thread = Thread::new(process.clone());
				let tester = testers::Threadable::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_status(&Status::Ended);
				assert!(handle_called.load(Ordering::Acquire));
				assert_eq!(process.exit_status(), ExitStatus::Good);
			},
		);
	}

	#[cfg(unix)]
	#[test]
	fn run_write_error() {
		struct TestModule;

		impl Module for TestModule {
			fn handle_event(&mut self, _: Event) -> Results {
				Results::from(ExitStatus::Good)
			}
		}

		testers::process(
			create_test_module_handler(TestModule {}),
			|testers::ProcessTestContext { process, app_data, .. }| {
				let todo_file = app_data.todo_file();
				let todo_file = File::open(todo_file.lock().get_filepath()).unwrap();
				let mut permissions = todo_file.metadata().unwrap().permissions();
				permissions.set_readonly(true);
				todo_file.set_permissions(permissions).unwrap();

				let thread = Thread::new(process.clone());
				let tester = testers::Threadable::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_error_status();
				assert_eq!(process.exit_status(), ExitStatus::FileWriteError);
			},
		);
	}

	#[test]
	fn run_kill() {
		struct TestModule;

		impl Module for TestModule {
			fn handle_event(&mut self, _: Event) -> Results {
				Results::from(ExitStatus::Kill)
			}
		}

		testers::process(
			create_test_module_handler(TestModule {}),
			|testers::ProcessTestContext { process, .. }| {
				let thread = Thread::new(process.clone());
				let tester = testers::Threadable::new();
				tester.start_threadable(&thread, THEAD_NAME);
				tester.wait_for_status(&Status::Ended);
				assert_eq!(process.exit_status(), ExitStatus::Kill);
			},
		);
	}
}
