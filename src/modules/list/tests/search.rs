use super::*;
use crate::{
	action_line,
	assert_rendered_output,
	assert_results,
	process::Artifact,
	render_line,
	test_helpers::{assertions::assert_rendered_output::AssertRenderOptions, testers},
};

#[test]
fn start_edit() {
	testers::module(
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
	testers::module(
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
	testers::module(
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
	testers::module(
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
	testers::module(
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
	testers::module(
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
	testers::module(
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
	testers::module(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
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
	testers::module(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
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
	testers::module(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
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
	testers::module(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
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
	testers::module(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ToggleVisualMode),
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
	testers::module(
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
	testers::module(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
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
	testers::module(
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
	testers::module(
		&[
			"pick aaaaaaaa x1",
			"pick aaaaaaaa a",
			"pick bbbbbbbb x2",
			"pick aaaaaaaa b",
			"pick bbbbbbbb x3",
		],
		&[
			Event::from(StandardEvent::MoveCursorDown),
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
	testers::module(
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
	testers::module(
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
	testers::module(
		&["pick aaaaaaaa x1", "pick bbbbbbbb x2", "pick cccccccc x3"],
		&[
			Event::from(StandardEvent::SearchStart),
			Event::from(StandardEvent::SearchFinish),
			Event::from(StandardEvent::Abort),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let mut results: Vec<_> = test_context.handle_all_events(&mut module);
			assert_results!(
				results.remove(results.len() - 1),
				Artifact::Event(Event::from(StandardEvent::Abort)),
				Artifact::ChangeState(State::ConfirmAbort)
			);
		},
	);
}
