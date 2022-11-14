use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn normal_mode_abort() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::Abort)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::Abort)),
				Artifact::ChangeState(State::ConfirmAbort)
			);
		},
	);
}

#[test]
fn visual_mode_abort() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ToggleVisualMode), Event::from(MetaEvent::Abort)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::Abort)),
				Artifact::ChangeState(State::ConfirmAbort)
			);
		},
	);
}

#[test]
fn normal_mode_force_abort() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ForceAbort)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ForceAbort)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(test_context.todo_file_context.todo_file().is_empty());
		},
	);
}

#[test]
fn visual_mode_force_abort() {
	module_test(
		&["pick aaa c1"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::ForceAbort),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ForceAbort)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(test_context.todo_file_context.todo_file().is_empty());
		},
	);
}

#[test]
fn normal_mode_rebase() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::Rebase)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::Rebase)),
				Artifact::ChangeState(State::ConfirmRebase)
			);
		},
	);
}

#[test]
fn visual_mode_rebase() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ToggleVisualMode), Event::from(MetaEvent::Rebase)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::Rebase)),
				Artifact::ChangeState(State::ConfirmRebase)
			);
		},
	);
}

#[test]
fn normal_mode_force_rebase() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ForceRebase)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ForceRebase)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(!test_context.todo_file_context.todo_file().is_noop());
		},
	);
}

#[test]
fn visual_mode_force_rebase() {
	module_test(
		&["pick aaa c1"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::ForceRebase),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ForceRebase)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(!test_context.todo_file_context.todo_file().is_noop());
		},
	);
}
