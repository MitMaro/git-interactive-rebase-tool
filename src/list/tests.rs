use super::*;
use crate::{
	assert_process_result,
	assert_rendered_output,
	display::size::Size,
	process::testutil::{process_module_test, TestContext, ViewState},
};

#[test]
#[serial_test::serial]
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
#[serial_test::serial]
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
#[serial_test::serial]
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
			..ViewState::default()
		},
		&[],
		|test_context: TestContext<'_>| {
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
#[serial_test::serial]
fn move_cursor_down_1() {
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
			..ViewState::default()
		},
		&[Input::MoveCursorDown],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn move_cursor_down_view_end() {
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
			..ViewState::default()
		},
		&[Input::MoveCursorDown; 2],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn move_cursor_down_scroll_1() {
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
			..ViewState::default()
		},
		&[Input::MoveCursorDown; 3],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn move_cursor_down_scroll_bottom() {
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
			..ViewState::default()
		},
		&[Input::MoveCursorDown; 4],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c5"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn move_cursor_down_scroll_bottom_move_up_one() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 4);
			test_context.build_view_data(&mut module);
			test_context.handle_input(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn move_cursor_down_scroll_bottom_move_up_top() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 4);
			test_context.build_view_data(&mut module);
			test_context.handle_n_inputs(&mut module, 3);
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
#[serial_test::serial]
fn move_cursor_up_attempt_above_top() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::MoveCursorUp, Input::MoveCursorUp],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn move_cursor_down_attempt_below_bottom() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::MoveCursorDown; 4],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn move_cursor_page_up_from_top() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3", "pick aaa c4"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::MoveCursorPageUp],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn move_cursor_page_up_from_one_page_down() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
			"pick aaa c6",
		],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::MoveCursorDown, Input::MoveCursorDown, Input::MoveCursorPageUp],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 2);
			test_context.build_view_data(&mut module);
			test_context.handle_input(&mut module);
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
#[serial_test::serial]
fn move_cursor_page_up_from_one_page_down_plus_1() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
			"pick aaa c6",
		],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorPageUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 3);
			test_context.build_view_data(&mut module);
			test_context.handle_input(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c2",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn move_cursor_page_up_from_one_page_down_minus_1() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
			"pick aaa c6",
		],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::MoveCursorDown, Input::MoveCursorDown, Input::MoveCursorPageUp],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 2);
			test_context.build_view_data(&mut module);
			test_context.handle_input(&mut module);
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
#[serial_test::serial]
fn move_cursor_page_up_from_bottom() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
			"pick aaa c6",
		],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorPageUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 5);
			test_context.build_view_data(&mut module);
			test_context.handle_input(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c6"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn move_cursor_page_down_from_bottom() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
			"pick aaa c6",
		],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorPageDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 5);
			test_context.build_view_data(&mut module);
			test_context.handle_input(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c6"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn move_cursor_page_down_one_from_bottom() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
			"pick aaa c6",
		],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorPageDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 4);
			test_context.build_view_data(&mut module);
			test_context.handle_input(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c6"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn move_cursor_page_down_one_page_from_bottom() {
	process_module_test(
		&[
			"pick aaa c1",
			"pick aaa c2",
			"pick aaa c3",
			"pick aaa c4",
			"pick aaa c5",
			"pick aaa c6",
		],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorPageDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 3);
			test_context.build_view_data(&mut module);
			test_context.handle_input(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c4",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c5",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c6"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_start() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ToggleVisualMode],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn visual_mode_start_cursor_down_one() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ToggleVisualMode, Input::MoveCursorDown],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn visual_mode_start_move_down_below_view() {
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
			..ViewState::default()
		},
		&[
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
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
#[serial_test::serial]
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
			..ViewState::default()
		},
		&[Input::ToggleVisualMode, Input::MoveCursorPageDown],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c1",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c3"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_start_cursor_page_down_below_view() {
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
			..ViewState::default()
		},
		&[
			Input::ToggleVisualMode,
			Input::MoveCursorPageDown,
			Input::MoveCursorPageDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c4",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c5"
			);
		},
	);
}

