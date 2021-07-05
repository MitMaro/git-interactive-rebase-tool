use ::input::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind};
use config::testutil::create_config;
use view::assert_rendered_output;

use super::*;
use crate::{assert_process_result, testutil::module_test};

#[test]
fn render_empty_list() {
	module_test(&[], &[], |test_context| {
		let mut module = List::new(&create_config());
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}{HELP}",
			"{LEADING}",
			"{IndicatorColor}Rebase todo file is empty"
		);
	});
}

#[test]
fn render_full() {
	module_test(
		&[
			"pick aaaaaaaa comment 1",
			"drop bbbbbbbb comment 2",
			"fixup cccccccc comment 3",
			"exec echo 'foo'",
			"pick dddddddd comment 4",
			"reword eeeeeeee comment 5",
			"break",
			"squash ffffffff comment 6",
			"edit 11111111 comment 7",
			"label ref",
			"reset ref",
			"merge command",
		],
		&[],
		|test_context| {
			let mut module = List::new(&create_config());
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaaaaaaa {Normal}comment 1{Normal}{Pad( )}",
				"{Normal}   {ActionDrop}drop   {Normal}bbbbbbbb {Normal}comment 2",
				"{Normal}   {ActionFixup}fixup  {Normal}cccccccc {Normal}comment 3",
				"{Normal}   {ActionExec}exec   {Normal}echo 'foo'",
				"{Normal}   {ActionPick}pick   {Normal}dddddddd {Normal}comment 4",
				"{Normal}   {ActionReword}reword {Normal}eeeeeeee {Normal}comment 5",
				"{Normal}   {ActionBreak}break  ",
				"{Normal}   {ActionSquash}squash {Normal}ffffffff {Normal}comment 6",
				"{Normal}   {ActionEdit}edit   {Normal}11111111 {Normal}comment 7",
				"{Normal}   {ActionLabel}label  {Normal}ref",
				"{Normal}   {ActionReset}reset  {Normal}ref",
				"{Normal}   {ActionMerge}merge  {Normal}command"
			);
		},
	);
}

#[test]
fn render_compact() {
	module_test(
		&[
			"pick aaaaaaaa comment 1",
			"drop bbbbbbbb comment 2",
			"fixup cccccccc comment 3",
			"exec echo 'foo'",
			"pick dddddddd comment 4",
			"reword eeeeeeee comment 5",
			"break",
			"squash ffffffff comment 6",
			"edit 11111111 comment 7",
			"label ref",
			"reset ref",
			"merge command",
		],
		&[],
		|mut test_context| {
			test_context.render_context.update(30, 300);
			let mut module = List::new(&create_config());
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal}>{ActionPick}p {Normal}aaa {Normal}comment 1{Normal}{Pad( )}",
				"{Normal} {ActionDrop}d {Normal}bbb {Normal}comment 2",
				"{Normal} {ActionFixup}f {Normal}ccc {Normal}comment 3",
				"{Normal} {ActionExec}x {Normal}echo 'foo'",
				"{Normal} {ActionPick}p {Normal}ddd {Normal}comment 4",
				"{Normal} {ActionReword}r {Normal}eee {Normal}comment 5",
				"{Normal} {ActionBreak}b ",
				"{Normal} {ActionSquash}s {Normal}fff {Normal}comment 6",
				"{Normal} {ActionEdit}e {Normal}111 {Normal}comment 7",
				"{Normal} {ActionLabel}l {Normal}ref",
				"{Normal} {ActionReset}t {Normal}ref",
				"{Normal} {ActionMerge}m {Normal}command"
			);
		},
	);
}

