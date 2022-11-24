use input::{KeyCode, KeyModifiers, MouseEvent};
use rstest::rstest;

use super::*;
use crate::testutil::read_event_test;

fn create_list_module() -> List {
	List::new(&Config::new())
}

#[test]
fn edit_mode_passthrough_event() {
	read_event_test(Event::from('p'), |mut context| {
		let mut module = create_list_module();
		module.state = ListState::Edit;
		assert_eq!(context.read_event(&mut module), Event::from('p'));
	});
}

#[test]
fn normal_mode_help() {
	read_event_test(Event::from('?'), |mut context| {
		let mut module = create_list_module();
		module.normal_mode_help.set_active();
		assert_eq!(context.read_event(&mut module), Event::from(StandardEvent::Help));
	});
}

#[test]
fn visual_mode_help() {
	read_event_test(Event::from('?'), |mut context| {
		let mut module = create_list_module();
		module.visual_mode_help.set_active();
		assert_eq!(context.read_event(&mut module), Event::from(StandardEvent::Help));
	});
}

#[test]
fn search() {
	read_event_test(Event::from('p'), |mut context| {
		let mut module = create_list_module();
		module.search_bar.start_search(Some(""));
		assert_eq!(context.read_event(&mut module), Event::from('p'));
	});
}

#[rstest]
#[case::abort('q', MetaEvent::Abort)]
#[case::actionbreak('b', MetaEvent::ActionBreak)]
#[case::actiondrop('d', MetaEvent::ActionDrop)]
#[case::actionedit('e', MetaEvent::ActionEdit)]
#[case::actionfixup('f', MetaEvent::ActionFixup)]
#[case::actionpick('p', MetaEvent::ActionPick)]
#[case::actionreword('r', MetaEvent::ActionReword)]
#[case::actionsquash('s', MetaEvent::ActionSquash)]
#[case::edit('E', MetaEvent::Edit)]
#[case::forceabort('Q', MetaEvent::ForceAbort)]
#[case::forcerebase('W', MetaEvent::ForceRebase)]
#[case::insertline('I', MetaEvent::InsertLine)]
#[case::swapselecteddown('j', MetaEvent::SwapSelectedDown)]
#[case::swapselectedup('k', MetaEvent::SwapSelectedUp)]
#[case::openineditor('!', MetaEvent::OpenInEditor)]
#[case::rebase('w', MetaEvent::Rebase)]
#[case::showcommit('c', MetaEvent::ShowCommit)]
#[case::togglevisualmode('v', MetaEvent::ToggleVisualMode)]
fn default_events_single_char(#[case] binding: char, #[case] expected: MetaEvent) {
	read_event_test(Event::from(binding), |mut context| {
		let mut module = create_list_module();
		assert_eq!(context.read_event(&mut module), Event::from(expected));
	});
}

// Move

#[rstest]
#[case::movecursordown(KeyCode::Down, MetaEvent::MoveCursorDown)]
#[case::movecursorpagedown(KeyCode::PageDown, MetaEvent::MoveCursorPageDown)]
#[case::movecursorend(KeyCode::End, MetaEvent::MoveCursorEnd)]
#[case::movecursorhome(KeyCode::Home, MetaEvent::MoveCursorHome)]
#[case::movecursorleft(KeyCode::Left, MetaEvent::MoveCursorLeft)]
#[case::movecursorright(KeyCode::Right, MetaEvent::MoveCursorRight)]
#[case::movecursorup(KeyCode::Up, MetaEvent::MoveCursorUp)]
#[case::movecursorpageup(KeyCode::PageUp, MetaEvent::MoveCursorPageUp)]
#[case::delete(KeyCode::Delete, MetaEvent::Delete)]
fn default_events_special(#[case] code: KeyCode, #[case] expected: MetaEvent) {
	read_event_test(Event::from(code), |mut context| {
		let mut module = create_list_module();
		assert_eq!(context.read_event(&mut module), Event::from(expected));
	});
}

#[test]
fn mouse_move_down() {
	read_event_test(
		Event::from(MouseEvent {
			kind: MouseEventKind::ScrollDown,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::empty(),
		}),
		|mut context| {
			let mut module = create_list_module();
			assert_eq!(context.read_event(&mut module), Event::from(MetaEvent::MoveCursorDown));
		},
	);
}

#[test]
fn mouse_move_up() {
	read_event_test(
		Event::from(MouseEvent {
			kind: MouseEventKind::ScrollUp,
			column: 0,
			row: 0,
			modifiers: KeyModifiers::empty(),
		}),
		|mut context| {
			let mut module = create_list_module();
			assert_eq!(context.read_event(&mut module), Event::from(MetaEvent::MoveCursorUp));
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
	read_event_test(mouse_event, |mut context| {
		let mut module = create_list_module();
		assert_eq!(context.read_event(&mut module), mouse_event);
	});
}

#[test]
fn event_other() {
	read_event_test(Event::None, |mut context| {
		let mut module = create_list_module();
		assert_eq!(context.read_event(&mut module), Event::None);
	});
}
