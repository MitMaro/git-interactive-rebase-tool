use rstest::rstest;

use super::*;
use crate::{
	input::{KeyCode, KeyModifiers, MouseEvent},
	test_helpers::testers,
};

#[test]
fn edit_mode_passthrough_event() {
	testers::read_event(Event::from('p'), |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		module.state = ListState::Edit;
		assert_eq!(context.read_event(&module), Event::from('p'));
	});
}

#[test]
fn normal_mode_help() {
	testers::read_event(Event::from('?'), |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		module.normal_mode_help.set_active();
		assert_eq!(context.read_event(&module), Event::from(StandardEvent::Help));
	});
}

#[test]
fn visual_mode_help() {
	testers::read_event(Event::from('?'), |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		module.visual_mode_help.set_active();
		assert_eq!(context.read_event(&module), Event::from(StandardEvent::Help));
	});
}

#[test]
fn search() {
	testers::read_event(Event::from('p'), |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		module.search_bar.start_search(Some(""));
		assert_eq!(context.read_event(&module), Event::from('p'));
	});
}

#[rstest]
#[case::abort('q', StandardEvent::Abort)]
#[case::actionbreak('b', StandardEvent::ActionBreak)]
#[case::actiondrop('d', StandardEvent::ActionDrop)]
#[case::actionedit('e', StandardEvent::ActionEdit)]
#[case::actionfixup('f', StandardEvent::ActionFixup)]
#[case::actionpick('p', StandardEvent::ActionPick)]
#[case::actionreword('r', StandardEvent::ActionReword)]
#[case::actionsquash('s', StandardEvent::ActionSquash)]
#[case::edit('E', StandardEvent::Edit)]
#[case::forceabort('Q', StandardEvent::ForceAbort)]
#[case::forcerebase('W', StandardEvent::ForceRebase)]
#[case::insertline('I', StandardEvent::InsertLine)]
#[case::swapselecteddown('j', StandardEvent::SwapSelectedDown)]
#[case::swapselectedup('k', StandardEvent::SwapSelectedUp)]
#[case::openineditor('!', StandardEvent::OpenInEditor)]
#[case::rebase('w', StandardEvent::Rebase)]
#[case::showcommit('c', StandardEvent::ShowCommit)]
#[case::togglevisualmode('v', StandardEvent::ToggleVisualMode)]
fn default_events_single_char(#[case] binding: char, #[case] expected: StandardEvent) {
	testers::read_event(Event::from(binding), |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		assert_eq!(context.read_event(&module), Event::from(expected));
	});
}

#[rstest]
#[case::movecursordown(KeyCode::Down, StandardEvent::MoveCursorDown)]
#[case::movecursorpagedown(KeyCode::PageDown, StandardEvent::MoveCursorPageDown)]
#[case::movecursorend(KeyCode::End, StandardEvent::MoveCursorEnd)]
#[case::movecursorhome(KeyCode::Home, StandardEvent::MoveCursorHome)]
#[case::movecursorleft(KeyCode::Left, StandardEvent::MoveCursorLeft)]
#[case::movecursorright(KeyCode::Right, StandardEvent::MoveCursorRight)]
#[case::movecursorup(KeyCode::Up, StandardEvent::MoveCursorUp)]
#[case::movecursorpageup(KeyCode::PageUp, StandardEvent::MoveCursorPageUp)]
#[case::delete(KeyCode::Delete, StandardEvent::Delete)]
fn default_events_special(#[case] code: KeyCode, #[case] expected: StandardEvent) {
	testers::read_event(Event::from(code), |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		assert_eq!(context.read_event(&module), Event::from(expected));
	});
}

#[rstest]
#[case::abort('u', StandardEvent::FixupKeepMessage)]
#[case::abort('U', StandardEvent::FixupKeepMessageWithEditor)]
#[case::abort('p', StandardEvent::ActionPick)]
fn fixup_events(#[case] binding: char, #[case] expected: StandardEvent) {
	testers::read_event(Event::from(binding), |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		module.selected_line_action = Some(Action::Fixup);
		assert_eq!(context.read_event(&module), Event::from(expected));
	});
}

#[rstest]
#[case::abort('u')]
#[case::abort('U')]
fn fixup_events_with_non_fixpo_event(#[case] binding: char) {
	testers::read_event(Event::from(binding), |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		module.selected_line_action = Some(Action::Pick);
		assert_eq!(context.read_event(&module), Event::from(binding));
	});
}

#[test]
fn mouse_move_down() {
	testers::read_event(
		Event::from(MouseEvent {
			kind: MouseEventKind::ScrollDown,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::empty(),
		}),
		|mut context| {
			let mut module = create_list(&create_config(), context.take_todo_file());
			assert_eq!(context.read_event(&module), Event::from(StandardEvent::MoveCursorDown));
		},
	);
}

#[test]
fn mouse_move_up() {
	testers::read_event(
		Event::from(MouseEvent {
			kind: MouseEventKind::ScrollUp,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::empty(),
		}),
		|mut context| {
			let mut module = create_list(&create_config(), context.take_todo_file());
			assert_eq!(context.read_event(&module), Event::from(StandardEvent::MoveCursorUp));
		},
	);
}

#[test]
fn mouse_other() {
	let mouse_event = Event::from(MouseEvent {
		kind: MouseEventKind::Moved,
		column: 0,
		row: 0,
		modifiers: KeyModifiers::empty(),
	});
	testers::read_event(mouse_event, |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		assert_eq!(context.read_event(&module), mouse_event);
	});
}

#[test]
fn event_other() {
	testers::read_event(Event::None, |mut context| {
		let mut module = create_list(&create_config(), context.take_todo_file());
		assert_eq!(context.read_event(&module), Event::None);
	});
}
