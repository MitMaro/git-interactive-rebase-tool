use std::path::Path;

use anyhow::anyhow;

use super::*;
use crate::{
	assert_results,
	events::KeyBindings,
	input::InputOptions,
	module::{Module, DEFAULT_INPUT_OPTIONS, DEFAULT_VIEW_DATA},
	runtime::{testutils::MockNotifier, Status},
	search::{Interrupter, SearchResult},
	testutil::{
		create_default_test_module_handler,
		create_test_module_handler,
		process_test,
		MockedSearchable,
		ProcessTestContext,
	},
	todo_file::Line,
	view::{ViewData, REFRESH_THREAD_NAME},
};

#[derive(Clone)]
struct TestModule {
	trace: Arc<Mutex<Vec<String>>>,
}

impl TestModule {
	fn new() -> Self {
		Self {
			trace: Arc::new(Mutex::new(vec![])),
		}
	}

	fn assert_trace(&self, expected: &[&str]) {
		let actual = self.trace.lock().join("\n");
		assert_eq!(actual, expected.join("\n"));
	}
}

impl Module for TestModule {
	fn activate(&mut self, previous_state: State) -> Results {
		self.trace.lock().push(format!("activate(state = {previous_state:?})"));
		Results::new()
	}

	fn deactivate(&mut self) -> Results {
		self.trace.lock().push(String::from("deactivate"));
		Results::new()
	}

	fn build_view_data(&mut self, _render_context: &RenderContext) -> &ViewData {
		self.trace.lock().push(String::from("build_view_data"));
		&DEFAULT_VIEW_DATA
	}

	fn input_options(&self) -> &InputOptions {
		self.trace.lock().push(String::from("input_options"));
		&DEFAULT_INPUT_OPTIONS
	}

	fn read_event(&self, event: Event, _key_bindings: &KeyBindings) -> Event {
		self.trace.lock().push(format!("read_event(event = {event:?})"));
		event
	}

	fn handle_event(&mut self, event: Event, _view_state: &crate::view::State) -> Results {
		self.trace.lock().push(format!("handle_event(event = {event:?})"));
		Results::new()
	}

	fn handle_error(&mut self, error: &Error) -> Results {
		self.trace.lock().push(format!("handle_error(error = {error})"));
		Results::new()
	}
}

#[test]
fn ended() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.end();
			assert!(process.is_ended());
		},
	);
}

#[test]
fn state() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.set_state(State::ShowCommit);
			assert_eq!(process.state(), State::ShowCommit);
		},
	);
}

#[test]
fn exit_status() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.set_exit_status(ExitStatus::FileReadError);
			assert_eq!(process.exit_status(), ExitStatus::FileReadError);
		},
	);
}

#[test]
fn should_exit_none() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.set_exit_status(ExitStatus::None);
			assert!(!process.should_exit());
		},
	);
}

#[test]
fn should_exit_not_none() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.set_exit_status(ExitStatus::Good);
			assert!(process.should_exit());
		},
	);
}

#[test]
fn should_exit_ended() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.set_exit_status(ExitStatus::None);
			process.end();
			assert!(process.should_exit());
		},
	);
}

#[test]
fn is_exit_status_kill_without_kill() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.set_exit_status(ExitStatus::None);
			assert!(!process.is_exit_status_kill());
		},
	);
}

#[test]
fn is_exit_status_kill_with_kill() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.set_exit_status(ExitStatus::Kill);
			assert!(process.is_exit_status_kill());
		},
	);
}

#[test]
fn activate() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module.clone()),
		|ProcessTestContext { process, .. }| {
			assert_results!(process.activate(State::ShowCommit), Artifact::EnqueueResize);
			module.assert_trace(&["activate(state = ShowCommit)"]);
		},
	);
}

#[test]
fn render() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext {
		     process, view_context, ..
		 }| {
			process.render();
			view_context.assert_sent_messages(vec!["Render"]);
		},
	);
}

#[test]
fn write_todo_file() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process
				.todo_file
				.lock()
				.set_lines(vec![Line::parse("fixup ddd comment").unwrap()]);
			process.write_todo_file().unwrap();
			process.todo_file.lock().load_file().unwrap();
			assert_eq!(
				process.todo_file.lock().get_line(0).unwrap(),
				&Line::parse("fixup ddd comment").unwrap()
			);
		},
	);
}

#[test]
fn deactivate() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module.clone()),
		|ProcessTestContext { process, .. }| {
			_ = process.deactivate(State::List);
			module.assert_trace(&["deactivate"]);
		},
	);
}

