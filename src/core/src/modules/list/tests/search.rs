use super::*;
use crate::{
	action_line,
	assert_rendered_output,
	assert_results,
	process::Artifact,
	render_line,
	testutil::module_test,
	view::testutil::AssertRenderOptions,
};

#[test]
fn start_edit() {
	module_test(
		&["pick aaaaaaaa comment"],
		&[Event::from(StandardEvent::SearchStart)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Not Contains "IndicatorColor"),
				"{TRAILING}",
				"{Normal}/{Normal,Underline}"
			);
		},
	);
}

#[test]
fn with_match_on_hash() {
	module_test(
		&["pick aaaaaaaa comment1"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from('a'),
			Event::from('a'),
			Event::from('a'),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "IndicatorColor"),
				"{TRAILING}",
				"{Normal}/aaa{Normal,Underline}"
			);
		},
	);
}

#[test]
fn with_no_match() {
	module_test(
		&["pick aaaaaaaa comment1"],
		&[Event::from(StandardEvent::SearchStart), Event::from('x')],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Not Contains "IndicatorColor"),
				"{TRAILING}",
				"{Normal}/x{Normal,Underline}"
			);
		},
	);
}

#[test]
fn start_with_matches_and_with_term() {
	module_test(
		&["pick aaaaaaaa comment1"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from('a'),
			Event::from(StandardEvent::SearchFinish),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor,Underline}"),
				"{TRAILING}",
				"{Normal}[a]: 1/1"
			);
		},
	);
}

#[test]
fn start_with_no_matches_and_with_term() {
	module_test(
		&["pick aaaaaaaa comment1"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Not Contains "IndicatorColor"),
				"{TRAILING}",
				"{Normal}[x]: No Results"
			);
		},
	);
}

#[test]
fn start_with_no_term() {
	module_test(
		&["pick aaaaaaaa comment1"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from(StandardEvent::SearchFinish),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				action_line!(Selected Pick "aaaaaaaa", "comment1")
			);
		},
	);
}

#[test]
fn normal_mode_next() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchNext),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor}"),
				render_line!(Contains "{IndicatorColor,Underline}"),
				render_line!(Contains "{IndicatorColor}"),
				"{TRAILING}",
				"{Normal}[x]: 2/3"
			);
		},
	);
}

#[test]
fn visual_mode_next() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchNext),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(
					All render_line!(Contains "Dimmed"),
					render_line!(Contains "{IndicatorColor}")
				),
				render_line!(Contains "{IndicatorColor,Underline}"),
				render_line!(
					All render_line!(Not Contains "Dimmed"),
					render_line!(Contains "{IndicatorColor}")
				),
				"{TRAILING}",
				"{Normal}[x]: 2/3"
			);
		},
	);
}

#[test]
fn normal_mode_next_with_wrap() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchNext),
			Event::from(StandardEvent::SearchNext),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor,Underline}"),
				render_line!(Contains "{IndicatorColor}"),
				render_line!(Contains "{IndicatorColor}"),
				"{TRAILING}",
				"{Normal}[x]: 1/3"
			);
		},
	);
}

#[test]
fn visual_mode_next_with_wrap() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchNext),
			Event::from(StandardEvent::SearchNext),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor,Underline}"),
				render_line!(
					All render_line!(Contains "Dimmed"),
					render_line!(Contains "{IndicatorColor}")
				),
				render_line!(Contains "{IndicatorColor}"),
				"{TRAILING}",
				"{Normal}[x]: 1/3"
			);
		},
	);
}

#[test]
fn normal_mode_previous() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchPrevious),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor}"),
				render_line!(Contains "{IndicatorColor,Underline}"),
				render_line!(Contains "{IndicatorColor}"),
				"{TRAILING}",
				"{Normal}[x]: 2/3"
			);
		},
	);
}

#[test]
fn visual_mode_previous() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchPrevious),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor}"),
				render_line!(
					All render_line!(Not Contains "Dimmed"),
					render_line!(Contains "{IndicatorColor,Underline}")
				),
				render_line!(
					All render_line!(Contains "Dimmed"),
					render_line!(Contains "{IndicatorColor}")
				),
				"{TRAILING}",
				"{Normal}[x]: 2/3"
			);
		},
	);
}

#[test]
fn normal_mode_previous_with_wrap() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchPrevious),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor}"),
				render_line!(Contains "{IndicatorColor}"),
				render_line!(Contains "{IndicatorColor,Underline}"),
				"{TRAILING}",
				"{Normal}[x]: 3/3"
			);
		},
	);
}

#[test]
fn visual_mode_previous_with_wrap() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchPrevious),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(All render_line!(Contains "Dimmed"), render_line!(Contains "{IndicatorColor}")),
				render_line!(All render_line!(Contains "Dimmed"), render_line!(Contains "{IndicatorColor}")),
				render_line!(
					All render_line!(Not Contains "Dimmed"),
					render_line!(Contains "{IndicatorColor,Underline}")
				),
				"{TRAILING}",
				"{Normal}[x]: 3/3"
			);
		},
	);
}

#[test]
fn cancel() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::SearchStart),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				action_line!(Selected Pick "aaaaaaaa", "x1"),
				action_line!(Pick "bbbbbbbb", "x2"),
				action_line!(Pick "cccccccc", "x3")
			);
		},
	);
}

#[test]
fn set_search_start_hint() {
	module_test(
		&[
			"pick aaaaaaaa x1",
			"pick aaaaaaaa a",
			"pick bbbbbbbb x2",
			"pick aaaaaaaa b",
			"pick bbbbbbbb x3",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(StandardEvent::SearchStart),
			Event::from('x'),
			Event::from(StandardEvent::SearchFinish),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor}"),
				render_line!(Not Contains "{IndicatorColor}"),
				render_line!(Contains "{IndicatorColor,Underline}"),
				render_line!(Not Contains "{IndicatorColor}"),
				render_line!(Contains "{IndicatorColor}"),
				"{TRAILING}",
				"{Normal}[x]: 2/3"
			);
		},
	);
}

#[test]
fn highlight_multiple() {
	module_test(
		&["pick 12345678 xaxxaxxx"],
		&[Event::from(StandardEvent::SearchStart), Event::from('x')],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Contains "{IndicatorColor}x{Normal}a{IndicatorColor}xx{Normal}a{IndicatorColor}xxx"),
				"{TRAILING}",
				"{Normal}/x{Normal,Underline}"
			);
		},
	);
}

#[test]
fn skip_no_content() {
	module_test(
		&["break"],
		&[Event::from(StandardEvent::SearchStart), Event::from('x')],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				render_line!(Not Contains "{IndicatorColor}"),
				"{TRAILING}",
				"{Normal}/x{Normal,Underline}"
			);
		},
	);
}

#[test]
fn handle_other_event() {
	module_test(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from(StandardEvent::SearchFinish),
			Event::from(MetaEvent::Abort),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let mut results: Vec<_> = test_context.handle_all_events(&mut module);
			assert_results!(
				results.remove(results.len() - 1),
				Artifact::Event(Event::from(MetaEvent::Abort)),
				Artifact::ChangeState(State::ConfirmAbort)
			);
		},
	);
}
