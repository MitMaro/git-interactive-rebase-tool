use super::*;
use crate::{
	assert_process_result,
	assert_rendered_output,
	process::testutil::{process_module_test, TestContext, ViewState},
};

#[test]
#[serial_test::serial]
fn activate() {
	process_module_test(&[], ViewState::default(), &[], |test_context: TestContext<'_>| {
		let mut module = Insert::new();
		assert_process_result!(test_context.activate(&mut module, State::List));
	});
}

#[test]
#[serial_test::serial]
fn render_prompt() {
	process_module_test(&[], ViewState::default(), &[], |test_context: TestContext<'_>| {
		let mut module = Insert::new();
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"{Normal}Select the type of line to insert:",
			"",
			"{BODY}",
			"{Normal}e) exec <command>",
			"{Normal}p) pick <hash>",
			"{Normal}l) label <label>",
			"{Normal}r) reset <label>",
			"{Normal}m) merge [-C <commit> | -c <commit>] <label> [# <oneline>]",
			"{Normal}q) Cancel add line",
			"",
			"{IndicatorColor}Please choose an option."
		);
	});
}

#[test]
#[serial_test::serial]
fn prompt_cancel() {
	process_module_test(
		&[],
		ViewState::default(),
		&[Input::Character('q')],
		|mut test_context: TestContext<'_>| {
			let mut module = Insert::new();
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Character('q'),
				state = State::List
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn edit_render_exec() {
	process_module_test(
		&[],
		ViewState::default(),
		&[
			Input::Character('e'),
			Input::Character('f'),
			Input::Character('o'),
			Input::Character('o'),
			Input::Enter,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = Insert::new();
			test_context.handle_n_inputs(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{IndicatorColor}Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"{Normal,Dimmed}exec {Normal}foo{Normal,Underline} ",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Enter,
				state = State::List
			);
			assert_eq!(test_context.rebase_todo_file.get_line(0).unwrap().to_text(), "exec foo");
		},
	);
}

#[test]
#[serial_test::serial]
fn edit_render_pick() {
	process_module_test(
		&[],
		ViewState::default(),
		&[
			Input::Character('p'),
			Input::Character('a'),
			Input::Character('b'),
			Input::Character('c'),
			Input::Enter,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = Insert::new();
			test_context.handle_n_inputs(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{IndicatorColor}Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"{Normal,Dimmed}pick {Normal}abc{Normal,Underline} ",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Enter,
				state = State::List
			);
			assert_eq!(
				test_context.rebase_todo_file.get_line(0).unwrap().to_text(),
				"pick abc "
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn edit_render_label() {
	process_module_test(
		&[],
		ViewState::default(),
		&[
			Input::Character('l'),
			Input::Character('f'),
			Input::Character('o'),
			Input::Character('o'),
			Input::Enter,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = Insert::new();
			test_context.handle_n_inputs(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{IndicatorColor}Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"{Normal,Dimmed}label {Normal}foo{Normal,Underline} ",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Enter,
				state = State::List
			);
			assert_eq!(
				test_context.rebase_todo_file.get_line(0).unwrap().to_text(),
				"label foo"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn edit_render_reset() {
	process_module_test(
		&[],
		ViewState::default(),
		&[
			Input::Character('r'),
			Input::Character('f'),
			Input::Character('o'),
			Input::Character('o'),
			Input::Enter,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = Insert::new();
			test_context.handle_n_inputs(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{IndicatorColor}Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"{Normal,Dimmed}reset {Normal}foo{Normal,Underline} ",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Enter,
				state = State::List
			);
			assert_eq!(
				test_context.rebase_todo_file.get_line(0).unwrap().to_text(),
				"reset foo"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn edit_render_merge() {
	process_module_test(
		&[],
		ViewState::default(),
		&[
			Input::Character('m'),
			Input::Character('f'),
			Input::Character('o'),
			Input::Character('o'),
			Input::Enter,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = Insert::new();
			test_context.handle_n_inputs(&mut module, 4);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}",
				"{LEADING}",
				"{IndicatorColor}Enter contents of the new line. Empty content cancels creation of a new line.",
				"",
				"{BODY}",
				"{Normal,Dimmed}merge {Normal}foo{Normal,Underline} ",
				"{TRAILING}",
				"{IndicatorColor}Enter to finish"
			);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Enter,
				state = State::List
			);
			assert_eq!(
				test_context.rebase_todo_file.get_line(0).unwrap().to_text(),
				"merge foo"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn edit_select_next_index() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[
			Input::Character('e'),
			Input::Character('f'),
			Input::Character('o'),
			Input::Character('o'),
			Input::Enter,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = Insert::new();
			test_context.handle_all_inputs(&mut module);
			assert_eq!(test_context.rebase_todo_file.get_selected_line_index(), 1);
		},
	);
}

#[test]
#[serial_test::serial]
fn cancel_edit() {
	process_module_test(
		&[],
		ViewState::default(),
		&[Input::Character('e'), Input::Enter],
		|mut test_context: TestContext<'_>| {
			let mut module = Insert::new();
			test_context.handle_all_inputs(&mut module);
			assert!(test_context.rebase_todo_file.is_empty());
		},
	);
}