#[test]
fn move_cursor_down_1() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::MoveCursorDown)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn move_cursor_down_view_end() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::MoveCursorDown); 2],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn move_cursor_down_past_end() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::MoveCursorDown); 3],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn move_cursor_down_scroll_bottom_move_up_one() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn move_cursor_down_scroll_bottom_move_up_top() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn move_cursor_up_attempt_above_top() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_down_attempt_below_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(MetaEvent::MoveCursorDown); 4],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn move_cursor_page_up_from_top() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(MetaEvent::MoveCursorPageUp)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_up_from_one_page_down() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_up_from_one_page_down_minus_1() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_up_from_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_home() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorHome),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_end() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(MetaEvent::MoveCursorEnd)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn move_cursor_page_down_past_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[Event::from(MetaEvent::MoveCursorPageDown); 3],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn move_cursor_page_down_one_from_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageDown),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn move_cursor_page_down_one_page_from_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageDown),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn mouse_scroll() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollDown,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollDown,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
			Event::Mouse(MouseEvent {
				kind: MouseEventKind::ScrollUp,
				column: 0,
				row: 0,
				modifiers: KeyModifiers::empty(),
			}),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn visual_mode_start() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::ToggleVisualMode)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn visual_mode_start_cursor_down_one() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn visual_mode_start_cursor_page_down() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorPageDown),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.height = 4;
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_start_cursor_from_bottom_move_up() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c5{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_start_cursor_from_bottom_to_top() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c5{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_to_drop() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionDrop)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionDrop}drop   {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_to_edit() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionEdit)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionEdit}edit   {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_to_fixup() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionFixup)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionFixup}fixup  {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_to_pick() {
	module_test(
		&["drop aaa c1"],
		&[Event::from(MetaEvent::ActionPick)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_to_reword() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionReword)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionReword}reword {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_to_squash() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionSquash)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionSquash}squash {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_toggle_break_add() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionBreak)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionBreak}break  {Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_toggle_break_remove() {
	module_test(
		&["pick aaa c1", "break"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionBreak),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_toggle_break_above_existing() {
	module_test(
		&["pick aaa c1", "break"],
		&[Event::from(MetaEvent::ActionBreak)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionBreak}break  "
			);
		},
	);
}

#[test]
fn change_selected_line_auto_select_next_with_next_line() {
	module_test(
		&["pick aaa c1", "pick aaa c2"],
		&[Event::from(MetaEvent::ActionSquash)],
		|mut test_context| {
			let mut config = create_config();
			config.auto_select_next = true;
			let mut module = List::new(&config);
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionSquash}squash {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn change_selected_line_swap_down() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[Event::from(MetaEvent::SwapSelectedDown)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn change_selected_line_swap_up() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2"
			);
		},
	);
}

#[test]
fn normal_mode_show_commit_when_hash_available() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ShowCommit)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ShowCommit),
				state = State::ShowCommit
			);
		},
	);
}

#[test]
fn normal_mode_show_commit_when_no_selected_line() {
	module_test(&[], &[Event::from(MetaEvent::ShowCommit)], |mut test_context| {
		let mut module = List::new(&create_config());
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(MetaEvent::ShowCommit)
		);
	});
}

#[test]
fn normal_mode_do_not_show_commit_when_hash_not_available() {
	module_test(
		&["exec echo foo"],
		&[Event::from(MetaEvent::ShowCommit)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ShowCommit)
			);
		},
	);
}

#[test]
fn normal_mode_abort() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::Abort)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Abort),
				state = State::ConfirmAbort
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
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ForceAbort),
				exit_status = ExitStatus::Good
			);
			assert!(test_context.rebase_todo_file.is_empty());
		},
	);
}

#[test]
fn normal_mode_rebase() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::Rebase)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Rebase),
				state = State::ConfirmRebase
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
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ForceRebase),
				exit_status = ExitStatus::Good
			);
			assert!(!test_context.rebase_todo_file.is_noop());
		},
	);
}

#[test]
fn normal_mode_edit_with_edit_content() {
	module_test(
		&["exec echo foo"],
		&[Event::from(MetaEvent::Edit)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Edit)
			);
			assert_eq!(module.state, ListState::Edit);
		},
	);
}

#[test]
fn normal_mode_edit_without_edit_content() {
	module_test(&["pick aaa c1"], &[Event::from(MetaEvent::Edit)], |mut test_context| {
		let mut module = List::new(&create_config());
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(MetaEvent::Edit)
		);
		assert_eq!(module.state, ListState::Normal);
	});
}

