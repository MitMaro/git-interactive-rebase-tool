use std::{
	io::Write,
	path::Path,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};

use anyhow::anyhow;
use config::Theme;
use display::{testutil::CrossTerm, Display, Size};
use tempfile::Builder;
use view::ViewData;

use super::*;
use crate::{
	module::Module,
	testutil::{create_default_test_module_handler, create_test_module_handler},
};

fn create_shadow_todo_file(todo_file: &TodoFile) -> TodoFile {
	TodoFile::new(todo_file.get_filepath(), 1, "#")
}

fn process_test<C>(events: &[Event], callback: C)
where C: FnOnce(Process) {
	let todo_file_path = Builder::new()
		.prefix("git-rebase-todo-scratch")
		.suffix("")
		.tempfile()
		.unwrap();
	write!(todo_file_path.as_file(), "pick aaa comment").unwrap();
	let mut todo_file = TodoFile::new(todo_file_path.path().to_str().unwrap(), 1, "#");
	todo_file.load_file().unwrap();

	let mut crossterm = CrossTerm::new();
	crossterm.set_size(Size::new(100, 300));
	let display = Display::new(crossterm, &Theme::new());
	let view = View::new(display, "~", "?");
	let process = Process::new(todo_file, view);
	for event in events {
		process.event_sender.enqueue_event(*event).unwrap();
	}
	process
		.event_sender
		.enqueue_event(Event::from(StandardEvent::Kill))
		.unwrap();

	callback(process);
}

#[test]
fn view_start_error() {
	process_test(&[], |mut process| {
		while process.view_sender.end().is_ok() {}

		assert_eq!(
			process.run(create_default_test_module_handler()).unwrap(),
			ExitStatus::StateError
		);
	});
}

#[test]
fn window_too_small_on_start() {
	process_test(&[Event::Resize(1, 1)], |mut process| {
		let _ = process.run(create_default_test_module_handler()).unwrap();
		assert_eq!(process.state, State::WindowSizeError);
	});
}

#[test]
fn render_error() {
	struct TestModule {
		view_data: ViewData,
		view_sender: ViewSender,
	}

	impl TestModule {
		fn new(view_sender: ViewSender) -> Self {
			Self {
				view_data: ViewData::new(|_| {}),
				view_sender,
			}
		}
	}

	impl Module for TestModule {
		fn build_view_data(&mut self, _: &RenderContext, _: &TodoFile) -> &ViewData {
			while self.view_sender.end().is_ok() {}
			&self.view_data
		}
	}

	process_test(&[], |mut process| {
		let handler = create_test_module_handler(TestModule::new(process.view_sender.clone()));
		assert_eq!(process.run(handler).unwrap(), ExitStatus::StateError);
	});
}

#[test]
fn view_sender_is_poisoned() {
	process_test(&[], |mut process| {
		let handler = create_default_test_module_handler();
		process.view_sender.clone_poisoned().store(true, Ordering::Release);
		assert_eq!(process.run(handler).unwrap(), ExitStatus::StateError);
	});
}

#[test]
fn stop_error() {
	struct TestModule {}

	impl Module for TestModule {
		fn handle_event(&mut self, _: Event, view_sender: &ViewSender, _: &mut TodoFile) -> Results {
			while view_sender.end().is_ok() {}
			Results::new()
		}
	}

	process_test(&[], |mut process| {
		let handler = create_test_module_handler(TestModule {});
		assert_eq!(process.run(handler).unwrap(), ExitStatus::StateError);
	});
}

#[test]
fn handle_exit_event_that_is_not_kill() {
	struct TestModule {}
	impl Module for TestModule {
		fn handle_event(&mut self, _: Event, _: &ViewSender, _: &mut TodoFile) -> Results {
			let mut results = Results::new();
			results.exit_status(ExitStatus::Good);
			results
		}
	}

	process_test(&[Event::from('q')], |mut process| {
		let handler = create_test_module_handler(TestModule {});
		process.rebase_todo.set_lines(vec![]);
		assert_eq!(process.run(handler).unwrap(), ExitStatus::Good);
		let mut shadow_rebase_file = create_shadow_todo_file(&process.rebase_todo);
		shadow_rebase_file.load_file().unwrap();
		assert!(shadow_rebase_file.is_empty());
	});
}

#[test]
fn handle_exit_event_that_is_kill() {
	struct TestModule {}
	impl Module for TestModule {
		fn handle_event(&mut self, _: Event, _: &ViewSender, _: &mut TodoFile) -> Results {
			let mut results = Results::new();
			results.exit_status(ExitStatus::Kill);
			results
		}
	}

	process_test(&[Event::from('q')], |mut process| {
		let handler = create_test_module_handler(TestModule {});
		process.rebase_todo.set_lines(vec![]);
		assert_eq!(process.run(handler).unwrap(), ExitStatus::Kill);
		let mut shadow_rebase_file = create_shadow_todo_file(&process.rebase_todo);
		shadow_rebase_file.load_file().unwrap();
		assert!(!shadow_rebase_file.is_empty());
	});
}

#[test]
fn handle_none_event() {
	struct TestModule {}
	impl Module for TestModule {
		fn handle_event(&mut self, event: Event, _: &ViewSender, _: &mut TodoFile) -> Results {
			let mut results = Results::new();
			// None events should be ignored, and never reach here, if it does send a kill
			if event != Event::None {
				results.exit_status(ExitStatus::Kill);
			}
			results.exit_status(ExitStatus::Good);
			results
		}
	}

	process_test(&[Event::None, Event::from('q')], |mut process| {
		let handler = create_test_module_handler(TestModule {});
		assert_eq!(process.run(handler).unwrap(), ExitStatus::Good);
	});
}

