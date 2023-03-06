use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn normal_mode_abort() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::Abort)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_event(&mut module);
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
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ForceAbort)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(module.todo_file.lock().is_empty());
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
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ForceAbort)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(module.todo_file.lock().is_empty());
		},
	);
}

#[test]
fn normal_mode_rebase() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::Rebase)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
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
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_event(&mut module);
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
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ForceRebase)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(!module.todo_file.lock().is_noop());
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
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::ForceRebase)),
				Artifact::ExitStatus(ExitStatus::Good)
			);
			assert!(!module.todo_file.lock().is_noop());
		},
	);
}