#[test]
fn normal_mode_edit_without_selected_line() {
	module_test(&[], &[Event::from(MetaEvent::Edit)], |mut test_context| {
		let mut module = List::new(&create_config());
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(MetaEvent::Edit)
		);
		assert_eq!(module.state, ListState::Normal);
	});
}

#[test]
fn normal_mode_insert_line() {
	module_test(&[], &[Event::from(MetaEvent::InsertLine)], |mut test_context| {
		let mut module = List::new(&create_config());
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(MetaEvent::InsertLine),
			state = State::Insert
		);
	});
}

#[test]
fn normal_mode_open_external_editor() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::OpenInEditor)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::OpenInEditor),
				state = State::ExternalEditor
			);
		},
	);
}

#[test]
fn normal_mode_undo() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionDrop), Event::from(MetaEvent::Undo)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Undo)
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn normal_mode_undo_visual_mode_change() {
	module_test(
		&["pick aaa c1", "pick bbb c2"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionDrop),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::Undo),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      {Normal}c2{Normal}{Pad( )}"
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn normal_mode_redo() {
	module_test(
		&["drop aaa c1"],
		&[
			Event::from(MetaEvent::ActionPick),
			Event::from(MetaEvent::Undo),
			Event::from(MetaEvent::Redo),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_event(&mut module);
			let _ = test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Redo)
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn normal_mode_redo_visual_mode_change() {
	module_test(
		&["drop aaa c1", "drop bbb c2"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionPick),
			Event::from(MetaEvent::Undo),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::Redo),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      {Normal}c2{Normal}{Pad( )}"
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn normal_mode_remove_line_first() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[Event::from(MetaEvent::Delete)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      {Normal}c2{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}ccc      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}ddd      {Normal}c4",
				"{Normal}   {ActionPick}pick   {Normal}eee      {Normal}c5"
			);
		},
	);
}

#[test]
fn normal_mode_remove_line_end() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}bbb      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}ccc      {Normal}c3",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}ddd      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn normal_mode_toggle_visual_mode() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ToggleVisualMode)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ToggleVisualMode)
			);
			assert_eq!(module.visual_index_start, Some(0));
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn normal_mode_other_event() {
	module_test(&["pick aaa c1"], &[Event::from(KeyCode::Null)], |mut test_context| {
		let mut module = List::new(&create_config());
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(KeyCode::Null)
		);
	});
}

#[test]
fn visual_mode_action_change_top_bottom() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionReword),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionReword}reword {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionReword}reword {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionReword}reword {Normal}aaa      {Normal}c3{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_bottom_top() {
	module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::ActionReword),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionReword}reword {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionReword}reword {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionReword}reword {Normal}aaa      {Normal}c3{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_drop() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionDrop),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal,Dimmed} > {ActionDrop}drop   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionDrop}drop   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionDrop}drop   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_edit() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionEdit),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal,Dimmed} > {ActionEdit}edit   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionEdit}edit   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionEdit}edit   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_fixup() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionFixup),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal,Dimmed} > {ActionFixup}fixup  {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionFixup}fixup  {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionFixup}fixup  {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_pick() {
	module_test(
		&[
			"drop aaa c1",
			"drop aaa c2",
			"drop aaa c3",
			"drop aaa c4",
			"drop aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionPick),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionDrop}drop   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionDrop}drop   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_reword() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionReword),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal,Dimmed} > {ActionReword}reword {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionReword}reword {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionReword}reword {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_squash() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionSquash),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal,Dimmed} > {ActionSquash}squash {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionSquash}squash {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionSquash}squash {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
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
			let mut module = List::new(&create_config());
			let _ = test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Abort),
				state = State::ConfirmAbort
			);
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
			let mut module = List::new(&create_config());
			let _ = test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ForceAbort),
				exit_status = ExitStatus::Good
			);
			assert!(test_context.rebase_todo_file.is_empty());
		},
	);
}

