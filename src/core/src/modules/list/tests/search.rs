use view::assert_rendered_output;

use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn start_edit() {
	module_test(
		&["pick aaaaaaaa comment"],
		&[Event::from(StandardEvent::SearchStart)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}aaaaaaaa comment{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {IndicatorColor}aaaaaaaa{Normal} comment1{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}aaaaaaaa comment1{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {IndicatorColor,Underline}aaaaaaaa{Normal} comment1{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}aaaaaaaa comment1{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}aaaaaaaa comment1{Pad( )}"
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor}x{Normal}1",
				"{Selected}{Normal} > {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor,Underline}x{Normal}2{Pad( )}",
				"{Normal}   {ActionPick}pick {Normal}cccccccc {IndicatorColor}x{Normal}3",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor}x{Normal}1{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor,Underline}x{Normal}2{Pad( )}",
				"{Normal}   {ActionPick}pick {Normal}cccccccc {IndicatorColor}x{Normal}3",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor,Underline}x{Normal}1{Pad( )}",
				"{Normal}   {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor}x{Normal}2",
				"{Normal}   {ActionPick}pick {Normal}cccccccc {IndicatorColor}x{Normal}3",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor,Underline}x{Normal}1{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor}x{Normal}2{Pad( )}",
				"{Normal}   {ActionPick}pick {Normal}cccccccc {IndicatorColor}x{Normal}3",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor}x{Normal}1",
				"{Selected}{Normal} > {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor,Underline}x{Normal}2{Pad( )}",
				"{Normal}   {ActionPick}pick {Normal}cccccccc {IndicatorColor}x{Normal}3",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor}x{Normal}1",
				"{Selected}{Normal} > {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor,Underline}x{Normal}2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick {Normal}cccccccc {IndicatorColor}x{Normal}3{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor}x{Normal}1",
				"{Normal}   {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor}x{Normal}2",
				"{Selected}{Normal} > {ActionPick}pick {Normal}cccccccc {IndicatorColor,Underline}x{Normal}3{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor}x{Normal}1{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor}x{Normal}2{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}cccccccc {IndicatorColor,Underline}x{Normal}3{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}aaaaaaaa x1{Pad( )}",
				"{Normal}   {ActionPick}pick {Normal}bbbbbbbb x2",
				"{Normal}   {ActionPick}pick {Normal}cccccccc x3"
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick {Normal}aaaaaaaa {IndicatorColor}x{Normal}1",
				"{Normal}   {ActionPick}pick {Normal}aaaaaaaa a",
				"{Selected}{Normal} > {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor,Underline}x{Normal}2{Pad( )}",
				"{Normal}   {ActionPick}pick {Normal}aaaaaaaa b",
				"{Normal}   {ActionPick}pick {Normal}bbbbbbbb {IndicatorColor}x{Normal}3",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick {Normal}12345678 \
				 {IndicatorColor}x{Normal}a{IndicatorColor}xx{Normal}a{IndicatorColor}xxx{Normal}{Pad( )}",
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
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionBreak}break {Normal}{Pad( )}",
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
