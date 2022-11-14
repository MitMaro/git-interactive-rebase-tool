use view::assert_rendered_output;

use super::*;
use crate::{assert_results, process::Artifact, testutil::module_test};

#[test]
fn normal_mode_undo() {
	module_test(
		&["pick aaa c1"],
		&[Event::from(MetaEvent::ActionDrop), Event::from(StandardEvent::Undo)],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Undo))
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}"
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
			Event::from(StandardEvent::Undo),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      c2{Pad( )}"
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
			Event::from(StandardEvent::Undo),
			Event::from(StandardEvent::Redo),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_event(&mut module);
			let _ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Redo))
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}"
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
			Event::from(StandardEvent::Undo),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(StandardEvent::Redo),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      c2{Pad( )}"
			);
			assert_eq!(module.state, ListState::Visual);
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
			Event::from(StandardEvent::Undo),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_n_events(&mut module, 3);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Undo))
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      c2{Pad( )}"
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
			Event::from(StandardEvent::Undo),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_n_events(&mut module, 3);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Undo))
			);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionPick}pick   {Normal}bbb      c2"
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
			Event::from(StandardEvent::Undo),
			Event::from(StandardEvent::Redo),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal,Dimmed} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}bbb      c2{Pad( )}"
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
			Event::from(StandardEvent::Undo),
			Event::from(StandardEvent::Redo),
		],
		|mut test_context| {
			let mut module = List::new(&Config::new());
			let _ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected}{Normal} > {ActionPick}pick   {Normal}aaa      c1{Pad( )}",
				"{Normal}   {ActionDrop}drop   {Normal}bbb      c2"
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}