#[test]
fn handle_results_error() {
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.error(anyhow!("Test error"));
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.state, State::Error);
	});
}

#[test]
fn handle_results_with_return_error() {
	struct TestModule {}
	impl Module for TestModule {
		fn activate(&mut self, _rebase_todo: &TodoFile, previous_state: State) -> Results {
			if previous_state != State::ExternalEditor {
				Results::from(ExitStatus::Kill)
			}
			else {
				Results::new()
			}
		}
	}
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.error_with_return(anyhow!("Test error"), State::ExternalEditor);
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.state, State::Error);
		assert!(process.exit_status.is_none());
	});
}

#[test]
fn handle_results_change_state() {
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.state(State::Error);
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.state, State::Error);
	});
}

#[test]
fn handle_results_state_same() {
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.state(State::List);
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.state, State::List);
	});
}

#[test]
fn handle_results_exit_event() {
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.event(Event::from(StandardEvent::Exit));
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.exit_status, Some(ExitStatus::Abort));
	});
}

#[test]
fn handle_results_kill_event() {
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.event(Event::from(StandardEvent::Kill));
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.exit_status, Some(ExitStatus::Kill));
	});
}

#[test]
fn handle_results_resize_event_not_too_small() {
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.event(Event::Resize(100, 200));
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.render_context.width(), 100);
		assert_eq!(process.render_context.height(), 200);
		assert_eq!(process.state, State::List);
	});
}

#[test]
fn handle_results_resize_event_too_small() {
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.event(Event::Resize(10, 20));
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.render_context.width(), 10);
		assert_eq!(process.render_context.height(), 20);
		assert_eq!(process.state, State::WindowSizeError);
	});
}

#[test]
fn handle_results_other_event() {
	process_test(&[], |mut process| {
		let mut results = Results::new();
		results.event(Event::from('a'));
		process.handle_results(&mut create_default_test_module_handler(), results);
		assert_eq!(process.exit_status, None);
		assert_eq!(process.state, State::List);
	});
}

#[test]
fn handle_results_external_command_not_executable() {
	#[derive(Clone)]
	struct TestModule {
		error_called: Arc<AtomicBool>,
	}

	impl TestModule {
		fn new() -> Self {
			Self {
				error_called: Arc::new(AtomicBool::from(false)),
			}
		}
	}

	impl Module for TestModule {
		fn handle_error(&mut self, _: &Error) {
			self.error_called.store(true, Ordering::Relaxed);
		}
	}

	process_test(&[], |mut process| {
		let module = TestModule::new();
		let command = String::from(
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("..")
				.join("..")
				.join("test")
				.join("not-executable.sh")
				.to_str()
				.unwrap(),
		);
		let mut results = Results::new();
		results.external_command(command.clone(), vec![]);
		process.handle_results(&mut create_test_module_handler(module.clone()), results);
		assert_eq!(process.state, State::Error);
		assert!(module.error_called.load(Ordering::Relaxed));
	});
}

#[test]
fn handle_results_external_command_executable_not_found() {
	#[derive(Clone)]
	struct TestModule {
		error_called: Arc<AtomicBool>,
	}

	impl TestModule {
		fn new() -> Self {
			Self {
				error_called: Arc::new(AtomicBool::from(false)),
			}
		}
	}

	impl Module for TestModule {
		fn handle_error(&mut self, _: &Error) {
			self.error_called.store(true, Ordering::Relaxed);
		}
	}

	process_test(&[], |mut process| {
		let module = TestModule::new();
		let command = String::from(
			Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("test")
				.join("not-found.sh")
				.to_str()
				.unwrap(),
		);

		let mut results = Results::new();
		results.external_command(command.clone(), vec![]);
		process.handle_results(&mut create_test_module_handler(module.clone()), results);
		assert_eq!(process.state, State::Error);
		assert!(module.error_called.load(Ordering::Relaxed));
	});
}

#[test]
fn handle_results_external_command_status_success() {
	process_test(&[], |mut process| {
		let command = String::from("true");
		let mut results = Results::new();
		results.external_command(command.clone(), vec![]);
		process.handle_results(&mut create_default_test_module_handler(), results);
		process.event_sender.end().unwrap();
		let mut last_event = Event::None;
		while !process.event_sender.is_poisoned() {}
		loop {
			let event = process.event_sender.read_event();
			if event == Event::None {
				break;
			}
			last_event = event
		}
		assert_eq!(last_event, Event::from(MetaEvent::ExternalCommandSuccess));
	});
}

#[test]
fn handle_results_external_command_status_error() {
	process_test(&[], |mut process| {
		let command = String::from("false");
		let mut results = Results::new();
		results.external_command(command.clone(), vec![]);
		process.handle_results(&mut create_default_test_module_handler(), results);
		process.event_sender.end().unwrap();
		let mut last_event = Event::None;
		while !process.event_sender.is_poisoned() {}
		loop {
			let event = process.event_sender.read_event();
			if event == Event::None {
				break;
			}
			last_event = event
		}
		assert_eq!(last_event, Event::from(MetaEvent::ExternalCommandError));
	});
}