#[test]
#[serial_test::serial]
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
			..ViewState::default()
		},
		&[
			Input::MoveCursorPageDown,
			Input::MoveCursorPageDown,
			Input::MoveCursorPageDown,
			Input::ToggleVisualMode,
			Input::MoveCursorUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      {Normal}c3",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c4",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c5"
			);
		},
	);
}

#[test]
#[serial_test::serial]
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
			..ViewState::default()
		},
		&[
			Input::MoveCursorPageDown,
			Input::MoveCursorPageDown,
			Input::MoveCursorPageDown,
			Input::ToggleVisualMode,
			Input::MoveCursorPageUp,
			Input::MoveCursorPageUp,
			Input::MoveCursorPageUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaa      {Normal(selected)}c1",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c2",
				"{Normal(selected),Dimmed} > {ActionPick(selected)}pick   {Normal(selected)}aaa      \
				 {Normal(selected)}c3"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn change_selected_line_to_drop() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionDrop],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_to_edit() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionEdit],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_to_fixup() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionFixup],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_to_pick() {
	process_module_test(
		&["drop aaa c1"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionPick],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_to_reword() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionReword],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_to_squash() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionSquash],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_toggle_break_add() {
	process_module_test(
		&["pick aaa c1"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionBreak],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_toggle_break_remove() {
	process_module_test(
		&["pick aaa c1", "break"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::MoveCursorDown, Input::ActionBreak],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_toggle_break_above_existing() {
	process_module_test(
		&["pick aaa c1", "break"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionBreak],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_auto_select_next_with_next_line() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2"],
		ViewState {
			size: Size::new(120, 4),
			..ViewState::default()
		},
		&[Input::ActionSquash],
		|mut test_context: TestContext<'_>| {
			let mut config = test_context.config.clone();
			config.auto_select_next = true;
			let mut module = List::new(&config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_swap_down() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[Input::SwapSelectedDown],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn change_selected_line_swap_up() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[Input::MoveCursorDown, Input::MoveCursorDown, Input::SwapSelectedUp],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn normal_mode_show_commit_when_hash_available() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ShowCommit],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::ShowCommit,
				state = State::ShowCommit
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_show_commit_when_no_selected_line() {
	process_module_test(
		&[],
		ViewState::default(),
		&[Input::ShowCommit],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ShowCommit);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_do_not_show_commit_when_hash_not_available() {
	process_module_test(
		&["exec echo foo"],
		ViewState::default(),
		&[Input::ShowCommit],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ShowCommit);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_abort() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::Abort],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Abort,
				state = State::ConfirmAbort
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_force_abort() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ForceAbort],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::ForceAbort,
				exit_status = ExitStatus::Good
			);
			assert!(test_context.rebase_todo_file.is_empty());
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_rebase() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::Rebase],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Rebase,
				state = State::ConfirmRebase
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_force_rebase() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ForceRebase],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::ForceRebase,
				exit_status = ExitStatus::Good
			);
			assert!(!test_context.rebase_todo_file.is_noop());
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_edit_with_edit_content() {
	process_module_test(
		&["exec echo foo"],
		ViewState::default(),
		&[Input::Edit],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Edit);
			assert_eq!(module.state, ListState::Edit);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_edit_without_edit_content() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::Edit],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Edit);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_edit_without_selected_line() {
	process_module_test(
		&[],
		ViewState::default(),
		&[Input::Edit],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Edit);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_open_external_editor() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::OpenInEditor],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::OpenInEditor,
				state = State::ExternalEditor
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_undo() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ActionDrop, Input::Undo],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_input(&mut module);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Undo);
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
#[serial_test::serial]
fn normal_mode_undo_visual_mode_change() {
	process_module_test(
		&["pick aaa c1", "pick bbb c2"],
		ViewState::default(),
		&[
			Input::ToggleVisualMode,
			Input::Down,
			Input::ActionDrop,
			Input::ToggleVisualMode,
			Input::Undo,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn normal_mode_redo() {
	process_module_test(
		&["drop aaa c1"],
		ViewState::default(),
		&[Input::ActionPick, Input::Undo, Input::Redo],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_input(&mut module);
			test_context.handle_input(&mut module);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Redo);
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
#[serial_test::serial]
fn normal_mode_redo_visual_mode_change() {
	process_module_test(
		&["drop aaa c1", "drop bbb c2"],
		ViewState::default(),
		&[
			Input::ToggleVisualMode,
			Input::Down,
			Input::ActionPick,
			Input::Undo,
			Input::ToggleVisualMode,
			Input::Redo,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
		&[Input::Delete],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::Delete,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn normal_mode_toggle_visual_mode() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ToggleVisualMode],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ToggleVisualMode);
			assert_eq!(module.visual_index_start, Some(0));
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_other_input() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::Other],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_action_change_top_bottom() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ActionReword,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn visual_mode_action_change_bottom_top() {
	process_module_test(
		&["pick aaa c1", "pick aaa c2", "pick aaa c3"],
		ViewState::default(),
		&[
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
			Input::ActionReword,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ActionDrop,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ActionEdit,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ActionFixup,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ActionPick,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ActionReword,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ActionSquash,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn visual_mode_abort() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ToggleVisualMode, Input::Abort],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_input(&mut module);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Abort,
				state = State::ConfirmAbort
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_force_abort() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ToggleVisualMode, Input::ForceAbort],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_input(&mut module);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::ForceAbort,
				exit_status = ExitStatus::Good
			);
			assert!(test_context.rebase_todo_file.is_empty());
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_rebase() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ToggleVisualMode, Input::Rebase],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_input(&mut module);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::Rebase,
				state = State::ConfirmRebase
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_force_rebase() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ToggleVisualMode, Input::ForceRebase],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_input(&mut module);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::ForceRebase,
				exit_status = ExitStatus::Good
			);
			assert!(!test_context.rebase_todo_file.is_noop());
		},
	);
}

