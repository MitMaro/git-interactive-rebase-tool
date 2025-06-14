use super::*;
use crate::{assert_results, process::Artifact};

#[test]
fn normal_mode_abort() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Abort)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Abort)),
				Artifact::ChangeState(State::ConfirmAbort)
			);
		},
	);
}

#[test]
fn visual_mode_abort() {
	testers::module(
		&["pick aaa c1"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::Abort),
		],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Abort)),
				Artifact::ChangeState(State::ConfirmAbort)
			);
		},
	);
}

#[test]
fn normal_mode_force_abort() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ForceAbort)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ForceAbort)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(module.todo_file.lock().is_empty());
		},
	);
}

#[test]
fn visual_mode_force_abort() {
	testers::module(
		&["pick aaa c1"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::ForceAbort),
		],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ForceAbort)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(module.todo_file.lock().is_empty());
		},
	);
}

#[test]
fn normal_mode_rebase() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::Rebase)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Rebase)),
				Artifact::ChangeState(State::ConfirmRebase)
			);
		},
	);
}

#[test]
fn visual_mode_rebase() {
	testers::module(
		&["pick aaa c1"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::Rebase),
		],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Rebase)),
				Artifact::ChangeState(State::ConfirmRebase)
			);
		},
	);
}

#[test]
fn normal_mode_force_rebase() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ForceRebase)],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ForceRebase)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(!module.todo_file.lock().is_noop());
		},
	);
}

#[test]
fn visual_mode_force_rebase() {
	testers::module(
		&["pick aaa c1"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::ForceRebase),
		],
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::ForceRebase)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(!module.todo_file.lock().is_noop());
		},
	);
}
