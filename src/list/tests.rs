use super::*;
use crate::{
	assert_process_result,
	assert_render_action,
	assert_rendered_output,
	display::size::Size,
	input::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind},
	process::testutil::{process_module_test, TestContext, ViewState},
};

#[test]
fn render_empty_list() {
	process_module_test(&[], ViewState::default(), &[], |test_context: TestContext<'_>| {
		let mut module = List::new(test_context.config);
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
	process_module_test(
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
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaaaaaaa \
				 {Normal(selected)}comment 1",
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
	process_module_test(
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
		ViewState {
			size: Size::new(30, 100),
		},
		&[],
		|mut test_context: TestContext<'_>| {
			test_context.render_context.update(30, 300);
			let mut module = List::new(test_context.config);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)}>{ActionPick(selected)}p {Normal(selected)}aaa {Normal(selected)}comment 1",
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
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[Event::from(MetaEvent::MoveCursorDown)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn move_cursor_down_view_end() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[Event::from(MetaEvent::MoveCursorDown); 2],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3"
			);
		},
	);
}

#[test]
fn move_cursor_down_past_end() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[Event::from(MetaEvent::MoveCursorDown); 3],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3"
			);
		},
	);
}

#[test]
fn move_cursor_down_scroll_bottom_move_up_one() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn move_cursor_down_scroll_bottom_move_up_top() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn move_cursor_up_attempt_above_top() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_down_attempt_below_bottom() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::MoveCursorDown); 4],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_up_from_top() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::MoveCursorPageUp)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.height = 4;
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_up_from_one_page_down() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.height = 4;
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_up_from_one_page_down_minus_1() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.height = 4;
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_up_from_bottom() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.height = 4;
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_home() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorHome),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
fn move_cursor_end() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState::default(),
		&[Event::from(MetaEvent::MoveCursorEnd)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_down_past_bottom() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState::default(),
		&[Event::from(MetaEvent::MoveCursorPageDown); 3],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.height = 4;
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_down_one_from_bottom() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageDown),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn move_cursor_page_down_one_page_from_bottom() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorPageDown),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.height = 4;
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn mouse_scroll() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
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
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn visual_mode_start() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ToggleVisualMode)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn visual_mode_start_cursor_down_one() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn visual_mode_start_cursor_page_down() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState {
			size: Size::new(120, 4),
		},
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorPageDown),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.height = 4;
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c1",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_start_cursor_from_bottom_move_up() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState {
			size: Size::new(120, 4),
		},
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c5"
			);
		},
	);
}

#[test]
fn visual_mode_start_cursor_from_bottom_to_top() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState {
			size: Size::new(120, 4),
		},
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
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c4",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c5"
			);
		},
	);
}

#[test]
fn change_selected_line_to_drop() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionDrop)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionDrop(selected)}drop   {Normal(selected)}aaa      {Normal(selected)}c1"
			);
		},
	);
}

#[test]
fn change_selected_line_to_edit() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionEdit)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionEdit(selected)}edit   {Normal(selected)}aaa      {Normal(selected)}c1"
			);
		},
	);
}

#[test]
fn change_selected_line_to_fixup() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionFixup)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionFixup(selected)}fixup  {Normal(selected)}aaa      {Normal(selected)}c1"
			);
		},
	);
}

#[test]
fn change_selected_line_to_pick() {
	process_module_test(
		&["drop aaa c1"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionPick)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1"
			);
		},
	);
}

#[test]
fn change_selected_line_to_reword() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionReword)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionReword(selected)}reword {Normal(selected)}aaa      {Normal(selected)}c1"
			);
		},
	);
}

#[test]
fn change_selected_line_to_squash() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionSquash)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionSquash(selected)}squash {Normal(selected)}aaa      {Normal(selected)}c1"
			);
		},
	);
}

#[test]
fn change_selected_line_toggle_break_add() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionBreak)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionBreak(selected)}break  "
			);
		},
	);
}

#[test]
fn change_selected_line_toggle_break_remove() {
	process_module_test(
		&["pick aaa c1", "break"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionBreak),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1"
			);
		},
	);
}

#[test]
fn change_selected_line_toggle_break_above_existing() {
	process_module_test(
		&["pick aaa c1", "break"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionBreak)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionBreak}break  "
			);
		},
	);
}

#[test]
fn change_selected_line_auto_select_next_with_next_line() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2"],
		ViewState {
			size: Size::new(120, 4),
		},
		&[Event::from(MetaEvent::ActionSquash)],
		|mut test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.auto_select_next = true;
			let mut module = List::new(&config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionSquash}squash {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2"
			);
		},
	);
}