#[test]
fn visual_mode_rebase() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ToggleVisualMode), Event::from(MetaEvent::Rebase)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Rebase),
				state = State::ConfirmRebase
			);
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
			let mut module = List::new(&create_config());
			let _ = test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ForceRebase),
				exit_status = ExitStatus::Good
			);
			assert!(!test_context.rebase_todo_file.is_noop());
		},
	);
}

#[test]
fn visual_mode_swap_down_from_top_to_bottom_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_from_bottom_to_top_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_from_top_to_bottom_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_from_bottom_to_top_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_to_limit_from_bottom_to_top_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedDown),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_to_limit_from_top_to_bottom_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedDown),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_to_limit_from_top_to_bottom_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_to_limit_from_bottom_to_top_selection() {
	module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedUp),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c2{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c3{Normal}{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_toggle_visual_mode() {
	module_test(
		&["pick aaa c1"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::ToggleVisualMode),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ToggleVisualMode)
			);
			assert_eq!(module.visual_index_start, None);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn visual_mode_open_external_editor() {
	module_test(
		&["pick aaa c1"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::OpenInEditor),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::OpenInEditor),
				state = State::ExternalEditor
			);
		},
	);
}

#[test]
fn visual_mode_undo() {
	module_test(
		&["pick aaa c1", "pick bbb c2"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionDrop),
			Event::from(MetaEvent::Undo),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_n_events(&mut module, 3);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Undo)
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      {Normal}c2{Normal}{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_undo_normal_mode_change() {
	module_test(
		&["pick aaa c1", "pick bbb c2"],
		&[
			Event::from(MetaEvent::ActionDrop),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Undo),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_n_events(&mut module, 3);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Undo)
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}bbb      {Normal}c2"
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn visual_mode_redo() {
	module_test(
		&["drop aaa c1", "drop bbb c2"],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionPick),
			Event::from(MetaEvent::Undo),
			Event::from(MetaEvent::Redo),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      {Normal}c2{Normal}{Pad( )}"
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}
#[test]
fn visual_mode_redo_normal_mode_change() {
	module_test(
		&["drop aaa c1", "drop bbb c2"],
		&[
			Event::from(MetaEvent::ActionPick),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Undo),
			Event::from(MetaEvent::Redo),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      {Normal}c1{Normal}{Pad( )}",
				"{Normal}   {ActionDrop}drop   {Normal}bbb      {Normal}c2"
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn visual_mode_remove_lines_start_index_first() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}ddd      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}eee      {Normal}c5"
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				test_context.rebase_todo_file.get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_end_index_first() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}ddd      {Normal}c4{Normal}{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}eee      {Normal}c5"
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				test_context.rebase_todo_file.get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_start_index_last() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      {Normal}c2{Normal}{Pad( )}"
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				test_context.rebase_todo_file.get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_end_index_last() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      {Normal}c2{Normal}{Pad( )}"
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				test_context.rebase_todo_file.get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_other_event() {
	module_test(&["pick aaa c1"], &[Event::from(KeyCode::Null)], |mut test_context| {
		let mut module = List::new(&create_config());
		assert_process_result!(
			test_context.handle_event(&mut module),
			event = Event::from(KeyCode::Null)
		);
	});
}

#[test]
fn edit_mode_render() {
	module_test(&["exec foo"], &[Event::from(MetaEvent::Edit)], |mut test_context| {
		let mut module = List::new(&create_config());
		let _ = test_context.handle_all_events(&mut module);
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"{IndicatorColor}Modifying line: exec foo",
			"",
			"{BODY}",
			"{Normal,Dimmed}exec {Normal}foo{Normal,Underline} ",
			"{TRAILING}",
			"{IndicatorColor}Enter to finish"
		);
	});
}

#[test]
fn edit_mode_handle_event() {
	module_test(
		&["exec foo"],
		&[
			Event::from(MetaEvent::Edit),
			Event::from(KeyCode::Backspace),
			Event::from(KeyCode::Enter),
		],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.build_view_data(&mut module);
			let _ = test_context.handle_all_events(&mut module);
			assert_eq!(test_context.rebase_todo_file.get_line(0).unwrap().get_content(), "fo");
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn scroll_right() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::MoveCursorRight)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			test_context.view_sender_context.assert_render_action(&["ScrollRight"]);
		},
	);
}