#[test]
fn handle_event() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module.clone()),
		|ProcessTestContext { process, .. }| {
			let event = Event::from('a');
			_ = process.handle_event();
			module.assert_trace(&[
				"input_options",
				format!("read_event(event = {event:?})").as_str(),
				format!("handle_event(event = {event:?})").as_str(),
			]);
		},
	);
}

#[test]
fn handle_event_artifact_exit_event() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			let event = Event::from(StandardEvent::Exit);
			assert_results!(
				process.handle_event_artifact(event),
				Artifact::ExitStatus(ExitStatus::Abort)
			);
		},
	);
}

#[test]
fn handle_event_artifact_kill_event() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			let event = Event::from(StandardEvent::Kill);
			assert_results!(
				process.handle_event_artifact(event),
				Artifact::ExitStatus(ExitStatus::Kill)
			);
		},
	);
}

#[test]
fn handle_event_artifact_resize_event() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext {
		     process, view_context, ..
		 }| {
			let event = Event::Resize(100, 200);
			assert_results!(process.handle_event_artifact(event));
			view_context.assert_sent_messages(vec!["Resize(100, 200)"]);
		},
	);
}

#[test]
fn handle_event_artifact_resize_event_to_small() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext {
		     process, view_context, ..
		 }| {
			process.set_state(State::List);
			let event = Event::Resize(1, 1);
			assert_results!(
				process.handle_event_artifact(event),
				Artifact::ChangeState(State::WindowSizeError)
			);
			view_context.assert_sent_messages(vec!["Resize(1, 1)"]);
		},
	);
}

#[test]
fn handle_event_artifact_resize_event_to_small_already_window_size_error_state() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext {
		     process, view_context, ..
		 }| {
			process.set_state(State::WindowSizeError);
			let event = Event::Resize(1, 1);
			assert_results!(process.handle_event_artifact(event));
			view_context.assert_sent_messages(vec!["Resize(1, 1)"]);
		},
	);
}

#[test]
fn handle_event_artifact_other_event() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			let event = Event::from('a');
			assert_results!(process.handle_event_artifact(event));
		},
	);
}

#[test]
fn handle_state_with_no_change() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module.clone()),
		|ProcessTestContext { process, .. }| {
			process.set_state(State::List);
			_ = process.handle_state(State::List);
			module.assert_trace(&[]);
		},
	);
}

#[test]
fn handle_state_with_change() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module.clone()),
		|ProcessTestContext { process, .. }| {
			process.set_state(State::List);
			_ = process.handle_state(State::ShowCommit);
			assert_eq!(process.state(), State::ShowCommit);
			module.assert_trace(&["deactivate", "activate(state = List)"]);
		},
	);
}

#[test]
fn handle_error_with_previous_state() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module.clone()),
		|ProcessTestContext { process, .. }| {
			process.set_state(State::List);
			_ = process.handle_error(&anyhow!("Error"), Some(State::ShowCommit));
			assert_eq!(process.state(), State::Error);
			module.assert_trace(&["activate(state = ShowCommit)", "handle_error(error = Error)"]);
		},
	);
}

#[test]
fn handle_error_with_no_previous_state() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module.clone()),
		|ProcessTestContext { process, .. }| {
			process.set_state(State::List);
			_ = process.handle_error(&anyhow!("Error"), None);
			assert_eq!(process.state(), State::Error);
			module.assert_trace(&["activate(state = List)", "handle_error(error = Error)"]);
		},
	);
}

#[test]
fn handle_exit_status() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			process.set_exit_status(ExitStatus::Abort);
			assert_eq!(process.exit_status(), ExitStatus::Abort);
		},
	);
}

#[test]
fn handle_enqueue_resize() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			_ = process.input_state.read_event(); // skip existing events
			process.render_context.lock().update(120, 130);
			_ = process.handle_enqueue_resize();
			assert_eq!(process.input_state.read_event(), Event::Resize(120, 130));
		},
	);
}

#[test]
fn handle_external_command_success() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			_ = process.input_state.read_event(); // clear existing event
			let mut notifier = MockNotifier::new(&process.thread_statuses);
			notifier.register_thread(REFRESH_THREAD_NAME, Status::Waiting);
			notifier.register_thread(crate::input::THREAD_NAME, Status::Waiting);
			assert_results!(process.handle_external_command(&(String::from("true"), vec![])));
			assert_eq!(
				process.input_state.read_event(),
				Event::from(MetaEvent::ExternalCommandSuccess)
			);
		},
	);
}

#[test]
fn handle_external_command_failure() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			_ = process.input_state.read_event(); // clear existing event
			let mut notifier = MockNotifier::new(&process.thread_statuses);
			notifier.register_thread(REFRESH_THREAD_NAME, Status::Waiting);
			notifier.register_thread(crate::input::THREAD_NAME, Status::Waiting);
			assert_results!(process.handle_external_command(&(String::from("false"), vec![])));
			assert_eq!(
				process.input_state.read_event(),
				Event::from(MetaEvent::ExternalCommandError)
			);
		},
	);
}

