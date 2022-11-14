use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn empty_list() {
	module_test(&[], &[], |test_context| {
		let mut module = List::new(&Config::new());
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
fn full() {
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
			let mut module = List::new(&Config::new());
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaaaaaaa comment 1{Pad( )}",
				"{Normal}   {ActionDrop}drop   {Normal}bbbbbbbb comment 2",
				"{Normal}   {ActionFixup}fixup  {Normal}cccccccc comment 3",
				"{Normal}   {ActionExec}exec   {Normal}echo 'foo'",
				"{Normal}   {ActionPick}pick   {Normal}dddddddd comment 4",
				"{Normal}   {ActionReword}reword {Normal}eeeeeeee comment 5",
				"{Normal}   {ActionBreak}break",
				"{Normal}   {ActionSquash}squash {Normal}ffffffff comment 6",
				"{Normal}   {ActionEdit}edit   {Normal}11111111 comment 7",
				"{Normal}   {ActionLabel}label  {Normal}ref",
				"{Normal}   {ActionReset}reset  {Normal}ref",
				"{Normal}   {ActionMerge}merge  {Normal}command"
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
			let mut module = List::new(&Config::new());
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal}>{ActionPick}p {Normal}aaa comment 1{Pad( )}",
				"{Normal} {ActionDrop}d {Normal}bbb comment 2",
				"{Normal} {ActionFixup}f {Normal}ccc comment 3",
				"{Normal} {ActionExec}x {Normal}echo 'foo'",
				"{Normal} {ActionPick}p {Normal}ddd comment 4",
				"{Normal} {ActionReword}r {Normal}eee comment 5",
				"{Normal} {ActionBreak}b",
				"{Normal} {ActionSquash}s {Normal}fff comment 6",
				"{Normal} {ActionEdit}e {Normal}111 comment 7",
				"{Normal} {ActionLabel}l {Normal}ref",
				"{Normal} {ActionReset}t {Normal}ref",
				"{Normal} {ActionMerge}m {Normal}command"
			);
		},
	);
}

// this can technically never happen, but it's worth testing, just in case of an invalid state
#[test]
fn noop_list() {
	module_test(&["break"], &[], |mut test_context| {
		let mut module = List::new(&Config::new());
		test_context.todo_file_context.todo_file_mut().remove_lines(0, 0);
		test_context
			.todo_file_context
			.todo_file_mut()
			.add_line(0, Line::new("noop").unwrap());
		let view_data = test_context.build_view_data(&mut module);
		assert_rendered_output!(
			view_data,
			"{TITLE}{HELP}",
			"{BODY}",
			"{Selected}{Normal} > noop   {Pad( )}"
		);
	});
}