#[test]
fn scroll_left() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::MoveCursorLeft)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			let _ = test_context.handle_all_events(&mut module);
			test_context.view_sender_context.assert_render_action(&["ScrollLeft"]);
		},
	);
}

#[test]
fn normal_mode_help() {
	module_test(&["pick aaa c1"], &[Event::from(MetaEvent::Help)], |mut test_context| {
		let mut module = List::new(&create_config());
		module.state = ListState::Normal;
		let _ = test_context.handle_all_events(&mut module);
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"{Normal,Underline} Key      Action{Normal,Underline}{Pad( )}",
			"{BODY}",
			"{IndicatorColor} Up      {Normal,Dimmed}|{Normal}Move selection up",
			"{IndicatorColor} Down    {Normal,Dimmed}|{Normal}Move selection down",
			"{IndicatorColor} PageUp  {Normal,Dimmed}|{Normal}Move selection up 5 lines",
			"{IndicatorColor} PageDown{Normal,Dimmed}|{Normal}Move selection down 5 lines",
			"{IndicatorColor} Home    {Normal,Dimmed}|{Normal}Move selection to top of the list",
			"{IndicatorColor} End     {Normal,Dimmed}|{Normal}Move selection to end of the list",
			"{IndicatorColor} Left    {Normal,Dimmed}|{Normal}Scroll content to the left",
			"{IndicatorColor} Right   {Normal,Dimmed}|{Normal}Scroll content to the right",
			"{IndicatorColor} q       {Normal,Dimmed}|{Normal}Abort interactive rebase",
			"{IndicatorColor} Q       {Normal,Dimmed}|{Normal}Immediately abort interactive rebase",
			"{IndicatorColor} w       {Normal,Dimmed}|{Normal}Write interactive rebase file",
			"{IndicatorColor} W       {Normal,Dimmed}|{Normal}Immediately write interactive rebase file",
			"{IndicatorColor} v       {Normal,Dimmed}|{Normal}Enter visual mode",
			"{IndicatorColor} ?       {Normal,Dimmed}|{Normal}Show help",
			"{IndicatorColor} c       {Normal,Dimmed}|{Normal}Show commit information",
			"{IndicatorColor} j       {Normal,Dimmed}|{Normal}Move selected commit down",
			"{IndicatorColor} k       {Normal,Dimmed}|{Normal}Move selected commit up",
			"{IndicatorColor} b       {Normal,Dimmed}|{Normal}Toggle break action",
			"{IndicatorColor} p       {Normal,Dimmed}|{Normal}Set selected commit to be picked",
			"{IndicatorColor} r       {Normal,Dimmed}|{Normal}Set selected commit to be reworded",
			"{IndicatorColor} e       {Normal,Dimmed}|{Normal}Set selected commit to be edited",
			"{IndicatorColor} s       {Normal,Dimmed}|{Normal}Set selected commit to be squashed",
			"{IndicatorColor} f       {Normal,Dimmed}|{Normal}Set selected commit to be fixed-up",
			"{IndicatorColor} d       {Normal,Dimmed}|{Normal}Set selected commit to be dropped",
			"{IndicatorColor} E       {Normal,Dimmed}|{Normal}Edit an exec action's command",
			"{IndicatorColor} I       {Normal,Dimmed}|{Normal}Insert a new line",
			"{IndicatorColor} Delete  {Normal,Dimmed}|{Normal}Completely remove the selected line",
			"{IndicatorColor} Controlz{Normal,Dimmed}|{Normal}Undo the last change",
			"{IndicatorColor} Controly{Normal,Dimmed}|{Normal}Redo the previous undone change",
			"{IndicatorColor} !       {Normal,Dimmed}|{Normal}Open the todo file in the default editor",
			"{TRAILING}",
			"{IndicatorColor}Press any key to close"
		);
	});
}

