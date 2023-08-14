use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn empty_list() {
	module_test(&[], &[], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			Style view_data,
			"{TITLE}{HELP}",
			"{LEADING}",
			"{IndicatorColor}Rebase todo file is empty"
		);
	});
}

#[test]
fn full() {
	module_test(
		&[
			"pick aaaaaaaa comment 1",
			"drop bbbbbbbb comment 2",
			"fixup cccccccc comment 3",
			"fixup -c cccccccb comment 3b",
			"exec echo 'foo'",
			"pick dddddddd comment 4",
			"reword eeeeeeee comment 5",
			"break",
			"squash ffffffff comment 6",
			"edit 11111111 comment 7",
			"label ref",
			"reset ref",
			"merge command",
			"update-ref reference",
		],
		&[],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick     {Normal}aaaaaaaa comment 1{Pad( )}",
				"{Normal}   {ActionDrop}drop     {Normal}bbbbbbbb comment 2",
				"{Normal}   {ActionFixup}fixup    {Normal}cccccccc comment 3",
				"{Normal}   {ActionFixup}fixup -c {Normal}cccccccb comment 3b",
				"{Normal}   {ActionExec}exec     {Normal}echo 'foo'",
				"{Normal}   {ActionPick}pick     {Normal}dddddddd comment 4",
				"{Normal}   {ActionReword}reword   {Normal}eeeeeeee comment 5",
				"{Normal}   {ActionBreak}break",
				"{Normal}   {ActionSquash}squash   {Normal}ffffffff comment 6",
				"{Normal}   {ActionEdit}edit     {Normal}11111111 comment 7",
				"{Normal}   {ActionLabel}label    {Normal}ref",
				"{Normal}   {ActionReset}reset    {Normal}ref",
				"{Normal}   {ActionMerge}merge    {Normal}command",
				"{Normal}   {ActionUpdateRef}update-ref {Normal}reference"
			);
		},
	);
}

#[test]
fn compact() {
	module_test(
		&[
			"pick aaaaaaaa comment 1",
			"drop bbbbbbbb comment 2",
			"fixup cccccccc comment 3",
			"fixup -c cccccccb comment 3b",
			"exec echo 'foo'",
			"pick dddddddd comment 4",
			"reword eeeeeeee comment 5",
			"break",
			"squash ffffffff comment 6",
			"edit 11111111 comment 7",
			"label ref",
			"reset ref",
			"merge command",
			"update-ref reference",
		],
		&[],
		|mut test_context| {
			test_context.render_context.update(30, 300);
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Style view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal}>{ActionPick}p {Normal}aaa comment 1{Pad( )}",
				"{Normal} {ActionDrop}d {Normal}bbb comment 2",
				"{Normal} {ActionFixup}f {Normal}ccc comment 3",
				"{Normal} {ActionFixup}f*{Normal}ccc comment 3b",
				"{Normal} {ActionExec}x {Normal}echo 'foo'",
				"{Normal} {ActionPick}p {Normal}ddd comment 4",
				"{Normal} {ActionReword}r {Normal}eee comment 5",
				"{Normal} {ActionBreak}b",
				"{Normal} {ActionSquash}s {Normal}fff comment 6",
				"{Normal} {ActionEdit}e {Normal}111 comment 7",
				"{Normal} {ActionLabel}l {Normal}ref",
				"{Normal} {ActionReset}t {Normal}ref",
				"{Normal} {ActionMerge}m {Normal}command",
				"{Normal} {ActionUpdateRef}u {Normal}reference"
			);
		},
	);
}

// this can technically never happen, but it's worth testing, just in case of an invalid state
#[test]
fn noop_list() {
	module_test(&["break"], &[], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		let mut todo_file = module.todo_file.lock();
		todo_file.remove_lines(0, 0);
		todo_file.add_line(0, Line::parse("noop").unwrap());
		drop(todo_file);

		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			Style view_data,
			"{TITLE}{HELP}",
			"{BODY}",
			"{Selected}{Normal} > noop   {Pad( )}"
		);
	});
}

#[test]
fn pinned_segments() {
	module_test(
		&[
			"break",
			"drop aaa c1",
			"edit aaa c1",
			"fixup aaa c1",
			"pick aaa c1",
			"reword aaa c1",
			"squash aaa c1",
			"exec command",
			"label reference",
			"reset reference",
			"merge command",
		],
		&[Event::from(MetaEvent::ActionDrop)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::INCLUDE_PINNED,
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Pin(3)}{Selected} > break  {Pad( )}",
				"{Pin(2)}   drop   aaa      c1",
				"{Pin(2)}   edit   aaa      c1",
				"{Pin(2)}   fixup  aaa      c1",
				"{Pin(2)}   pick   aaa      c1",
				"{Pin(2)}   reword aaa      c1",
				"{Pin(2)}   squash aaa      c1",
				"{Pin(3)}   exec   command",
				"{Pin(3)}   label  reference",
				"{Pin(3)}   reset  reference",
				"{Pin(3)}   merge  command"
			);
		},
	);
}

#[test]
fn full_with_short_actions() {
	module_test(&["pick aaaaaaaa comment 1"], &[], |mut test_context| {
		let mut module = create_list(&Config::new(), test_context.take_todo_file());
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			Style view_data,
			"{TITLE}{HELP}",
			"{BODY}",
			"{Selected}{Normal} > {ActionPick}pick   {Normal}aaaaaaaa comment 1{Pad( )}"
		);
	});
}