#[test]
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::SwapSelectedDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
			Input::SwapSelectedDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::SwapSelectedUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
			Input::SwapSelectedUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
			Input::SwapSelectedDown,
			Input::SwapSelectedDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::SwapSelectedDown,
			Input::SwapSelectedDown,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::SwapSelectedUp,
			Input::SwapSelectedUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
			Input::SwapSelectedUp,
			Input::SwapSelectedUp,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn visual_mode_toggle_visual_mode() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ToggleVisualMode, Input::ToggleVisualMode],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_input(&mut module);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::ToggleVisualMode);
			assert_eq!(module.visual_index_start, None);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_open_external_editor() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::ToggleVisualMode, Input::OpenInEditor],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_input(&mut module);
			assert_process_result!(
				test_context.handle_input(&mut module),
				input = Input::OpenInEditor,
				state = State::ExternalEditor
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_undo() {
	process_module_test(
		&["pick aaa c1", "pick bbb c2"],
		ViewState::default(),
		&[Input::ToggleVisualMode, Input::Down, Input::ActionDrop, Input::Undo],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 3);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Undo);
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
#[serial_test::serial]
fn visual_mode_undo_normal_mode_change() {
	process_module_test(
		&["pick aaa c1", "pick bbb c2"],
		ViewState::default(),
		&[Input::ActionDrop, Input::ToggleVisualMode, Input::Down, Input::Undo],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_n_inputs(&mut module, 3);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Undo);
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
#[serial_test::serial]
fn visual_mode_redo() {
	process_module_test(
		&["drop aaa c1", "drop bbb c2"],
		ViewState::default(),
		&[
			Input::ToggleVisualMode,
			Input::Down,
			Input::ActionPick,
			Input::Undo,
			Input::Redo,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn visual_mode_redo_normal_mode_change() {
	process_module_test(
		&["drop aaa c1", "drop bbb c2"],
		ViewState::default(),
		&[
			Input::ActionPick,
			Input::ToggleVisualMode,
			Input::Down,
			Input::Undo,
			Input::Redo,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::Delete,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
			Input::Delete,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorUp,
			Input::MoveCursorUp,
			Input::Delete,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
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
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::ToggleVisualMode,
			Input::MoveCursorDown,
			Input::MoveCursorDown,
			Input::Delete,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.handle_all_inputs(&mut module);
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
#[serial_test::serial]
fn visual_mode_other_input() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[Input::Other],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			assert_process_result!(test_context.handle_input(&mut module), input = Input::Other);
		},
	);
}

#[test]
#[serial_test::serial]
fn edit_mode_render() {
	process_module_test(
		&["exec foo"],
		ViewState::default(),
		&[Input::Edit],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
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
#[serial_test::serial]
fn edit_mode_handle_input() {
	process_module_test(
		&["exec foo"],
		ViewState::default(),
		&[Input::Edit, Input::Backspace, Input::Enter],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
			assert_eq!(test_context.rebase_todo_file.get_line(0).unwrap().get_content(), "fo");
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
#[serial_test::serial]
fn scroll_right() {
	process_module_test(
		&[
			"pick aaaaaaaaaaaa this comment needs to be longer than the width of the view",
			"pick bbbbbbbbbbbb this comment needs to be longer than the width of the view",
			"pick cccccccccccc this comment needs to be longer than the width of the view",
		],
		ViewState {
			size: Size::new(50, 4),
			..ViewState::default()
		},
		&[Input::MoveCursorRight; 3],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaaaaaaa {Normal(selected)}s \
				 comment needs to be longer th",
				"{Normal}   {ActionPick}pick   {Normal}bbbbbbbb {Normal}s comment needs to be longer th",
				"{Normal}   {ActionPick}pick   {Normal}cccccccc {Normal}s comment needs to be longer th"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn scroll_left() {
	process_module_test(
		&[
			"pick aaaaaaaaaaaa this comment needs to be longer than the width of the view",
			"pick bbbbbbbbbbbb this comment needs to be longer than the width of the view",
			"pick cccccccccccc this comment needs to be longer than the width of the view",
		],
		ViewState {
			size: Size::new(50, 4),
			..ViewState::default()
		},
		&[
			Input::MoveCursorRight,
			Input::MoveCursorRight,
			Input::MoveCursorRight,
			Input::MoveCursorLeft,
		],
		|mut test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			test_context.build_view_data(&mut module);
			test_context.handle_all_inputs(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal(selected)} > {ActionPick(selected)}pick   {Normal(selected)}aaaaaaaa {Normal(selected)}is \
				 comment needs to be longer t",
				"{Normal}   {ActionPick}pick   {Normal}bbbbbbbb {Normal}is comment needs to be longer t",
				"{Normal}   {ActionPick}pick   {Normal}cccccccc {Normal}is comment needs to be longer t"
			);
		},
	);
}

#[test]
#[serial_test::serial]
fn normal_mode_help() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.state = ListState::Normal;
			let help = module.get_help_keybindings_descriptions().unwrap();
			assert_eq!(help.len(), 27);
		},
	);
}

#[test]
#[serial_test::serial]
fn visual_mode_help() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.state = ListState::Visual;
			let help = module.get_help_keybindings_descriptions().unwrap();
			assert_eq!(help.len(), 19);
		},
	);
}

#[test]
#[serial_test::serial]
fn edit_mode_help() {
	process_module_test(
		&["pick aaa c1"],
		ViewState::default(),
		&[],
		|test_context: TestContext<'_>| {
			let mut module = List::new(test_context.config);
			module.state = ListState::Edit;
			assert!(module.get_help_keybindings_descriptions().is_none());
		},
	);
}

// this can technically never happen, but it's worth testing, just in case of an invalid state
#[test]
#[serial_test::serial]
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