#[test]
fn change_selected_line_swap_down() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[Event::from(MetaEvent::SwapSelectedDown)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3"
			);
		},
	);
}

#[test]
fn change_selected_line_swap_up() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2"
			);
		},
	);
}

#[test]
fn normal_mode_show_commit_when_hash_available() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::ShowCommit)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
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
	process_module_test(
		&[],
		ViewState::default(),
		&[Event::from(MetaEvent::ShowCommit)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ShowCommit)
			);
		},
	);
}

#[test]
fn normal_mode_do_not_show_commit_when_hash_not_available() {
	process_module_test(
		&["exec echo foo"],
		ViewState::default(),
		&[Event::from(MetaEvent::ShowCommit)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::ShowCommit)
			);
		},
	);
}

#[test]
fn normal_mode_abort() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::Abort)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::ForceAbort)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::Rebase)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::ForceRebase)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
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
	process_module_test(
		&["exec echo foo"],
		ViewState::default(),
		&[Event::from(MetaEvent::Edit)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::Edit)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Edit)
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn normal_mode_edit_without_selected_line() {
	process_module_test(
		&[],
		ViewState::default(),
		&[Event::from(MetaEvent::Edit)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Edit)
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn normal_mode_insert_line() {
	process_module_test(
		&[],
		ViewState::default(),
		&[Event::from(MetaEvent::InsertLine)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::InsertLine),
				state = State::Insert
			);
		},
	);
}

#[test]
fn normal_mode_open_external_editor() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::OpenInEditor)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::ActionDrop), Event::from(MetaEvent::Undo)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Undo)
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1"
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn normal_mode_undo_visual_mode_change() {
	process_module_test(
		&["pick aaa c1", "pick bbb c2"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionDrop),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::Undo),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}bbb      {Normal(selected)}c2"
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn normal_mode_redo() {
	process_module_test(
		&["drop aaa c1"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ActionPick),
			Event::from(MetaEvent::Undo),
			Event::from(MetaEvent::Redo),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_event(&mut module);
			test_context.handle_event(&mut module);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Redo)
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1"
			);
		},
	);
}

#[test]
fn normal_mode_redo_visual_mode_change() {
	process_module_test(
		&["drop aaa c1", "drop bbb c2"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionPick),
			Event::from(MetaEvent::Undo),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::Redo),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}bbb      {Normal(selected)}c2"
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn normal_mode_remove_line_first() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		ViewState::default(),
		&[Event::from(MetaEvent::Delete)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}bbb      {Normal(selected)}c2",
				"{Normal}   {ActionPick}pick   {Normal}ccc      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}ddd      {Normal}c4",
				"{Normal}   {ActionPick}pick   {Normal}eee      {Normal}c5"
			);
		},
	);
}

#[test]
fn normal_mode_remove_line_end() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}bbb      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}ccc      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}ddd      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn normal_mode_toggle_visual_mode() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::ToggleVisualMode)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(KeyCode::Null)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(KeyCode::Null)
			);
		},
	);
}

#[test]
fn visual_mode_action_change_top_bottom() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionReword),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
				 {Normal(selected)}c1",
				"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected)} > {ActionReword(selected)}reword {Normal(selected)}aaa      {Normal(selected)}c3"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_bottom_top() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::ActionReword),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionReword(selected)}reword {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
				 {Normal(selected)}c3"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_drop() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionDrop),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected),Dimmed} > {ActionDrop(selected)}drop   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionDrop(selected)}drop   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionDrop(selected)}drop   {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_edit() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionEdit),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected),Dimmed} > {ActionEdit(selected)}edit   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionEdit(selected)}edit   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionEdit(selected)}edit   {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_fixup() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionFixup),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected),Dimmed} > {ActionFixup(selected)}fixup  {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionFixup(selected)}fixup  {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionFixup(selected)}fixup  {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_pick() {
	process_module_test(
		&[
			"drop aaa c1",
			"drop aaa c2",
			"drop aaa c3",
			"drop aaa c4",
			"drop aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionPick),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionDrop}drop   {Normal}aaa      {Normal}c1",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionDrop}drop   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_reword() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionReword),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionReword(selected)}reword {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionReword(selected)}reword {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_squash() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionSquash),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected),Dimmed} > {ActionSquash(selected)}squash {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionSquash(selected)}squash {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionSquash(selected)}squash {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_abort() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::ToggleVisualMode), Event::from(MetaEvent::Abort)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_event(&mut module);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::ForceAbort),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_event(&mut module);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::ToggleVisualMode), Event::from(MetaEvent::Rebase)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_event(&mut module);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::ForceRebase),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_event(&mut module);
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
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_from_bottom_to_top_selection() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_from_top_to_bottom_selection() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_from_bottom_to_top_selection() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_to_limit_from_bottom_to_top_selection() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
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
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn visual_mode_swap_down_to_limit_from_top_to_bottom_selection() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedDown),
			Event::from(MetaEvent::SwapSelectedDown),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_to_limit_from_top_to_bottom_selection() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::SwapSelectedUp),
			Event::from(MetaEvent::SwapSelectedUp),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_swap_up_to_limit_from_bottom_to_top_selection() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
		],
		ViewState::default(),
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
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5"
			);
		},
	);
}

