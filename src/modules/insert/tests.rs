use super::*;
use crate::{assert_rendered_output, assert_results, input::KeyCode, process::Artifact, test_helpers::testers};

fn create_insert(todo_file: TodoFile) -> Insert {
	Insert::new(Arc::new(Mutex::new(todo_file)))
}

#[test]
fn activate() {
	testers::module(&[], &[], |mut test_context| {
		let mut module = create_insert(test_context.take_todo_file());
		assert_results!(test_context.activate(&mut module, State::List));
	});
}

#[test]
fn render_prompt() {
	testers::module(&[], &[], |mut test_context| {
		let mut module = create_insert(test_context.take_todo_file());
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"Select the type of line to insert:",
			"",
			"{BODY}",
			"e) exec <command>",
			"p) pick <hash>",
			"l) label <label>",
			"r) reset <label>",
			"m) merge [-C <commit> | -c <commit>] <label> [# <oneline>]",
			"u) update-ref <reference>",
			"q) Cancel add line",
			"",
			"Please choose an option."
		);
	});
}

#[test]
fn prompt_cancel() {
	testers::module(&[], &[Event::from('q')], |mut test_context| {
		let mut module = create_insert(test_context.take_todo_file());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from('q')),
			Artifact::ChangeState(State::List)
		);
	});
}

#[test]
fn edit_render_exec() {
	testers::module(
		&[],
		&[
			Event::from('e'),
			Event::from('f'),
			Event::from('o'),
			Event::from('o'),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = create_insert(test_context.take_todo_file());
			_ = test_context.handle_n_events(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"exec foo",
				"{TRAILING}",
				"Enter to finish"
			);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(KeyCode::Enter)),
				Artifact::ChangeState(State::List)
			);
			assert_eq!(module.todo_file.lock().get_line(0).unwrap().to_text(), "exec foo");
		},
	);
}

#[test]
fn edit_render_pick() {
	testers::module(
		&[],
		&[
			Event::from('p'),
			Event::from('a'),
			Event::from('b'),
			Event::from('c'),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = create_insert(test_context.take_todo_file());
			_ = test_context.handle_n_events(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"pick abc",
				"{TRAILING}",
				"Enter to finish"
			);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(KeyCode::Enter)),
				Artifact::ChangeState(State::List)
			);
			assert_eq!(module.todo_file.lock().get_line(0).unwrap().to_text(), "pick abc ");
		},
	);
}

#[test]
fn edit_render_label() {
	testers::module(
		&[],
		&[
			Event::from('l'),
			Event::from('f'),
			Event::from('o'),
			Event::from('o'),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = create_insert(test_context.take_todo_file());
			_ = test_context.handle_n_events(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"label foo",
				"{TRAILING}",
				"Enter to finish"
			);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(KeyCode::Enter)),
				Artifact::ChangeState(State::List)
			);
			assert_eq!(module.todo_file.lock().get_line(0).unwrap().to_text(), "label foo");
		},
	);
}

#[test]
fn edit_render_reset() {
	testers::module(
		&[],
		&[
			Event::from('r'),
			Event::from('f'),
			Event::from('o'),
			Event::from('o'),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = create_insert(test_context.take_todo_file());
			_ = test_context.handle_n_events(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"reset foo",
				"{TRAILING}",
				"Enter to finish"
			);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(KeyCode::Enter)),
				Artifact::ChangeState(State::List)
			);
			assert_eq!(module.todo_file.lock().get_line(0).unwrap().to_text(), "reset foo");
		},
	);
}

#[test]
fn edit_render_merge() {
	testers::module(
		&[],
		&[
			Event::from('m'),
			Event::from('f'),
			Event::from('o'),
			Event::from('o'),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = create_insert(test_context.take_todo_file());
			_ = test_context.handle_n_events(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"merge foo",
				"{TRAILING}",
				"Enter to finish"
			);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(KeyCode::Enter)),
				Artifact::ChangeState(State::List)
			);
			assert_eq!(module.todo_file.lock().get_line(0).unwrap().to_text(), "merge foo");
		},
	);
}

#[test]
fn update_ref_render_merge() {
	testers::module(
		&[],
		&[
			Event::from('u'),
			Event::from('f'),
			Event::from('o'),
			Event::from('o'),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = create_insert(test_context.take_todo_file());
			_ = test_context.handle_n_events(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"update-ref foo",
				"{TRAILING}",
				"Enter to finish"
			);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(KeyCode::Enter)),
				Artifact::ChangeState(State::List)
			);
			assert_eq!(module.todo_file.lock().get_line(0).unwrap().to_text(), "update-ref foo");
		},
	);
}

#[test]
fn edit_select_next_index() {
	testers::module(
		&["pick aaa c1"],
		&[
			Event::from('e'),
			Event::from('f'),
			Event::from('o'),
			Event::from('o'),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = create_insert(test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_eq!(module.todo_file.lock().get_selected_line_index(), 1);
		},
	);
}

#[test]
fn cancel_edit() {
	testers::module(
		&[],
		&[Event::from('e'), Event::from(KeyCode::Enter)],
		|mut test_context| {
			let mut module = create_insert(test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert!(module.todo_file.lock().is_empty());
		},
	);
}