#[test]
fn normal_mode_help_event() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::Help), Event::from(MetaEvent::SwapSelectedDown)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.state = ListState::Normal;
			let _ = test_context.handle_all_events(&mut module);
			assert!(!module.normal_mode_help.is_active());
		},
	);
}

#[test]
fn visual_mode_help() {
	module_test(&["pick aaa c1"], &[Event::from(MetaEvent::Help)], |mut test_context| {
		let mut module = List::new(&create_config());
		module.state = ListState::Visual;
		let _ = test_context.handle_all_events(&mut module);
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}",
			"{LEADING}",
			"{Normal,Underline} Key      Action{Normal,Underline}{Pad( )}",
			"{BODY}",
			"{IndicatorColor} Up      {Normal,Dimmed}|{Normal}Move selection up",
			"{IndicatorColor} Down    {Normal,Dimmed}|{Normal}Move selection down",
			"{IndicatorColor} PageUp  {Normal,Dimmed}|{Normal}Move selection up 5 lines",
			"{IndicatorColor} PageDown{Normal,Dimmed}|{Normal}Move selection down 5 lines",
			"{IndicatorColor} Home    {Normal,Dimmed}|{Normal}Move selection to top of the list",
			"{IndicatorColor} End     {Normal,Dimmed}|{Normal}Move selection to end of the list",
			"{IndicatorColor} Left    {Normal,Dimmed}|{Normal}Scroll content to the left",
			"{IndicatorColor} Right   {Normal,Dimmed}|{Normal}Scroll content to the right",
			"{IndicatorColor} ?       {Normal,Dimmed}|{Normal}Show help",
			"{IndicatorColor} j       {Normal,Dimmed}|{Normal}Move selected commits down",
			"{IndicatorColor} k       {Normal,Dimmed}|{Normal}Move selected commits up",
			"{IndicatorColor} p       {Normal,Dimmed}|{Normal}Set selected commits to be picked",
			"{IndicatorColor} r       {Normal,Dimmed}|{Normal}Set selected commits to be reworded",
			"{IndicatorColor} e       {Normal,Dimmed}|{Normal}Set selected commits to be edited",
			"{IndicatorColor} s       {Normal,Dimmed}|{Normal}Set selected commits to be squashed",
			"{IndicatorColor} f       {Normal,Dimmed}|{Normal}Set selected commits to be fixed-up",
			"{IndicatorColor} d       {Normal,Dimmed}|{Normal}Set selected commits to be dropped",
			"{IndicatorColor} Delete  {Normal,Dimmed}|{Normal}Completely remove the selected lines",
			"{IndicatorColor} Controlz{Normal,Dimmed}|{Normal}Undo the last change",
			"{IndicatorColor} Controly{Normal,Dimmed}|{Normal}Redo the previous undone change",
			"{IndicatorColor} v       {Normal,Dimmed}|{Normal}Exit visual mode",
			"{TRAILING}",
			"{IndicatorColor}Press any key to close"
		);
	});
}

#[test]
fn visual_mode_help_event() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::Help), Event::from(MetaEvent::SwapSelectedDown)],
		|mut test_context| {
			let mut module = List::new(&create_config());
			module.state = ListState::Visual;
			let _ = test_context.handle_all_events(&mut module);
			assert!(!module.visual_mode_help.is_active());
		},
	);
}

// this can technically never happen, but it's worth testing, just in case of an invalid state
#[test]
fn render_noop_list() {
	module_test(&["break"], &[], |mut test_context| {
		let mut module = List::new(&create_config());
		test_context.rebase_todo_file.remove_lines(0, 0);
		test_context.rebase_todo_file.add_line(0, Line::new("noop").unwrap());
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}{HELP}",
			"{BODY}",
			"{Selected}{Normal} > {Normal}noop   {Normal}{Pad( )}"
		);
	});
}

#[test]
fn resize() {
	module_test(&["pick aaa c1"], &[Event::Resize(100, 200)], |mut test_context| {
		let mut module = List::new(&create_config());
		let _ = test_context.handle_all_events(&mut module);
		assert_eq!(module.height, 200);
	});
}
