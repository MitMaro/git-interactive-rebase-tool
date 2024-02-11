use super::*;
use crate::{assert_rendered_output, assert_results, input::KeyCode, process::Artifact, testutil::module_test};

#[test]
fn edit_with_edit_content() {
	module_test(
		&["exec echo foo"],
		&[Event::from(MetaEvent::Edit)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(MetaEvent::Edit))
			);
			assert_eq!(module.state, ListState::Edit);
		},
	);
}

#[test]
fn edit_without_edit_content() {
	module_test(&["pick aaa c1"], &[Event::from(MetaEvent::Edit)], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from(MetaEvent::Edit))
		);
		assert_eq!(module.state, ListState::Normal);
	});
}

#[test]
fn edit_without_selected_line() {
	module_test(&[], &[Event::from(MetaEvent::Edit)], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		assert_results!(
			test_context.handle_event(&mut module),
			Artifact::Event(Event::from(MetaEvent::Edit))
		);
		assert_eq!(module.state, ListState::Normal);
	});
}

#[test]
fn handle_event() {
	module_test(
		&["exec foo"],
		&[
			Event::from(MetaEvent::Edit),
			Event::from(KeyCode::Backspace),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.build_view_data(&mut module);
			_ = test_context.handle_all_events(&mut module);
			assert_eq!(module.todo_file.lock().get_line(0).unwrap().get_content(), "fo");
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn render() {
	module_test(&["exec foo"], &[Event::from(MetaEvent::Edit)], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		_ = test_context.handle_all_events(&mut module);
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			Style view_data,
			"{TITLE}",
			"{LEADING}",
			"{IndicatorColor}Modifying line: exec foo",
			"",
			"{BODY}",
			"{Normal,Dimmed}exec {Normal}foo{Normal,Underline}",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}