#[test]
fn visual_mode_toggle_visual_mode() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::ToggleVisualMode),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_event(&mut module);
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::OpenInEditor),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_event(&mut module);
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
	process_module_test(
		&["pick aaa c1", "pick bbb c2"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionDrop),
			Event::from(MetaEvent::Undo),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_events(&mut module, 3);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Undo)
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}bbb      {Normal(selected)}c2"
			);
		},
	);
}

#[test]
fn visual_mode_undo_normal_mode_change() {
	process_module_test(
		&["pick aaa c1", "pick bbb c2"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ActionDrop),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Undo),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_events(&mut module, 3);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(MetaEvent::Undo)
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionPick}pick   {Normal}bbb      {Normal}c2"
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn visual_mode_redo() {
	process_module_test(
		&["drop aaa c1", "drop bbb c2"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ActionPick),
			Event::from(MetaEvent::Undo),
			Event::from(MetaEvent::Redo),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}bbb      {Normal(selected)}c2"
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}
#[test]
fn visual_mode_redo_normal_mode_change() {
	process_module_test(
		&["drop aaa c1", "drop bbb c2"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ActionPick),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Undo),
			Event::from(MetaEvent::Redo),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal}   {ActionDrop}drop   {Normal}bbb      {Normal}c2"
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn visual_mode_remove_lines_start_index_first() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}ddd      {Normal(selected)}c4",
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
	process_module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}ddd      {Normal(selected)}c4",
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
	process_module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		ViewState::default(),
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
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}bbb      {Normal(selected)}c2"
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
	process_module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		ViewState::default(),
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c1",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}bbb      {Normal(selected)}c2"
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
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(KeyCode::Null)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_event(&mut module),
				event = Event::from(KeyCode::Null)
			);
		},
	);
}

#[test]
fn edit_mode_render() {
	process_module_test(
		&["exec foo"],
		ViewState::default(),
		&[Event::from(MetaEvent::Edit)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
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
		},
	);
}

#[test]
fn edit_mode_handle_event() {
	process_module_test(
		&["exec foo"],
		ViewState::default(),
		&[
			Event::from(MetaEvent::Edit),
			Event::from(KeyCode::Backspace),
			Event::from(KeyCode::Enter),
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_events(&mut module);
			assert_eq!(test_context.rebase_todo_file.get_line(0).unwrap().get_content(), "fo");
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn scroll_right() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::MoveCursorRight)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_render_action!(&test_context.event_handler_context.view_sender, "ScrollRight");
		},
	);
}

#[test]
fn scroll_left() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::MoveCursorLeft)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_render_action!(&test_context.event_handler_context.view_sender, "ScrollLeft");
		},
	);
}

#[test]
fn normal_mode_help() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(200, 100),
		},
		&[Event::from(MetaEvent::Help)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.state = ListState::Normal;
			test_context.handle_all_events(&mut module);
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
		},
	);
}

#[test]
fn normal_mode_help_event() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::Help), Event::from(MetaEvent::SwapSelectedDown)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.state = ListState::Normal;
			test_context.handle_all_events(&mut module);
			assert!(!module.normal_mode_help.is_active());
		},
	);
}

#[test]
fn visual_mode_help() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(200, 100),
		},
		&[Event::from(MetaEvent::Help)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.state = ListState::Visual;
			test_context.handle_all_events(&mut module);
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
		},
	);
}

#[test]
fn visual_mode_help_event() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::from(MetaEvent::Help), Event::from(MetaEvent::SwapSelectedDown)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.state = ListState::Visual;
			test_context.handle_all_events(&mut module);
			assert!(!module.visual_mode_help.is_active());
		},
	);
}

// this can technically never happen, but it's worth testing, just in case of an invalid state
#[test]
fn render_noop_list() {
	process_module_test(
		&["break"],
		ViewState::default(),
		&[],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.rebase_todo_file.remove_lines(0, 0);
			test_context.rebase_todo_file.add_line(0, Line::new("noop").unwrap());
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {Normal(selected)}noop   "
			);
		},
	);
}

#[test]
fn resize() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Event::Resize(100, 200)],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_events(&mut module);
			assert_eq!(module.height, 200);
		},
	);
}
