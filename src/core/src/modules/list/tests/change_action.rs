use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn normal_mode_action_change_to_drop() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionDrop)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionDrop}drop   {Normal}aaa      c1{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_drop() {
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
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Selected}{Normal,Dimmed} > {ActionDrop}drop   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionDrop}drop   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionDrop}drop   {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_edit() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionEdit)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionEdit}edit   {Normal}aaa      c1{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_edit() {
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
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Selected}{Normal,Dimmed} > {ActionEdit}edit   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionEdit}edit   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionEdit}edit   {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_fixup() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionFixup)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionFixup}fixup  {Normal}aaa      c1{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_fixup() {
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
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Selected}{Normal,Dimmed} > {ActionFixup}fixup  {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionFixup}fixup  {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionFixup}fixup  {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_pick() {
	module_test(
		&["drop aaa c1"],
		&[Event::from(MetaEvent::ActionPick)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_pick() {
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
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionDrop}drop   {Normal}aaa      c1",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionDrop}drop   {Normal}aaa      c5"
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_reword() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionReword)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionReword}reword {Normal}aaa      c1{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_reword() {
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
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Selected}{Normal,Dimmed} > {ActionReword}reword {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionReword}reword {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionReword}reword {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}

#[test]
fn normal_mode_action_change_to_squash() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionSquash)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionSquash}squash {Normal}aaa      c1{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_action_change_to_squash() {
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
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			let view_data = test_context.build_view_data(&mut module);
			assert_rendered_output!(
				view_data,
				"{TITLE}{HELP}",
				"{BODY}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c1",
				"{Selected}{Normal,Dimmed} > {ActionSquash}squash {Normal}aaa      c2{Pad( )}",
				"{Selected}{Normal,Dimmed} > {ActionSquash}squash {Normal}aaa      c3{Pad( )}",
				"{Selected}{Normal} > {ActionSquash}squash {Normal}aaa      c4{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}aaa      c5"
			);
		},
	);
}