#[cfg(unix)]
#[test]
fn handle_external_command_not_executable() {
	let command = String::from(
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("not-executable.sh")
			.to_str()
			.unwrap(),
	);
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			_ = process.input_state.read_event(); // clear existing event
			let mut notifier = MockNotifier::new(&process.thread_statuses);
			notifier.register_thread(REFRESH_THREAD_NAME, Status::Waiting);
			notifier.register_thread(crate::input::THREAD_NAME, Status::Waiting);
			assert_results!(
				process.handle_external_command(&(command, vec![])),
				Artifact::Error(
					anyhow!("Unable to run {0} : File not executable: {0}", command),
					Some(State::List)
				)
			);
		},
	);
}

#[cfg(unix)]
#[test]
fn handle_external_command_not_found() {
	let command = String::from(
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("not-found.sh")
			.to_str()
			.unwrap(),
	);
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			_ = process.input_state.read_event(); // clear existing event
			let mut notifier = MockNotifier::new(&process.thread_statuses);
			notifier.register_thread(REFRESH_THREAD_NAME, Status::Waiting);
			notifier.register_thread(crate::input::THREAD_NAME, Status::Waiting);
			assert_results!(
				process.handle_external_command(&(command, vec![])),
				Artifact::Error(
					anyhow!("Unable to run {0} : File does not exist: {0}", command),
					Some(State::List)
				)
			);
		},
	);
}

#[test]
fn handle_results_change_state() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			process.set_state(State::List);
			let results = Results::from(State::ShowCommit);
			process.handle_results(results);
			assert_eq!(process.state(), State::ShowCommit);
		},
	);
}

#[test]
fn handle_results_enqueue_resize() {
	process_test(
		create_default_test_module_handler(),
		|ProcessTestContext { process, .. }| {
			_ = process.input_state.read_event(); // skip existing events
			process.render_context.lock().update(120, 130);
			let mut results = Results::new();
			results.enqueue_resize();
			process.handle_results(results);
			assert_eq!(process.input_state.read_event(), Event::Resize(120, 130));
		},
	);
}

#[test]
fn handle_results_error() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			process.set_state(State::List);
			let results = Results::from(anyhow!("Error"));
			process.handle_results(results);
			assert_eq!(process.state(), State::Error);
		},
	);
}

#[test]
fn handle_results_event() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			let results = Results::from(Event::from(StandardEvent::Kill));
			process.handle_results(results);
			assert_eq!(process.exit_status(), ExitStatus::Kill);
		},
	);
}

#[test]
fn handle_results_exit_status() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			let results = Results::from(ExitStatus::Abort);
			process.handle_results(results);
			assert_eq!(process.exit_status(), ExitStatus::Abort);
		},
	);
}

#[test]
fn handle_results_external_command_success() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext { process, .. }| {
			_ = process.input_state.read_event(); // clear existing event
			let mut notifier = MockNotifier::new(&process.thread_statuses);
			notifier.register_thread(REFRESH_THREAD_NAME, Status::Waiting);
			notifier.register_thread(crate::input::THREAD_NAME, Status::Waiting);
			let mut results = Results::new();
			results.external_command(String::from("true"), vec![]);
			process.handle_results(results);
			assert_eq!(
				process.input_state.read_event(),
				Event::from(MetaEvent::ExternalCommandSuccess)
			);
		},
	);
}

#[test]
fn handle_search_cancel() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext {
		     process,
		     search_context,
		     ..
		 }| {
			let mut results = Results::new();
			results.search_cancel();
			process.handle_results(results);
			assert!(matches!(search_context.state.receive_update(), Action::Cancel));
		},
	);
}

#[test]
fn handle_search_term() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext {
		     process,
		     search_context,
		     ..
		 }| {
			let mut results = Results::new();
			let search_term = String::from("foo");
			results.search_term(search_term.as_str());
			process.handle_results(results);
			assert!(matches!(
				search_context.state.receive_update(),
				Action::Start(search_term)
			));
		},
	);
}

#[test]
fn handle_searchable() {
	let module = TestModule::new();
	process_test(
		create_test_module_handler(module),
		|ProcessTestContext {
		     process,
		     search_context,
		     ..
		 }| {
			let searchable: Box<dyn Searchable> = Box::new(MockedSearchable {});
			let results = Results::from(searchable);
			process.handle_results(results);
			assert!(matches!(
				search_context.state.receive_update(),
				Action::SetSearchable(_)
			));
		},
	);
}
